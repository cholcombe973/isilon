

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolPolicyFileMatchingPattern {
  #[serde(rename = "or_criteria")]
  or_criteria: Vec<::models::FilepoolPolicyFileMatchingPatternOrCriteriaItem>
}

