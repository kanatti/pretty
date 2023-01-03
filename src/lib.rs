use std::{cmp, collections::HashMap, fs};

use serde_json::{Result, Value};

pub mod args;
pub mod draw;

use draw::Header;

pub fn run(args: args::Args) {
    let data = fs::read_to_string(args.file_name).unwrap();

    let value: Result<Value> = serde_json::from_str(&data);

    match value {
        Ok(val) => {
            render(val);
        }
        Err(e) => {
            println!("{:#?}", e);
        }
    }
}

fn render(value: Value) {
    match value {
        Value::Array(values) => {
            render_table(&values);
        }
        Value::Object(mapped_values) => println!("Will render object"),
        _ => println!("Unexpected path"),
    }
}

fn render_table(values: &Vec<Value>) {
    // Get Headers
    let headers = get_headers(values);
    let rows: Vec<Vec<String>> = values
        .iter()
        .map(|value| value_to_vec(value, &headers))
        .collect();

    println!("{}", draw::draw_table(&headers, &rows));
}

fn get_headers(values: &Vec<Value>) -> Vec<Header> {
    let mut seen: HashMap<&str, usize> = HashMap::new();
    let mut headers: Vec<String> = Vec::new();

    values.iter().for_each(|value| match value {
        Value::Object(map) => {
            for (key, value) in map.iter() {
                match seen.get(key as &str) {
                    Some(max_width) => {
                        seen.insert(key, cmp::max(*max_width, len(value)));
                    }
                    None => {
                        seen.insert(key, cmp::max(key.len(), len(value)));
                        headers.push(String::from(key));
                    }
                }
            }
        }
        _ => println!("Not implemented"),
    });

    headers
        .into_iter()
        .map(|name| Header {
            max_width: *seen.get(&name as &str).unwrap_or(&0),
            name,
        })
        .collect()
}

fn len(value: &Value) -> usize {
    match value {
        Value::Null => 4,    // null
        Value::Bool(_) => 5, // true or false
        Value::Number(n) => n.to_string().len(),
        Value::String(s) => s.len() + 2,
        Value::Array(_) => 4,  // [..]
        Value::Object(_) => 4, // {..}
    }
}

fn value_to_vec(value: &Value, headers: &Vec<Header>) -> Vec<String> {
    match value {
        Value::Object(object) => headers
            .iter()
            .map(|header| {
                object
                    .get(&header.name)
                    .map(|val| to_string(val))
                    .unwrap_or(String::from(""))
            })
            .collect(),
        _ => vec![],
    }
}

fn to_string(value: &Value) -> String {
    match value {
        Value::Null => String::from("null"),
        Value::Bool(bool) => match bool {
            true => String::from("true"),
            false => String::from("false"),
        },
        Value::Number(n) => n.to_string(),
        Value::String(s) => format!("\"{}\"", s),
        Value::Array(_) => String::from("[..]"),
        Value::Object(_) => String::from("{..}"),
    }
}
