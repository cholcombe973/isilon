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
pub struct SnapshotAliasesExtended {
  #[serde(rename = "aliases")]
  aliases: Option<Vec<::models::SnapshotAliasExtended>>,
  /// Resume token value to use in subsequent calls for continuation.
  #[serde(rename = "resume")]
  resume: Option<String>,
  /// Total number of items available.
  #[serde(rename = "total")]
  total: Option<i32>
}

impl SnapshotAliasesExtended {
  pub fn new() -> SnapshotAliasesExtended {
    SnapshotAliasesExtended {
      aliases: None,
      resume: None,
      total: None
    }
  }

  pub fn set_aliases(&mut self, aliases: Vec<::models::SnapshotAliasExtended>) {
    self.aliases = Some(aliases);
  }

  pub fn with_aliases(mut self, aliases: Vec<::models::SnapshotAliasExtended>) -> SnapshotAliasesExtended {
    self.aliases = Some(aliases);
    self
  }

  pub fn aliases(&self) -> Option<&Vec<::models::SnapshotAliasExtended>> {
    self.aliases.as_ref()
  }

  pub fn reset_aliases(&mut self) {
    self.aliases = None;
  }

  pub fn set_resume(&mut self, resume: String) {
    self.resume = Some(resume);
  }

  pub fn with_resume(mut self, resume: String) -> SnapshotAliasesExtended {
    self.resume = Some(resume);
    self
  }

  pub fn resume(&self) -> Option<&String> {
    self.resume.as_ref()
  }

  pub fn reset_resume(&mut self) {
    self.resume = None;
  }

  pub fn set_total(&mut self, total: i32) {
    self.total = Some(total);
  }

  pub fn with_total(mut self, total: i32) -> SnapshotAliasesExtended {
    self.total = Some(total);
    self
  }

  pub fn total(&self) -> Option<&i32> {
    self.total.as_ref()
  }

  pub fn reset_total(&mut self) {
    self.total = None;
  }

}


