#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncJobPolicyFileMatchingPatternOrCriteriaItemAndCriteriaItem {
    /// For \"custom_attribute\" type criteria.  The file will match as long as the attribute named by \"field\" exists.  Default is true.
    #[serde(rename = "attribute_exists")]
    pub attribute_exists: Option<bool>,
    /// If true, the value comparison will be case sensitive.  Default is true.
    #[serde(rename = "case_sensitive")]
    pub case_sensitive: Option<bool>,
    /// The name of the file attribute to match on (only required if this is a custom_attribute type criterion).  Default is an empty string \"\".
    #[serde(rename = "field")]
    pub field: Option<String>,
    /// How to compare the specified attribute of each file to the specified value.
    #[serde(rename = "operator")]
    pub operator: Option<String>,
    /// The type of this criterion, that is, which file attribute to match on.
    #[serde(rename = "type")]
    pub _type: String,
    /// The value to compare the specified attribute of each file to.
    #[serde(rename = "value")]
    pub value: Option<String>,
    /// If true, the attribute must match the entire word.  Default is true.
    #[serde(rename = "whole_word")]
    pub whole_word: Option<bool>,
}
