

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceMetadataAttrs {
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "namespace")]
  namespace: Option<String>,
  #[serde(rename = "op")]
  op: Option<String>,
  #[serde(rename = "value")]
  value: Option<String>
}

