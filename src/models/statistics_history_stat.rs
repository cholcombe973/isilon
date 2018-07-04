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
pub struct StatisticsHistoryStat {
  /// Devid of node of statistic or 0 for cluster scoped statistics.
  #[serde(rename = "devid")]
  devid: i32,
  /// Key specific error string, if applicable.
  #[serde(rename = "error")]
  error: Option<String>,
  /// Key specific error number, if applicable.
  #[serde(rename = "error_code")]
  error_code: Option<i32>,
  /// Key name of statistic.
  #[serde(rename = "key")]
  key: String,
  /// The interval for which these results were figured (averaged against.)
  #[serde(rename = "resolution")]
  resolution: i32,
  /// Time-series values.
  #[serde(rename = "values")]
  values: Option<Vec<::models::StatisticsHistoryStatValue>>
}

impl StatisticsHistoryStat {
  pub fn new(devid: i32, key: String, resolution: i32) -> StatisticsHistoryStat {
    StatisticsHistoryStat {
      devid: devid,
      error: None,
      error_code: None,
      key: key,
      resolution: resolution,
      values: None
    }
  }

  pub fn set_devid(&mut self, devid: i32) {
    self.devid = devid;
  }

  pub fn with_devid(mut self, devid: i32) -> StatisticsHistoryStat {
    self.devid = devid;
    self
  }

  pub fn devid(&self) -> &i32 {
    &self.devid
  }


  pub fn set_error(&mut self, error: String) {
    self.error = Some(error);
  }

  pub fn with_error(mut self, error: String) -> StatisticsHistoryStat {
    self.error = Some(error);
    self
  }

  pub fn error(&self) -> Option<&String> {
    self.error.as_ref()
  }

  pub fn reset_error(&mut self) {
    self.error = None;
  }

  pub fn set_error_code(&mut self, error_code: i32) {
    self.error_code = Some(error_code);
  }

  pub fn with_error_code(mut self, error_code: i32) -> StatisticsHistoryStat {
    self.error_code = Some(error_code);
    self
  }

  pub fn error_code(&self) -> Option<&i32> {
    self.error_code.as_ref()
  }

  pub fn reset_error_code(&mut self) {
    self.error_code = None;
  }

  pub fn set_key(&mut self, key: String) {
    self.key = key;
  }

  pub fn with_key(mut self, key: String) -> StatisticsHistoryStat {
    self.key = key;
    self
  }

  pub fn key(&self) -> &String {
    &self.key
  }


  pub fn set_resolution(&mut self, resolution: i32) {
    self.resolution = resolution;
  }

  pub fn with_resolution(mut self, resolution: i32) -> StatisticsHistoryStat {
    self.resolution = resolution;
    self
  }

  pub fn resolution(&self) -> &i32 {
    &self.resolution
  }


  pub fn set_values(&mut self, values: Vec<::models::StatisticsHistoryStatValue>) {
    self.values = Some(values);
  }

  pub fn with_values(mut self, values: Vec<::models::StatisticsHistoryStatValue>) -> StatisticsHistoryStat {
    self.values = Some(values);
    self
  }

  pub fn values(&self) -> Option<&Vec<::models::StatisticsHistoryStatValue>> {
    self.values.as_ref()
  }

  pub fn reset_values(&mut self) {
    self.values = None;
  }

}


