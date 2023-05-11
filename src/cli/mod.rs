use std::fmt::Display;

pub fn print_help(prefix: impl Display) {
    println!("{}
something <FILE_NAME>

something -h | --help
    prints this help message", prefix);
}

