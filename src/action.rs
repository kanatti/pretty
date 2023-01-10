use serde_json::Value;

mod select;
mod flatten;
mod sort;
mod filter;

pub trait Action {
    fn apply(&self, values: Vec<Value>) -> Vec<Value>;
}

pub use select::Select;
pub use flatten::Flatten;
pub use sort::Sort;
pub use filter::Filter;
