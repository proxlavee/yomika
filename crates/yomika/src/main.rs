#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use yomika::app;
use yomika::panic;
use yomika::sentry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = sentry::initialize();
    panic::install();
    app::run().await
}
