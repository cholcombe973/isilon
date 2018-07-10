#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilepoolPolicyFileMatchingPatternOrCriteriaItemAndCriteriaItem {
    /// Indicates whether the existence of an attribute indicates a match (valid only with 'type' = 'custom_attribute')
    #[serde(rename = "attribute_exists")]
    pub attribute_exists: Option<bool>,
    /// True to match files recursively under the given path. (valid only with 'type' = 'path')
    #[serde(rename = "begins_with")]
    pub begins_with: Option<bool>,
    /// True to indicate case sensitivity when comparing file attributes (valid only with 'type' = 'name' or 'type' = 'path')
    #[serde(rename = "case_sensitive")]
    pub case_sensitive: Option<bool>,
    /// File attribute field name to be compared in a custom comparison (valid only with 'type' = 'custom_attribute')
    #[serde(rename = "field")]
    pub field: Option<String>,
    /// The comparison operator to use while comparing an attribute with its value
    #[serde(rename = "operator")]
    pub operator: Option<String>,
    /// The file attribute to be compared to a given value
    #[serde(rename = "type")]
    pub _type: String,
    /// Size unit value. One of 'B','KB','MB','GB','TB','PB','EB' (valid only with 'type' = 'size')
    #[serde(rename = "units")]
    pub units: Option<String>,
    /// Whether time units refer to a calendar date and time (e.g., Jun 3, 2009) or a relative duration (e.g., 2 weeks) (valid only with 'type' in {accessed_time, birth_time, changed_time or metadata_changed_time}
    #[serde(rename = "use_relative_time")]
    pub use_relative_time: Option<bool>,
    /// The value to be compared against a file attribute
    #[serde(rename = "value")]
    pub value: Option<String>,
}
