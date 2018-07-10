

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobJobExtended {
  /// State to which the job is transitioning; if control_state is identical to state, the job's state is stable.
  #[serde(rename = "control_state")]
  control_state: Option<String>,
  /// The time the job was queued, in seconds since the epoch.
  #[serde(rename = "create_time")]
  create_time: i32,
  /// The current phase of the job.
  #[serde(rename = "current_phase")]
  current_phase: Option<i32>,
  /// A text representation of the job.
  #[serde(rename = "description")]
  description: Option<String>,
  /// The time the job ended, in seconds since the Epoch.
  #[serde(rename = "end_time")]
  end_time: Option<i32>,
  /// The ID of the job.
  #[serde(rename = "id")]
  id: i32,
  /// The current impact of the job.
  #[serde(rename = "impact")]
  impact: String,
  /// The set of devids working on the job.
  #[serde(rename = "participants")]
  participants: Option<Vec<i32>>,
  /// Paths for which the job was queued.
  #[serde(rename = "paths")]
  paths: Option<Vec<String>>,
  /// Current impact policy of the job.
  #[serde(rename = "policy")]
  policy: String,
  /// Current priority of the job; lower numbers preempt higher numbers.
  #[serde(rename = "priority")]
  priority: i32,
  /// A text representation of the job's progress.
  #[serde(rename = "progress")]
  progress: Option<String>,
  /// The number of retries remaining if the job fails.
  #[serde(rename = "retries_remaining")]
  retries_remaining: i32,
  /// The number of seconds the job has executed.
  #[serde(rename = "running_time")]
  running_time: Option<i32>,
  /// The time the job started, in seconds since the Epoch.
  #[serde(rename = "start_time")]
  start_time: Option<i32>,
  /// Current state of the job.
  #[serde(rename = "state")]
  state: String,
  /// The total number of phases of the job type.
  #[serde(rename = "total_phases")]
  total_phases: i32,
  /// The job type.
  #[serde(rename = "type")]
  _type: String,
  /// The ID of a job for which this job is waiting.
  #[serde(rename = "waiting_on")]
  waiting_on: Option<i32>,
  /// The reason the job is waiting.
  #[serde(rename = "waiting_reason")]
  waiting_reason: Option<String>
}

