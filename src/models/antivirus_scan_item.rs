
/// AntivirusScanItem : Parameters for a manual antivirus scan.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AntivirusScanItem {
  /// The full path of the file to scan.
  #[serde(rename = "file")]
  file: String,
  /// Forces the scan to run regardless of whether the files were recently scanned. The default value is true.
  #[serde(rename = "force_run")]
  force_run: Option<bool>,
  /// The ID of the policy to use for the scan. By default, the scan will use the MANUAL policy.
  #[serde(rename = "policy")]
  policy: Option<String>,
  /// The ID for the report for this scan. A report ID will be generated if one is not provided.
  #[serde(rename = "report_id")]
  report_id: Option<String>
}

