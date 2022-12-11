use std::env;

mod config;
mod debug;
mod info;

pub struct Arguments {
    pub debug: bool,
    pub start_config: Option<usize>,
    pub path: Option<String>,
    pub help: bool
}

pub fn get_args() -> Arguments {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments {
        debug: debug::matches(&args),
        start_config: config::matches_start(&args),
        path: config::matches_path(&args),
        help: info::matches_help(&args),
    };
    return arguments;
}
