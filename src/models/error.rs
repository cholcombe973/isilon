

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
  #[serde(rename = "code")]
  code: i32,
  #[serde(rename = "message")]
  message: String
}

