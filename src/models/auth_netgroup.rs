

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthNetgroup {
  #[serde(rename = "domainname")]
  domainname: Option<String>,
  #[serde(rename = "hostname")]
  hostname: Option<String>,
  #[serde(rename = "id")]
  id: Option<i32>,
  #[serde(rename = "netgroup")]
  netgroup: Option<String>,
  #[serde(rename = "username")]
  username: Option<String>
}

