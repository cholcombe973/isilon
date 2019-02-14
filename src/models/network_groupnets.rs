#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkGroupnets {
    #[serde(rename = "groupnets")]
    pub groupnets: Option<Vec <crate::models::NetworkGroupnetExtended>>,
}
