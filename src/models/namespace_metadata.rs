

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceMetadata {
  #[serde(rename = "action")]
  action: Option<String>,
  #[serde(rename = "attrs")]
  attrs: Option<Vec<::models::NamespaceMetadataAttrs>>
}

