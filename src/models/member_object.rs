

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberObject {
  #[serde(rename = "id")]
  id: Option<String>,
  #[serde(rename = "name")]
  name: Option<String>,
  #[serde(rename = "type")]
  _type: Option<String>
}

