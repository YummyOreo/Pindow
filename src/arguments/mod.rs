use std::env;

mod config;
mod debug;
mod info;

#[derive(Debug, Clone, Default)]
pub struct Arguments {
    pub debug: bool,
    pub start_config: Option<usize>,
    pub path: Option<String>,
    pub help: bool,
    pub get_path: bool,
}

pub fn get_args() -> Arguments {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments {
        debug: debug::matches(&args),
        start_config: config::matches_start(&args),
        path: config::matches_path(&args),
        help: info::matches_help(&args),
        get_path: info::matches_path(&args),

    };
    return arguments;
}
