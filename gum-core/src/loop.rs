pub enum RunnerMode {
    Oneshot,
    FSWatchLoop
}

pub struct Runner {
    mode: RunnerMode,
    working_dir: String
}

pub enum RunnableType {
    Chained,
    Always
}