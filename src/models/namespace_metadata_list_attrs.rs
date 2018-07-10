

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceMetadataListAttrs {
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "namespace")]
  namespace: Option<String>,
  #[serde(rename = "value")]
  value: Option<String>
}

