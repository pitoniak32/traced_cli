use opentelemetry::{Key, KeyValue};
use opentelemetry_api::global;
use opentelemetry_api::global::shutdown_tracer_provider;
use opentelemetry_api::trace::TraceError;
use opentelemetry_api::trace::{TraceContextExt, Tracer};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};
use std::error::Error;

const APP_NAME: &str = "traced_cli";

fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("https://localhost:4317"),
        )
        .with_trace_config(
            sdktrace::config().with_resource(Resource::new(vec![KeyValue::new(
                opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                APP_NAME,
            )])),
        )
        .install_batch(runtime::Tokio)
}

// Used for consistent span keys.
const LEMONS_KEY: Key = Key::from_static_str("lemons");
const ANOTHER_KEY: Key = Key::from_static_str("ex.com/another");

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // TODO: configure otel collector through env variables for API key.

    // By binding the result to an unused variable, the lifetime of the variable
    // matches the containing block, reporting traces and metrics during the whole
    // execution.
    let _ = init_tracer()?;

    let tracer = global::tracer("ex.com/basic");

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
