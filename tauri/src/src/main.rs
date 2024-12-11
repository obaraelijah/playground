#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use example_app::app;

fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    app()?.run(|_, _| {});
    Ok(())
}
