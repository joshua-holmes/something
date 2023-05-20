use std::any::Any;

pub struct Block { statements: Vec<Statement> }

impl Block {
    fn new() -> Self {
        Self { statements: Vec::new() }
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

enum Expression<'a> {
    Calcuation(&'a Expression, &'a Operator, &'a Expression),
    Variable(&'a str),
    Value(Box<&'a dyn Any>)
}

enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
}
