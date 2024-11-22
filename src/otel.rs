use opentelemetry::{
    global::{self},
    trace::TracerProvider,
    KeyValue,
};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{runtime, Resource};
use opentelemetry_semantic_conventions::{
    resource::{DEPLOYMENT_ENVIRONMENT_NAME, SERVICE_NAME, SERVICE_VERSION},
    SCHEMA_URL,
};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter};

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

pub fn setup_tracing_subscriber() -> anyhow::Result<OtelGuard> {
    let filter = if std::env::var("RUST_LOG").is_ok() {
        EnvFilter::builder().from_env_lossy()
    } else {
        "info"
            .to_string()
            .parse()
            .expect("valid EnvFilter value can be parsed")
    };

    println!("Using filter: {}", filter);

    let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(
            SpanExporter::builder()
                .with_tonic()
                .with_endpoint("grpc://localhost:4317")
                .build()?,
            runtime::Tokio,
        )
        .with_config(opentelemetry_sdk::trace::Config::default().with_resource(resource()))
        .build();

    global::set_tracer_provider(tracer_provider.clone());

    tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(filter) // Read global subscriber filter from `RUST_LOG`
            .with(tracing_subscriber::fmt::layer())
            .with(OpenTelemetryLayer::new(tracer_provider.tracer(TRACER_NAME))),
    )
    .unwrap();

    Ok(OtelGuard { tracer_provider })
}

pub struct OtelGuard {
    tracer_provider: opentelemetry_sdk::trace::TracerProvider,
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        println!("Dropping OtelGuard!");
        println!("Shutting down TracerProvider!");
        self.tracer_provider.shutdown().expect("TracerProvider should shutdown properly");
    }
}
