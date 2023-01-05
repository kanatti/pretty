use std::{
    cmp,
    collections::{HashMap, HashSet},
    fs, process,
};

use serde_json::Value;

pub mod args;
pub mod draw;

use draw::Header;

pub fn run(args: args::Args) {
    let data = fs::read_to_string(&args.file_name).unwrap();

    if args.file_name.ends_with(".jsonl") {
        render_json_lines(&data, &args);
    } else {
        render_json(&data, &args);
    }
}

fn render_json(data: &str, args: &args::Args) {
    let value = deserialize(data);

    match value {
        Value::Array(values) => {
            render_table(values, &args.flatten);
        }
        Value::Object(mapped_values) => println!("Will render object"),
        _ => println!("Unexpected path"),
    }
}

fn render_json_lines(data: &str, args: &args::Args) {
    let values: Vec<Value> = data.lines().map(|line| {
        deserialize(line)
    }).collect();

    render_table(values, &args.flatten);
}

// Handle error better way, that matches Clap style
fn deserialize(data: &str) -> Value {
    match serde_json::from_str(&data) {
        Ok(value) => {
           value
        }
        Err(e) => {
            eprintln!("Invalid JSON {}", e);
            process::exit(1)
        }
    }
}


fn render_table(mut values: Vec<Value>, flatten_fields: &Vec<String>) {
    if !flatten_fields.is_empty() {
        values = flatten(values, &flatten_fields);
    }

    let headers = get_headers(&values);

    let rows: Vec<Vec<String>> = values
        .iter()
        .map(|value| value_to_vec(value, &headers))
        .collect();

    println!("{}", draw::draw_table(&headers, &rows));
}

fn flatten(mut values: Vec<Value>, flatten_fields: &Vec<String>) -> Vec<Value> {
    let headers = get_header_set(&values);

    // TODO: Change to error and handle gracefully
    let valid_fields: Vec<&String> = flatten_fields
        .iter()
        .filter(|&field| headers.contains(&*field))
        .collect();

    for field in valid_fields {
        flatten_values(&mut values, field)
    }

    values
}

fn flatten_values(values: &mut Vec<Value>, field: &String) {
    for value in values.iter_mut() {
        if let Value::Object(value) = value {
            let inner = value.remove(field);

            match inner {
                Some(Value::Object(inner)) => {
                    for (key, val) in inner.into_iter() {
                        let new_key = format!("{}.{}", field, key);
                        value.insert(String::from(new_key), val);
                    }
                }
                Some(val) => {
                    value.insert(String::from(field), val);
                }
                _ => {}
            }
        }
    }
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

fn get_header_set(values: &Vec<Value>) -> HashSet<String> {
    let mut headers: HashSet<String> = HashSet::new();

    values.iter().for_each(|value| match value {
        Value::Object(map) => {
            map.keys().for_each(|key| {
                headers.insert(String::from(key));
            });
        }
        _ => {}
    });

    headers
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
