/// NdmpSessions : View probe info of a NDMP session

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NdmpSessions {
    #[serde(rename = "sessions")]
    pub sessions: Option<Vec<::models::NdmpSession>>,
}
