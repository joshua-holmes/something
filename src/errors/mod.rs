use std::fmt;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ArgError {
    NotEnoughArgs,
    WrongArgs(Vec<String>)
}

struct ErrMsg<'a> {
    type_as_string: &'a str,
    msg: String
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_msg = match self {
            ArgError::NotEnoughArgs => { ErrMsg {
                type_as_string: "NotEnoughArgs",
                msg: String::from("Not enough arguments!")
            } }
            ArgError::WrongArgs(args) => { ErrMsg {
                type_as_string: "WrongArgs",
                msg: format!("Incorrect argument(s): {}", args.join(", "))
            } }
        };

        write!(f, "ArgError::{} => {}\n", err_msg.type_as_string, err_msg.msg)
    }
}