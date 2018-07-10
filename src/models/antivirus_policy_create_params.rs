#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AntivirusPolicyCreateParams {
    /// A description for the policy.
    #[serde(rename = "description")]
    pub description: Option<String>,
    /// Whether the policy is enabled.
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    /// Forces the scan to run regardless of whether the files were recently scanned.
    #[serde(rename = "force_run")]
    pub force_run: Option<bool>,
    #[serde(rename = "impact")]
    pub impact: Option<String>,
    /// The name of the policy.
    #[serde(rename = "name")]
    pub name: String,
    /// Paths to include in the scan.
    #[serde(rename = "paths")]
    pub paths: Option<Vec<String>>,
    /// The depth to recurse in directories.  The default of -1 gives unlimited recursion.
    #[serde(rename = "recursion_depth")]
    pub recursion_depth: Option<i32>,
    #[serde(rename = "schedule")]
    pub schedule: Option<String>,
}
