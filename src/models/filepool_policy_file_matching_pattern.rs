#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolPolicyFileMatchingPattern {
    #[serde(rename = "or_criteria")]
    pub or_criteria: Vec <crate::models::FilepoolPolicyFileMatchingPatternOrCriteriaItem>,
}
