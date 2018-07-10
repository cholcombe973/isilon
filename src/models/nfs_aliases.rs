

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NfsAliases {
  #[serde(rename = "aliases")]
  aliases: Option<Vec<::models::NfsAliasExtended>>
}

