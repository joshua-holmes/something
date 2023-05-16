use std::fs::File;
use std::io::{prelude::*, Result as ResultIO};

pub struct Block(Vec<Statement>);

impl Block {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn push(&mut self, value: Statement) {
        self.0.push(value);
    }
}

struct Statement {
    string: String
}

impl Statement {
    fn new() -> Self {
        Self { string: String::new() }
    }
    fn set(&mut self, value: String) {
        self.string = value;
    }
    fn get(&self) -> &String {
        &self.string
    }
}

pub fn file_to_string(file_name: &str) -> ResultIO<String> {
    let mut buffer = String::new();
    let mut file = File::open(file_name)?;
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

// TODO: write this function
pub fn string_to_ast(text: String) -> Block {
    let mut block = Block::new();
    for line in text.split("\n") {
    }

    block
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
}
