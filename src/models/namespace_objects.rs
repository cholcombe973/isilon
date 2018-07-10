#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceObjects {
    #[serde(rename = "children")]
    pub children: Option<Vec<::models::NamespaceObject>>,
    #[serde(rename = "resume")]
    pub resume: Option<String>,
}
