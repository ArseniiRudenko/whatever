use std::fs::File;
use std::io::{BufReader, Read};
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{Item, read_all};
async fn index(_req: HttpRequest) -> impl Responder {
    "Hello."
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //load env file
    dotenvy::dotenv().unwrap();

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

    let bind_to= dotenvy::var("bind.address")
        .unwrap_or(String::from("127.0.0.1:8080"));

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind_rustls(bind_to, config)?
        .run()
        .await
}