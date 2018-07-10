#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResponse {
    /// ID of created item that can be used to refer to item in the collection-item resource path.
    #[serde(rename = "id")]
    pub id: String,
}
