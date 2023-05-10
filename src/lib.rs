use std::option::Iter;
use std::env;




struct FileConfig {
    input_file_name: String
}

enum ConfigType {
    HelpConfig,
    InterpreterConfig(FileConfig)
}

enum ArgError {
    NotEnoughArgs,
    WrongArg
}

fn parse(mut args: impl Iterator<Item = String>) -> Result<ConfigType, ArgError> {

    args.next();

    let mut file_config = FileConfig { input_file_name: String::from("") };
    for arg in args {
        if ["-h", "--help"] {
            return Ok(ConfigType::HelpConfig);
        } else if arg.chars().nth(1).unwrap() != '-' {
            file_config.input_file_name = String::from(*arg);
        } else {
            return Err(ArgError::WrongArg);
        }
    }
    Err(ArgError::NotEnoughArgs)
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::env;

    #[test]
    fn creates_config_with_correct_type() {
        
    }

    #[test]
    fn creates_config_with_correct_values() {

    }

    #[test]
    fn returns_error_if_wrong_args() {
        let args = vec!["--bob"].iter();
        let result = parse(args);

        assert!(!result.is_ok());
    }
}
