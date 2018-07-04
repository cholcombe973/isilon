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
pub struct SyncJobs {
  #[serde(rename = "jobs")]
  jobs: Option<Vec<::models::SyncJobExtended>>
}

impl SyncJobs {
  pub fn new() -> SyncJobs {
    SyncJobs {
      jobs: None
    }
  }

  pub fn set_jobs(&mut self, jobs: Vec<::models::SyncJobExtended>) {
    self.jobs = Some(jobs);
  }

  pub fn with_jobs(mut self, jobs: Vec<::models::SyncJobExtended>) -> SyncJobs {
    self.jobs = Some(jobs);
    self
  }

  pub fn jobs(&self) -> Option<&Vec<::models::SyncJobExtended>> {
    self.jobs.as_ref()
  }

  pub fn reset_jobs(&mut self) {
    self.jobs = None;
  }

}


