pub mod config;
pub mod cli;
pub mod errors;
pub mod interpreter;

pub type ResultDyn<T> = Result<T, Box<dyn std::error::Error>>;
