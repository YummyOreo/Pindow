use std::env;

mod debug;
mod config;

pub struct Arguments {
    pub debug: bool,
    pub start_config: Option<usize>,
}

pub fn get_args() -> Arguments {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments{
        debug: debug::matches(&args),
        start_config: config::matches(&args),
    };
    return arguments;
}
