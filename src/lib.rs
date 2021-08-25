use appinsights::{
    telemetry::{SeverityLevel, Telemetry, TraceTelemetry},
    InMemoryChannel, TelemetryClient,
};
use tracing::{field::Visit, Level, Subscriber};
use tracing_subscriber::Layer;

fn into_severity_level(level: &tracing::Level) -> SeverityLevel {
    match *level {
        Level::TRACE => SeverityLevel::Verbose,
        Level::DEBUG => SeverityLevel::Information,
        Level::INFO => SeverityLevel::Warning,
        Level::WARN => SeverityLevel::Error,
        Level::ERROR => SeverityLevel::Critical,
    }
}

pub struct AppInsightsLayer {
    client: TelemetryClient<InMemoryChannel>,
}

impl AppInsightsLayer {
    pub fn new(client: TelemetryClient<InMemoryChannel>) -> Self {
        AppInsightsLayer { client }
    }
}

impl<S> Layer<S> for AppInsightsLayer
where
    S: Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut telemetry = TraceTelemetry::new(
            "figuring out how to get the message",
            into_severity_level(event.metadata().level()),
        );

        event.record(&mut SpanEventVisitor(&mut telemetry));

        self.client.track(telemetry);
    }
}

struct SpanEventVisitor<'a>(&'a mut TraceTelemetry);

impl<'a> Visit for SpanEventVisitor<'a> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0
            .properties_mut()
            .insert(field.to_string(), format!("{:?}", value));
    }
}
