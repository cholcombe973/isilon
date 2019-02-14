# \ProtocolsApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_hdfs_proxyuser**](ProtocolsApi.md#create_hdfs_proxyuser) | **Post** /platform/1/protocols/hdfs/proxyusers | 
[**create_hdfs_rack**](ProtocolsApi.md#create_hdfs_rack) | **Post** /platform/1/protocols/hdfs/racks | 
[**create_ndmp_settings_preferred_ip**](ProtocolsApi.md#create_ndmp_settings_preferred_ip) | **Post** /platform/4/protocols/ndmp/settings/preferred-ips | 
[**create_ndmp_settings_variable**](ProtocolsApi.md#create_ndmp_settings_variable) | **Post** /platform/3/protocols/ndmp/settings/variables/{NdmpSettingsVariableId} | 
[**create_ndmp_user**](ProtocolsApi.md#create_ndmp_user) | **Post** /platform/3/protocols/ndmp/users | 
[**create_nfs_alias**](ProtocolsApi.md#create_nfs_alias) | **Post** /platform/2/protocols/nfs/aliases | 
[**create_nfs_export**](ProtocolsApi.md#create_nfs_export) | **Post** /platform/4/protocols/nfs/exports | 
[**create_nfs_netgroup_check_item**](ProtocolsApi.md#create_nfs_netgroup_check_item) | **Post** /platform/3/protocols/nfs/netgroup/check | 
[**create_nfs_netgroup_flush_item**](ProtocolsApi.md#create_nfs_netgroup_flush_item) | **Post** /platform/3/protocols/nfs/netgroup/flush | 
[**create_nfs_nlm_sessions_check_item**](ProtocolsApi.md#create_nfs_nlm_sessions_check_item) | **Post** /platform/3/protocols/nfs/nlm/sessions-check | 
[**create_nfs_reload_item**](ProtocolsApi.md#create_nfs_reload_item) | **Post** /platform/3/protocols/nfs/reload | 
[**create_ntp_server**](ProtocolsApi.md#create_ntp_server) | **Post** /platform/3/protocols/ntp/servers | 
[**create_smb_log_level_filter**](ProtocolsApi.md#create_smb_log_level_filter) | **Post** /platform/3/protocols/smb/log-level/filters | 
[**create_smb_share**](ProtocolsApi.md#create_smb_share) | **Post** /platform/4/protocols/smb/shares | 
[**create_swift_account**](ProtocolsApi.md#create_swift_account) | **Post** /platform/3/protocols/swift/accounts | 
[**delete_hdfs_proxyuser**](ProtocolsApi.md#delete_hdfs_proxyuser) | **Delete** /platform/1/protocols/hdfs/proxyusers/{HdfsProxyuserId} | 
[**delete_hdfs_rack**](ProtocolsApi.md#delete_hdfs_rack) | **Delete** /platform/1/protocols/hdfs/racks/{HdfsRackId} | 
[**delete_ndmp_contexts_backup_by_id**](ProtocolsApi.md#delete_ndmp_contexts_backup_by_id) | **Delete** /platform/3/protocols/ndmp/contexts/backup/{NdmpContextsBackupId} | 
[**delete_ndmp_contexts_bre_by_id**](ProtocolsApi.md#delete_ndmp_contexts_bre_by_id) | **Delete** /platform/3/protocols/ndmp/contexts/bre/{NdmpContextsBreId} | 
[**delete_ndmp_contexts_restore_by_id**](ProtocolsApi.md#delete_ndmp_contexts_restore_by_id) | **Delete** /platform/3/protocols/ndmp/contexts/restore/{NdmpContextsRestoreId} | 
[**delete_ndmp_dumpdate**](ProtocolsApi.md#delete_ndmp_dumpdate) | **Delete** /platform/3/protocols/ndmp/dumpdates/{NdmpDumpdateId} | 
[**delete_ndmp_session**](ProtocolsApi.md#delete_ndmp_session) | **Delete** /platform/3/protocols/ndmp/sessions/{NdmpSessionId} | 
[**delete_ndmp_settings_preferred_ip**](ProtocolsApi.md#delete_ndmp_settings_preferred_ip) | **Delete** /platform/4/protocols/ndmp/settings/preferred-ips/{NdmpSettingsPreferredIpId} | 
[**delete_ndmp_settings_variable**](ProtocolsApi.md#delete_ndmp_settings_variable) | **Delete** /platform/3/protocols/ndmp/settings/variables/{NdmpSettingsVariableId} | 
[**delete_ndmp_user**](ProtocolsApi.md#delete_ndmp_user) | **Delete** /platform/3/protocols/ndmp/users/{NdmpUserId} | 
[**delete_nfs_alias**](ProtocolsApi.md#delete_nfs_alias) | **Delete** /platform/2/protocols/nfs/aliases/{NfsAliasId} | 
[**delete_nfs_export**](ProtocolsApi.md#delete_nfs_export) | **Delete** /platform/4/protocols/nfs/exports/{NfsExportId} | 
[**delete_nfs_nlm_session**](ProtocolsApi.md#delete_nfs_nlm_session) | **Delete** /platform/3/protocols/nfs/nlm/sessions/{NfsNlmSessionId} | 
[**delete_ntp_server**](ProtocolsApi.md#delete_ntp_server) | **Delete** /platform/3/protocols/ntp/servers/{NtpServerId} | 
[**delete_ntp_servers**](ProtocolsApi.md#delete_ntp_servers) | **Delete** /platform/3/protocols/ntp/servers | 
[**delete_smb_log_level_filter**](ProtocolsApi.md#delete_smb_log_level_filter) | **Delete** /platform/3/protocols/smb/log-level/filters/{SmbLogLevelFilterId} | 
[**delete_smb_log_level_filters**](ProtocolsApi.md#delete_smb_log_level_filters) | **Delete** /platform/3/protocols/smb/log-level/filters | 
[**delete_smb_openfile**](ProtocolsApi.md#delete_smb_openfile) | **Delete** /platform/1/protocols/smb/openfiles/{SmbOpenfileId} | 
[**delete_smb_session**](ProtocolsApi.md#delete_smb_session) | **Delete** /platform/1/protocols/smb/sessions/{SmbSessionId} | 
[**delete_smb_sessions_computer_user**](ProtocolsApi.md#delete_smb_sessions_computer_user) | **Delete** /platform/1/protocols/smb/sessions/{Computer}/{SmbSessionsComputerUser} | 
[**delete_smb_share**](ProtocolsApi.md#delete_smb_share) | **Delete** /platform/4/protocols/smb/shares/{SmbShareId} | 
[**delete_smb_shares**](ProtocolsApi.md#delete_smb_shares) | **Delete** /platform/4/protocols/smb/shares | 
[**delete_swift_account**](ProtocolsApi.md#delete_swift_account) | **Delete** /platform/3/protocols/swift/accounts/{SwiftAccountId} | 
[**get_ftp_settings**](ProtocolsApi.md#get_ftp_settings) | **Get** /platform/3/protocols/ftp/settings | 
[**get_hdfs_log_level**](ProtocolsApi.md#get_hdfs_log_level) | **Get** /platform/3/protocols/hdfs/log-level | 
[**get_hdfs_proxyuser**](ProtocolsApi.md#get_hdfs_proxyuser) | **Get** /platform/1/protocols/hdfs/proxyusers/{HdfsProxyuserId} | 
[**get_hdfs_rack**](ProtocolsApi.md#get_hdfs_rack) | **Get** /platform/1/protocols/hdfs/racks/{HdfsRackId} | 
[**get_hdfs_ranger_plugin_settings**](ProtocolsApi.md#get_hdfs_ranger_plugin_settings) | **Get** /platform/4/protocols/hdfs/ranger-plugin/settings | 
[**get_hdfs_settings**](ProtocolsApi.md#get_hdfs_settings) | **Get** /platform/4/protocols/hdfs/settings | 
[**get_http_settings**](ProtocolsApi.md#get_http_settings) | **Get** /platform/3/protocols/http/settings | 
[**get_ndmp_contexts_backup**](ProtocolsApi.md#get_ndmp_contexts_backup) | **Get** /platform/3/protocols/ndmp/contexts/backup | 
[**get_ndmp_contexts_backup_by_id**](ProtocolsApi.md#get_ndmp_contexts_backup_by_id) | **Get** /platform/3/protocols/ndmp/contexts/backup/{NdmpContextsBackupId} | 
[**get_ndmp_contexts_bre**](ProtocolsApi.md#get_ndmp_contexts_bre) | **Get** /platform/3/protocols/ndmp/contexts/bre | 
[**get_ndmp_contexts_bre_by_id**](ProtocolsApi.md#get_ndmp_contexts_bre_by_id) | **Get** /platform/3/protocols/ndmp/contexts/bre/{NdmpContextsBreId} | 
[**get_ndmp_contexts_restore**](ProtocolsApi.md#get_ndmp_contexts_restore) | **Get** /platform/3/protocols/ndmp/contexts/restore | 
[**get_ndmp_contexts_restore_by_id**](ProtocolsApi.md#get_ndmp_contexts_restore_by_id) | **Get** /platform/3/protocols/ndmp/contexts/restore/{NdmpContextsRestoreId} | 
[**get_ndmp_diagnostics**](ProtocolsApi.md#get_ndmp_diagnostics) | **Get** /platform/3/protocols/ndmp/diagnostics | 
[**get_ndmp_dumpdate**](ProtocolsApi.md#get_ndmp_dumpdate) | **Get** /platform/3/protocols/ndmp/dumpdates/{NdmpDumpdateId} | 
[**get_ndmp_logs**](ProtocolsApi.md#get_ndmp_logs) | **Get** /platform/3/protocols/ndmp/logs | 
[**get_ndmp_session**](ProtocolsApi.md#get_ndmp_session) | **Get** /platform/3/protocols/ndmp/sessions/{NdmpSessionId} | 
[**get_ndmp_sessions**](ProtocolsApi.md#get_ndmp_sessions) | **Get** /platform/3/protocols/ndmp/sessions | 
[**get_ndmp_settings_dmas**](ProtocolsApi.md#get_ndmp_settings_dmas) | **Get** /platform/3/protocols/ndmp/settings/dmas | 
[**get_ndmp_settings_global**](ProtocolsApi.md#get_ndmp_settings_global) | **Get** /platform/3/protocols/ndmp/settings/global | 
[**get_ndmp_settings_preferred_ip**](ProtocolsApi.md#get_ndmp_settings_preferred_ip) | **Get** /platform/4/protocols/ndmp/settings/preferred-ips/{NdmpSettingsPreferredIpId} | 
[**get_ndmp_settings_variable**](ProtocolsApi.md#get_ndmp_settings_variable) | **Get** /platform/3/protocols/ndmp/settings/variables/{NdmpSettingsVariableId} | 
[**get_ndmp_user**](ProtocolsApi.md#get_ndmp_user) | **Get** /platform/3/protocols/ndmp/users/{NdmpUserId} | 
[**get_nfs_alias**](ProtocolsApi.md#get_nfs_alias) | **Get** /platform/2/protocols/nfs/aliases/{NfsAliasId} | 
[**get_nfs_check**](ProtocolsApi.md#get_nfs_check) | **Get** /platform/2/protocols/nfs/check | 
[**get_nfs_export**](ProtocolsApi.md#get_nfs_export) | **Get** /platform/4/protocols/nfs/exports/{NfsExportId} | 
[**get_nfs_exports_summary**](ProtocolsApi.md#get_nfs_exports_summary) | **Get** /platform/2/protocols/nfs/exports-summary | 
[**get_nfs_log_level**](ProtocolsApi.md#get_nfs_log_level) | **Get** /platform/3/protocols/nfs/log-level | 
[**get_nfs_netgroup**](ProtocolsApi.md#get_nfs_netgroup) | **Get** /platform/3/protocols/nfs/netgroup | 
[**get_nfs_nlm_locks**](ProtocolsApi.md#get_nfs_nlm_locks) | **Get** /platform/2/protocols/nfs/nlm/locks | 
[**get_nfs_nlm_session**](ProtocolsApi.md#get_nfs_nlm_session) | **Get** /platform/3/protocols/nfs/nlm/sessions/{NfsNlmSessionId} | 
[**get_nfs_nlm_sessions**](ProtocolsApi.md#get_nfs_nlm_sessions) | **Get** /platform/3/protocols/nfs/nlm/sessions | 
[**get_nfs_nlm_waiters**](ProtocolsApi.md#get_nfs_nlm_waiters) | **Get** /platform/2/protocols/nfs/nlm/waiters | 
[**get_nfs_settings_export**](ProtocolsApi.md#get_nfs_settings_export) | **Get** /platform/2/protocols/nfs/settings/export | 
[**get_nfs_settings_global**](ProtocolsApi.md#get_nfs_settings_global) | **Get** /platform/3/protocols/nfs/settings/global | 
[**get_nfs_settings_zone**](ProtocolsApi.md#get_nfs_settings_zone) | **Get** /platform/2/protocols/nfs/settings/zone | 
[**get_ntp_server**](ProtocolsApi.md#get_ntp_server) | **Get** /platform/3/protocols/ntp/servers/{NtpServerId} | 
[**get_ntp_settings**](ProtocolsApi.md#get_ntp_settings) | **Get** /platform/3/protocols/ntp/settings | 
[**get_smb_log_level**](ProtocolsApi.md#get_smb_log_level) | **Get** /platform/3/protocols/smb/log-level | 
[**get_smb_log_level_filter**](ProtocolsApi.md#get_smb_log_level_filter) | **Get** /platform/3/protocols/smb/log-level/filters/{SmbLogLevelFilterId} | 
[**get_smb_openfiles**](ProtocolsApi.md#get_smb_openfiles) | **Get** /platform/1/protocols/smb/openfiles | 
[**get_smb_sessions**](ProtocolsApi.md#get_smb_sessions) | **Get** /platform/1/protocols/smb/sessions | 
[**get_smb_settings_global**](ProtocolsApi.md#get_smb_settings_global) | **Get** /platform/3/protocols/smb/settings/global | 
[**get_smb_settings_share**](ProtocolsApi.md#get_smb_settings_share) | **Get** /platform/3/protocols/smb/settings/share | 
[**get_smb_share**](ProtocolsApi.md#get_smb_share) | **Get** /platform/4/protocols/smb/shares/{SmbShareId} | 
[**get_smb_shares_summary**](ProtocolsApi.md#get_smb_shares_summary) | **Get** /platform/1/protocols/smb/shares-summary | 
[**get_snmp_settings**](ProtocolsApi.md#get_snmp_settings) | **Get** /platform/5/protocols/snmp/settings | 
[**get_swift_account**](ProtocolsApi.md#get_swift_account) | **Get** /platform/3/protocols/swift/accounts/{SwiftAccountId} | 
[**list_hdfs_proxyusers**](ProtocolsApi.md#list_hdfs_proxyusers) | **Get** /platform/1/protocols/hdfs/proxyusers | 
[**list_hdfs_racks**](ProtocolsApi.md#list_hdfs_racks) | **Get** /platform/1/protocols/hdfs/racks | 
[**list_ndmp_settings_preferred_ips**](ProtocolsApi.md#list_ndmp_settings_preferred_ips) | **Get** /platform/4/protocols/ndmp/settings/preferred-ips | 
[**list_ndmp_users**](ProtocolsApi.md#list_ndmp_users) | **Get** /platform/3/protocols/ndmp/users | 
[**list_nfs_aliases**](ProtocolsApi.md#list_nfs_aliases) | **Get** /platform/2/protocols/nfs/aliases | 
[**list_nfs_exports**](ProtocolsApi.md#list_nfs_exports) | **Get** /platform/4/protocols/nfs/exports | 
[**list_ntp_servers**](ProtocolsApi.md#list_ntp_servers) | **Get** /platform/3/protocols/ntp/servers | 
[**list_smb_log_level_filters**](ProtocolsApi.md#list_smb_log_level_filters) | **Get** /platform/3/protocols/smb/log-level/filters | 
[**list_smb_shares**](ProtocolsApi.md#list_smb_shares) | **Get** /platform/4/protocols/smb/shares | 
[**list_swift_accounts**](ProtocolsApi.md#list_swift_accounts) | **Get** /platform/3/protocols/swift/accounts | 
[**update_ftp_settings**](ProtocolsApi.md#update_ftp_settings) | **Put** /platform/3/protocols/ftp/settings | 
[**update_hdfs_log_level**](ProtocolsApi.md#update_hdfs_log_level) | **Put** /platform/3/protocols/hdfs/log-level | 
[**update_hdfs_proxyuser**](ProtocolsApi.md#update_hdfs_proxyuser) | **Put** /platform/1/protocols/hdfs/proxyusers/{HdfsProxyuserId} | 
[**update_hdfs_rack**](ProtocolsApi.md#update_hdfs_rack) | **Put** /platform/1/protocols/hdfs/racks/{HdfsRackId} | 
[**update_hdfs_ranger_plugin_settings**](ProtocolsApi.md#update_hdfs_ranger_plugin_settings) | **Put** /platform/4/protocols/hdfs/ranger-plugin/settings | 
[**update_hdfs_settings**](ProtocolsApi.md#update_hdfs_settings) | **Put** /platform/4/protocols/hdfs/settings | 
[**update_http_settings**](ProtocolsApi.md#update_http_settings) | **Put** /platform/3/protocols/http/settings | 
[**update_ndmp_diagnostics**](ProtocolsApi.md#update_ndmp_diagnostics) | **Put** /platform/3/protocols/ndmp/diagnostics | 
[**update_ndmp_settings_global**](ProtocolsApi.md#update_ndmp_settings_global) | **Put** /platform/3/protocols/ndmp/settings/global | 
[**update_ndmp_settings_preferred_ip**](ProtocolsApi.md#update_ndmp_settings_preferred_ip) | **Put** /platform/4/protocols/ndmp/settings/preferred-ips/{NdmpSettingsPreferredIpId} | 
[**update_ndmp_settings_variable**](ProtocolsApi.md#update_ndmp_settings_variable) | **Put** /platform/3/protocols/ndmp/settings/variables/{NdmpSettingsVariableId} | 
[**update_ndmp_user**](ProtocolsApi.md#update_ndmp_user) | **Put** /platform/3/protocols/ndmp/users/{NdmpUserId} | 
[**update_nfs_alias**](ProtocolsApi.md#update_nfs_alias) | **Put** /platform/2/protocols/nfs/aliases/{NfsAliasId} | 
[**update_nfs_export**](ProtocolsApi.md#update_nfs_export) | **Put** /platform/4/protocols/nfs/exports/{NfsExportId} | 
[**update_nfs_log_level**](ProtocolsApi.md#update_nfs_log_level) | **Put** /platform/3/protocols/nfs/log-level | 
[**update_nfs_netgroup**](ProtocolsApi.md#update_nfs_netgroup) | **Put** /platform/3/protocols/nfs/netgroup | 
[**update_nfs_settings_export**](ProtocolsApi.md#update_nfs_settings_export) | **Put** /platform/2/protocols/nfs/settings/export | 
[**update_nfs_settings_global**](ProtocolsApi.md#update_nfs_settings_global) | **Put** /platform/3/protocols/nfs/settings/global | 
[**update_nfs_settings_zone**](ProtocolsApi.md#update_nfs_settings_zone) | **Put** /platform/2/protocols/nfs/settings/zone | 
[**update_ntp_server**](ProtocolsApi.md#update_ntp_server) | **Put** /platform/3/protocols/ntp/servers/{NtpServerId} | 
[**update_ntp_settings**](ProtocolsApi.md#update_ntp_settings) | **Put** /platform/3/protocols/ntp/settings | 
[**update_smb_log_level**](ProtocolsApi.md#update_smb_log_level) | **Put** /platform/3/protocols/smb/log-level | 
[**update_smb_settings_global**](ProtocolsApi.md#update_smb_settings_global) | **Put** /platform/3/protocols/smb/settings/global | 
[**update_smb_settings_share**](ProtocolsApi.md#update_smb_settings_share) | **Put** /platform/3/protocols/smb/settings/share | 
[**update_smb_share**](ProtocolsApi.md#update_smb_share) | **Put** /platform/4/protocols/smb/shares/{SmbShareId} | 
[**update_snmp_settings**](ProtocolsApi.md#update_snmp_settings) | **Put** /platform/5/protocols/snmp/settings | 
[**update_swift_account**](ProtocolsApi.md#update_swift_account) | **Put** /platform/3/protocols/swift/accounts/{SwiftAccountId} | 


# **create_hdfs_proxyuser**
>crate::models::CreateResponse create_hdfs_proxyuser(ctx, hdfs_proxyuser, optional)


Create a new HDFS proxyuser.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hdfs_proxyuser** | [**HdfsProxyuserCreateParams**](HdfsProxyuserCreateParams.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hdfs_proxyuser** | [**HdfsProxyuserCreateParams**](HdfsProxyuserCreateParams.md)|  | 
 **zone** | **String**| Access zone which contains HDFS proxyuser. | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_hdfs_rack**
>crate::models::CreateResponse create_hdfs_rack(ctx, hdfs_rack, optional)


Create a new HDFS rack.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hdfs_rack** | [**HdfsRackCreateParams**](HdfsRackCreateParams.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hdfs_rack** | [**HdfsRackCreateParams**](HdfsRackCreateParams.md)|  | 
 **zone** | **String**| Access zone which contains HDFS rack. | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_ndmp_settings_preferred_ip**
>crate::models::Empty create_ndmp_settings_preferred_ip(ctx, ndmp_settings_preferred_ip)


Create a preferred ip preference.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_settings_preferred_ip** | [**NdmpSettingsPreferredIpCreateParams**](NdmpSettingsPreferredIpCreateParams.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_ndmp_settings_variable**
>crate::models::Empty create_ndmp_settings_variable(ctx, ndmp_settings_variable, ndmp_settings_variable_id)


Create a preferred NDMP environment variable.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_settings_variable** | [**NdmpSettingsVariableCreateParams**](NdmpSettingsVariableCreateParams.md)|  | 
  **ndmp_settings_variable_id** | **String**| Create a preferred NDMP environment variable. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_ndmp_user**
>crate::models::Empty create_ndmp_user(ctx, ndmp_user)


Created a new user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_user** | [**NdmpUserCreateParams**](NdmpUserCreateParams.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_nfs_alias**
>crate::models::CreateNfsAliasResponse create_nfs_alias(ctx, nfs_alias, optional)


Create a new NFS alias.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_alias** | [**NfsAliasCreateParams**](NfsAliasCreateParams.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_alias** | [**NfsAliasCreateParams**](NfsAliasCreateParams.md)|  | 
 **zone** | **String**| Access zone | 

### Return type

[**::models::CreateNfsAliasResponse**](CreateNfsAliasResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_nfs_export**
>crate::models::CreateQuotaReportResponse create_nfs_export(ctx, nfs_export, optional)


Create a new NFS export.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_export** | [**NfsExportCreateParams**](NfsExportCreateParams.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_export** | [**NfsExportCreateParams**](NfsExportCreateParams.md)|  | 
 **force** | **bool**| If true, the export will be created even if it conflicts with another export. | 
 **ignore_unresolvable_hosts** | **bool**| Ignore unresolvable hosts. | 
 **zone** | **String**| Access zone | 
 **ignore_conflicts** | **bool**| Ignore conflicts with existing exports. | 
 **ignore_bad_paths** | **bool**| Ignore nonexistent or otherwise bad paths. | 
 **ignore_bad_auth** | **bool**| Ignore invalid users. | 

### Return type

[**::models::CreateQuotaReportResponse**](CreateQuotaReportResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_nfs_netgroup_check_item**
>crate::models::Empty create_nfs_netgroup_check_item(ctx, nfs_netgroup_check_item, optional)


Update the NFS netgroups in the cache.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_netgroup_check_item** | [**Empty**](Empty.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_netgroup_check_item** | [**Empty**](Empty.md)|  | 
 **host** | **String**| IP address of node to update. If unspecified, the local nodes cache is updated. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_nfs_netgroup_flush_item**
>crate::models::Empty create_nfs_netgroup_flush_item(ctx, nfs_netgroup_flush_item, optional)


Flush the NFS netgroups in the cache.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_netgroup_flush_item** | [**Empty**](Empty.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_netgroup_flush_item** | [**Empty**](Empty.md)|  | 
 **host** | **String**| IP address of node to flush. If unspecified, all nodes on the cluster are flushed. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_nfs_nlm_sessions_check_item**
>crate::models::CreateNfsNlmSessionsCheckItemResponse create_nfs_nlm_sessions_check_item(ctx, nfs_nlm_sessions_check_item, optional)


Perform an active scan for lost NFSv3 locks.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_nlm_sessions_check_item** | [**Empty**](Empty.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_nlm_sessions_check_item** | [**Empty**](Empty.md)|  | 
 **cluster_ip** | **String**| An IP address for which NSM has client records | 
 **zone** | **String**| Represents an extant auth zone | 

### Return type

[**::models::CreateNfsNlmSessionsCheckItemResponse**](CreateNfsNlmSessionsCheckItemResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_nfs_reload_item**
>crate::models::Empty create_nfs_reload_item(ctx, nfs_reload_item, optional)


Reload default NFS export configuration.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_reload_item** | [**Empty**](Empty.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_reload_item** | [**Empty**](Empty.md)|  | 
 **zone** | **String**| Access zone | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_ntp_server**
>crate::models::Empty create_ntp_server(ctx, ntp_server)


Create an NTP server entry.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ntp_server** | [**NtpServerCreateParams**](NtpServerCreateParams.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_smb_log_level_filter**
>crate::models::CreateAuthRefreshItemResponse create_smb_log_level_filter(ctx, smb_log_level_filter)


Add an SMB log filter.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **smb_log_level_filter** | [**SmbLogLevelFilter**](SmbLogLevelFilter.md)|  | 

### Return type

[**::models::CreateAuthRefreshItemResponse**](CreateAuthRefreshItemResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_smb_share**
>crate::models::CreateResponse create_smb_share(ctx, smb_share, optional)


Create a new share.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **smb_share** | [**SmbShareCreateParams**](SmbShareCreateParams.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **smb_share** | [**SmbShareCreateParams**](SmbShareCreateParams.md)|  | 
 **zone** | **String**| Zone which contains this share. | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_swift_account**
>crate::models::Empty create_swift_account(ctx, swift_account, optional)


Create a new Swift account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **swift_account** | [**SwiftAccount**](SwiftAccount.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **swift_account** | [**SwiftAccount**](SwiftAccount.md)|  | 
 **zone** | **String**| Access zone which contains Swift account. | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_hdfs_proxyuser**
> delete_hdfs_proxyuser(ctx, hdfs_proxyuser_id, optional)


Delete an HDFS proxyuser.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hdfs_proxyuser_id** | **String**| Delete an HDFS proxyuser. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hdfs_proxyuser_id** | **String**| Delete an HDFS proxyuser. | 
 **zone** | **String**| Access zone which contains HDFS proxyuser. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_hdfs_rack**
> delete_hdfs_rack(ctx, hdfs_rack_id, optional)


Delete the HDFS rack.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hdfs_rack_id** | **String**| Delete the HDFS rack. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hdfs_rack_id** | **String**| Delete the HDFS rack. | 
 **zone** | **String**| Access zone which contains HDFS rack. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_ndmp_contexts_backup_by_id**
> delete_ndmp_contexts_backup_by_id(ctx, ndmp_contexts_backup_id)


Delete a backup context

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_contexts_backup_id** | **String**| Delete a backup context | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_ndmp_contexts_bre_by_id**
> delete_ndmp_contexts_bre_by_id(ctx, ndmp_contexts_bre_id)


Delete a NDMP BRE context

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_contexts_bre_id** | **String**| Delete a NDMP BRE context | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_ndmp_contexts_restore_by_id**
> delete_ndmp_contexts_restore_by_id(ctx, ndmp_contexts_restore_id)


Delete a restore context

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_contexts_restore_id** | **String**| Delete a restore context | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_ndmp_dumpdate**
> delete_ndmp_dumpdate(ctx, ndmp_dumpdate_id, optional)


Delete dumpdates entries of a specified path.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_dumpdate_id** | **String**| Delete dumpdates entries of a specified path. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ndmp_dumpdate_id** | **String**| Delete dumpdates entries of a specified path. | 
 **level** | **i32**| Level is an input from 0 to 10. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_ndmp_session**
> delete_ndmp_session(ctx, ndmp_session_id, optional)


Delete the ndmp session.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_session_id** | **String**| Delete the ndmp session. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ndmp_session_id** | **String**| Delete the ndmp session. | 
 **lnn** | **String**| Logical node number. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_ndmp_settings_preferred_ip**
> delete_ndmp_settings_preferred_ip(ctx, ndmp_settings_preferred_ip_id)


Delete a preferred ip preference.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_settings_preferred_ip_id** | **String**| Delete a preferred ip preference. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_ndmp_settings_variable**
> delete_ndmp_settings_variable(ctx, ndmp_settings_variable_id, optional)


Delete preferred environment variable entries

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_settings_variable_id** | **String**| Delete preferred environment variable entries | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ndmp_settings_variable_id** | **String**| Delete preferred environment variable entries | 
 **name** | **String**| Name of the variable to delete. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_ndmp_user**
> delete_ndmp_user(ctx, ndmp_user_id)


Delete the user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_user_id** | **String**| Delete the user. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_nfs_alias**
> delete_nfs_alias(ctx, nfs_alias_id, optional)


Delete the export.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_alias_id** | **String**| Delete the export. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_alias_id** | **String**| Delete the export. | 
 **zone** | **String**| Access zone | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_nfs_export**
> delete_nfs_export(ctx, nfs_export_id, optional)


Delete the export.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_export_id** | **String**| Delete the export. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_export_id** | **String**| Delete the export. | 
 **zone** | **String**| Access zone | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_nfs_nlm_session**
> delete_nfs_nlm_session(ctx, nfs_nlm_session_id, optional)


Delete all lock state for this host.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_nlm_session_id** | **String**| Delete all lock state for this host. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_nlm_session_id** | **String**| Delete all lock state for this host. | 
 **cluster_ip** | **String**| An IP address for which NSM has client records | 
 **zone** | **String**| Represents an extant auth zone | 
 **refresh** | **bool**| if set to true, the client will be given a chance to reclaim its locks before they are destroyed | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_ntp_server**
> delete_ntp_server(ctx, ntp_server_id)


Delete an NTP server entry.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ntp_server_id** | **String**| Delete an NTP server entry. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_ntp_servers**
> delete_ntp_servers(ctx, )


Delete all NTP server entries.

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_smb_log_level_filter**
> delete_smb_log_level_filter(ctx, smb_log_level_filter_id)


Delete log filter.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **smb_log_level_filter_id** | **String**| Delete log filter. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_smb_log_level_filters**
> delete_smb_log_level_filters(ctx, optional)


Delete existing SMB log filters.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **level** | **String**| Valid SMB logging levels | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_smb_openfile**
> delete_smb_openfile(ctx, smb_openfile_id)


Close the file in the SMB server.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **smb_openfile_id** | **String**| Close the file in the SMB server. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_smb_session**
> delete_smb_session(ctx, smb_session_id)


Close the SMB session.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **smb_session_id** | **String**| Close the SMB session. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_smb_sessions_computer_user**
> delete_smb_sessions_computer_user(ctx, smb_sessions_computer_user, computer)


Close the SMB session.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **smb_sessions_computer_user** | **String**| Close the SMB session. | 
  **computer** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_smb_share**
> delete_smb_share(ctx, smb_share_id, optional)


Delete the share.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **smb_share_id** | **String**| Delete the share. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **smb_share_id** | **String**| Delete the share. | 
 **zone** | **String**| Zone which contains this share. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_smb_shares**
> delete_smb_shares(ctx, )


Delete multiple smb shares.

### Required Parameters
This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_swift_account**
> delete_swift_account(ctx, swift_account_id, optional)


Delete a Swift account.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **swift_account_id** | **String**| Delete a Swift account. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **swift_account_id** | **String**| Delete a Swift account. | 
 **zone** | **String**| Access zone which contains Swift account. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ftp_settings**
>crate::models::FtpSettings get_ftp_settings(ctx, )


Retrieve the FTP settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::FtpSettings**](FtpSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_hdfs_log_level**
>crate::models::HdfsLogLevel get_hdfs_log_level(ctx, )


Retrieve the HDFS service log-level.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::HdfsLogLevel**](HdfsLogLevel.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_hdfs_proxyuser**
>crate::models::HdfsProxyusers get_hdfs_proxyuser(ctx, hdfs_proxyuser_id, optional)


View the proxyuser.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hdfs_proxyuser_id** | **String**| View the proxyuser. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hdfs_proxyuser_id** | **String**| View the proxyuser. | 
 **zone** | **String**| Access zone which contains HDFS proxyuser. | 

### Return type

[**::models::HdfsProxyusers**](HdfsProxyusers.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_hdfs_rack**
>crate::models::HdfsRacks get_hdfs_rack(ctx, hdfs_rack_id, optional)


Retrieve the HDFS rack.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hdfs_rack_id** | **String**| Retrieve the HDFS rack. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hdfs_rack_id** | **String**| Retrieve the HDFS rack. | 
 **zone** | **String**| Access zone which contains HDFS rack. | 

### Return type

[**::models::HdfsRacks**](HdfsRacks.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_hdfs_ranger_plugin_settings**
>crate::models::HdfsRangerPluginSettings get_hdfs_ranger_plugin_settings(ctx, optional)


Retrieve HDFS ranger-plugin properties.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **zone** | **String**| Access zone which contains HDFS ranger-plugin settings. | 

### Return type

[**::models::HdfsRangerPluginSettings**](HdfsRangerPluginSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_hdfs_settings**
>crate::models::HdfsSettings get_hdfs_settings(ctx, optional)


Retrieve HDFS properties.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **zone** | **String**| Access zone which contains HDFS settings. | 

### Return type

[**::models::HdfsSettings**](HdfsSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_http_settings**
>crate::models::HttpSettings get_http_settings(ctx, )


Retrieve HTTP properties.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::HttpSettings**](HttpSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_contexts_backup**
>crate::models::NdmpContextsBackupExtended get_ndmp_contexts_backup(ctx, optional)


Get List of NDMP Backup Contexts.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::NdmpContextsBackupExtended**](NdmpContextsBackupExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_contexts_backup_by_id**
>crate::models::NdmpContextsBackup get_ndmp_contexts_backup_by_id(ctx, ndmp_contexts_backup_id)


View a NDMP backup context

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_contexts_backup_id** | **String**| View a NDMP backup context | 

### Return type

[**::models::NdmpContextsBackup**](NdmpContextsBackup.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_contexts_bre**
>crate::models::NdmpContextsBreExtended get_ndmp_contexts_bre(ctx, optional)


Get list of NDMP BRE Contexts.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::NdmpContextsBreExtended**](NdmpContextsBreExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_contexts_bre_by_id**
>crate::models::NdmpContextsBre get_ndmp_contexts_bre_by_id(ctx, ndmp_contexts_bre_id)


View a NDMP BRE context

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_contexts_bre_id** | **String**| View a NDMP BRE context | 

### Return type

[**::models::NdmpContextsBre**](NdmpContextsBre.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_contexts_restore**
>crate::models::NdmpContextsBackupExtended get_ndmp_contexts_restore(ctx, optional)


Get List of NDMP Restore Contexts.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::NdmpContextsBackupExtended**](NdmpContextsBackupExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_contexts_restore_by_id**
>crate::models::NdmpContextsBackup get_ndmp_contexts_restore_by_id(ctx, ndmp_contexts_restore_id)


View a NDMP restore context

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_contexts_restore_id** | **String**| View a NDMP restore context | 

### Return type

[**::models::NdmpContextsBackup**](NdmpContextsBackup.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_diagnostics**
>crate::models::NdmpDiagnostics get_ndmp_diagnostics(ctx, )


List ndmp diagnostics settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::NdmpDiagnostics**](NdmpDiagnostics.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_dumpdate**
>crate::models::NdmpDumpdates get_ndmp_dumpdate(ctx, ndmp_dumpdate_id, optional)


List of dumpdates entries.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_dumpdate_id** | **String**| List of dumpdates entries. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ndmp_dumpdate_id** | **String**| List of dumpdates entries. | 
 **sort** | **String**| The field that will be used for sorting. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **level** | **i32**| Filter by dumpdate level. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **path** | **String**| Filter by file path. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::NdmpDumpdates**](NdmpDumpdates.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_logs**
>crate::models::NdmpLogs get_ndmp_logs(ctx, optional)


Get NDMP logs

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lnn** | **String**| Logical node number. | 
 **page** | **i32**| The page number of the NDMP logs file. | 
 **pagesize** | **i32**| The page size of each page of the NDMP log file. | 

### Return type

[**::models::NdmpLogs**](NdmpLogs.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_session**
>crate::models::NdmpSessions get_ndmp_session(ctx, ndmp_session_id, optional)


Retrieve the ndmp session.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_session_id** | **String**| Retrieve the ndmp session. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ndmp_session_id** | **String**| Retrieve the ndmp session. | 
 **lnn** | **String**| Logical node number. | 

### Return type

[**::models::NdmpSessions**](NdmpSessions.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_sessions**
>crate::models::NdmpSessionsExtended get_ndmp_sessions(ctx, optional)


List all ndmp sessions.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **consolidate** | **bool**| Combine sessions in the same context. | 
 **node** | **String**| Only return sessions of the node. | 
 **session** | **String**| Only return the specified session. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::NdmpSessionsExtended**](NdmpSessionsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_settings_dmas**
>crate::models::NdmpSettingsDmas get_ndmp_settings_dmas(ctx, )


List of supported dma vendors.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::NdmpSettingsDmas**](NdmpSettingsDmas.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_settings_global**
>crate::models::NdmpSettingsGlobal get_ndmp_settings_global(ctx, )


List global ndmp settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::NdmpSettingsGlobal**](NdmpSettingsGlobal.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_settings_preferred_ip**
>crate::models::NdmpSettingsPreferredIps get_ndmp_settings_preferred_ip(ctx, ndmp_settings_preferred_ip_id)


Get one preference by id.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_settings_preferred_ip_id** | **String**| Get one preference by id. | 

### Return type

[**::models::NdmpSettingsPreferredIps**](NdmpSettingsPreferredIps.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_settings_variable**
>crate::models::NdmpSettingsVariables get_ndmp_settings_variable(ctx, ndmp_settings_variable_id, optional)


List of preferred environment variables.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_settings_variable_id** | **String**| List of preferred environment variables. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ndmp_settings_variable_id** | **String**| List of preferred environment variables. | 
 **path** | **String**| Return variables of the path. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::NdmpSettingsVariables**](NdmpSettingsVariables.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ndmp_user**
>crate::models::NdmpUsers get_ndmp_user(ctx, ndmp_user_id)


Retrieve the user.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_user_id** | **String**| Retrieve the user. | 

### Return type

[**::models::NdmpUsers**](NdmpUsers.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nfs_alias**
>crate::models::NfsAliases get_nfs_alias(ctx, nfs_alias_id, optional)


Retrieve export information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_alias_id** | **String**| Retrieve export information. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_alias_id** | **String**| Retrieve export information. | 
 **scope** | **String**| When specified as &#39;effective&#39;, or not specified, all fields are returned. When specified as &#39;user&#39;, only fields with non-default values are shown. When specified as &#39;default&#39;, the original values are returned. | 
 **check** | **bool**| Check for conflicts when viewing alias. | 
 **zone** | **String**| Access zone | 

### Return type

[**::models::NfsAliases**](NfsAliases.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nfs_check**
>crate::models::NfsCheckExtended get_nfs_check(ctx, optional)


Retrieve NFS export validation information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ignore_bad_paths** | **bool**| Ignore nonexistent or otherwise bad paths. | 
 **ignore_bad_auth** | **bool**| Ignore invalid users. | 
 **zone** | **String**| Access zone | 
 **ignore_unresolvable_hosts** | **bool**| Ignore unresolvable hosts. | 

### Return type

[**::models::NfsCheckExtended**](NfsCheckExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nfs_export**
>crate::models::NfsExports get_nfs_export(ctx, nfs_export_id, optional)


Retrieve export information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_export_id** | **String**| Retrieve export information. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_export_id** | **String**| Retrieve export information. | 
 **scope** | **String**| When specified as &#39;effective&#39;, or not specified, all fields are returned. When specified as &#39;user&#39;, only fields with non-default values are shown. When specified as &#39;default&#39;, the original values are returned. | 
 **zone** | **String**| Access zone | 

### Return type

[**::models::NfsExports**](NfsExports.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nfs_exports_summary**
>crate::models::NfsExportsSummary get_nfs_exports_summary(ctx, optional)


Retrieve NFS export summary information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **zone** | **String**| Access zone | 

### Return type

[**::models::NfsExportsSummary**](NfsExportsSummary.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nfs_log_level**
>crate::models::NfsLogLevel get_nfs_log_level(ctx, )


Get the current NFS service logging level.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::NfsLogLevel**](NfsLogLevel.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nfs_netgroup**
>crate::models::NfsNetgroup get_nfs_netgroup(ctx, optional)


Get the current NFS netgroup cache settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **host** | **String**| Host to retrieve netgroup cache settings from. | 

### Return type

[**::models::NfsNetgroup**](NfsNetgroup.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nfs_nlm_locks**
>crate::models::NfsNlmLocks get_nfs_nlm_locks(ctx, optional)


List all NLM locks.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| The field that will be used for sorting. | 
 **created** | **String**| Return locks created after the specified unix epoch time. | 
 **lin** | **String**| Filter locks by the specified LIN in /ifs that is locked. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **client** | **String**| Filter locks by the specified client host name and IP address. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **client_id** | **String**| Filter locks by the specified client ID. | 
 **path** | **String**| Filter locks by the specified path under /ifs. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::NfsNlmLocks**](NfsNlmLocks.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nfs_nlm_session**
>crate::models::NfsNlmSessions get_nfs_nlm_session(ctx, nfs_nlm_session_id, optional)


Retrieve all lock state for a single client.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_nlm_session_id** | **String**| Retrieve all lock state for a single client. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_nlm_session_id** | **String**| Retrieve all lock state for a single client. | 
 **cluster_ip** | **String**| An IP address for which NSM has client records | 
 **zone** | **String**| Represents an extant auth zone | 

### Return type

[**::models::NfsNlmSessions**](NfsNlmSessions.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nfs_nlm_sessions**
>crate::models::NfsNlmSessionsExtended get_nfs_nlm_sessions(ctx, optional)


List all NSM clients (optionally filtered by either zone or IP)

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| The field that will be used for sorting. | 
 **ip** | **String**| An IP address for which NSM has client records | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **zone** | **String**| Represents an extant auth zone | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::NfsNlmSessionsExtended**](NfsNlmSessionsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nfs_nlm_waiters**
>crate::models::NfsNlmWaiters get_nfs_nlm_waiters(ctx, optional)


List all NLM lock waiters.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| The field that will be used for sorting. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::NfsNlmWaiters**](NfsNlmWaiters.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nfs_settings_export**
>crate::models::NfsSettingsExport get_nfs_settings_export(ctx, optional)


Retrieve export information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 
 **zone** | **String**| Access zone | 

### Return type

[**::models::NfsSettingsExport**](NfsSettingsExport.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nfs_settings_global**
>crate::models::NfsSettingsGlobal get_nfs_settings_global(ctx, optional)


Retrieve the NFS configuration.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **String**| When specified as &#39;effective&#39;, or not specified, all fields are returned. When specified as &#39;user&#39;, only fields with non-default values are shown. When specified as &#39;default&#39;, the original values are returned. | 

### Return type

[**::models::NfsSettingsGlobal**](NfsSettingsGlobal.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_nfs_settings_zone**
>crate::models::NfsSettingsZone get_nfs_settings_zone(ctx, optional)


Retrieve the NFS server settings for this zone.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **zone** | **String**| Access zone | 

### Return type

[**::models::NfsSettingsZone**](NfsSettingsZone.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ntp_server**
>crate::models::NtpServers get_ntp_server(ctx, ntp_server_id)


Retrieve one NTP server.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ntp_server_id** | **String**| Retrieve one NTP server. | 

### Return type

[**::models::NtpServers**](NtpServers.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_ntp_settings**
>crate::models::NtpSettings get_ntp_settings(ctx, )


Retrieve the NTP settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::NtpSettings**](NtpSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_smb_log_level**
>crate::models::SmbLogLevel get_smb_log_level(ctx, )


Get the current SMB logging level.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SmbLogLevel**](SmbLogLevel.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_smb_log_level_filter**
>crate::models::SmbLogLevelFilters get_smb_log_level_filter(ctx, smb_log_level_filter_id)


View log filter.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **smb_log_level_filter_id** | **String**| View log filter. | 

### Return type

[**::models::SmbLogLevelFilters**](SmbLogLevelFilters.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_smb_openfiles**
>crate::models::SmbOpenfiles get_smb_openfiles(ctx, optional)


List open files.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| Order results by this field. Default is id. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::SmbOpenfiles**](SmbOpenfiles.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_smb_sessions**
>crate::models::SmbSessions get_smb_sessions(ctx, optional)


List open sessions.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| Order results by this field. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::SmbSessions**](SmbSessions.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_smb_settings_global**
>crate::models::SmbSettingsGlobal get_smb_settings_global(ctx, optional)


List all settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 

### Return type

[**::models::SmbSettingsGlobal**](SmbSettingsGlobal.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_smb_settings_share**
>crate::models::SmbSettingsShare get_smb_settings_share(ctx, optional)


List all settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 
 **zone** | **String**| Zone which contains these share settings. | 

### Return type

[**::models::SmbSettingsShare**](SmbSettingsShare.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_smb_share**
>crate::models::SmbShares get_smb_share(ctx, smb_share_id, optional)


Retrieve share.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **smb_share_id** | **String**| Retrieve share. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **smb_share_id** | **String**| Retrieve share. | 
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 
 **resolve_names** | **bool**| If true, resolve group and user names in personas. | 
 **zone** | **String**| Zone which contains this share. | 

### Return type

[**::models::SmbShares**](SmbShares.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_smb_shares_summary**
>crate::models::SmbSharesSummary get_smb_shares_summary(ctx, optional)


Return summary information about shares.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **zone** | **String**| Specifies which access zone or zones to use. | 

### Return type

[**::models::SmbSharesSummary**](SmbSharesSummary.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_snmp_settings**
>crate::models::SnmpSettings get_snmp_settings(ctx, )


Retrieve the SNMP settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SnmpSettings**](SnmpSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_swift_account**
>crate::models::SwiftAccounts get_swift_account(ctx, swift_account_id, optional)


List a swift account.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **swift_account_id** | **String**| List a swift account. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **swift_account_id** | **String**| List a swift account. | 
 **zone** | **String**| Access zone which contains Swift account. | 

### Return type

[**::models::SwiftAccounts**](SwiftAccounts.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_hdfs_proxyusers**
>crate::models::HdfsProxyusers list_hdfs_proxyusers(ctx, optional)


List all proxyusers.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **zone** | **String**| Access zone which contains HDFS proxyusers. | 

### Return type

[**::models::HdfsProxyusers**](HdfsProxyusers.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_hdfs_racks**
>crate::models::HdfsRacksExtended list_hdfs_racks(ctx, optional)


List all racks.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **zone** | **String**| Access zone which contains HDFS racks. | 

### Return type

[**::models::HdfsRacksExtended**](HdfsRacksExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_ndmp_settings_preferred_ips**
>crate::models::NdmpSettingsPreferredIps list_ndmp_settings_preferred_ips(ctx, optional)


Get list of preferences.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::NdmpSettingsPreferredIps**](NdmpSettingsPreferredIps.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_ndmp_users**
>crate::models::NdmpUsersExtended list_ndmp_users(ctx, )


List all ndmp administrators.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::NdmpUsersExtended**](NdmpUsersExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_nfs_aliases**
>crate::models::NfsAliasesExtended list_nfs_aliases(ctx, optional)


List all NFS aliases.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| The field that will be used for sorting. | 
 **zone** | **String**| Access zone | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **check** | **bool**| Check for conflicts when listing aliases. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::NfsAliasesExtended**](NfsAliasesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_nfs_exports**
>crate::models::NfsExportsExtended list_nfs_exports(ctx, optional)


List all NFS exports.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| The field that will be used for sorting. Default is id. | 
 **zone** | **String**| Access zone | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **scope** | **String**| When specified as &#39;effective&#39;, or not specified, all fields are returned. When specified as &#39;user&#39;, only fields with non-default values are shown. When specified as &#39;default&#39;, the original values are returned. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **offset** | **i32**| The position of the first item returned for a paginated query within the full result set. | 
 **path** | **String**| If specified, only exports that explicitly reference at least one of the given paths will be returned. | 
 **check** | **bool**| Check for conflicts when listing exports. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::NfsExportsExtended**](NfsExportsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_ntp_servers**
>crate::models::NtpServersExtended list_ntp_servers(ctx, optional)


List all NTP servers.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| The field that will be used for sorting. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 

### Return type

[**::models::NtpServersExtended**](NtpServersExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_smb_log_level_filters**
>crate::models::SmbLogLevelFilters list_smb_log_level_filters(ctx, optional)


Get the current SMB log filters.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| The field that will be used for sorting. | 
 **dir** | **String**| The direction of the sort. | 
 **level** | **String**| Valid SMB logging levels | 

### Return type

[**::models::SmbLogLevelFilters**](SmbLogLevelFilters.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_smb_shares**
>crate::models::SmbSharesExtended list_smb_shares(ctx, optional)


List all shares.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sort** | **String**| Order results by this field. Default is id. | 
 **zone** | **String**| Zone which contains this share. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **resolve_names** | **bool**| If true, resolve group and user names in personas. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **offset** | **i32**| The position of the first item returned for a paginated query within the full result set. | 
 **scope** | **String**| If specified as \&quot;effective\&quot; or not specified, all fields are returned.  If specified as \&quot;user\&quot;, only fields with non-default values are shown.  If specified as \&quot;default\&quot;, the original values are returned. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::SmbSharesExtended**](SmbSharesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_swift_accounts**
>crate::models::SwiftAccountsExtended list_swift_accounts(ctx, optional)


List all swift accounts.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **zone** | **String**| Access zone which contains Swift accounts. | 

### Return type

[**::models::SwiftAccountsExtended**](SwiftAccountsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_ftp_settings**
> update_ftp_settings(ctx, ftp_settings)


Modify the FTP settings. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ftp_settings** | [**FtpSettingsExtended**](FtpSettingsExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_hdfs_log_level**
> update_hdfs_log_level(ctx, hdfs_log_level)


Modify the HDFS service log-level.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hdfs_log_level** | [**HdfsLogLevel**](HdfsLogLevel.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_hdfs_proxyuser**
> update_hdfs_proxyuser(ctx, hdfs_proxyuser, hdfs_proxyuser_id, optional)


Modify an HDFS proxyuser.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hdfs_proxyuser** | [**Empty**](Empty.md)|  | 
  **hdfs_proxyuser_id** | **String**| Modify an HDFS proxyuser. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hdfs_proxyuser** | [**Empty**](Empty.md)|  | 
 **hdfs_proxyuser_id** | **String**| Modify an HDFS proxyuser. | 
 **zone** | **String**| Access zone which contains HDFS proxyuser. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_hdfs_rack**
> update_hdfs_rack(ctx, hdfs_rack, hdfs_rack_id, optional)


Modify the HDFS rack

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hdfs_rack** | [**HdfsRack**](HdfsRack.md)|  | 
  **hdfs_rack_id** | **String**| Modify the HDFS rack | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hdfs_rack** | [**HdfsRack**](HdfsRack.md)|  | 
 **hdfs_rack_id** | **String**| Modify the HDFS rack | 
 **zone** | **String**| Access zone which contains HDFS rack. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_hdfs_ranger_plugin_settings**
> update_hdfs_ranger_plugin_settings(ctx, hdfs_ranger_plugin_settings, optional)


Modify HDFS ranger-plugin properties.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hdfs_ranger_plugin_settings** | [**HdfsRangerPluginSettingsSettings**](HdfsRangerPluginSettingsSettings.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hdfs_ranger_plugin_settings** | [**HdfsRangerPluginSettingsSettings**](HdfsRangerPluginSettingsSettings.md)|  | 
 **zone** | **String**| Access zone which contains HDFS ranger-plugin settings. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_hdfs_settings**
> update_hdfs_settings(ctx, hdfs_settings, optional)


Modify HDFS properties.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **hdfs_settings** | [**HdfsSettingsSettings**](HdfsSettingsSettings.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **hdfs_settings** | [**HdfsSettingsSettings**](HdfsSettingsSettings.md)|  | 
 **zone** | **String**| Access zone which contains HDFS settings. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_http_settings**
> update_http_settings(ctx, http_settings)


Modify HTTP properties.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **http_settings** | [**HttpSettingsSettings**](HttpSettingsSettings.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_ndmp_diagnostics**
> update_ndmp_diagnostics(ctx, ndmp_diagnostics)


Modify ndmp diagnostics settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_diagnostics** | [**NdmpDiagnosticsDiagnostics**](NdmpDiagnosticsDiagnostics.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_ndmp_settings_global**
> update_ndmp_settings_global(ctx, ndmp_settings_global)


Modify one or more settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_settings_global** | [**NdmpSettingsGlobalGlobal**](NdmpSettingsGlobalGlobal.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_ndmp_settings_preferred_ip**
> update_ndmp_settings_preferred_ip(ctx, ndmp_settings_preferred_ip, ndmp_settings_preferred_ip_id)


Modify a preferred ip preference.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_settings_preferred_ip** | [**NdmpSettingsPreferredIp**](NdmpSettingsPreferredIp.md)|  | 
  **ndmp_settings_preferred_ip_id** | **String**| Modify a preferred ip preference. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_ndmp_settings_variable**
> update_ndmp_settings_variable(ctx, ndmp_settings_variable, ndmp_settings_variable_id, optional)


Modify or create a NDMP preferred environment variable.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_settings_variable** | [**NdmpSettingsVariable**](NdmpSettingsVariable.md)|  | 
  **ndmp_settings_variable_id** | **String**| Modify or create a NDMP preferred environment variable. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ndmp_settings_variable** | [**NdmpSettingsVariable**](NdmpSettingsVariable.md)|  | 
 **ndmp_settings_variable_id** | **String**| Modify or create a NDMP preferred environment variable. | 
 **name** | **String**| Name of the variable to modify. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_ndmp_user**
> update_ndmp_user(ctx, ndmp_user, ndmp_user_id)


Modify the user

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ndmp_user** | [**NdmpUser**](NdmpUser.md)|  | 
  **ndmp_user_id** | **String**| Modify the user | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_nfs_alias**
> update_nfs_alias(ctx, nfs_alias, nfs_alias_id, optional)


Modify the alias. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_alias** | [**NfsAlias**](NfsAlias.md)|  | 
  **nfs_alias_id** | **String**| Modify the alias. All input fields are optional, but one or more must be supplied. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_alias** | [**NfsAlias**](NfsAlias.md)|  | 
 **nfs_alias_id** | **String**| Modify the alias. All input fields are optional, but one or more must be supplied. | 
 **zone** | **String**| Access zone | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_nfs_export**
> update_nfs_export(ctx, nfs_export, nfs_export_id, optional)


Modify the export. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_export** | [**NfsExport**](NfsExport.md)|  | 
  **nfs_export_id** | **String**| Modify the export. All input fields are optional, but one or more must be supplied. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_export** | [**NfsExport**](NfsExport.md)|  | 
 **nfs_export_id** | **String**| Modify the export. All input fields are optional, but one or more must be supplied. | 
 **force** | **bool**| If true, the export will be updated even if that change conflicts with another export. | 
 **ignore_unresolvable_hosts** | **bool**| Ignore unresolvable hosts. | 
 **zone** | **String**| Access zone | 
 **ignore_conflicts** | **bool**| Ignore conflicts with existing exports. | 
 **ignore_bad_paths** | **bool**| Ignore nonexistent or otherwise bad paths. | 
 **ignore_bad_auth** | **bool**| Ignore invalid users. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_nfs_log_level**
> update_nfs_log_level(ctx, nfs_log_level)


Set the current NFS service logging level.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_log_level** | [**NfsLogLevel**](NfsLogLevel.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_nfs_netgroup**
> update_nfs_netgroup(ctx, nfs_netgroup, optional)


Modify the current NFS netgroup settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_netgroup** | [**NfsNetgroup**](NfsNetgroup.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_netgroup** | [**NfsNetgroup**](NfsNetgroup.md)|  | 
 **host** | **String**| Host to retrieve netgroup cache settings for. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_nfs_settings_export**
> update_nfs_settings_export(ctx, nfs_settings_export, optional)


Modify the default values for NFS exports. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_settings_export** | [**NfsSettingsExportSettings**](NfsSettingsExportSettings.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_settings_export** | [**NfsSettingsExportSettings**](NfsSettingsExportSettings.md)|  | 
 **zone** | **String**| Access zone | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_nfs_settings_global**
> update_nfs_settings_global(ctx, nfs_settings_global, optional)


Modify the default values for NFS exports. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_settings_global** | [**NfsSettingsGlobalSettings**](NfsSettingsGlobalSettings.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_settings_global** | [**NfsSettingsGlobalSettings**](NfsSettingsGlobalSettings.md)|  | 
 **scope** | **String**| When specified as &#39;effective&#39;, or not specified, all fields are returned. When specified as &#39;user&#39;, only fields with non-default values are shown. When specified as &#39;default&#39;, the original values are returned. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_nfs_settings_zone**
> update_nfs_settings_zone(ctx, nfs_settings_zone, optional)


Modify the NFS server settings for this zone.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **nfs_settings_zone** | [**NfsSettingsZoneSettings**](NfsSettingsZoneSettings.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **nfs_settings_zone** | [**NfsSettingsZoneSettings**](NfsSettingsZoneSettings.md)|  | 
 **zone** | **String**| Access zone | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_ntp_server**
> update_ntp_server(ctx, ntp_server, ntp_server_id)


Modify the key value for an NTP server.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ntp_server** | [**NtpServer**](NtpServer.md)|  | 
  **ntp_server_id** | **String**| Modify the key value for an NTP server. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_ntp_settings**
> update_ntp_settings(ctx, ntp_settings)


Modify the NTP settings. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **ntp_settings** | [**NtpSettingsSettings**](NtpSettingsSettings.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_smb_log_level**
> update_smb_log_level(ctx, smb_log_level)


Set the current SMB logging level.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **smb_log_level** | [**SmbLogLevel**](SmbLogLevel.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_smb_settings_global**
> update_smb_settings_global(ctx, smb_settings_global)


Modify one or more settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **smb_settings_global** | [**SmbSettingsGlobalExtended**](SmbSettingsGlobalExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_smb_settings_share**
> update_smb_settings_share(ctx, smb_settings_share, optional)


Modify one or more settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **smb_settings_share** | [**SmbSettingsShareExtended**](SmbSettingsShareExtended.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **smb_settings_share** | [**SmbSettingsShareExtended**](SmbSettingsShareExtended.md)|  | 
 **zone** | **String**| Zone which contains these share settings. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_smb_share**
> update_smb_share(ctx, smb_share, smb_share_id, optional)


Modify share. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **smb_share** | [**SmbShare**](SmbShare.md)|  | 
  **smb_share_id** | **String**| Modify share. All input fields are optional, but one or more must be supplied. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **smb_share** | [**SmbShare**](SmbShare.md)|  | 
 **smb_share_id** | **String**| Modify share. All input fields are optional, but one or more must be supplied. | 
 **zone** | **String**| Zone which contains this share. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_snmp_settings**
> update_snmp_settings(ctx, snmp_settings)


Modify the SNMP settings. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **snmp_settings** | [**SnmpSettingsExtended**](SnmpSettingsExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_swift_account**
> update_swift_account(ctx, swift_account, swift_account_id, optional)


Modify a Swift account

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **swift_account** | [**SwiftAccount**](SwiftAccount.md)|  | 
  **swift_account_id** | **String**| Modify a Swift account | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **swift_account** | [**SwiftAccount**](SwiftAccount.md)|  | 
 **swift_account_id** | **String**| Modify a Swift account | 
 **zone** | **String**| Access zone which contains Swift account. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

