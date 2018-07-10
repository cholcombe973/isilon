

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoragepoolSuggestedProtection {
  #[serde(rename = "suggested_protection")]
  suggested_protection: Option<Vec<::models::StoragepoolSuggestedProtectionSuggestedProtectionItem>>
}

