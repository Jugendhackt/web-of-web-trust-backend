use crate::{core::types::APIResponse, ruegen::responses::RuegenInformation};
use actix_web::{
    get, post,
    web::{Json, ServiceConfig},
};

#[get("/fetch")]
#[tracing::instrument]
pub async fn fetch() -> APIResponse {
    todo!()
}

#[post("/update")]
#[tracing::instrument]
pub async fn update(_ruege: Json<RuegenInformation>) -> APIResponse {
    todo!()
}

pub fn services(cfg: &mut ServiceConfig) {
    cfg.service(update);
    cfg.service(fetch);
}
