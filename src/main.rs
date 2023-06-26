use actix_cors::Cors;
use actix_web::{http::header, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;

pub const APPLICATION_JSON: &str = "application/json";

#[derive(Deserialize)]
struct ContactFields {
    to: String,
    message: String,
}

#[post("/contact")]
async fn contact(fields: web::Json<ContactFields>) -> impl Responder {
    sleep(Duration::from_secs(2)).await;

    let body = json!({
        "pass": true,
        "data": format!("Message of length {} sent to: {}", fields.message.clone().len(), fields.to.clone())
    });

    println!("{}", body);

    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .body(body.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::CONTENT_TYPE, header::ACCEPT])
            .allowed_origin("http://127.0.0.1:8080")
            .supports_credentials();
        //.send_wildcard();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(contact)
    })
    .bind(("127.0.0.1", 8888))?
    .run()
    .await
}
