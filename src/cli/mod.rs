pub fn print_help(warning: Option<&str>) {
    let prelude = match warning {
        Some(message) => format!("{}\n\n", message),
        None => String::new()
    };
    println!("{}something <FILE_NAME>

something -h | --help
    prints this help message", prelude);
}

