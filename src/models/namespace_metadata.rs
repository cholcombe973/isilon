#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceMetadata {
    #[serde(rename = "action")]
    pub action: Option<String>,
    #[serde(rename = "attrs")]
    pub attrs: Option<Vec<::models::NamespaceMetadataAttrs>>,
}
