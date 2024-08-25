use anyhow::Result;
use std::process;
use tracing_loki::BackgroundTask;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use url::Url;

pub fn build() -> Result<BackgroundTask> {
    let (layer, task) = tracing_loki::builder()
        .label("host", "pam")?
        .extra_field("pid", format!("{}", process::id()))?
        .build_url(Url::parse("http://127.0.0.1:3100").unwrap())?;

    // We need to register our layer with `tracing`.
    tracing_subscriber::registry()
        .with(layer)
        // One could add more layers here, for example logging to stdout:
        // .with(tracing_subscriber::fmt::Layer::new())
        .init();

    Ok(task)
}
