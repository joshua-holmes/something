use crate::config::ExeConfig;
use super::ResultDyn;

mod parser;
mod ast;

pub fn interpret(exe_config: ExeConfig) -> ResultDyn<()> {
    let contents = parser::file_to_string(exe_config.input_file_name.as_str())?;
    let ast = parser::string_to_ast(contents);
    Ok(())
}

