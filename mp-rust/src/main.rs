use actix_web::{App, HttpServer, HttpResponse, Responder, post};
//use reqwest::Client;
use serde_json::json;

#[post("/marco")]
async fn marco() -> impl Responder {
    println!("Received 'Marco'");
    
    //let client = Client::new();
    //let _ = client.post("http://service1:5000/marco").send().await;
    //let _ = client.post("http://service3:5000/marco").send().await;

    HttpResponse::Ok().json(json!({"message": "Polo"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("##################################");
    println!("Starting marco-polo @ 127.0.1:5001");
    println!("##################################");

    HttpServer::new(|| {
        App::new()
            .service(marco)
    })
    .bind("127.0.0.1:5001")?
    .run()
    .await
}
