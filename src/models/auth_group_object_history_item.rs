#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthGroupObjectHistoryItem {
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "gid")]
    pub gid: Option<::models::AuthAccessAccessItemFileGroup>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "sid")]
    pub sid: Option<::models::AuthAccessAccessItemFileGroup>,
    /// Specifies properties for a persona, which consists of either a 'type' and a 'name' or an 'ID'.
    #[serde(rename = "uid")]
    pub uid: Option<::models::AuthAccessAccessItemFileGroup>,
}
