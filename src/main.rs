use std::{env, process::exit};
use something::{config, cli, interpreter};

fn main() {
    let config = match config::parse_config(env::args()) {
        Ok(config) => { config }
        Err(err) => {
            cli::print_help(err);
            exit(1);
        }
    };

    match config {
        config::ConfigType::InterpreterConfig(exe_config) => {
            interpreter::interpret(exe_config).unwrap();
        }
        config::ConfigType::HelpConfig => {
            cli::print_help("Welcome to Something!");
        }
    }
}

#[cfg(test)]
mod tests {}

// Zeke wuz here
// Zeke wuz also here
