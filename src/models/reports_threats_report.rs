#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportsThreatsReport {
    /// The file that contained the threat.
    #[serde(rename = "file")]
    pub file: Option<String>,
    /// A unique identifier for the report.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The id of the policy that found this threat.
    #[serde(rename = "policy_id")]
    pub policy_id: Option<String>,
    /// The action that was taken to remediate the threat.
    #[serde(rename = "remediation")]
    pub remediation: Option<String>,
    /// The id of the scan report this threat is associated with.
    #[serde(rename = "scan_id")]
    pub scan_id: Option<String>,
    /// A description of the threat that was found.
    #[serde(rename = "threat")]
    pub threat: Option<String>,
    #[serde(rename = "time")]
    pub time: Option<i32>,
}
