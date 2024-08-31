use anyhow::Result;
use std::process;
use tokio::task::JoinHandle;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;
use url::Url;

pub fn init_tracing() -> Result<(tracing_loki::BackgroundTaskController, JoinHandle<()>)> {
    // Tunable via `RUST_LOG` env variable
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));

    let (layer, controller, task) = tracing_loki::builder()
        .label("host", "pam")?
        .extra_field("pid", format!("{}", process::id()))?
        .build_controller_url(Url::parse("http://127.0.0.1:3100")?)?;

    // We need to register our layer with `tracing`.
    tracing_subscriber::registry()
        .with(env_filter)
        .with(layer)
        // One could add more layers here, for example logging to stdout:
        // .with(tracing_subscriber::fmt::Layer::new())
        // Set the global_default
        .init();

    Ok((controller, tokio::spawn(task)))
}
