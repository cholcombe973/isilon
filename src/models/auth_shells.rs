
/// AuthShells : The list of supported shells.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthShells {
  #[serde(rename = "shells")]
  shells: Option<Vec<String>>
}

