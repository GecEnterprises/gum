use tokio::time::{timeout, Duration};
use tokio::process::{Command, Child};
use anyhow::{anyhow, Context, Result};
use std::process::Stdio;

use tokio::io::AsyncReadExt;

pub struct Subprocess {
    command: Command,
    timeout: Option<Duration>,
    working_dir: Option<String>,
}

impl Subprocess {
    pub fn new(command: &str) -> Self {
        Self {
            command: Command::new(command),
            timeout: None,
            working_dir: None,
        }
    }

    pub fn timeout(mut self, timeout_duration: Duration) -> Self {
        self.timeout = Some(timeout_duration);
        self
    }

    pub fn arg(mut self, arg: &str) -> Self {
        self.command.arg(arg);
        self
    }

    pub fn working_dir(mut self, dir: &str) -> Self {
        self.working_dir = Some(dir.to_string());
        self
    }

    pub async fn run(&mut self) -> Result<(String, String)> {
        if let Some(ref dir) = self.working_dir {
            self.command.current_dir(dir);
        }

        self.command.stdout(Stdio::piped());
        self.command.stderr(Stdio::piped());

        let mut child = self.command
            .spawn()
            .map_err(|_| anyhow!("Program {:?} not found", self.command))
            .context("Failed spawning process")?;

        let result = if let Some(timeout_duration) = self.timeout {
            match timeout(timeout_duration, self.wait_and_capture_output(&mut child)).await {
                Ok(output) => output,
                Err(_) => {
                    child.kill().await.context("Failed to kill process on timeout")?;
                    Err(anyhow!("Process timed out after {:?}", timeout_duration))
                }
            }
        } else {
            self.wait_and_capture_output(&mut child).await
        };

        result
    }

    async fn wait_and_capture_output(&self, child: &mut Child) -> Result<(String, String)> {
        let mut stdout = String::new();
        let mut stderr = String::new();

        if let Some(mut stdout_pipe) = child.stdout.take() {
            stdout_pipe.read_to_string(&mut stdout).await.context("Failed to read stdout")?;
        }

        if let Some(mut stderr_pipe) = child.stderr.take() {
            stderr_pipe.read_to_string(&mut stderr).await.context("Failed to read stderr")?;
        }

        child.wait().await.context("Failed running subprocess")?;

        Ok((stdout, stderr))
    }
}