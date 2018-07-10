

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncJobPolicyFileMatchingPattern {
  /// An array containing objects with \"and_criteria\" properties, each set of and_criteria will be logically OR'ed together to create the full file matching pattern.
  #[serde(rename = "or_criteria")]
  or_criteria: Option<Vec<::models::SyncJobPolicyFileMatchingPatternOrCriteriaItem>>
}

