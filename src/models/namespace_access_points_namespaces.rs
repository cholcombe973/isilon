

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceAccessPointsNamespaces {
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "path")]
  path: Option<String>
}

