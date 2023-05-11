use std::env;
use something::{config, cli};

fn main() {
    let args = env::args();
    let config = config::parse_config(args);
    match config {
        Ok(config_type) => { match config_type {
            config::ConfigType::InterpreterConfig(file_config) => {
                println!("LOADING FILE: {}", file_config.input_file_name)
            }
            config::ConfigType::HelpConfig => {
                cli::print_help("Welcome to Something!")
            }
        } }
        Err(err) => { cli::print_help(err) }
    }
}

#[cfg(test)]
mod tests {}
