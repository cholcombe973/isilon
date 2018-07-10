#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CopyErrors {
    #[serde(rename = "copy_errors")]
    pub copy_errors: Option<Vec<::models::CopyErrorsCopyErrors>>,
}
