#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobJob {
    /// Impact policy of this job instance.
    #[serde(rename = "policy")]
    pub policy: Option<String>,
    /// Priority of this job instance; lower numbers preempt higher numbers.
    #[serde(rename = "priority")]
    pub priority: Option<i32>,
    /// Desired new state of this job instance.
    #[serde(rename = "state")]
    pub state: Option<String>,
}
