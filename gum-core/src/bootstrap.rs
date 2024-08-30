use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;
use anyhow::{Result};
use crate::subprocess::Subprocess;

pub struct Bootstrapper {
    working_dir: Box<Path>,
    flags: HashSet<BootstrapperFlag>
}

#[derive(Hash, Eq, PartialEq)]
pub enum BootstrapperFlag {
    DenoFFILocalDirective
}

impl Bootstrapper {
    pub fn new<P: AsRef<Path>>(working_dir: P) -> Self {
        Self {
            working_dir: working_dir.as_ref().to_path_buf().into_boxed_path(),
            flags: HashSet::new()
        }
    }

    pub fn with_flag(mut self, flag: BootstrapperFlag) -> Self {
        self.flags.insert(flag);
        self
    }
}

impl Bootstrapper{
    pub async fn strap(mut self) -> Result<()> {
        let mut subprocess = Subprocess::new("deno")
            .arg("init")
            .arg("gumland")
            .working_dir(self.working_dir.to_str().unwrap())
            .timeout(Duration::from_secs(5));

        // match subprocess.run().await {
        //     Ok((stdout, stderr)) => {
        //         println!("Command completed successfully within 5 seconds.");
        //         println!("STDOUT: {} <- end of stdout", stdout);
        //         println!("STDERR: {} <- end of stderr", stderr);
        //     }
        //     Err(e) => {
        //         println!("Command failed or timed out: {}", e);
        //     }
        // }

        Ok(())
    }
}