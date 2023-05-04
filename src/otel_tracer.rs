use opentelemetry::{Key, KeyValue};
use opentelemetry_api::trace::TraceError;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace, Resource};

pub const APP_NAME: &str = "traced_cli";

pub fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
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
pub const LEMONS_KEY: Key = Key::from_static_str("lemons");
pub const ANOTHER_KEY: Key = Key::from_static_str("ex.com/another");
