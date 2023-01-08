use serde_json::Value;

use super::Action;

pub struct Flatten<'a> {
    pub fields: &'a Vec<String>,
}

impl<'a> Action for Flatten<'a> {
    fn apply(&self, mut values: Vec<Value>) -> Vec<Value> {
        if self.fields.is_empty() {
            return values;
        }

        for field in self.fields {
            flatten_values(&mut values, &field)
        }

        values
    }
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
