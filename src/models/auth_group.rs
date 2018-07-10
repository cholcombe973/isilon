/// AuthGroup : Specifies the configuration properties for a group.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGroup {
    /// Specifies the numeric group identifier.
    #[serde(rename = "gid")]
    pub gid: Option<i32>,
}
