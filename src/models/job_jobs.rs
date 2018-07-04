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
pub struct JobJobs {
  #[serde(rename = "jobs")]
  jobs: Option<Vec<::models::JobJobExtended>>
}

impl JobJobs {
  pub fn new() -> JobJobs {
    JobJobs {
      jobs: None
    }
  }

  pub fn set_jobs(&mut self, jobs: Vec<::models::JobJobExtended>) {
    self.jobs = Some(jobs);
  }

  pub fn with_jobs(mut self, jobs: Vec<::models::JobJobExtended>) -> JobJobs {
    self.jobs = Some(jobs);
    self
  }

  pub fn jobs(&self) -> Option<&Vec<::models::JobJobExtended>> {
    self.jobs.as_ref()
  }

  pub fn reset_jobs(&mut self) {
    self.jobs = None;
  }

}


