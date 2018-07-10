#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSession {
    /// Unique display id.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Text information from probing the session
    #[serde(rename = "probe_info")]
    pub probe_info: Option<String>,
    /// session ID
    #[serde(rename = "session")]
    pub session: Option<String>,
}
