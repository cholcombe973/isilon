#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventCategories {
    #[serde(rename = "categories")]
    pub categories: Option<Vec <crate::models::EventCategory>>,
}
