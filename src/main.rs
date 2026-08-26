use schemars::schema_for;
use crate::srv::types::{MaybeValidWallsSrvFile};

pub mod srv;
pub mod types;

fn main() {
    let schema = schema_for!(MaybeValidWallsSrvFile);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}

// #[derive(AvroSchema, Serialize, Deserialize, PartialEq, Debug)]
// struct Foo {
//     a: i64,
//     b: String,
//     // Otherwise it will be serialized as an array of integers
//     #[avro(with)]
//     #[serde(with = "apache_avro::serde::bytes")]
//     c: Vec<u8>,
//     d: OValidWallsSrvFileption<String>
// }

// #[derive(Error, Debug)]
// pub enum GoError {
//     #[error("Avro error")]
//     Avro(#[from] apache_avro::Error),
//     #[error("IO error")]
//     IO(#[from] std::io::Error),
//     #[error("Serde error")]
//     SerdeJson(#[from] serde_json::Error),
//     #[error("Unknown error")]
//     Unknown,
// }

// fn main() {
//     match go() {
//         Ok(_) => {},
//         Err(e) => eprintln!("Failed: {}", e)
//     }
// }


// fn go() -> Result<(), GoError> {
//     let schema = Shot::get_schema();
//     let icf = schema.canonical_form();
//     let value: serde_json::Value = serde_json::from_str(&icf)?;
//     println!("{}", serde_json::to_string_pretty(&value)?);
//     println!("");

//     let source = Source::SchemaStr(&icf);
//     let mut out = std::io::stdout();

//     let g = Generator::new().unwrap();
//     g.generate(&source, &mut out).unwrap();

//     // // Creating this schema is expensive, reuse it as much as possible
//     // let schema = Foo::get_schema();
//     // println!("Schema: {}", schema.canonical_form());

//     // // A writer needs the schema of the type that is going to be written
//     // let mut writer = Writer::new(&schema, Vec::new())?;

//     // let foo = Foo {
//     //     a: 42,
//     //     b: "Hello".to_string(),
//     //     c: b"Data".to_vec(),
//     //     d: None
//     // };

//     // let avro_value = to_value(&foo)?;
//     // let json= serde_json::Value::try_from(avro_value)?;
//     // match serde_json::to_string(&json) {
//     //     Ok(json_str) => println!("{}", json_str),
//     //     Err(e) => eprintln!("Failed to convert to json: {}", e)
//     // }

//     // // Serialize as many items as you want.
//     // writer.append_ser(&foo)?;
//     // writer.append_ser(&foo)?;
//     // writer.append_ser(&foo)?;

//     // // Always flush
//     // writer.flush()?;
//     // // Or consume the writer
//     // let data = writer.into_inner()?;
//     // fs::write("value.avro", &data)?;

//     // // The reader does not need a schema as it's included in the data
//     // let reader = Reader::new(Cursor::new(data))?;
//     // // The reader is an iterator
//     // for result in reader {
//     //     let value = result?;
//     //     let new_foo: Foo = from_value(&value)?;
//     //     assert_eq!(new_foo, foo);
//     // }
//     return Ok(());
// }
