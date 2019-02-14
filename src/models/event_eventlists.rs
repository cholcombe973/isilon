#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEventlists {
    #[serde(rename = "eventlists")]
    pub eventlists: Option<Vec <crate::models::EventEventlist>>,
}
