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
pub struct SummarySystem {
  #[serde(rename = "system")]
  system: Option<Vec<::models::SummarySystemSystemItem>>
}

impl SummarySystem {
  pub fn new() -> SummarySystem {
    SummarySystem {
      system: None
    }
  }

  pub fn set_system(&mut self, system: Vec<::models::SummarySystemSystemItem>) {
    self.system = Some(system);
  }

  pub fn with_system(mut self, system: Vec<::models::SummarySystemSystemItem>) -> SummarySystem {
    self.system = Some(system);
    self
  }

  pub fn system(&self) -> Option<&Vec<::models::SummarySystemSystemItem>> {
    self.system.as_ref()
  }

  pub fn reset_system(&mut self) {
    self.system = None;
  }

}


