#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct HardeningStateState {
    /// Full path name of issues file, or null if no issues file is configured. This file contains all issues found when the cluster configuration is checked against expected configuration.
    #[serde(rename = "issues_file")]
    pub issues_file: Option<String>,
    /// This contains more information and details about the operation state.
    #[serde(rename = "message")]
    pub message: Option<String>,
    /// The state of the hardening operation. In case there is no operation currently going on, this will display the last state of operation.
    #[serde(rename = "state")]
    pub state: Option<String>,
}
