use tracing::info_span;

use self::github::context::GithubContext;

mod github;

mod otel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let _otel_guard = otel::setup_tracing_subscriber()?;

    // let tracer = opentelemetry::global::tracer(TRACER_NAME);
    // let span_builder = SpanBuilder::from_name("main").with_start_time(SystemTime::now());
    // let mut _span = tracer.build(span_builder);
    // _span.set_attribute(KeyValue::new("test", "tester"));

    let root_span = info_span!("test");
    let _span_gaurd = root_span.enter();

    let fruit = github::command::get_input("fruit")?;

    let context = GithubContext::new();

    println!("{context:#?}");

    println!("Hello, {fruit}!");

    debug!("Fruit was {fruit}");
    notice!("Fruit was {fruit}");
    warn!("Fruit was {fruit}");
    error!("Fruit was {fruit}");

    // _span.end();

    Ok(())
}
