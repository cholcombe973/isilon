
/// CreateAntivirusScanItemResponse : The result of a manual antivirus scan.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAntivirusScanItemResponse {
  /// The ID for the report for this scan. A report ID will be generated if one is not provided. 
  #[serde(rename = "report_id")]
  report_id: Option<String>,
  /// The result of the scan.
  #[serde(rename = "result")]
  result: Option<String>
}

