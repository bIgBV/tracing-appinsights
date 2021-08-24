use appinsights::TelemetryClient;
use tracing::Subscriber;
use tracing_subscriber::Layer;

pub struct AppInsightsLayer<C> {
    client: TelemetryClient<C>,
}

impl<C> AppInsightsLayer<C> {
    pub fn new(client: TelemetryClient<C>) -> Self {
        AppInsightsLayer { client }
    }
}

impl<S, C> Layer<S> for AppInsightsLayer<C>
where
    S: Subscriber,
    C: 'static,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        println!("Got an event: {:?}", event);
    }
}
