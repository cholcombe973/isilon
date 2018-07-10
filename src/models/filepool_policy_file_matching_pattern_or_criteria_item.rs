#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolPolicyFileMatchingPatternOrCriteriaItem {
    #[serde(rename = "and_criteria")]
    pub and_criteria: Vec<::models::FilepoolPolicyFileMatchingPatternOrCriteriaItemAndCriteriaItem>,
}
