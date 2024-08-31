use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::Duration;
use anyhow::{Result};
use crate::looprunner::{Runnable, RunnableType, Runner, RunnerMode};
use crate::subprocess::Subprocess;

pub struct Bootstrapper {
    working_dir: Box<Path>,
    flags: HashSet<BootstrapperFlag>,
}

#[derive(Hash, Eq, PartialEq)]
pub enum BootstrapperFlag {
    DenoFFILocalDirective
}

impl Bootstrapper {
    pub fn new<P: AsRef<Path>>(working_dir: P) -> Self {
        Self {
            working_dir: working_dir.as_ref().to_path_buf().into_boxed_path(),
            flags: HashSet::new(),
        }
    }

    pub fn new_from_named(strpath: &str) -> Self {
        let working_dir = if strpath == "." {
            std::env::current_dir().expect("Failed to get current directory")
        } else {
            let mut path = std::env::current_dir().expect("Failed to get current directory");
            path.push(strpath);
            fs::create_dir_all(&path).expect("Failed to create directory");
            path
        };

        Self {
            working_dir: working_dir.into_boxed_path(),
            flags: HashSet::new(),
        }
    }

    pub fn with_flag(mut self, flag: BootstrapperFlag) -> Self {
        self.flags.insert(flag);
        self
    }
}

impl Bootstrapper {
    pub async fn strap(self) -> Result<()> {
        let subprocess = Subprocess::new("deno")
            .arg("init")
            .arg(".")
            .timeout(Duration::from_secs(5));

        Runner::new(RunnerMode::Oneshot, self.working_dir.to_str().unwrap().to_string())
            .add_runnable(Runnable {
                command: subprocess,
                runnable_type: RunnableType::Chained,
            })
            .run()
            .await?;

        Ok(())
    }
}