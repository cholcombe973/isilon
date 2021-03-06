#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvidersKrb5 {
    #[serde(rename = "krb5")]
    pub krb5: Option<Vec <crate::models::ProvidersKrb5Krb5Item>>,
}
