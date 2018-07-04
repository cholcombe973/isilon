/* 
 * Isilon SDK
 *
 * Isilon SDK - Language bindings for the OneFS API
 *
 * OpenAPI spec version: 5
 * Contact: sdk@isilon.com
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DedupeReports {
  #[serde(rename = "reports")]
  reports: Option<Vec<::models::DedupeReportExtended>>
}

impl DedupeReports {
  pub fn new() -> DedupeReports {
    DedupeReports {
      reports: None
    }
  }

  pub fn set_reports(&mut self, reports: Vec<::models::DedupeReportExtended>) {
    self.reports = Some(reports);
  }

  pub fn with_reports(mut self, reports: Vec<::models::DedupeReportExtended>) -> DedupeReports {
    self.reports = Some(reports);
    self
  }

  pub fn reports(&self) -> Option<&Vec<::models::DedupeReportExtended>> {
    self.reports.as_ref()
  }

  pub fn reset_reports(&mut self) {
    self.reports = None;
  }

}


