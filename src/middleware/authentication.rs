use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;

pub enum Auth{
    User(User),
    Anonymous
}

impl FromRequest for Auth{
    type Error = ();
    type Future = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        todo!()
    }
}