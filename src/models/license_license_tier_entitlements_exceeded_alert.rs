#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct LicenseLicenseTierEntitlementsExceededAlert {
    /// Current usage.
    #[serde(rename = "current")]
    pub current: Option<i32>,
    /// Alert type. The unit of measure for the current and licensed fields for capacity is terabytes. For nodes_with_seds_count, it is the number of nodes that have one or more self-encrypting drives.
    #[serde(rename = "issue_type")]
    pub issue_type: String,
    /// Licensed amount.
    #[serde(rename = "licensed")]
    pub licensed: Option<i32>,
}
