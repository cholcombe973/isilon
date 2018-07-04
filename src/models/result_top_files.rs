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
pub struct ResultTopFiles {
  /// Access time enabled.
  #[serde(rename = "atime_enabled")]
  atime_enabled: bool,
  /// Change in file ranking from result set comparison.
  #[serde(rename = "change")]
  change: Option<i32>,
  /// Directory access time enabled.
  #[serde(rename = "dir_atime_enabled")]
  dir_atime_enabled: bool,
  /// Files listing.
  #[serde(rename = "files")]
  files: Vec<::models::ResultTopFilesFile>,
  /// Limit on number of top results.
  #[serde(rename = "top_n_max")]
  top_n_max: i32,
  /// Total count of file results.
  #[serde(rename = "total_count")]
  total_count: i32
}

impl ResultTopFiles {
  pub fn new(atime_enabled: bool, dir_atime_enabled: bool, files: Vec<::models::ResultTopFilesFile>, top_n_max: i32, total_count: i32) -> ResultTopFiles {
    ResultTopFiles {
      atime_enabled: atime_enabled,
      change: None,
      dir_atime_enabled: dir_atime_enabled,
      files: files,
      top_n_max: top_n_max,
      total_count: total_count
    }
  }

  pub fn set_atime_enabled(&mut self, atime_enabled: bool) {
    self.atime_enabled = atime_enabled;
  }

  pub fn with_atime_enabled(mut self, atime_enabled: bool) -> ResultTopFiles {
    self.atime_enabled = atime_enabled;
    self
  }

  pub fn atime_enabled(&self) -> &bool {
    &self.atime_enabled
  }


  pub fn set_change(&mut self, change: i32) {
    self.change = Some(change);
  }

  pub fn with_change(mut self, change: i32) -> ResultTopFiles {
    self.change = Some(change);
    self
  }

  pub fn change(&self) -> Option<&i32> {
    self.change.as_ref()
  }

  pub fn reset_change(&mut self) {
    self.change = None;
  }

  pub fn set_dir_atime_enabled(&mut self, dir_atime_enabled: bool) {
    self.dir_atime_enabled = dir_atime_enabled;
  }

  pub fn with_dir_atime_enabled(mut self, dir_atime_enabled: bool) -> ResultTopFiles {
    self.dir_atime_enabled = dir_atime_enabled;
    self
  }

  pub fn dir_atime_enabled(&self) -> &bool {
    &self.dir_atime_enabled
  }


  pub fn set_files(&mut self, files: Vec<::models::ResultTopFilesFile>) {
    self.files = files;
  }

  pub fn with_files(mut self, files: Vec<::models::ResultTopFilesFile>) -> ResultTopFiles {
    self.files = files;
    self
  }

  pub fn files(&self) -> &Vec<::models::ResultTopFilesFile> {
    &self.files
  }


  pub fn set_top_n_max(&mut self, top_n_max: i32) {
    self.top_n_max = top_n_max;
  }

  pub fn with_top_n_max(mut self, top_n_max: i32) -> ResultTopFiles {
    self.top_n_max = top_n_max;
    self
  }

  pub fn top_n_max(&self) -> &i32 {
    &self.top_n_max
  }


  pub fn set_total_count(&mut self, total_count: i32) {
    self.total_count = total_count;
  }

  pub fn with_total_count(mut self, total_count: i32) -> ResultTopFiles {
    self.total_count = total_count;
    self
  }

  pub fn total_count(&self) -> &i32 {
    &self.total_count
  }


}


