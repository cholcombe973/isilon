#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobJobSnaprevertParams {
    /// Snapshot to revert.
    #[serde(rename = "snapid")]
    pub snapid: i32,
}
