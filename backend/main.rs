use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use living_hash::{LivingHash, SpongeState};

mod living_hash;

#[derive(Serialize, Deserialize)]
struct AbsorbRequest {
    input: String,
}

#[derive(Serialize, Deserialize)]
struct SqueezeRequest {
    length: usize,
}

#[derive(Serialize, Deserialize)]
struct TraceResponse {
    trace: Vec<SpongeState>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let engine = web::Data::new(tokio::sync::Mutex::new(LivingHash::new()));

    println!("Starting Living Hash Backend at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(engine.clone())
            .route("/absorb", web::post().to(absorb))
            .route("/squeeze", web::post().to(squeeze))
            .route("/trace", web::get().to(get_trace))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn absorb(
    engine: web::Data<tokio::sync::Mutex<LivingHash>>,
    req: web::Json<AbsorbRequest>,
) -> impl Responder {
    let mut engine = engine.lock().await;
    engine.absorb(req.input.as_bytes());
    "Input absorbed successfully"
}

async fn squeeze(
    engine: web::Data<tokio::sync::Mutex<LivingHash>>,
    req: web::Json<SqueezeRequest>,
) -> impl Responder {
    let mut engine = engine.lock().await;
    let output = engine.squeeze(req.length);
    format!("Output: {:?}", output)
}

async fn get_trace(
    engine: web::Data<tokio::sync::Mutex<LivingHash>>,
) -> impl Responder {
    let engine = engine.lock().await;
    web::Json(TraceResponse {
        trace: engine.get_trace().to_vec(),
    })
}