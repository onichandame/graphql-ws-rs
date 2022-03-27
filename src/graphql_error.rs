use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error as StdError, fmt::Display};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLError {
    pub locations: Option<Vec<SourceLocation>>,
    pub path: Option<Vec<StringOrU64>>,
    /// too much to structure
    pub nodes: Option<Vec<Value>>,
    pub source: Option<Source>,
    pub positions: Option<Vec<u64>>,
    pub original_error: Option<Error>,
    pub extensions: Option<Value>,

    #[serde(flatten)]
    pub error: Error,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SourceLocation {
    pub line: u64,
    pub column: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum StringOrU64 {
    String(String),
    U64(u64),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub body: String,
    pub name: String,
    pub location_offset: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Error {
    pub message: String,
}

impl StdError for GraphQLError {
    fn description(&self) -> &str {
        self.error.message.as_str()
    }
}

impl Display for GraphQLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error.message)
    }
}
