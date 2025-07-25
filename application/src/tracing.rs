use std::{str::FromStr, sync::OnceLock};

use opentelemetry::{
    KeyValue,
    global::{self},
};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::{Resource, trace::SdkTracerProvider};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Tracing;

// Initializes a global Resource containing service name and version.
// Use OnceLock  to ensure the resource is created only once;
fn get_resource(name: String, version: String) -> Resource {
    static RESOURCE: OnceLock<Resource> = OnceLock::new();

    RESOURCE
        .get_or_init(|| {
            Resource::builder()
                .with_service_name(name)
                .with_attribute(KeyValue::new("service.version", version))
                .build()
        })
        .clone()
}

// Sets up the OTLP exporter and builds the SdkTraceProvider
fn init_traces(
    endpoint: &str,
    protocol: &str,
    resource: Resource,
) -> anyhow::Result<SdkTracerProvider> {
    let protocol = match protocol {
        "http/json" => Protocol::HttpJson,
        "http/protobuf" => Protocol::HttpBinary,
        other => anyhow::bail!("Unsupported OTLP protocol: {}", other),
    };

    let exporter = opentelemetry_otlp::HttpExporterBuilder::default()
        .with_endpoint(endpoint)
        .with_protocol(protocol)
        .build_span_exporter()?;

    Ok(SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource)
        .build())
}

pub fn setup_tracing(
    name: String,
    version: String,
    tracing: Tracing,
) -> anyhow::Result<Option<SdkTracerProvider>> {
    let mut filter_str = "info,opentelemetry=info";
    if tracing.filter_str.is_some() {
        filter_str = tracing.filter_str.unwrap().as_str();
    }

    let filter = EnvFilter::from_str(filter_str)?;

    let layer = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default());

    if !tracing.enable_otlp {
        layer.init();
        return Ok(None);
    }

    let tracing_provider: SdkTracerProvider = init_traces(
        tracing.otlp_endpoint.unwrap().as_str(),
        tracing.otlp_protocol.unwrap().as_str(),
        get_resource(name, version),
    )?;

    global::set_tracer_provider(tracing_provider.clone());

    let tracer = tracing_provider.tracer(env!("CARGO_PKG_NAME"));
    let otel_layer = OpenTelemetryLayer::new(tracer);
    layer.with(otel_layer).init();

    Ok(Some(tracing_provider))
}
