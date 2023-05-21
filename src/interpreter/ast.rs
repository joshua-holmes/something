use std::any::Any;

pub struct Block { pub statements: Vec<Statement> }

impl Block {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }
}

pub struct Statement<'a> {
    string: &'a str
}

impl <'a> Statement<'a> {
    pub fn new(string: &str) -> Self {
        Self { string }
    }

    pub fn set(&mut self, value: &str) {
        self.string = value;
    }

    pub fn get(&self) -> &str {
        &self.string
    }
}

enum Expression<'a> {
    Calcuation(&'a Expression<'a>, &'a Operator, &'a Expression<'a>),
    Variable(&'a str),
    Value(Box<&'a dyn Any>)
}

enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
}
