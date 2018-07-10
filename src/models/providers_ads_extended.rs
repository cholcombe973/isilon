#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersAdsExtended {
    #[serde(rename = "ads")]
    pub ads: Option<Vec<::models::ProvidersAdsAdsItemExtended>>,
}
