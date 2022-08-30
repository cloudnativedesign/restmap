mod parser;
mod statemanager;
use clap::{App, Arg};

/// Exposes the management Interface to the parts of the program elements required for conducting
/// core operations for the user


pub fn connect_to_cli() -> Vec<String> {
    let matches = App::new("RESTMap")
        .version("0.1.0")
        .author("Frank Winkler <frank.fichtenmueller@outlook.com>")
        .about("Manage complex REST API integrations through stateful YAML configuration")
        .arg(
           Arg::with_name("location")
                .short('l')
                .value_name("LOCATION")
                .help("Filepath to configuration YAML file to be processed")
                .required(true)
            )
        .get_matches()
    let text = matches.values_of_lossy("location").unwrap();
    text

    
    
}

