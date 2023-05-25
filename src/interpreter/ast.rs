use std::any::Any;

pub struct Block<'a> {
    pub statements: Vec<Statement<'a>>
}

impl <'a> Block<'a> {
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }
}

#[derive(Debug)]
pub enum ValidType<'a> {
    Int(&'a i32),
    Str(&'a str)
}

#[derive(Debug)]
pub enum Statement<'a> {
    Set { key: &'a str, value: &'a Expression<'a> },
    DoNothing { value: &'a Expression<'a> },
}

#[derive(Debug)]
pub enum Expression<'a> {
    Calcuation(&'a Expression<'a>, &'a Operator, &'a Expression<'a>),
    Variable(&'a str),
    Value(&'a ValidType<'a>)
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
}

