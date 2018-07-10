

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncJobPolicyFileMatchingPatternOrCriteriaItem {
  /// An array containing individual file criterion objects each describing one criterion.  These are logically AND'ed together to form a set of criteria.
  #[serde(rename = "and_criteria")]
  and_criteria: Option<Vec<::models::SyncJobPolicyFileMatchingPatternOrCriteriaItemAndCriteriaItem>>
}

