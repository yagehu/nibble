use tracing_subscriber::Registry;

use tracing_subscriber::{
    fmt::format::FmtSpan,
    layer::SubscriberExt as _,
    EnvFilter,
};

pub fn init(
    env: &str,
) -> Result<(), tracing::subscriber::SetGlobalDefaultError> {
    let tracer = opentelemetry::sdk::export::trace::stdout::new_pipeline()
        .install_simple();
    let subscriber = Registry::default()
        .with(EnvFilter::from_env(env))
        .with(tracing_subscriber::fmt::layer().pretty())
        .with(tracing_opentelemetry::layer().with_tracer(tracer));

    tracing::subscriber::set_global_default(subscriber)
}
