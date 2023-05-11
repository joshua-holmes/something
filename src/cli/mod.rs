use std::fmt::Display;

pub fn print_help(prefix: impl Display) {
    println!("{}

something <FILE_NAME>
    executes the specified 'something' script

something [-h | --help]
    prints this help message", prefix);
}

