use std::{
    cmp,
    collections::{HashMap, HashSet},
    fs, process,
};

use serde_json::{json, Map, Value};

pub mod args;
pub mod draw;
pub mod field_path;

use draw::{Cell, DrawOptions, Header};
use field_path::{FieldPath, Selector};

pub fn run(args: args::Args) {
    let data = fs::read_to_string(&args.file).unwrap();

    if args.file.ends_with(".jsonl") {
        render_json_lines(&data, &args);
    } else {
        render_json(&data, &args);
    }
}

fn render_json(data: &str, args: &args::Args) {
    let select_path = get_select_path(&args.select);

    println!("Select path is {:?}", select_path);

    let value = deserialize(data);

    match value {
        Value::Array(mut values) => {
            if !select_path.selectors.is_empty() {
                match args.select_mode {
                    args::SelectMode::Only => values = select(values, &select_path),
                    args::SelectMode::Append => select_append(&mut values, &select_path),
                    args::SelectMode::Auto => todo!(),
                }
            }

            render_table(values, &args, false);
        }
        Value::Object(_) => {
            render_table(vec![value], &args, true);
        }
        _ => println!("Unexpected path"),
    }
}

fn get_select_path(path: &str) -> FieldPath {
    match FieldPath::parse(path) {
        Ok(path) => path,
        Err(error) => exit_with_error(&format!("Invalid select path {}", error)),
    }
}

fn render_json_lines(data: &str, args: &args::Args) {
    let values: Vec<Value> = data.lines().map(|line| deserialize(line)).collect();

    render_table(values, &args, false);
}

fn select(values: Vec<Value>, path: &FieldPath) -> Vec<Value> {
    values
        .into_iter()
        .map(|value| select_from_value(&value, &path.selectors))
        .map(|value| json!({ path.path_str: value }))
        .collect()
}

fn select_append(values: &mut Vec<Value>, path: &FieldPath) {
    values
        .into_iter()
        .for_each(|value| select_and_append_from_value(value, &path));
}

fn select_and_append_from_value(value: &mut Value, path: &FieldPath) {
    let selected = select_from_value(&value, &path.selectors);

    match value {
        Value::Null => todo!(),
        Value::Bool(_) => todo!(),
        Value::Number(_) => todo!(),
        Value::String(_) => todo!(),
        Value::Array(_) => todo!(),
        Value::Object(obj) => {
            obj.insert(String::from(path.path_str), selected);
        }
    }
}

fn select_from_value<'a>(value: &Value, path: &[Selector]) -> Value {
    if let Some(selector) = path.first() {
        match value {
            Value::Array(arr) => select_from_value(&select_from_array(arr, selector), &path[1..]),
            Value::Object(obj) => select_from_value(&select_from_obj(obj, selector), &path[1..]),
            _ => value.clone(),
        }
    } else {
        value.clone()
    }
}

fn select_from_array(arr: &Vec<Value>, selector: &Selector) -> Value {
    match selector {
        Selector::Field(field) => {
            exit_with_error(&format!("Can't select field {} from array", field))
        }
        Selector::IntoArray(index) => match arr.get(*index) {
            Some(value) => value.clone(),
            None => Value::Null,
        },
    }
}

fn select_from_obj(obj: &Map<String, Value>, selector: &Selector) -> Value {
    match selector {
        Selector::IntoArray(index) => {
            exit_with_error(&format!("Can't index at {} from object", index))
        }
        Selector::Field(field) => match obj.get(*field) {
            Some(value) => value.clone(),
            None => Value::Null,
        },
    }
}

// Handle error better way, that matches Clap style
fn deserialize(data: &str) -> Value {
    match serde_json::from_str(&data) {
        Ok(value) => value,
        Err(e) => exit_with_error(&format!("Invalid JSON {}", e)),
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
        flip,
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

fn exit_with_error(error: &str) -> ! {
    eprintln!("{}", error);
    process::exit(1)
}
