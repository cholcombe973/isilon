#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolSuggestedProtection {
    #[serde(rename = "suggested_protection")]
    pub suggested_protection:
        Option<Vec <crate::models::StoragepoolSuggestedProtectionSuggestedProtectionItem>>,
}
