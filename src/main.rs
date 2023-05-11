use std::env;
use something::{config, cli};

fn main() {
    let config = config::parse_config(env::args());
    match config {
        Ok(config_type) => { match config_type {
            config::ConfigType::InterpreterConfig(exe_config) => {
                println!("LOADING FILE: {}", exe_config.input_file_name)
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
