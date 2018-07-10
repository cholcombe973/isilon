#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct JobPolicyInterval {
    /// Beginning time for the corresponding impact, in the format 'WWWW HH:MM', where 'WWWW' is the full English name of the day of the week, 'HH' is the hour (00-23), and 'MM' is the minute (00-59).
    #[serde(rename = "begin")]
    pub begin: String,
    /// Ending time for the corresponding impact, in the format 'WWWW HH:MM', where 'WWWW' is the full English name of the day of the week, 'HH' is the hour (00-23), and 'MM' is the minute (00-59).
    #[serde(rename = "end")]
    pub end: String,
    /// Impact for the corresponding time span.
    #[serde(rename = "impact")]
    pub impact: String,
}
