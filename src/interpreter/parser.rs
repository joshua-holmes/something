use std::fs::File;
use std::io::{prelude::*, Result as ResultIO};

use super::ast;

pub fn file_to_string(file_name: &str) -> ResultIO<String> {
    let mut buffer = String::new();
    let mut file = File::open(file_name)?;
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub fn hoist<'a>(text: String) -> ast::Block<'a> {
    let mut block = ast::Block::new();
    for line in text.split(";") {
        insert_into_block(line, &mut block);
    }

    block
}

// TODO: write this function so test passes
fn insert_into_block(line: &str, block: &mut ast::Block) {
    println!("{}", line)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn parses_file_contents_as_string() {
        let file_name = "parses_file_contents_as_string.sg";
        let mut file = File::create(file_name).unwrap();
        let file_contents = "Some text\nin multiple lines";
        file.write_all(file_contents.as_bytes()).unwrap();

        let result = file_to_string(file_name);
        fs::remove_file(file_name).unwrap();
        let extracted = result.unwrap();

        assert_eq!(extracted, String::from(file_contents))
    }

    #[test]
    fn string_to_set_statement() {
        let line = "six = 6";
        let mut block = ast::Block::new();
        insert_into_block(line, &mut block);

        if let Some(statement) = block.statements.first() {
            assert_eq!(statement.key, "six");
        } else {
            assert!(false, "`block.statements` should have contained at least 1 value");
        }
    }
}
