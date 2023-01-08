use std::process;

use serde_json::{json, Map, Value};

use crate::{
    args::SelectMode,
    path::{FieldPath, Selector},
    Result,
};

use super::Action;

pub struct Select<'a> {
    pub path: FieldPath<'a>,
    pub mode: SelectMode,
}

impl<'a> Select<'a> {
    pub fn new(path_str: &'a str, mode: SelectMode) -> Result<Select<'a>> {
        let path = FieldPath::parse(&path_str)?;

        Ok(Select { path, mode })
    }
}

impl<'a> Action for Select<'a> {
    fn apply(&self, mut values: Vec<Value>) -> Vec<Value> {
        if self.path.selectors.is_empty() {
            return values;
        }

        match self.mode {
            SelectMode::Only => values = select(values, &self.path),
            SelectMode::Append => select_append(&mut values, &self.path),
            SelectMode::Auto => todo!(),
        }

        values
    }
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

fn exit_with_error(error: &str) -> ! {
    eprintln!("{}", error);
    process::exit(1)
}
