

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudJobExtended {
  /// The time at which the job was completed (if applicable)
  #[serde(rename = "completion_time")]
  completion_time: Option<i32>,
  /// The time at which the job was created
  #[serde(rename = "create_time")]
  create_time: Option<i32>,
  /// A brief description of the job contents
  #[serde(rename = "description")]
  description: Option<String>,
  /// The effective state of the job (e.g,. the combination of operation_state and job_state)
  #[serde(rename = "effective_state")]
  effective_state: Option<String>,
  /// The files and filters addressed by this job
  #[serde(rename = "files")]
  files: Option<::models::CloudJobFiles>,
  /// The job's ID
  #[serde(rename = "id")]
  id: Option<i32>,
  /// Information about the related job engine job if there is one
  #[serde(rename = "job_engine_job")]
  job_engine_job: Option<::models::CloudJobJobEngineJob>,
  /// The current state of the job
  #[serde(rename = "job_state")]
  job_state: Option<String>,
  /// The current state of the operation associated with the job
  #[serde(rename = "operation_state")]
  operation_state: Option<String>,
  /// The last time at which the job state changed
  #[serde(rename = "state_change_time")]
  state_change_time: Option<i32>,
  /// The type of cloud action to be performed by this job.
  #[serde(rename = "type")]
  _type: Option<String>
}

