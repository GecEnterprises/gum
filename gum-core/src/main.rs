mod subprocess;

use std::time::Duration;
use crate::subprocess::Subprocess;
use anyhow::{Context, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut subprocess = Subprocess::new("deno")
        .timeout(Duration::from_secs(5));

    match subprocess.run().await {
        Ok((stdout, stderr)) => {
            println!("Command completed successfully within 5 seconds.");
            println!("STDOUT: {}", stdout);
            println!("STDERR: {}", stderr);
        }
        Err(e) => {
            println!("Command failed or timed out: {}", e);
        }
    }

    Ok(())
}
