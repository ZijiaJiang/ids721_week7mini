use actix_web::{App, HttpServer};
use mini7::{index, submit};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(submit))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
