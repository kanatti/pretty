use std::fs;

use serde_json::{Result, Value};

pub mod args;

pub fn run(args: args::Args) {
    let data = fs::read_to_string(args.file_name).unwrap();

    let value: Result<Value> = serde_json::from_str(&data);

    match value {
        Ok(val) => {
            println!("{}", serde_json::to_string_pretty(&val).unwrap());
        }
        Err(e) => {
            println!("{:#?}", e);
        }
    }
}
