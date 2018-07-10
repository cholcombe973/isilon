

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceMetadataList {
  #[serde(rename = "attrs")]
  attrs: Option<Vec<::models::NamespaceMetadataListAttrs>>
}

