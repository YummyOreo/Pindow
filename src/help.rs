use directories::BaseDirs;
use std::process;

pub fn print_help_menue() {
    //! This will stop the program!
    println!("Usage: pindow [OPTIONS]");
    println!("Options:");
    println!("    -h, --help            Display this message");
    println!("    -c, --config NUM      Start the app with different config");
    println!("                          It will start based on the number");
    println!("    -p, --path PATH       Start the app with a different ");
    println!("                          config file");
    println!("    --get-path            Gets the path of the config");
    println!("    -d, --debug           Start the app in debug mode");
    process::exit(0);
}

pub fn print_config_path() {
    //! This will stop the program!
    let base_dirs = BaseDirs::new().unwrap();
    let path = base_dirs.data_dir().to_str().unwrap().to_string() + "\\pindow\\config.json";
    println!("The path is: {}", path);
    process::exit(0);
}
