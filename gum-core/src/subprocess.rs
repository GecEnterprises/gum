use tokio::time::{Duration};
use tokio::process::{Command};
use anyhow::{anyhow, Context, Result};
use std::process::Stdio;

pub struct Subprocess {
    command: Command,
    timeout: Option<Duration>,
    working_dir: Option<String>,
    stdout: Option<*Stdio>,
    stderr: Option<*Stdio>,
}

impl Subprocess {
    pub fn new(command: &str) -> Self {
        Self {
            command: Command::new(command),
            timeout: None,
            working_dir: None,
            stdout: None,
            stderr: None,
        }
    }

    pub fn pipe_stdout(mut self, stdio: &Stdio) -> Self {
        self.stdout = Some(stdio);
        self
    }

    pub fn pipe_stderr(mut self, stdio: &Stdio) -> Self {
        self.stdout = Some(stdio);
        self
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

    pub async fn run(&mut self) -> Result<()> {
        if let Some(ref dir) = self.working_dir {
            self.command.current_dir(dir);
        }

        // Rather than using channels
        // Expose the Stdio::piped

        if let Some(stdout) = &self.stdout {
            self.command.stdout(stdout);
        } else {
            self.command.stdout(Stdio::null());
        }

        if let Some(stderr) = &self.stderr {
            self.command.stderr(stderr);
        } else {
            self.command.stderr(Stdio::null());
        }

        let mut child = self.command
            .spawn()
            .map_err(|_| anyhow!("Program {:?} not found", self.command))
            .context("Failed spawning process")?;

        // let result = if let Some(timeout_duration) = self.timeout {
        //     match timeout(timeout_duration, self.wait_and_capture_output(&mut child)).await {
        //         Ok(output) => output,
        //         Err(_) => {
        //             child.kill().await.context("Failed to kill process on timeout")?;
        //             Err(anyhow!("Process timed out after {:?}", timeout_duration))
        //         }
        //     }
        // } else {
        // self.wait_and_capture_output(&mut child).await
        // };
        Err(anyhow!("no"))?;

        // result
        Ok(())
    }
}