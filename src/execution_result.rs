use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::graphql_error::GraphQLError;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ExecutionResult {
    pub data: Value,
    pub errors: Option<Vec<GraphQLError>>,
    pub extensions: Option<Value>,
}
