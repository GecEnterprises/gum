mod subprocess;

use std::time::Duration;
use crate::subprocess::Subprocess;
use anyhow::{Context, Result};
use tempfile::TempDir;

#[tokio::main]
async fn main() -> Result<()> {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");

    println!("{}", temp_dir.path().to_str().unwrap());

    let mut subprocess = Subprocess::new("deno")
        .arg("init")
        .arg("gumland")
        .working_dir(temp_dir.path().to_str().unwrap())
        .timeout(Duration::from_secs(5));

    match subprocess.run().await {
        Ok((stdout, stderr)) => {
            println!("Command completed successfully within 5 seconds.");
            println!("STDOUT: {} <- end of stdout", stdout);
            println!("STDERR: {} <- end of stderr", stderr);
        }
        Err(e) => {
            println!("Command failed or timed out: {}", e);
        }
    }

    tokio::time::sleep(Duration::from_secs(60 * 10)).await;

    Ok(())
}
