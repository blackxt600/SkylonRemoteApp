mod bike_controller;

use actix_web::{get, post, web, App, HttpServer, Responder};
use actix_files as fs;
use serde::Serialize;
use std::sync::Arc;
use bike_controller::BikeController;

#[derive(Serialize)]
struct BikeStatus {
    speed: f32,
    rpm: u16,
    power: u16,
    connected: bool,
}

#[get("/status")]
async fn status(data: web::Data<Arc<BikeController>>) -> impl Responder {
    let d = data.get_data().await;
    web::Json(BikeStatus {
        speed: d.speed,
        rpm: d.rpm,
        power: d.power,
        connected: d.connected,
    })
}

#[get("/power")]
async fn get_power(data: web::Data<Arc<BikeController>>) -> impl Responder {
    let power = data.get_power().await;
    web::Json(serde_json::json!({"power": power}))
}

#[post("/power/{level}")]
async fn set_power(level: web::Path<u16>, data: web::Data<Arc<BikeController>>) -> impl Responder {
    match data.set_power(*level).await {
        Ok(_) => actix_web::HttpResponse::Ok().body("Puissance mise à jour"),
        Err(e) => actix_web::HttpResponse::BadRequest().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Démarrage du serveur elliptique...");

    let bike_controller = BikeController::new().await
        .expect("Impossible d'initialiser le contrôleur");

    bike_controller.clone().start_polling(1); // mise à jour toutes les secondes

    println!("🌐 Serveur web démarré sur http://0.0.0.0:8080");
    println!("   Ouvrez http://localhost:8080 dans votre navigateur");
    println!();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(bike_controller.clone()))
            .service(status)
            .service(get_power)
            .service(set_power)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
