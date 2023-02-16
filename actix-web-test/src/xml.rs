use actix_web::body::EitherBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
use serde::Serialize;
use serde_xml_rs::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum XmlError {
    SerializingError(Error),
}

impl Display for XmlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "XML serialize error")
    }
}

impl ResponseError for XmlError {}

#[derive(Serialize)]
pub struct Xml<T>(pub T);

impl<T> Responder for Xml<T>
where
    T: Serialize,
{
    type Body = EitherBody<String>;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        return match serde_xml_rs::to_string(&self) {
            Ok(val) => {
                match HttpResponse::Ok()
                    .insert_header(ContentType::xml())
                    .message_body(val)
                {
                    Ok(respond) => respond.map_into_left_body(),
                    Err(e) => HttpResponse::from_error(e).map_into_right_body(),
                }
            }
            Err(e) => HttpResponse::from_error(XmlError::SerializingError(e)).map_into_right_body(),
        };
    }
}
