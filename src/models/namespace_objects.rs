

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceObjects {
  #[serde(rename = "children")]
  children: Option<Vec<::models::NamespaceObject>>,
  #[serde(rename = "resume")]
  resume: Option<String>
}

