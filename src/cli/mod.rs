use std::fmt::Debug;

pub fn print_help(prefix: impl Debug) {
    println!("{:?}
something <FILE_NAME>

something -h | --help
    prints this help message", prefix);
}

