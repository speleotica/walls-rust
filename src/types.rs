use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct SourcePos {
  line: u32,
  column: u32,
  index: u32,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct SourceLoc {
  start: SourcePos,
  end: SourcePos,
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub enum ParseIssueSeverity {
  Error,
  Warning
}

#[derive(JsonSchema, Serialize, Deserialize, PartialEq, Debug)]
#[schemars(deny_unknown_fields)]
pub struct ParseIssue {
  severity: ParseIssueSeverity,
  code: String,
  message: Option<String>,
  loc: Option<SourceLoc>,
}