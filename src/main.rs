use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::process::Command;

async fn greet(req: HttpRequest) -> impl Responder {
    let css = "<input type='submit'>";
    HttpResponse::Ok().content_type("text/html").body(css)
}

async fn start(req: HttpRequest) -> impl Responder {
    Command::new("andaman")
        .args(&["-s"])
        .spawn()
        .expect("failed to start");
    HttpResponse::Ok()
}

async fn stop(req: HttpRequest) -> impl Responder {
    Command::new("andaman").output().expect("failed to stop");
    HttpResponse::Ok()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/true", web::get().to(start))
            .route("/false", web::get().to(stop))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
