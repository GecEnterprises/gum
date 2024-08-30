use crate::subprocess::Subprocess;
use anyhow::Result;

#[derive(Eq, PartialEq, Debug)]
pub enum RunnerMode {
    /// Runs the chain of runnable once and then exits.
    Oneshot,
    /// Uses the `notify` crate to watch for file system changes and execute the chain of runnables.
    /// However, when chain of runnables is running, it will not restart if a file system change is
    /// detected.
    FSWatchLoop,
}

pub struct Runner {
    mode: RunnerMode,
    working_dir: String,

    chain: Vec<Runnable>,
}

impl Runner {
    pub fn new(mode: RunnerMode, working_dir: String) -> Self {
        Self {
            mode,
            working_dir,
            chain: Vec::new(),
        }
    }

    pub fn add_runnable(&mut self, runnable: Runnable) {
        self.chain.push(runnable);
    }

    pub fn set_mode(&mut self, mode: RunnerMode) {
        self.mode = mode;
    }

    pub async fn run(&mut self) -> Result<()> {
        match self.mode {
            RunnerMode::Oneshot => {
                let mut failing = false;

                for runnable in self.chain.iter_mut() {
                    if failing && runnable.runnable_type == RunnableType::Chained {
                        continue;
                    }

                    let ok = runnable.command.run().await;

                    if !ok?.status.success() && runnable.runnable_type == RunnableType::Chained {
                        failing = true;
                    }
                }

                if failing {
                    Err(anyhow::anyhow!("One or more runnables failed"))
                } else {
                    Ok(())
                }
            }
            RunnerMode::FSWatchLoop => {
                todo!()
            }
        }
    }
}

pub struct Runnable {
    command: Subprocess,
    runnable_type: RunnableType,
}

#[derive(Eq, PartialEq, Debug)]
pub enum RunnableType {
    /// Represents a runnable operation that will only run if the previous operation in the chain
    /// was successful.
    Chained,

    /// Represents a runnable operation that will always run, even if one of the items in the
    /// runnable chain fails. This ensures that the specified operation will execute regardless of
    /// the success or failure of preceding operations.
    Always,
}

#[cfg(test)]
mod runner_tests {
    use super::*;

    #[tokio::test]
    async fn test_runner_initialization() {
        let runner = Runner::new(RunnerMode::Oneshot, "/tmp".to_string());

        assert_eq!(runner.mode, RunnerMode::Oneshot);
        assert_eq!(runner.working_dir, "/tmp".to_string());
        assert!(runner.chain.is_empty());
    }

    #[tokio::test]
    async fn test_adding_runnables() {
        let mut runner = Runner::new(RunnerMode::Oneshot, "/tmp".to_string());

        let subprocess = Subprocess::new("echo").arg("Hello, World!");
        let runnable = Runnable { command: subprocess, runnable_type: RunnableType::Always };

        runner.add_runnable(runnable);
        assert_eq!(runner.chain.len(), 1);
    }

    #[tokio::test]
    async fn test_changing_runner_mode() {
        let mut runner = Runner::new(RunnerMode::Oneshot, "/tmp".to_string());

        runner.set_mode(RunnerMode::FSWatchLoop);
        assert_eq!(runner.mode, RunnerMode::FSWatchLoop);
    }

    #[tokio::test]
    async fn test_runner_oneshot_mode_success() -> Result<()> {
        let mut runner = Runner::new(RunnerMode::Oneshot, "/tmp".to_string());

        let subprocess1 = Subprocess::new("echo").arg("First Command");
        let runnable1 = Runnable { command: subprocess1, runnable_type: RunnableType::Chained };

        let subprocess2 = Subprocess::new("echo").arg("Second Command");
        let runnable2 = Runnable { command: subprocess2, runnable_type: RunnableType::Chained };

        runner.add_runnable(runnable1);
        runner.add_runnable(runnable2);

        assert!(runner.run().await.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_runner_oneshot_mode_failure() -> Result<()> {
        let mut runner = Runner::new(RunnerMode::Oneshot, "/tmp".to_string());

        let subprocess1 = Subprocess::new("false"); // This command will fail
        let runnable1 = Runnable { command: subprocess1, runnable_type: RunnableType::Chained };

        let subprocess2 = Subprocess::new("echo").arg("This should not run");
        let runnable2 = Runnable { command: subprocess2, runnable_type: RunnableType::Chained };

        runner.add_runnable(runnable1);
        runner.add_runnable(runnable2);

        assert!(runner.run().await.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_runner_oneshot_mode_always_runs() -> Result<()> {
        // TODO use tmp directory library
        let mut runner = Runner::new(RunnerMode::Oneshot, "/tmp".to_string());

        let subprocess1 = Subprocess::new("false"); // This command will fail
        let runnable1 = Runnable { command: subprocess1, runnable_type: RunnableType::Chained };

        let subprocess2 = Subprocess::new("echo").arg("This should still run");
        let runnable2 = Runnable { command: subprocess2, runnable_type: RunnableType::Always };

        runner.add_runnable(runnable1);
        runner.add_runnable(runnable2);

        // TODO Add checks that subprocess2 actually ran
        assert!(runner.run().await.is_err());

        Ok(())
    }
}
