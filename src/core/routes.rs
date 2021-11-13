use super::types::APIResponse;
use actix_web::{get, web::ServiceConfig, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct APIVersion {
    versions: [&'static str; 1],
    notes: &'static str,
}

#[get("/version")]
async fn version() -> APIResponse {
    Ok(HttpResponse::Ok().json(APIVersion {
        versions: ["0.1.0b0"],
        notes: "Experimental web-of-web-trust backend",
    }))
}

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(version);
}
