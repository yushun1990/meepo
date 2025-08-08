use opentelemetry::trace::TracerProvider;
use serde::Deserialize;
use std::{str::FromStr, sync::OnceLock};

use opentelemetry::{
    KeyValue,
    global::{self},
};
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::{Resource, trace::SdkTracerProvider};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Deserialize)]
pub struct TracingConfig {
    pub filter_str: Option<String>,
    pub enable_otlp: Option<bool>,
    pub otlp_endpoint: Option<String>,
    pub otlp_protocol: Option<String>,
}

pub trait Tracing {
    fn config(&self) -> TracingConfig;
    fn setup(&self, name: &str, version: &str) -> anyhow::Result<Option<SdkTracerProvider>>;
}

impl Tracing for TracingConfig {
    fn config(&self) -> TracingConfig {
        self.clone()
    }

    fn setup(&self, name: &str, version: &str) -> anyhow::Result<Option<SdkTracerProvider>> {
        let mut filter_str = "info,opentelemetry=info";
        if let Some(fs) = self.filter_str.as_ref() {
            filter_str = fs;
        }

        let filter = EnvFilter::from_str(filter_str)?;

        let layer = tracing_subscriber::registry()
            .with(filter)
            .with(fmt::Layer::default());

        if let Some(enable_otlp) = self.enable_otlp
            && enable_otlp
        {
            let tracing_provider =
                self.init_traces(Self::get_resource(name.to_string(), version.to_string()))?;

            global::set_tracer_provider(tracing_provider.clone());

            let tracer = tracing_provider.tracer(name.to_string());
            let otel_layer = OpenTelemetryLayer::new(tracer);
            layer.with(otel_layer).init();

            return Ok(Some(tracing_provider));
        }

        layer.init();
        return Ok(None);
    }
}

impl TracingConfig {
    // Sets up the OTLP exporter and builds the SdkTraceProvider
    fn init_traces(&self, resource: Resource) -> anyhow::Result<SdkTracerProvider> {
        let protocol = match self.otlp_protocol.clone() {
            Some(protocol) => match protocol.as_str() {
                "http/json" => Protocol::HttpJson,
                "http/protobuf" => Protocol::HttpBinary,
                other => anyhow::bail!("Unsupported OTLP protcol: {other}"),
            },
            _ => anyhow::bail!("Must provid OELP protocol!"),
        };

        if let Some(endpoint) = self.otlp_endpoint.clone() {
            let exporter = opentelemetry_otlp::HttpExporterBuilder::default()
                .with_endpoint(endpoint)
                .with_protocol(protocol)
                .build_span_exporter()?;

            Ok(SdkTracerProvider::builder()
                .with_batch_exporter(exporter)
                .with_resource(resource)
                .build())
        } else {
            anyhow::bail!("Must provid OELP endpoint!")
        }
    }

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

    pub fn shutdown_opentelemetry(tracing_provider: SdkTracerProvider) -> anyhow::Result<()> {
        tracing_provider.shutdown()?;
        Ok(())
    }
}
