mod middleware;
mod routes;
mod models;
use std::fs::File;
use std::io::{BufReader, Read};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{Item, read_all};
use crate::middleware::redirect::RedirectSchemeBuilder;


async fn index(_req: HttpRequest) -> impl Responder {
    "Hello."
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //load env file
    dotenvy::dotenv().ok();
    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout cert.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let pem_file_name= dotenvy::var("tls.cert")
        .unwrap_or(String::from("cert.pem"));

    //pem certificate and key file
    let mut pem_file_reader = BufReader::new(File::open(pem_file_name).unwrap());
    //read pem file
    let cert_chain = read_all(&mut pem_file_reader).unwrap();
    //take only certificates
    let certs = cert_chain.iter().filter_map(|item|
            match item {
                Item::X509Certificate(cert) => Some(Certificate(cert.clone())),
                _=> None
            }).collect();
    //take keys
    let keys: Vec<&Vec<u8>> = cert_chain.iter().filter_map(|item|
        match item {
            Item::PKCS8Key(key) => Some(key),
            _ => None
        }).collect();

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs,PrivateKey(keys[0].clone()))
        .unwrap();

    let host = dotenvy::var("bind.host")
        .unwrap_or("localhost".to_string());
    let port = ":".to_string() + & dotenvy::var("bind.port")
        .unwrap_or("8080".to_string());
    let tls_port =":".to_string() + & dotenvy::var("bind.tls_port")
        .unwrap_or("8443".to_string());

    let bind_host = host.clone()+&port;
    let bind_tls_host = host.clone()+&tls_port;

    let http= HttpServer::new(move ||
        App::new().wrap(
            RedirectSchemeBuilder::new()
            .replacements(&[(&port,&tls_port)])
            .build()
        )
    )
    .bind(bind_host)?.run();

    let https=HttpServer::new(||
        App::new()
        .route("/", web::get().to(index))
    )
    .bind_rustls(bind_tls_host, config)?.run();

    let result=futures::join!(https,http);
    match result.0 {
        Ok(_) => result.1,
        Err(_) => result.0
    }
}