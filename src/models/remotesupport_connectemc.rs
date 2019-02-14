#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct RemotesupportConnectemc {
    ///
    #[serde(rename = "connectemc")]
    pub connectemc: Option <crate::models::RemotesupportConnectemcConnectemc>,
}
