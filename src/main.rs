use crate::srv::types::MaybeValidWallsSrvFile;
use schemars::schema_for;

pub mod srv;
pub mod types;

fn main() {
    let schema = schema_for!(MaybeValidWallsSrvFile);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
