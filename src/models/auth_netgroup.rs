#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthNetgroup {
    #[serde(rename = "domainname")]
    pub domainname: Option<String>,
    #[serde(rename = "hostname")]
    pub hostname: Option<String>,
    #[serde(rename = "id")]
    pub id: Option<i32>,
    #[serde(rename = "netgroup")]
    pub netgroup: Option<String>,
    #[serde(rename = "username")]
    pub username: Option<String>,
}
