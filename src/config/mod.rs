use super::errors::ArgError;

#[derive(Debug)]
pub struct ExeConfig {
    pub input_file_name: String
}

impl ExeConfig {
    fn new() -> Self {
        Self { input_file_name: String::new() }
    }

    fn is_empty(&self) -> bool {
        self.input_file_name == String::new()
    }
}

#[derive(Debug)]
pub enum ConfigType {
    HelpConfig,
    InterpreterConfig(ExeConfig)
}

pub fn parse_config(mut args: impl Iterator<Item = String>) -> Result<ConfigType, ArgError> {
    args.next();

    let mut exe_config = ExeConfig::new();
    let mut wrong_args = Vec::new();
    for ref arg in args {
        if [String::from("-h"), String::from("--help")].contains(arg) {
            return Ok(ConfigType::HelpConfig);
        } else if arg.chars().nth(0).unwrap() != '-' && exe_config.is_empty() {
            exe_config.input_file_name = String::clone(arg);
        } else {
            wrong_args.push(String::clone(arg));
        }
    }

    if wrong_args.len() > 0 {
        Err(ArgError::WrongArgs { wrong_args })
    } else if exe_config.input_file_name == String::new() {
        Err(ArgError::NotEnoughArgs)
    } else {
        Ok(ConfigType::InterpreterConfig(exe_config))
    }
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
            _ => { assert!(false, "Argument '-h' did not return HelpConfig type") }
        }

        let args = vec![String::new(), String::from("--help")].into_iter();
        let result = parse_config(args);

        match result.unwrap() {
            ConfigType::HelpConfig => { assert!(true) }
            _ => { assert!(false, "Argument '--help' did not return HelpConfig type") }
        }
    }

    #[test]
    fn creates_config_with_correct_file_name() {
        let args = vec![String::new(), String::from("fun-file-name.sg")].into_iter();
        let result = parse_config(args);

        match result.unwrap() {
            ConfigType::InterpreterConfig(exe_config) => {
                assert_eq!(exe_config.input_file_name, String::from("fun-file-name.sg"))
            }
            _ => assert!(false, "Did not return InterpreterConfig type")
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
        let args = vec![
            String::new(),
            String::from("fname"),
            String::from("wrongarg"),
            String::from("-w"),
            String::from("--wrongarg")
        ].into_iter();
        let result = parse_config(args);

        assert!(result.is_err(), "Parsing config should have returned an error and did not!");
        match result.unwrap_err() {
            ArgError::WrongArgs { wrong_args } => {
                assert_eq!(wrong_args, [
                    String::from("wrongarg"),
                    String::from("-w"),
                    String::from("--wrongarg"),
                ])
            }
            _ => assert!(false, "Parsing config should have returned ArgError::WrongArgs")
        }
    }
}

