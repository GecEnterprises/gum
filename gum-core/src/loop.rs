use crate::subprocess::Subprocess;

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

pub struct Runnable {
    command: Subprocess,
    runnable_type: RunnableType,
}

pub enum RunnableType {
    /// Represents a runnable operation that will only run if the previous operation in the chain
    /// was successful.
    Chained,

    /// Represents a runnable operation that will always run, even if one of the items in the
    /// runnable chain fails. This ensures that the specified operation will execute regardless of
    /// the success or failure of preceding operations.
    Always,
}