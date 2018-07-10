

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobJobs {
  #[serde(rename = "jobs")]
  jobs: Option<Vec<::models::JobJobExtended>>
}

