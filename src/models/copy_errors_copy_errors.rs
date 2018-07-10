#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyErrorsCopyErrors {
    #[serde(rename = "error_src")]
    pub error_src: Option<String>,
    #[serde(rename = "message")]
    pub message: Option<String>,
    #[serde(rename = "source")]
    pub source: Option<String>,
    #[serde(rename = "target")]
    pub target: Option<String>,
}
