use std::future::{Future, Ready, ready};
use std::pin::Pin;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Either, Error, http, HttpResponse
};

pub struct RedirectSchemeService<S> {
    pub service: S,
    pub temporary: bool,
    pub replacements: Vec<(String, String)>
}

impl<S> Service<ServiceRequest> for RedirectSchemeService<S>
    where
        S: Service<ServiceRequest, Response = ServiceResponse, Error = Error>,
        S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        if req.connection_info().scheme() == "https"
        {
            Box::pin( self.service.call(req))
        } else {
            let host = req.connection_info().host().to_owned();
            let uri = req.uri().to_owned();
            let mut url = format!("https://{}{}", host, uri);
            for (s1, s2) in self.replacements.iter() {
                url = url.replace(s1, s2);
            }
            Box::pin(ready(Ok(req.into_response(
                if self.temporary {
                    HttpResponse::TemporaryRedirect()
                } else {
                    HttpResponse::MovedPermanently()
                }
                .insert_header((http::header::LOCATION, url))
                .finish()
            ))))
        }
    }
}
