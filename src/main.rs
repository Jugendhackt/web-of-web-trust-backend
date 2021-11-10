use actix_web::{dev, http, middleware, App, HttpServer};
use actix_web_opentelemetry::RequestMetrics;
use opentelemetry::{
    global,
    sdk::propagation::TraceContextPropagator,
    sdk::trace::Tracer as SdkTracer,
    trace::{TraceContextExt, TraceError, Tracer},
    Key,
};
use opentelemetry_jaeger::Propagator;
use paperclip::actix::{
    api_v2_operation,
    // use this instead of actix_web::web
    web::{self, Json},
    Apiv2Schema,
    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt,
};
use std::io;

fn init_tracer() -> Result<SdkTracer, TraceError> {
    opentelemetry_jaeger::new_pipeline()
        .with_agent_endpoint("http://127.0.0.1:14268/api/traces")
        .with_service_name("trace-http-demo")
        .install_batch(opentelemetry::runtime::TokioCurrentThread)
}

#[api_v2_operation]
async fn index() -> &'static str {
    let tracer = global::tracer("request");
    tracer.in_span("index", |ctx| {
        ctx.span().set_attribute(Key::new("parameter").i64(10));
        "Index"
    })
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Start a new jaeger trace pipeline
    global::set_text_map_propagator(Propagator::new());

    let _tracer = init_tracer().expect("Failed to initialise tracer.");

    // Start an (optional) otel prometheus metrics pipeline
    let metrics_exporter = opentelemetry_prometheus::exporter().init();

    // Start an otel jaeger trace pipeline
    global::set_text_map_propagator(TraceContextPropagator::new());

    let metrics_route =
        |req: &dev::ServiceRequest| req.path() == "/metrics" && req.method() == http::Method::GET;

    let request_metrics = RequestMetrics::new(
        global::meter("backend"),
        Some(metrics_route),
        Some(metrics_exporter),
    );

    HttpServer::new(move || {
        App::new()
            .wrap_api()
            .wrap(request_metrics.clone())
            .wrap(middleware::Compress::new(http::ContentEncoding::Zstd))
            .service(web::resource("/users/{id}").to(index))
            .with_json_spec_at("/api/spec")
            .build()
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    // Ensure all spans have been reported
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
