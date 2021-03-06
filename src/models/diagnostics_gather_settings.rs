#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosticsGatherSettings {
    /// Use ESRS for upload of gather.
    #[serde(rename = "esrs")]
    pub esrs: Option<bool>,
    #[serde(rename = "ftp_upload")]
    pub ftp_upload: Option<bool>,
    /// Alternate FTP host to upload to.
    #[serde(rename = "ftp_upload_host")]
    pub ftp_upload_host: Option<String>,
    /// FTP upload mode.
    #[serde(rename = "ftp_upload_mode")]
    pub ftp_upload_mode: Option<String>,
    /// Alternate FTP path to upload to.
    #[serde(rename = "ftp_upload_path")]
    pub ftp_upload_path: Option<String>,
    /// FTP proxy to use for upload.
    #[serde(rename = "ftp_upload_proxy")]
    pub ftp_upload_proxy: Option<String>,
    /// FTP proxy port to use for upload.
    #[serde(rename = "ftp_upload_proxy_port")]
    pub ftp_upload_proxy_port: Option<i32>,
    /// FTP user for upload.
    #[serde(rename = "ftp_upload_user")]
    pub ftp_upload_user: Option<String>,
    /// Set gather to full or incremental.
    #[serde(rename = "gather_mode")]
    pub gather_mode: Option<String>,
    /// Whether or not to use HTTP upload on completed gather.
    #[serde(rename = "http_upload")]
    pub http_upload: Option<bool>,
    /// Alternate HTTP Host to upload to.
    #[serde(rename = "http_upload_host")]
    pub http_upload_host: Option<String>,
    /// Alternate path to write gather to.
    #[serde(rename = "http_upload_path")]
    pub http_upload_path: Option<String>,
    /// Proxy to use for HTTP upload.
    #[serde(rename = "http_upload_proxy")]
    pub http_upload_proxy: Option<String>,
    /// Alternate port for proxy server
    #[serde(rename = "http_upload_proxy_port")]
    pub http_upload_proxy_port: Option<i32>,
    /// Upload gather to EMC.
    #[serde(rename = "upload")]
    pub upload: Option<bool>,
}
