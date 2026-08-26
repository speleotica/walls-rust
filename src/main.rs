use schemars::schema_for;
use crate::srv::types::{MaybeValidWallsSrvFile};

pub mod srv;
pub mod types;

fn main() {
    let schema = schema_for!(MaybeValidWallsSrvFile);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
