use std::time::SystemTime;

use opentelemetry::global::ObjectSafeSpan;
use opentelemetry::trace::{SpanBuilder, Tracer};
use opentelemetry::KeyValue;
use tracing::{info_span, Span};

use self::github::context::GithubContext;
use self::otel::TRACER_NAME;

mod github;

mod otel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let _guard = otel::setup_tracing_subscriber()?;

    let tracer = opentelemetry::global::tracer(TRACER_NAME);

//     let _span = &tracer.build(SpanBuilder {
//     name: "example-span-name".into(),
//     span_kind: Some(SpanKind::Server),
//     ..Default::default()
// });
    // tracer
    // .span_builder("example-span-name")
    // .start(&tracer);
    // .with_kind(SpanKind::Server)
    let span_builder = SpanBuilder::from_name("main").with_start_time(SystemTime::now());
    let mut _span = tracer.build(span_builder);
    _span.set_attribute(KeyValue::new("test", "tester"));

    // let root_span = info_span!("test");
    // let _gaurd = root_span.enter();

    let fruit = github::command::get_input("fruit")?;

    let context = GithubContext::new();

    println!("{context:#?}");

    println!("Hello, {fruit}!");

    debug!("Fruit was {fruit}");
    notice!("Fruit was {fruit}");
    warn!("Fruit was {fruit}");
    error!("Fruit was {fruit}");

    _span.end();
    // This is needed to export all remaining spans before exiting.
    opentelemetry::global::shutdown_tracer_provider();

    Ok(())
}
