use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::Payload;
use crate::models::user::UserClaim;

pub enum Auth{
    User(UserClaim),
    Anonymous
}

impl FromRequest for Auth{
    type Error = ();
    type Future = ();

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        todo!()
    }
}