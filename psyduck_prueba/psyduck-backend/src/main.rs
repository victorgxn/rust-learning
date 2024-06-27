use actix_web::{post, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use serde::Deserialize;
use redis::Commands;
use std::env;
use dotenv::dotenv;

#[derive(Deserialize)]
struct FormData {
    email: String,
    data: Vec<String>,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/submit")]
async fn submit_form(form: web::Json<FormData>) -> impl Responder {
    println!("Received request with email: {}", form.email);

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = redis::Client::open(redis_url).expect("Invalid Redis URL");
    let mut con = client.get_connection().expect("Failed to connect to Redis");

    let data_json = serde_json::to_string(&form.data).expect("Failed to serialize data");

    let _: () = con.set(&form.email, data_json).expect("Failed to save to Redis");

    HttpResponse::Ok().body("Data saved successfully")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(submit_form)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
