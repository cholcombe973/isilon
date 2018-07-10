#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingImport {
    #[serde(rename = "identities")]
    pub identities: Option<Vec<Vec<String>>>,
}
