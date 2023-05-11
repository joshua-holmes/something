#[derive(Debug)]
pub struct FileConfig {
    pub input_file_name: String
}

#[derive(Debug)]
pub enum ConfigType {
    HelpConfig,
    InterpreterConfig(FileConfig)
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ArgError {
    NotEnoughArgs,
    WrongArg
}

pub fn parse_config(mut args: impl Iterator<Item = String>) -> Result<ConfigType, ArgError> {
    args.next();

    let mut file_config = FileConfig { input_file_name: String::from("") };
    for ref arg in args {
        if [String::from("-h"), String::from("--help")].contains(arg) {
            return Ok(ConfigType::HelpConfig);
        } else if arg.chars().nth(0).unwrap() != '-' {
            file_config.input_file_name = String::from(arg);
        } else {
            return Err(ArgError::WrongArg);
        }
    }

    if file_config.input_file_name == String::new() {
        Err(ArgError::NotEnoughArgs)
    } else {
        Ok(ConfigType::InterpreterConfig(file_config))
    }
}

pub fn print_help(warning: Option<&str>) {
    let prelude = match warning {
        Some(message) => format!("{}\n\n", message),
        None => String::new()
    };
    println!("{}something <FILE_NAME>

something -h | --help
    prints this help message", prelude);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_config_with_help_type() {
        let args = vec![String::new(), String::from("-h")].into_iter();
        let result = parse_config(args);

        match result.unwrap() {
            ConfigType::HelpConfig => { assert!(true) }
            _ => { assert!(false) }
        }

        let args = vec![String::new(), String::from("--help")].into_iter();
        let result = parse_config(args);

        match result.unwrap() {
            ConfigType::HelpConfig => { assert!(true) }
            _ => { assert!(false) }
        }
    }

    #[test]
    fn creates_config_with_correct_file_name() {
        let args = vec![String::new(), String::from("fun-file-name.sg")].into_iter();
        let result = parse_config(args);

        match result.unwrap() {
            ConfigType::InterpreterConfig(file_config) => {
                assert_eq!(file_config.input_file_name, String::from("fun-file-name.sg"))
            }
            _ => assert!(false)
        }
     }

    #[test]
    fn returns_error_if_not_enough_args() {
        let args = vec![String::new()].into_iter();
        let result = parse_config(args);

        assert_eq!(result.unwrap_err(), ArgError::NotEnoughArgs);
    }

    #[test]
    fn returns_error_if_wrong_args() {
        let args = vec![String::new(), String::from("-b")].into_iter();
        let result = parse_config(args);

        assert_eq!(result.unwrap_err(), ArgError::WrongArg);

        let args = vec![String::new(), String::from("--bob")].into_iter();
        let result = parse_config(args);

        assert_eq!(result.unwrap_err(), ArgError::WrongArg);
    }
}
