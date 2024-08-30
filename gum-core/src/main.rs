mod subprocess;
mod bootstrap;

use std::time::Duration;
use anyhow::{Context, Result};
use tempfile::TempDir;
use crate::bootstrap::Bootstrapper;

#[tokio::main]
async fn main() -> Result<()> {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    Bootstrapper::new(temp_dir.path()).strap().await?;

    tokio::time::sleep(Duration::from_secs(60 * 10)).await;

    Ok(())
}
