use super::Action;

use crate::Result;

use serde_json::Value;

type FilterExpr = (String, Primitive);

pub enum Primitive {
    Bool(bool),
    String(String),
    Integer(u64),
    Float(f64),
}

impl Primitive {
    fn parse(expr: &str) -> Self {
        if expr == "true" || expr == "false" {
            return Self::Bool(expr.parse().unwrap());
        }

        let mut dot_count = 0;

        for b in expr.bytes() {
            if b == b'.' {
                dot_count += 1;
            }

            if dot_count > 1 {
                return Self::String(String::from(expr));
            }

            if b.is_ascii_digit() {
                continue;
            } else {
                return Self::String(String::from(expr));
            }
        }

        if dot_count == 0 {
            return Self::Integer(expr.parse().unwrap());
        }

        return Self::Float(expr.parse().unwrap());
    }
}

pub struct Filter {
    pub filter_expr: Option<FilterExpr>,
}

impl Filter {
    pub fn parse(expr: &Option<String>) -> Result<Self> {
        if let Some(expr) = expr {
            let parts: Vec<_> = expr.split("=").collect();

            return match parts.as_slice() {
                [] => Err("Invalid filter expression"),
                [_] => Err("Invalid filter expression"),
                [first, second] => Ok(Self {
                    filter_expr: Some((String::from(*first), Primitive::parse(second))),
                }),
                _ => Err("Invalid filter expression"),
            };
        }

        Ok(Self { filter_expr: None })
    }
}

impl Action for Filter {
    fn apply(&self, values: Vec<Value>) -> Vec<Value> {
        match &self.filter_expr {
            Some(expr) => values
                .into_iter()
                .filter(|value| matches(value, expr))
                .collect(),
            None => values,
        }
    }
}

fn matches(value: &Value, filter_expr: &FilterExpr) -> bool {
    match value {
        Value::Object(obj) => {
            if let Some(inner) = obj.get(&filter_expr.0) {
                match (inner, &filter_expr.1) {
                    (Value::Bool(actual), Primitive::Bool(expected)) => *actual == *expected,
                    (Value::String(actual), Primitive::String(expected)) => *actual == *expected,
                    (Value::Number(actual), Primitive::Integer(expected)) => {
                        if actual.is_u64() {
                            actual.as_u64().unwrap() == *expected
                        } else {
                            false
                        }
                    }
                    (Value::Number(actual), Primitive::Float(expected)) => {
                        if actual.is_f64() {
                            actual.as_f64().unwrap() == *expected
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}
