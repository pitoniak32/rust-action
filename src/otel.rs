use opentelemetry::{
    global::{self},
    trace::TracerProvider,
    KeyValue,
};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{
    runtime,
    trace::{RandomIdGenerator, Sampler, Tracer},
    Resource,
};
use opentelemetry_semantic_conventions::{
    resource::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub const TRACER_NAME: &'static str = "rust-action";

// Create a Resource that captures information about the entity for which telemetry is recorded.
fn resource() -> Resource {
    Resource::from_schema_url(
        [
            KeyValue::new(SERVICE_NAME, env!("CARGO_PKG_NAME")),
            KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
            KeyValue::new(DEPLOYMENT_ENVIRONMENT_NAME, "develop"),
        ],
        SCHEMA_URL,
    )
}

pub fn setup_tracing_subscriber() -> anyhow::Result<()> {
    // let filter = if std::env::var("RUST_LOG").is_ok() {
    //     EnvFilter::builder().from_env_lossy()
    // } else {
    //     "info"
    //         .to_string()
    //         .parse()
    //         .expect("valid EnvFilter value can be parsed")
    // };

    // let exporter = SpanExporter::builder()
    //     .with_tonic()
    //     .with_endpoint("localhost:4317")
    //     .build()?;
    let exporter = opentelemetry_stdout::SpanExporter::default(); 
    let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_simple_exporter(exporter)
        .with_config(opentelemetry_sdk::trace::Config::default().with_resource(resource()))
        .build();

    global::set_tracer_provider(tracer_provider.clone());

    let tracer = tracer_provider.tracer(TRACER_NAME);

    tracing_subscriber::Registry::default()
        // .with(filter)
        .with(tracing_subscriber::fmt::layer()) // Setup logging layer
        .with(OpenTelemetryLayer::new(tracer))
        .init();

    Ok(())
}
