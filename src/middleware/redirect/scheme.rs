use std::future::{Ready, ready};
use super::service::RedirectSchemeService;
use actix_web::body::BoxBody;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;

/// Middleware for `actix-web` which redirects between `http` and `https` requests with optional url
/// string replacements.
///
/// ## Usage
/// ```
/// extern crate actix_web_middleware_redirect_scheme;
///
/// use actix_web::{App, web, HttpResponse};
/// use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
///
/// App::new()
///     .wrap(RedirectSchemeBuilder::new().temporary().build())
///     .route("/", web::get().to(|| HttpResponse::Ok()
///                                     .content_type("text/plain")
///                                     .body("Always HTTPS!")));
/// ```
#[derive(Default, Clone)]
pub struct RedirectScheme {
    // Temporary redirect (true: 307 Temporary Redirect, false: 301 Moved Permanently)
    pub temporary: bool,
    // List of string replacements
    pub replacements: Vec<(String, String)>,
    // List of paths that are not redirected
    pub ignore_paths: Vec<String>,
}

impl<S> Transform<S, ServiceRequest> for RedirectScheme
    where
        S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
        S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = RedirectSchemeService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RedirectSchemeService {
            service,
            temporary: self.temporary,
            replacements: self.replacements.clone()
        }))
    }
}


