

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FsaResults {
  #[serde(rename = "results")]
  results: Option<Vec<::models::FsaResultExtended>>
}

