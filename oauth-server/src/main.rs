use actix_web::{App, HttpResponse, HttpServer, Responder};

#[get("/auth")]
async fn authorize() -> impl Responder {



    todo!()
}



#[actix::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
