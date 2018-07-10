/// AuditTopic : Specifies the configuration settings for an audit topic.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuditTopic {
    /// Specifies the system-provided ID for the audit topic.
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// Specifies the maximum number of messages that can be sent and received at the same time. Messages that are sent and received at the same time can be lost if a system crash occurs. You can prevent message loss by setting this property to 0, which sets audit logs to synchronous.
    #[serde(rename = "max_cached_messages")]
    pub max_cached_messages: Option<i32>,
}
