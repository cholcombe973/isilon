
/// AntivirusPolicy : An antivirus scan policy.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AntivirusPolicy {
  /// A description for the policy.
  #[serde(rename = "description")]
  description: Option<String>,
  /// Whether the policy is enabled.
  #[serde(rename = "enabled")]
  enabled: Option<bool>,
  /// Forces the scan to run regardless of whether the files were recently scanned.
  #[serde(rename = "force_run")]
  force_run: Option<bool>,
  #[serde(rename = "impact")]
  impact: Option<String>,
  /// The name of the policy.
  #[serde(rename = "name")]
  name: Option<String>,
  /// Paths to include in the scan.
  #[serde(rename = "paths")]
  paths: Option<Vec<String>>,
  /// The depth to recurse in directories.  The default of -1 gives unlimited recursion.
  #[serde(rename = "recursion_depth")]
  recursion_depth: Option<i32>,
  #[serde(rename = "schedule")]
  schedule: Option<String>
}

