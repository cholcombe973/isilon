#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct ClusterEmailSettings {
    /// This setting determines how notifications will be batched together to be sent by email.  'none' means each notification will be sent separately.  'severity' means notifications of the same severity will be sent together.  'category' means notifications of the same category will be sent together.  'all' means all notifications will be batched together and sent in a single email.
    #[serde(rename = "batch_mode")]
    pub batch_mode: String,
    /// The address of the SMTP server to be used for relaying the notification messages.  An SMTP server is required in order to send notifications.  If this string is empty, no emails will be sent.
    #[serde(rename = "mail_relay")]
    pub mail_relay: String,
    /// The full email address that will appear as the sender of notification messages.
    #[serde(rename = "mail_sender")]
    pub mail_sender: String,
    /// The subject line for notification messages from this cluster.
    #[serde(rename = "mail_subject")]
    pub mail_subject: String,
    /// Indicates if an SMTP authentication password is set.
    #[serde(rename = "smtp_auth_passwd_set")]
    pub smtp_auth_passwd_set: bool,
    /// The type of secure communication protocol to use if SMTP is being used.  If 'none', plain text will be used, if 'starttls', the encrypted STARTTLS protocol will be used.
    #[serde(rename = "smtp_auth_security")]
    pub smtp_auth_security: String,
    /// Username to authenticate with if SMTP authentication is being used.
    #[serde(rename = "smtp_auth_username")]
    pub smtp_auth_username: String,
    /// The port on the SMTP server to be used for relaying the notification messages.  
    #[serde(rename = "smtp_port")]
    pub smtp_port: i32,
    /// If true, this cluster will send SMTP authentication credentials to the SMTP relay server in order to send its notification emails.  If false, the cluster will attempt to send its notification emails without authentication.
    #[serde(rename = "use_smtp_auth")]
    pub use_smtp_auth: bool,
    /// Location of a custom template file that can be used to specify the layout of the notification emails.
    #[serde(rename = "user_template")]
    pub user_template: Option<String>,
}
