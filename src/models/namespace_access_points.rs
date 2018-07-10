

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceAccessPoints {
  #[serde(rename = "namespaces")]
  namespaces: Option<Vec<::models::NamespaceAccessPointsNamespaces>>,
  #[serde(rename = "versions")]
  versions: Option<Vec<String>>
}

