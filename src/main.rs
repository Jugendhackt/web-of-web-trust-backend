use actix_web::web::Data;
use actix_web::{dev, http, middleware, web, App, HttpServer};
use actix_web_opentelemetry::{RequestMetrics, RequestTracing};
use opentelemetry::{
    global, sdk::propagation::TraceContextPropagator, sdk::trace::Tracer as SdkTracer,
    trace::TraceError,
};
use opentelemetry_jaeger::Propagator;
use std::io;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;

mod core;
mod db;
mod domains;
mod ruegen;

#[cfg(test)]
mod tests;

use crate::core::config::CONFIG;

fn init_tracer() -> Result<SdkTracer, TraceError> {
    opentelemetry_jaeger::new_pipeline()
        .with_service_name(&CONFIG.tracing.service_name)
        .install_batch(opentelemetry::runtime::TokioCurrentThread)
}

async fn index(username: actix_web::web::Path<String>) -> String {
    greet_user(username.as_ref())
}

#[tracing::instrument]
fn greet_user(username: &str) -> String {
    tracing::info!("preparing to greet user");
    format!("Hello {}", username)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Start a new jaeger trace pipeline
    global::set_text_map_propagator(Propagator::new());

    let tracer = init_tracer().expect("Failed to initialize tracer.");

    // Start an (optional) otel prometheus metrics pipeline
    let metrics_exporter = opentelemetry_prometheus::exporter().init();

    // Start an open telemetry jaeger trace pipeline
    global::set_text_map_propagator(TraceContextPropagator::new());

    let metrics_route =
        |req: &dev::ServiceRequest| req.path() == "/metrics" && req.method() == http::Method::GET;

    let request_metrics = RequestMetrics::new(
        global::meter(&CONFIG.tracing.meter),
        Some(metrics_route),
        Some(metrics_exporter),
    );

    // Initialize `tracing` using `opentelemetry-tracing` and configure logging
    Registry::default()
        .with(tracing_subscriber::EnvFilter::new("INFO"))
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    let pool = db::util::init_pool().await;

    match sqlx::migrate!().run(&pool).await {
        Ok(_) => tracing::info!("Applied Migrations"),
        Err(e) => {
            eprintln!(
                "Failed to apply migrations. Stopping boot since migrations are required. ({})",
                e
            );
            std::process::exit(1);
        }
    }

    HttpServer::new(move || {
        App::new()
            .wrap(RequestTracing::new())
            .wrap(request_metrics.clone())
            .wrap(middleware::Compress::new(http::ContentEncoding::Zstd))
            .service(web::resource("/greet/{username}").to(index))
            .configure(core::routes::services)
            .service(
                web::scope("/v1")
                    .app_data(Data::new(pool.clone()))
                    .service(web::scope("/domains").configure(domains::routes::services))
                    .service(web::scope("/ruegen").configure(ruegen::routes::services)),
            )
    })
    .bind(format!("{}:{}", CONFIG.api.host, CONFIG.api.port))?
    .run()
    .await?;

    // Ensure all spans have been reported
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
