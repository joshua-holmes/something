use std::env;
use something::{self, ConfigType, print_help};

fn main() {
    let args = env::args();
    let config = something::parse_config(args);
    match config {
        Ok(config_type) => { match config_type {
            ConfigType::InterpreterConfig(file_config) => { println!("LOADING FILE: {}", file_config.input_file_name) }
            ConfigType::HelpConfig => {
                something::print_help(None)
            }
        } }
        Err(err) => { print_help(Some(format!("{:?}", err).as_str())) }
    }
}

#[cfg(test)]
mod tests {}
