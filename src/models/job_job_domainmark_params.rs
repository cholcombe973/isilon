#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobJobDomainmarkParams {
    /// Whether this is a delete operation.
    #[serde(rename = "delete")]
    pub delete: Option<bool>,
    /// Base IFS path to associate with the domain.
    #[serde(rename = "root")]
    pub root: String,
    /// The type of domain.
    #[serde(rename = "type")]
    pub _type: String,
}
