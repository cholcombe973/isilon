#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthWellknowns {
    #[serde(rename = "wellknowns")]
    pub wellknowns: Option<Vec <crate::models::AuthAccessAccessItemFileGroup>>,
}
