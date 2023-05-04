use opentelemetry::Key;
use opentelemetry_api::global;
use opentelemetry_api::global::shutdown_tracer_provider;
use opentelemetry_api::trace::{TraceContextExt, Tracer};
use otel_tracer::{init_tracer, ANOTHER_KEY, LEMONS_KEY, APP_NAME};
use std::error::Error;

mod otel_tracer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // TODO: configure otel collector through env variables for API key.

    // By binding the result to an unused variable, the lifetime of the variable
    // matches the containing block, reporting traces and metrics during the whole
    // execution.
    let _ = init_tracer()?;

    let tracer = global::tracer(format!("{}_run", APP_NAME));

    tracer.in_span("operation", |cx| {
        let span = cx.span();
        span.add_event(
            "Nice operation!".to_string(),
            vec![Key::new("bogons").i64(100)],
        );
        span.set_attribute(ANOTHER_KEY.string("yes"));

        tracer.in_span("Sub operation...", |cx| {
            let span = cx.span();
            span.set_attribute(LEMONS_KEY.string("five"));

            span.add_event("Sub span event", vec![]);
        });
    });

    shutdown_tracer_provider();

    Ok(())
}
