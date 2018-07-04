# \AntivirusApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_antivirus_policy**](AntivirusApi.md#create_antivirus_policy) | **Post** /platform/3/antivirus/policies | 
[**create_antivirus_scan_item**](AntivirusApi.md#create_antivirus_scan_item) | **Post** /platform/3/antivirus/scan | 
[**create_antivirus_server**](AntivirusApi.md#create_antivirus_server) | **Post** /platform/3/antivirus/servers | 
[**delete_antivirus_policies**](AntivirusApi.md#delete_antivirus_policies) | **Delete** /platform/3/antivirus/policies | 
[**delete_antivirus_policy**](AntivirusApi.md#delete_antivirus_policy) | **Delete** /platform/3/antivirus/policies/{AntivirusPolicyId} | 
[**delete_antivirus_server**](AntivirusApi.md#delete_antivirus_server) | **Delete** /platform/3/antivirus/servers/{AntivirusServerId} | 
[**delete_antivirus_servers**](AntivirusApi.md#delete_antivirus_servers) | **Delete** /platform/3/antivirus/servers | 
[**delete_reports_scan**](AntivirusApi.md#delete_reports_scan) | **Delete** /platform/3/antivirus/reports/scans/{ReportsScanId} | 
[**delete_reports_scans**](AntivirusApi.md#delete_reports_scans) | **Delete** /platform/3/antivirus/reports/scans | 
[**get_antivirus_policy**](AntivirusApi.md#get_antivirus_policy) | **Get** /platform/3/antivirus/policies/{AntivirusPolicyId} | 
[**get_antivirus_quarantine_path**](AntivirusApi.md#get_antivirus_quarantine_path) | **Get** /platform/3/antivirus/quarantine/{AntivirusQuarantinePath} | 
[**get_antivirus_server**](AntivirusApi.md#get_antivirus_server) | **Get** /platform/3/antivirus/servers/{AntivirusServerId} | 
[**get_antivirus_settings**](AntivirusApi.md#get_antivirus_settings) | **Get** /platform/3/antivirus/settings | 
[**get_reports_scan**](AntivirusApi.md#get_reports_scan) | **Get** /platform/3/antivirus/reports/scans/{ReportsScanId} | 
[**get_reports_scans**](AntivirusApi.md#get_reports_scans) | **Get** /platform/3/antivirus/reports/scans | 
[**get_reports_threat**](AntivirusApi.md#get_reports_threat) | **Get** /platform/3/antivirus/reports/threats/{ReportsThreatId} | 
[**get_reports_threats**](AntivirusApi.md#get_reports_threats) | **Get** /platform/3/antivirus/reports/threats | 
[**list_antivirus_policies**](AntivirusApi.md#list_antivirus_policies) | **Get** /platform/3/antivirus/policies | 
[**list_antivirus_servers**](AntivirusApi.md#list_antivirus_servers) | **Get** /platform/3/antivirus/servers | 
[**update_antivirus_policy**](AntivirusApi.md#update_antivirus_policy) | **Put** /platform/3/antivirus/policies/{AntivirusPolicyId} | 
[**update_antivirus_quarantine_path**](AntivirusApi.md#update_antivirus_quarantine_path) | **Put** /platform/3/antivirus/quarantine/{AntivirusQuarantinePath} | 
[**update_antivirus_server**](AntivirusApi.md#update_antivirus_server) | **Put** /platform/3/antivirus/servers/{AntivirusServerId} | 
[**update_antivirus_settings**](AntivirusApi.md#update_antivirus_settings) | **Put** /platform/3/antivirus/settings | 


# **create_antivirus_policy**
> ::models::CreateResponse create_antivirus_policy(ctx, antivirus_policy)


Create new antivirus scan policies.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **antivirus_policy** | [**AntivirusPolicyCreateParams**](AntivirusPolicyCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_antivirus_scan_item**
> ::models::CreateAntivirusScanItemResponse create_antivirus_scan_item(ctx, antivirus_scan_item)


Manually scan a file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **antivirus_scan_item** | [**AntivirusScanItem**](AntivirusScanItem.md)|  | 

### Return type

[**::models::CreateAntivirusScanItemResponse**](CreateAntivirusScanItemResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_antivirus_server**
> ::models::CreateResponse create_antivirus_server(ctx, antivirus_server)


Create new antivirus servers.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **antivirus_server** | [**AntivirusServerCreateParams**](AntivirusServerCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_antivirus_policies**
> delete_antivirus_policies(ctx, )


Delete all antivirus scan policies.

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

# **delete_antivirus_policy**
> delete_antivirus_policy(ctx, antivirus_policy_id)


Delete an antivirus scan policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **antivirus_policy_id** | **String**| Delete an antivirus scan policy. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_antivirus_server**
> delete_antivirus_server(ctx, antivirus_server_id)


Delete an antivirus server entry.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **antivirus_server_id** | **String**| Delete an antivirus server entry. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_antivirus_servers**
> delete_antivirus_servers(ctx, )


Delete all antivirus servers.

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

# **delete_reports_scan**
> delete_reports_scan(ctx, reports_scan_id)


Delete one antivirus scan report, and all of its associated threat reports.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **reports_scan_id** | **String**| Delete one antivirus scan report, and all of its associated threat reports. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_reports_scans**
> delete_reports_scans(ctx, optional)


Delete antivirus scan reports, and any threat reports associated with those scans.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **age** | **i32**| An amount of time in seconds. If present, only reports older than this age are deleted. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_antivirus_policy**
> ::models::AntivirusPolicies get_antivirus_policy(ctx, antivirus_policy_id)


Retrieve one antivirus scan policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **antivirus_policy_id** | **String**| Retrieve one antivirus scan policy. | 

### Return type

[**::models::AntivirusPolicies**](AntivirusPolicies.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_antivirus_quarantine_path**
> ::models::AntivirusQuarantine get_antivirus_quarantine_path(ctx, antivirus_quarantine_path)


Retrieve the quarantine status of the file at the specified path.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **antivirus_quarantine_path** | **String**| Retrieve the quarantine status of the file at the specified path. | 

### Return type

[**::models::AntivirusQuarantine**](AntivirusQuarantine.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_antivirus_server**
> ::models::AntivirusServers get_antivirus_server(ctx, antivirus_server_id)


Retrieve one antivirus server entry.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **antivirus_server_id** | **String**| Retrieve one antivirus server entry. | 

### Return type

[**::models::AntivirusServers**](AntivirusServers.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_antivirus_settings**
> ::models::AntivirusSettings get_antivirus_settings(ctx, )


Retrieve the Antivirus settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::AntivirusSettings**](AntivirusSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_reports_scan**
> ::models::ReportsScans get_reports_scan(ctx, reports_scan_id)


Retrieve one antivirus scan report.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **reports_scan_id** | **String**| Retrieve one antivirus scan report. | 

### Return type

[**::models::ReportsScans**](ReportsScans.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_reports_scans**
> ::models::ReportsScansExtended get_reports_scans(ctx, optional)


List antivirus scan reports.

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
 **status** | **String**| If present, only scan reports with this status will be returned. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **dir** | **String**| The direction of the sort. | 
 **policy_id** | **String**| If present, only reports for scans associated with this policy will be returned. | 

### Return type

[**::models::ReportsScansExtended**](ReportsScansExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_reports_threat**
> ::models::ReportsThreats get_reports_threat(ctx, reports_threat_id)


Retrieve one antivirus threat report.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **reports_threat_id** | **String**| Retrieve one antivirus threat report. | 

### Return type

[**::models::ReportsThreats**](ReportsThreats.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_reports_threats**
> ::models::ReportsThreatsExtended get_reports_threats(ctx, optional)


List antivirus threat reports.

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
 **scan_id** | **String**| If present, only returns threat reports associated with the given scan report. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **file** | **String**| If present, only returns threat reports for the given file. | 
 **remediation** | **String**| If present, only returns threat reports with the given remediation. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::ReportsThreatsExtended**](ReportsThreatsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_antivirus_policies**
> ::models::AntivirusPoliciesExtended list_antivirus_policies(ctx, optional)


List antivirus scan policies.

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

[**::models::AntivirusPoliciesExtended**](AntivirusPoliciesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_antivirus_servers**
> ::models::AntivirusServersExtended list_antivirus_servers(ctx, optional)


List antivirus servers.

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

[**::models::AntivirusServersExtended**](AntivirusServersExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_antivirus_policy**
> update_antivirus_policy(ctx, antivirus_policy, antivirus_policy_id)


Modify an antivirus scan policy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **antivirus_policy** | [**AntivirusPolicy**](AntivirusPolicy.md)|  | 
  **antivirus_policy_id** | **String**| Modify an antivirus scan policy. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_antivirus_quarantine_path**
> update_antivirus_quarantine_path(ctx, antivirus_quarantine_path_params, antivirus_quarantine_path)


Set the quarantine status of the file at the specified path.  Use either an empty object {} in the request body or {\"quarantined\":true} to quarantine the file, and {\"quarantined\":false} to unquarantine the file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **antivirus_quarantine_path_params** | [**AntivirusQuarantinePathParams**](AntivirusQuarantinePathParams.md)|  | 
  **antivirus_quarantine_path** | **String**| Set the quarantine status of the file at the specified path.  Use either an empty object {} in the request body or {\&quot;quarantined\&quot;:true} to quarantine the file, and {\&quot;quarantined\&quot;:false} to unquarantine the file. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_antivirus_server**
> update_antivirus_server(ctx, antivirus_server, antivirus_server_id)


Modify an antivirus server entry.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **antivirus_server** | [**AntivirusServer**](AntivirusServer.md)|  | 
  **antivirus_server_id** | **String**| Modify an antivirus server entry. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_antivirus_settings**
> update_antivirus_settings(ctx, antivirus_settings)


Modify the Antivirus settings. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **antivirus_settings** | [**AntivirusSettingsSettings**](AntivirusSettingsSettings.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

