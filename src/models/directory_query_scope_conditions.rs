#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryQueryScopeConditions {
    #[serde(rename = "attr")]
    pub attr: Option<String>,
    #[serde(rename = "operator")]
    pub operator: Option<String>,
    #[serde(rename = "value")]
    pub value: Option<String>,
}
