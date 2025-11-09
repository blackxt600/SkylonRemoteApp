mod bike_controller;
mod training_program;

use actix_web::{get, post, put, delete, web, App, HttpServer, Responder};
use actix_files as fs;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::process::Command;
use bike_controller::BikeController;
use training_program::{TrainingProgram, TrainingInterval};

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

// ===== Endpoints pour la gestion des programmes d'entraînement =====

#[derive(Deserialize)]
struct CreateProgramRequest {
    id: String,
    name: String,
    description: Option<String>,
    intervals: Vec<TrainingInterval>,
}

/// Crée un nouveau programme d'entraînement
#[post("/program")]
async fn create_program(
    req: web::Json<CreateProgramRequest>,
    data: web::Data<Arc<BikeController>>,
) -> impl Responder {
    let program = TrainingProgram {
        id: req.id.clone(),
        name: req.name.clone(),
        description: req.description.clone(),
        intervals: req.intervals.clone(),
    };

    match data.create_program(program).await {
        Ok(_) => actix_web::HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Programme créé avec succès"
        })),
        Err(e) => actix_web::HttpResponse::BadRequest().body(e.to_string()),
    }
}

/// Liste tous les programmes
#[get("/programs")]
async fn list_programs(data: web::Data<Arc<BikeController>>) -> impl Responder {
    let programs = data.list_programs().await;
    web::Json(programs)
}

/// Obtient un programme spécifique
#[get("/program/{id}")]
async fn get_program(id: web::Path<String>, data: web::Data<Arc<BikeController>>) -> impl Responder {
    match data.get_program(&id).await {
        Some(program) => actix_web::HttpResponse::Ok().json(program),
        None => actix_web::HttpResponse::NotFound().body("Programme introuvable"),
    }
}

/// Met à jour un programme existant
#[put("/program/{id}")]
async fn update_program(
    id: web::Path<String>,
    req: web::Json<CreateProgramRequest>,
    data: web::Data<Arc<BikeController>>,
) -> impl Responder {
    let program = TrainingProgram {
        id: id.into_inner(),
        name: req.name.clone(),
        description: req.description.clone(),
        intervals: req.intervals.clone(),
    };

    match data.update_program(program).await {
        Ok(_) => actix_web::HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Programme mis à jour"
        })),
        Err(e) => actix_web::HttpResponse::BadRequest().body(e.to_string()),
    }
}

/// Supprime un programme
#[delete("/program/{id}")]
async fn delete_program(id: web::Path<String>, data: web::Data<Arc<BikeController>>) -> impl Responder {
    match data.delete_program(&id).await {
        Ok(_) => actix_web::HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Programme supprimé"
        })),
        Err(e) => actix_web::HttpResponse::BadRequest().body(e.to_string()),
    }
}

/// Démarre un programme
#[post("/program/{id}/start")]
async fn start_program(
    id: web::Path<String>,
    data: web::Data<Arc<BikeController>>,
) -> impl Responder {
    match data.start_program(&id).await {
        Ok(_) => {
            // Démarrer la boucle de mise à jour du programme
            data.get_ref().clone().start_program_loop();

            actix_web::HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Programme démarré"
            }))
        }
        Err(e) => actix_web::HttpResponse::BadRequest().body(e.to_string()),
    }
}

/// Arrête le programme en cours
#[post("/program/stop")]
async fn stop_program(data: web::Data<Arc<BikeController>>) -> impl Responder {
    match data.stop_program().await {
        Ok(_) => actix_web::HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Programme arrêté"
        })),
        Err(e) => actix_web::HttpResponse::BadRequest().body(e.to_string()),
    }
}

/// Obtient l'état du programme actif
#[get("/program/active")]
async fn get_active_program(data: web::Data<Arc<BikeController>>) -> impl Responder {
    match data.get_active_program().await {
        Some(state) => actix_web::HttpResponse::Ok().json(state),
        None => actix_web::HttpResponse::Ok().json(serde_json::json!({
            "active": false,
            "message": "Aucun programme en cours"
        })),
    }
}

// ===== Endpoints pour la gestion du système =====

/// Arrête le Raspberry Pi (shutdown)
#[post("/system/shutdown")]
async fn shutdown_system() -> impl Responder {
    println!("🔴 Demande d'arrêt du système reçue");

    // Exécuter la commande shutdown en arrière-plan pour permettre à la réponse HTTP d'être envoyée
    tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("🔴 Exécution de la commande shutdown...");
        let _ = Command::new("sudo")
            .arg("shutdown")
            .arg("-h")
            .arg("now")
            .spawn();
    });

    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Arrêt du système dans 2 secondes..."
    }))
}

/// Redémarre le Raspberry Pi (reboot)
#[post("/system/reboot")]
async fn reboot_system() -> impl Responder {
    println!("🔄 Demande de redémarrage du système reçue");

    // Exécuter la commande reboot en arrière-plan pour permettre à la réponse HTTP d'être envoyée
    tokio::spawn(async {
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("🔄 Exécution de la commande reboot...");
        let _ = Command::new("sudo")
            .arg("reboot")
            .spawn();
    });

    actix_web::HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": "Redémarrage du système dans 2 secondes..."
    }))
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
            // Endpoints pour les programmes d'entraînement
            .service(create_program)
            .service(list_programs)
            .service(get_program)
            .service(update_program)
            .service(delete_program)
            .service(start_program)
            .service(stop_program)
            .service(get_active_program)
            // Endpoints pour la gestion du système
            .service(shutdown_system)
            .service(reboot_system)
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
