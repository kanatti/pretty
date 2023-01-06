use std::{
    cmp,
    collections::{HashMap, HashSet},
    fs, process,
};

use serde_json::Value;

pub mod args;
pub mod draw;

use draw::{Cell, Header, DrawOptions};

pub fn run(args: args::Args) {
    let data = fs::read_to_string(&args.file).unwrap();

    if args.file.ends_with(".jsonl") {
        render_json_lines(&data, &args);
    } else {
        render_json(&data, &args);
    }
}

fn render_json(data: &str, args: &args::Args) {
    let value = deserialize(data);

    match value {
        Value::Array(values) => {
            render_table(values, &args, false);
        }
        Value::Object(_) => {
            render_table(vec![value], &args, true);
        },
        _ => println!("Unexpected path"),
    }
}

fn render_json_lines(data: &str, args: &args::Args) {
    let values: Vec<Value> = data.lines().map(|line| deserialize(line)).collect();

    render_table(values, &args, false);
}

// Handle error better way, that matches Clap style
fn deserialize(data: &str) -> Value {
    match serde_json::from_str(&data) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Invalid JSON {}", e);
            process::exit(1)
        }
    }
}

fn render_table(mut values: Vec<Value>, args: &args::Args, flip: bool) {
    if !args.flatten.is_empty() {
        values = flatten(values, &args.flatten);
    }

    let headers = get_headers(&values);

    let rows: Vec<Vec<Cell>> = values
        .iter()
        .map(|value| value_to_vec(value, &headers))
        .collect();

    let draw_options = DrawOptions {
        color: args.color,
        flip
    };

    println!("{}", draw::draw_table(&headers, &rows, draw_options));
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

fn value_to_vec(value: &Value, headers: &Vec<Header>) -> Vec<Cell> {
    match value {
        Value::Object(object) => headers
            .iter()
            .map(|header| {
                object
                    .get(&header.name)
                    .map(|val| to_cell(val))
                    .unwrap_or(Cell::string(String::from("")))
            })
            .collect(),
        _ => vec![],
    }
}

fn to_cell(value: &Value) -> Cell {
    match value {
        Value::Null => Cell::null(String::from("null")),
        Value::Bool(bool) => match bool {
            true => Cell::bool(String::from("true")),
            false => Cell::bool(String::from("false")),
        },
        Value::Number(n) => Cell::number(n.to_string()),
        Value::String(s) => Cell::string(format!("\"{}\"", s)),
        Value::Array(_) => Cell::collapsed(String::from("[..]")),
        Value::Object(_) => Cell::collapsed(String::from("{..}")),
    }
}
