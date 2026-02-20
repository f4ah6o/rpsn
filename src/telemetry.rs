use opentelemetry::global;
use opentelemetry::trace::TracerProvider as _;
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::{self, SdkTracerProvider};
use opentelemetry_sdk::Resource;
use tracing_subscriber::filter::filter_fn;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Layer;
use tracing_subscriber::Registry;

pub struct TelemetryHandle {
    provider: Option<SdkTracerProvider>,
    enabled: bool,
}

impl TelemetryHandle {
    pub fn disabled() -> Self {
        Self {
            provider: None,
            enabled: false,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn shutdown(&mut self) {
        let Some(provider) = self.provider.take() else {
            return;
        };

        let _ = provider.force_flush();
        let _ = provider.shutdown();
    }
}

pub fn init_telemetry() -> TelemetryHandle {
    let endpoint = match std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT") {
        Ok(value) if !value.trim().is_empty() => value,
        _ => return TelemetryHandle::disabled(),
    };

    let service_name = std::env::var("OTEL_SERVICE_NAME")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| env!("CARGO_PKG_NAME").to_string());

    let resource = Resource::builder_empty()
        .with_attributes(vec![
            KeyValue::new("service.name", service_name),
            KeyValue::new("service.version", env!("CARGO_PKG_VERSION").to_string()),
        ])
        .build();

    let exporter = match opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(endpoint)
        .build()
    {
        Ok(exporter) => exporter,
        Err(err) => {
            eprintln!("Warning: failed to initialize OTLP exporter: {}", err);
            return TelemetryHandle::disabled();
        }
    };

    let provider = SdkTracerProvider::builder()
        .with_resource(resource)
        .with_sampler(parse_sampler_from_env())
        .with_batch_exporter(exporter)
        .build();

    let tracer = provider.tracer(env!("CARGO_PKG_NAME"));
    let otel_layer = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_filter(filter_fn(|metadata| metadata.target() == "rpsn.telemetry"));
    let subscriber = Registry::default().with(otel_layer);

    if tracing::subscriber::set_global_default(subscriber).is_err() {
        eprintln!("Warning: tracing subscriber already initialized; OTLP tracing disabled");
        return TelemetryHandle::disabled();
    }

    global::set_tracer_provider(provider.clone());

    TelemetryHandle {
        provider: Some(provider),
        enabled: true,
    }
}

fn parse_sampler_from_env() -> trace::Sampler {
    let sampler = std::env::var("OTEL_TRACES_SAMPLER").unwrap_or_default();
    let sampler_arg = std::env::var("OTEL_TRACES_SAMPLER_ARG")
        .ok()
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|v| (0.0..=1.0).contains(v))
        .unwrap_or(1.0);

    match sampler.as_str() {
        "always_on" => trace::Sampler::AlwaysOn,
        "always_off" => trace::Sampler::AlwaysOff,
        "traceidratio" => trace::Sampler::TraceIdRatioBased(sampler_arg),
        "parentbased_always_on" => trace::Sampler::ParentBased(Box::new(trace::Sampler::AlwaysOn)),
        "parentbased_always_off" => {
            trace::Sampler::ParentBased(Box::new(trace::Sampler::AlwaysOff))
        }
        "parentbased_traceidratio" => {
            trace::Sampler::ParentBased(Box::new(trace::Sampler::TraceIdRatioBased(sampler_arg)))
        }
        _ => trace::Sampler::ParentBased(Box::new(trace::Sampler::TraceIdRatioBased(1.0))),
    }
}
