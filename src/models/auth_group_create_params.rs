#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGroupCreateParams {
    /// Specifies the numeric group identifier.
    #[serde(rename = "gid")]
    pub gid: Option<i32>,
    /// Specifies the members of the group.
    #[serde(rename = "members")]
    pub members: Option<Vec<::models::AuthAccessAccessItemFileGroup>>,
    /// Specifies the group name.
    #[serde(rename = "name")]
    pub name: String,
    /// Specifies the security identifier.
    #[serde(rename = "sid")]
    pub sid: Option<String>,
}
