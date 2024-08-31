use tokio::time::{Duration};
use tokio::process::{Command};
use anyhow::{anyhow, Context, Result};
use tokio::io::{BufReader, AsyncBufReadExt};
use std::process::{ExitStatus, Stdio};


#[derive(Clone)]
pub struct Subprocess {
    command: String,
    args: Vec<String>,
    timeout: Option<Duration>,
    working_dir: Option<String>,
}

pub struct SubprocessResult {
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub status: ExitStatus
}

impl Subprocess {
    pub fn new(command: &str) -> Self {
        Self {
            command: command.to_string(),
            args: Vec::new(),
            timeout: None,
            working_dir: None,
        }
    }

    pub fn timeout(mut self, timeout_duration: Duration) -> Self {
        self.timeout = Some(timeout_duration);
        self
    }

    pub fn arg(mut self, arg: &str) -> Self {
        self.args.push(arg.to_string());
        self
    }

    pub fn working_dir(mut self, dir: &str) -> Self {
        self.working_dir = Some(dir.to_string());
        self
    }

    pub async fn run(&mut self) -> Result<SubprocessResult> {
        let command = match cfg!(target_os = "windows") {
            true => "cmd".to_string(),
            false => "/bin/sh".to_string(),
        };

        let mut command = Command::new(command);

        if cfg!(target_os = "windows") {
            command.arg("/C");
        } else {
            command.arg("-c");
        }

        command.arg(&self.command);

        for arg in &self.args {
            command.arg(arg);
        }

        if let Some(ref dir) = self.working_dir {
            command.current_dir(dir);
        }
        command.stdout(Stdio::piped());
        command.stderr(Stdio::piped());
        let mut child = command
            .spawn()
            .map_err(|_| anyhow!("Program {:?} not found", self.command))
            .context("Failed spawning process")?;

        let stdout = child.stdout.take().ok_or_else(|| anyhow!("Failed to get stdout"))?;
        let stderr = child.stderr.take().ok_or_else(|| anyhow!("Failed to get stderr"))?;

        let output = if let Some(timeout_duration) = self.timeout {
            match tokio::time::timeout(timeout_duration, child.wait()).await {
                Ok(result) => result.context("Child process failed")?,
                Err(_) => {
                    child.kill().await.context("Failed to kill process on timeout")?;
                    return Err(anyhow!("Process timed out after {:?}", timeout_duration));
                }
            }
        } else {
            child.wait().await.context("Failed to wait on child process")?
        };

        let mut stdout_lines = BufReader::new(stdout).lines();
        let mut stderr_lines = BufReader::new(stderr).lines();

        let mut stdout_output = String::new();
        let mut stderr_output = String::new();

        while let Some(line) = stdout_lines.next_line().await? {
            stdout_output.push_str(&line);
            stdout_output.push('\n');
        }

        while let Some(line) = stderr_lines.next_line().await? {
            stderr_output.push_str(&line);
            stderr_output.push('\n');
        }

        Ok(SubprocessResult {
            stdout: if stdout_output.is_empty() { None } else { Some(stdout_output.trim().to_string()) },
            stderr: if stderr_output.is_empty() { None } else { Some(stderr_output.trim().to_string()) },
            status: output,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_echo_command() -> Result<()> {
        let mut subprocess = Subprocess::new("echo");

        subprocess = subprocess.arg("Hello, World!");
        let output = subprocess.run().await?;

        assert!(output.status.success());

        Ok(())
    }

    #[tokio::test]
    async fn test_echo_command_with_stdout_pipe() -> Result<()> {
        let mut subprocess = Subprocess::new("echo");
        let stdout = Stdio::piped();

        subprocess = subprocess.arg("Hello, stdout!");

        let output = subprocess.run().await?;

        assert!(output.status.success());

        assert_eq!(output.stdout, Some("Hello, stdout!".to_string()));

        Ok(())
    }

    #[tokio::test]
    async fn test_echo_command_with_timeout() -> Result<()> {
        let mut subprocess = Subprocess::new("echo");

        subprocess = subprocess.arg("Hello, timeout!")
            .timeout(Duration::from_secs(10));

        let output = subprocess.run().await?;

        assert!(output.status.success());

        Ok(())
    }

    #[tokio::test]
    async fn test_echo_command_with_working_dir() -> Result<()> {
        // Assuming `echo` command behaves the same in different working directories
        let mut subprocess = Subprocess::new("echo");

        subprocess = subprocess.arg("Hello, working dir!")
            .working_dir("/"); // Adjust to a valid directory for your OS

        let output = subprocess.run().await?;

        assert!(output.status.success());

        Ok(())
    }
}

