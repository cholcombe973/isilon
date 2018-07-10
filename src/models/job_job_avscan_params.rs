#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobJobAvscanParams {
    /// Force files to be scanned, even if excluded by the policy.
    #[serde(rename = "force_run")]
    pub force_run: Option<bool>,
    /// The antivirus scan policy to run.
    #[serde(rename = "policy")]
    pub policy: String,
    /// An optional report id for the scan.
    #[serde(rename = "report_id")]
    pub report_id: Option<String>,
    /// Update the last run time for the policy.
    #[serde(rename = "update")]
    pub update: Option<bool>,
}
