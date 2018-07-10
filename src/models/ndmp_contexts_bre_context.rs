#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpContextsBreContext {
    /// Unique ID of NDMP BRE context
    #[serde(rename = "bre_context_id")]
    pub bre_context_id: Option<String>,
    /// Unique display id.
    #[serde(rename = "id")]
    pub id: Option<String>,
}
