
/// FtpSettingsExtended : FTP settings.

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct FtpSettingsExtended {
  /// The timeout, in seconds, for a remote client to establish a PASV style data connection.
  #[serde(rename = "accept_timeout")]
  accept_timeout: Option<i32>,
  /// Controls whether anonymous logins are permitted or not.
  #[serde(rename = "allow_anon_access")]
  allow_anon_access: Option<bool>,
  /// Controls whether anonymous users will be permitted to upload files.
  #[serde(rename = "allow_anon_upload")]
  allow_anon_upload: Option<bool>,
  /// If set to false, all directory list commands will return a permission denied error.
  #[serde(rename = "allow_dirlists")]
  allow_dirlists: Option<bool>,
  /// If set to false, all downloads requests will return a permission denied error.
  #[serde(rename = "allow_downloads")]
  allow_downloads: Option<bool>,
  /// Controls whether local logins are permitted or not.
  #[serde(rename = "allow_local_access")]
  allow_local_access: Option<bool>,
  /// This controls whether any FTP commands which change the filesystem are allowed or not.
  #[serde(rename = "allow_writes")]
  allow_writes: Option<bool>,
  /// This controls whether FTP will always initially change directories to the home directory of the user, regardless of whether it is chroot-ing.
  #[serde(rename = "always_chdir_homedir")]
  always_chdir_homedir: Option<bool>,
  /// This is the name of the user who is given ownership of anonymously uploaded files.
  #[serde(rename = "anon_chown_username")]
  anon_chown_username: Option<String>,
  /// A list of passwords for anonymous users.
  #[serde(rename = "anon_password_list")]
  anon_password_list: Option<Vec<String>>,
  /// This option represents a directory in /ifs which vsftpd will try to change into after an anonymous login.
  #[serde(rename = "anon_root_path")]
  anon_root_path: Option<String>,
  /// The value that the umask for file creation is set to for anonymous users.
  #[serde(rename = "anon_umask")]
  anon_umask: Option<i32>,
  /// Controls whether ascii mode data transfers are honored for various types of requests.
  #[serde(rename = "ascii_mode")]
  ascii_mode: Option<String>,
  /// A list of users that are not chrooted when logging in.
  #[serde(rename = "chroot_exception_list")]
  chroot_exception_list: Option<Vec<String>>,
  /// If set to 'all', all local users will be (by default) placed in a chroot() jail in their home directory after login. If set to 'all-with-exceptions', all local users except those listed in the chroot exception list (isi ftp chroot-exception-list) will be placed in a chroot() jail in their home directory after login. If set to 'none', no local users will be chrooted by default. If set to 'none-with-exceptions', only the local users listed in the chroot exception list (isi ftp chroot-exception-list) will be place in a chroot() jail in their home directory after login.
  #[serde(rename = "chroot_local_mode")]
  chroot_local_mode: Option<String>,
  /// The timeout, in seconds, for a remote client to respond to our PORT style data connection.
  #[serde(rename = "connect_timeout")]
  connect_timeout: Option<i32>,
  /// The timeout, in seconds, which is roughly the maximum time we permit data transfers to stall for with no progress. If the timeout triggers, the remote client is kicked off.
  #[serde(rename = "data_timeout")]
  data_timeout: Option<i32>,
  /// A list of uses that will be denied access.
  #[serde(rename = "denied_user_list")]
  denied_user_list: Option<Vec<String>>,
  /// If enabled, display directory listings with the time in your local time zone. The default is to display GMT. The times returned by the MDTM FTP command are also affected by this option.
  #[serde(rename = "dirlist_localtime")]
  dirlist_localtime: Option<bool>,
  /// When set to 'hide',  all user and group information in directory listings will be displayed as 'ftp'. When set to 'textual', textual names are shown in the user and group fields of directory listings. When set to 'numeric', numeric IDs are show in the user and group fields of directory listings.
  #[serde(rename = "dirlist_names")]
  dirlist_names: Option<String>,
  /// The permissions with which uploaded files are created. Umasks are applied on top of this value.
  #[serde(rename = "file_create_perm")]
  file_create_perm: Option<i32>,
  /// This field determines whether the anon_password_list is used.
  #[serde(rename = "limit_anon_passwords")]
  limit_anon_passwords: Option<bool>,
  /// This option represents a directory in /ifs which vsftpd will try to change into after a local login.
  #[serde(rename = "local_root_path")]
  local_root_path: Option<String>,
  /// The value that the umask for file creation is set to for local users.
  #[serde(rename = "local_umask")]
  local_umask: Option<i32>,
  /// If enabled, allow server-to-server (FXP) transfers.
  #[serde(rename = "server_to_server")]
  server_to_server: Option<bool>,
  /// This field controls whether the FTP daemon is running.
  #[serde(rename = "service")]
  service: Option<bool>,
  /// If enabled, maintain login sessions for each user through Pluggable Authentication Modules (PAM). Disabling this option prevents the ability to do automatic home directory creation if that functionality were otherwise available.
  #[serde(rename = "session_support")]
  session_support: Option<bool>,
  /// The timeout, in seconds, which is roughly the maximum time we permit data transfers to stall for with no progress. If the timeout triggers, the remote client is kicked off.
  #[serde(rename = "session_timeout")]
  session_timeout: Option<i32>,
  /// Specifies the directory where per-user config overrides can be found.
  #[serde(rename = "user_config_dir")]
  user_config_dir: Option<String>
}

