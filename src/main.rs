use actix_web::{App, HttpResponse, HttpServer, Responder, get, post};

#[post("/payments")]
async fn payments() -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/payments-summary")]
async fn payments_summary() -> impl Responder {
    HttpResponse::Ok().body("Payments summary works")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(payments).service(payments_summary))
        .bind(("127.0.0.1", 9999))?
        .run()
        .await
}
