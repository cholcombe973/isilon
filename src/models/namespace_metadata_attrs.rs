#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceMetadataAttrs {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "op")]
    pub op: Option<String>,
    #[serde(rename = "value")]
    pub value: Option<String>,
}
