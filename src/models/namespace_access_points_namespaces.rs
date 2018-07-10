#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceAccessPointsNamespaces {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "path")]
    pub path: Option<String>,
}
