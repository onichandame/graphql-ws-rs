use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{execution_result::ExecutionResult, graphql_error::GraphQLError};

#[derive(Deserialize, Serialize, Debug)]
pub struct ResponseError(Vec<GraphQLError>);

pub type Response = Result<ExecutionResult, ResponseError>;

impl Error for ResponseError {}

impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.0
                .iter()
                .map(|v| v.error.message.to_owned())
                .collect::<String>()
        )
    }
}
