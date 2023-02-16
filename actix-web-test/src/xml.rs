use actix_web::body::EitherBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse, Responder};
use serde::{Serialize, Serializer};

pub struct Xml<T>(pub T);

impl<T> Serialize for Xml<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<T> Responder for Xml<T>
where
    T: Serialize,
{
    type Body = EitherBody<String, String>;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        return match serde_xml_rs::to_string(&self) {
            Ok(val) => {
                match HttpResponse::Ok()
                    .insert_header(ContentType::xml())
                    .message_body(val)
                {
                    Ok(respond) => respond.map_into_left_body(),
                    Err(e) => HttpResponse::BadRequest()
                        .message_body(e.to_string())
                        .unwrap()
                        .map_into_right_body(),
                }
            }
            Err(e) => HttpResponse::BadRequest()
                .message_body(e.to_string())
                .unwrap()
                .map_into_right_body(),
        };
    }
}
