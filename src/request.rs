use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Request<TVariables = (), TExtensions = ()> {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<TVariables>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<TExtensions>,
}
