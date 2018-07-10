

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobJobCreateParams {
  /// Whether or not to queue the job if one of the same type is already running or queued.
  #[serde(rename = "allow_dup")]
  allow_dup: Option<bool>,
  /// 
  #[serde(rename = "avscan_params")]
  avscan_params: Option<::models::JobJobAvscanParams>,
  /// 
  #[serde(rename = "changelistcreate_params")]
  changelistcreate_params: Option<::models::JobJobChangelistcreateParams>,
  /// 
  #[serde(rename = "domainmark_params")]
  domainmark_params: Option<::models::JobJobDomainmarkParams>,
  /// For jobs which take paths, the IFS paths to pass to the job.
  #[serde(rename = "paths")]
  paths: Option<Vec<String>>,
  /// Impact policy of this job instance.
  #[serde(rename = "policy")]
  policy: Option<String>,
  /// 
  #[serde(rename = "prepair_params")]
  prepair_params: Option<::models::JobJobPrepairParams>,
  /// Priority of this job instance; lower numbers preempt higher numbers.
  #[serde(rename = "priority")]
  priority: Option<i32>,
  /// 
  #[serde(rename = "smartpoolstree_params")]
  smartpoolstree_params: Option<::models::JobJobSmartpoolstreeParams>,
  /// 
  #[serde(rename = "snaprevert_params")]
  snaprevert_params: Option<::models::JobJobSnaprevertParams>,
  /// Job type to queue.
  #[serde(rename = "type")]
  _type: String
}

