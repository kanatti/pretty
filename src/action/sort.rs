use std::cmp::Ordering;

use serde_json::Value;

use super::Action;

pub struct Sort<'a> {
    pub field: &'a Option<String>,
}

impl<'a> Action for Sort<'a> {
    fn apply(&self, mut values: Vec<Value>) -> Vec<Value> {
        if let Some(field) = &self.field {
            values.sort_by(|val1, val2| match (val1, val2) {
                (Value::Object(val1), Value::Object(val2)) => {
                    match (val1.get(field), val2.get(field)) {
                        (Some(a), Some(b)) => compare(a, b),
                        (Some(_), None) => Ordering::Greater,
                        (None, Some(_)) => Ordering::Less,
                        _ => Ordering::Equal,
                    }
                }
                _ => Ordering::Equal,
            });
        }

        values
    }
}

fn compare(val1: &Value, val2: &Value) -> Ordering {
    match (val1, val2) {
        // Null
        (Value::Null, _) => Ordering::Greater,

        // Bool
        (Value::Bool(bool1), Value::Bool(bool2)) => bool1.cmp(bool2),
        (Value::Bool(_), _) => Ordering::Less,

        // Number
        (Value::Number(n1), Value::Number(n2)) => match n1.as_f64().partial_cmp(&n2.as_f64()) {
            Some(ordering) => ordering,
            None => Ordering::Equal,
        },
        (Value::Number(_), _) => Ordering::Less,

        // String
        (Value::String(s1), Value::String(s2)) => s1.cmp(s2),
        (Value::String(_), _) => Ordering::Less,

        // Array or Object
        (_, _) => Ordering::Equal,
    }
}
