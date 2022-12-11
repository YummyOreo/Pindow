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
    println!("    -d, --debug           Start the app in debug mode");
    process::exit(0);
}

