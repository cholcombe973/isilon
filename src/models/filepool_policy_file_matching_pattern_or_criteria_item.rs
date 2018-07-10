

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolPolicyFileMatchingPatternOrCriteriaItem {
  #[serde(rename = "and_criteria")]
  and_criteria: Vec<::models::FilepoolPolicyFileMatchingPatternOrCriteriaItemAndCriteriaItem>
}

