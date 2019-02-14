#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobJobCreateParams {
    /// Whether or not to queue the job if one of the same type is already running or queued.
    #[serde(rename = "allow_dup")]
    pub allow_dup: Option<bool>,
    ///
    #[serde(rename = "avscan_params")]
    pub avscan_params: Option <crate::models::JobJobAvscanParams>,
    ///
    #[serde(rename = "changelistcreate_params")]
    pub changelistcreate_params: Option <crate::models::JobJobChangelistcreateParams>,
    ///
    #[serde(rename = "domainmark_params")]
    pub domainmark_params: Option <crate::models::JobJobDomainmarkParams>,
    /// For jobs which take paths, the IFS paths to pass to the job.
    #[serde(rename = "paths")]
    pub paths: Option<Vec<String>>,
    /// Impact policy of this job instance.
    #[serde(rename = "policy")]
    pub policy: Option<String>,
    ///
    #[serde(rename = "prepair_params")]
    pub prepair_params: Option <crate::models::JobJobPrepairParams>,
    /// Priority of this job instance; lower numbers preempt higher numbers.
    #[serde(rename = "priority")]
    pub priority: Option<i32>,
    ///
    #[serde(rename = "smartpoolstree_params")]
    pub smartpoolstree_params: Option <crate::models::JobJobSmartpoolstreeParams>,
    ///
    #[serde(rename = "snaprevert_params")]
    pub snaprevert_params: Option <crate::models::JobJobSnaprevertParams>,
    /// Job type to queue.
    #[serde(rename = "type")]
    pub _type: String,
}
