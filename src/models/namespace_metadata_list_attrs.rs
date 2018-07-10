#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceMetadataListAttrs {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "namespace")]
    pub namespace: Option<String>,
    #[serde(rename = "value")]
    pub value: Option<String>,
}
