/// CloudJob : A cloud job for archiving or recalling files

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudJob {
    /// Whether to apply to the given operation type or to all jobs of the given operation type
    #[serde(rename = "all")]
    pub all: Option<bool>,
    /// The desired state of the job or operation
    #[serde(rename = "state")]
    pub state: Option<String>,
}
