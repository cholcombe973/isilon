# FtpSettingsExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**accept_timeout** | **i32** | The timeout, in seconds, for a remote client to establish a PASV style data connection. | [optional] [default to null]
**allow_anon_access** | **bool** | Controls whether anonymous logins are permitted or not. | [optional] [default to null]
**allow_anon_upload** | **bool** | Controls whether anonymous users will be permitted to upload files. | [optional] [default to null]
**allow_dirlists** | **bool** | If set to false, all directory list commands will return a permission denied error. | [optional] [default to null]
**allow_downloads** | **bool** | If set to false, all downloads requests will return a permission denied error. | [optional] [default to null]
**allow_local_access** | **bool** | Controls whether local logins are permitted or not. | [optional] [default to null]
**allow_writes** | **bool** | This controls whether any FTP commands which change the filesystem are allowed or not. | [optional] [default to null]
**always_chdir_homedir** | **bool** | This controls whether FTP will always initially change directories to the home directory of the user, regardless of whether it is chroot-ing. | [optional] [default to null]
**anon_chown_username** | **String** | This is the name of the user who is given ownership of anonymously uploaded files. | [optional] [default to null]
**anon_password_list** | **Vec<String>** | A list of passwords for anonymous users. | [optional] [default to null]
**anon_root_path** | **String** | This option represents a directory in /ifs which vsftpd will try to change into after an anonymous login. | [optional] [default to null]
**anon_umask** | **i32** | The value that the umask for file creation is set to for anonymous users. | [optional] [default to null]
**ascii_mode** | **String** | Controls whether ascii mode data transfers are honored for various types of requests. | [optional] [default to null]
**chroot_exception_list** | **Vec<String>** | A list of users that are not chrooted when logging in. | [optional] [default to null]
**chroot_local_mode** | **String** | If set to &#39;all&#39;, all local users will be (by default) placed in a chroot() jail in their home directory after login. If set to &#39;all-with-exceptions&#39;, all local users except those listed in the chroot exception list (isi ftp chroot-exception-list) will be placed in a chroot() jail in their home directory after login. If set to &#39;none&#39;, no local users will be chrooted by default. If set to &#39;none-with-exceptions&#39;, only the local users listed in the chroot exception list (isi ftp chroot-exception-list) will be place in a chroot() jail in their home directory after login. | [optional] [default to null]
**connect_timeout** | **i32** | The timeout, in seconds, for a remote client to respond to our PORT style data connection. | [optional] [default to null]
**data_timeout** | **i32** | The timeout, in seconds, which is roughly the maximum time we permit data transfers to stall for with no progress. If the timeout triggers, the remote client is kicked off. | [optional] [default to null]
**denied_user_list** | **Vec<String>** | A list of uses that will be denied access. | [optional] [default to null]
**dirlist_localtime** | **bool** | If enabled, display directory listings with the time in your local time zone. The default is to display GMT. The times returned by the MDTM FTP command are also affected by this option. | [optional] [default to null]
**dirlist_names** | **String** | When set to &#39;hide&#39;,  all user and group information in directory listings will be displayed as &#39;ftp&#39;. When set to &#39;textual&#39;, textual names are shown in the user and group fields of directory listings. When set to &#39;numeric&#39;, numeric IDs are show in the user and group fields of directory listings. | [optional] [default to null]
**file_create_perm** | **i32** | The permissions with which uploaded files are created. Umasks are applied on top of this value. | [optional] [default to null]
**limit_anon_passwords** | **bool** | This field determines whether the anon_password_list is used. | [optional] [default to null]
**local_root_path** | **String** | This option represents a directory in /ifs which vsftpd will try to change into after a local login. | [optional] [default to null]
**local_umask** | **i32** | The value that the umask for file creation is set to for local users. | [optional] [default to null]
**server_to_server** | **bool** | If enabled, allow server-to-server (FXP) transfers. | [optional] [default to null]
**service** | **bool** | This field controls whether the FTP daemon is running. | [optional] [default to null]
**session_support** | **bool** | If enabled, maintain login sessions for each user through Pluggable Authentication Modules (PAM). Disabling this option prevents the ability to do automatic home directory creation if that functionality were otherwise available. | [optional] [default to null]
**session_timeout** | **i32** | The timeout, in seconds, which is roughly the maximum time we permit data transfers to stall for with no progress. If the timeout triggers, the remote client is kicked off. | [optional] [default to null]
**user_config_dir** | **String** | Specifies the directory where per-user config overrides can be found. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


