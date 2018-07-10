

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyErrorsCopyErrors {
  #[serde(rename = "error_src")]
  error_src: Option<String>,
  #[serde(rename = "message")]
  message: Option<String>,
  #[serde(rename = "source")]
  source: Option<String>,
  #[serde(rename = "target")]
  target: Option<String>
}

