use std::{cmp, collections::HashMap, fs, process};

use serde_json::Value;

pub mod action;
pub mod args;
pub mod path;
pub mod table;

use action::{Action, Flatten, Select, Sort};
use table::{Cell, DrawOptions, Header};

pub type Result<T> = std::result::Result<T, &'static str>;

pub fn run(args: args::Args) {
    let data = fs::read_to_string(&args.file).unwrap();

    let result = if args.file.ends_with(".jsonl") {
        render_json_lines(&data, &args)
    } else {
        render_json(&data, &args)
    };

    if let Err(err) = result {
        exit_with_error(err);
    }
}

fn render_json(data: &str, args: &args::Args) -> Result<()> {
    let value = deserialize(data);

    match value {
        Value::Array(values) => Ok(render_table(values, &args, false)?),
        Value::Object(_) => Ok(render_table(vec![value], &args, true)?),
        _ => Err("Unexpected path"),
    }
}

fn render_json_lines(data: &str, args: &args::Args) -> Result<()> {
    let values: Vec<Value> = data.lines().map(|line| deserialize(line)).collect();

    Ok(render_table(values, &args, false)?)
}

// Handle error better way, that matches Clap style
fn deserialize(data: &str) -> Value {
    match serde_json::from_str(&data) {
        Ok(value) => value,
        Err(e) => exit_with_error(&format!("Invalid JSON {}", e)),
    }
}

fn render_table(mut values: Vec<Value>, args: &args::Args, flip: bool) -> Result<()> {
    let actions: Vec<Box<dyn Action>>= vec![
        Box::new(Select::new(&args.select, args.select_mode)?),
        Box::new(Flatten {
            fields: &args.flatten,
        }),
        Box::new(Sort { field: &args.sort })
    ];

    for action in actions.iter() {
        values = action.apply(values);
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

    println!("{}", table::draw_table(&headers, &rows, draw_options));

    Ok(())
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
