#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolSuggestedProtectionSuggestedProtectionItem {
    /// The suggested protection policy.
    #[serde(rename = "suggested_protection")]
    pub suggested_protection: Option<String>,
}
