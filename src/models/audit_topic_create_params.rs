/// AuditTopicCreateParams : Specifies the configuration settings for an audit topic.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditTopicCreateParams {
    /// Specifies the maximum number of messages that can be sent and received at the same time. Messages that are sent and received at the same time can be lost if a system crash occurs. You can prevent message loss by setting this property to 0, which sets audit logs to synchronous.
    #[serde(rename = "max_cached_messages")]
    pub max_cached_messages: Option<i32>,
    /// Specifies the name of the audit topic.
    #[serde(rename = "name")]
    pub name: String,
}
