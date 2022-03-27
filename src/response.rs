use crate::{execution_result::ExecutionResult, graphql_error::GraphQLError};

pub enum Response {
    Normal(ExecutionResult),
    Error(Vec<GraphQLError>),
}
