# \CloudApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cloud_access_item**](CloudApi.md#create_cloud_access_item) | **Post** /platform/3/cloud/access | 
[**create_cloud_account**](CloudApi.md#create_cloud_account) | **Post** /platform/4/cloud/accounts | 
[**create_cloud_job**](CloudApi.md#create_cloud_job) | **Post** /platform/3/cloud/jobs | 
[**create_cloud_pool**](CloudApi.md#create_cloud_pool) | **Post** /platform/3/cloud/pools | 
[**create_cloud_proxy**](CloudApi.md#create_cloud_proxy) | **Post** /platform/4/cloud/proxies | 
[**create_settings_encryption_key_item**](CloudApi.md#create_settings_encryption_key_item) | **Post** /platform/3/cloud/settings/encryption-key | 
[**create_settings_reporting_eula_item**](CloudApi.md#create_settings_reporting_eula_item) | **Post** /platform/3/cloud/settings/reporting-eula | 
[**delete_cloud_access_guid**](CloudApi.md#delete_cloud_access_guid) | **Delete** /platform/3/cloud/access/{CloudAccessGuid} | 
[**delete_cloud_account**](CloudApi.md#delete_cloud_account) | **Delete** /platform/4/cloud/accounts/{CloudAccountId} | 
[**delete_cloud_pool**](CloudApi.md#delete_cloud_pool) | **Delete** /platform/3/cloud/pools/{CloudPoolId} | 
[**delete_cloud_proxy**](CloudApi.md#delete_cloud_proxy) | **Delete** /platform/4/cloud/proxies/{CloudProxyId} | 
[**delete_settings_reporting_eula**](CloudApi.md#delete_settings_reporting_eula) | **Delete** /platform/3/cloud/settings/reporting-eula | 
[**get_cloud_access_guid**](CloudApi.md#get_cloud_access_guid) | **Get** /platform/3/cloud/access/{CloudAccessGuid} | 
[**get_cloud_account**](CloudApi.md#get_cloud_account) | **Get** /platform/4/cloud/accounts/{CloudAccountId} | 
[**get_cloud_job**](CloudApi.md#get_cloud_job) | **Get** /platform/3/cloud/jobs/{CloudJobId} | 
[**get_cloud_jobs_file**](CloudApi.md#get_cloud_jobs_file) | **Get** /platform/3/cloud/jobs-files/{CloudJobsFileId} | 
[**get_cloud_pool**](CloudApi.md#get_cloud_pool) | **Get** /platform/3/cloud/pools/{CloudPoolId} | 
[**get_cloud_proxy**](CloudApi.md#get_cloud_proxy) | **Get** /platform/4/cloud/proxies/{CloudProxyId} | 
[**get_cloud_settings**](CloudApi.md#get_cloud_settings) | **Get** /platform/3/cloud/settings | 
[**list_cloud_access**](CloudApi.md#list_cloud_access) | **Get** /platform/3/cloud/access | 
[**list_cloud_accounts**](CloudApi.md#list_cloud_accounts) | **Get** /platform/4/cloud/accounts | 
[**list_cloud_jobs**](CloudApi.md#list_cloud_jobs) | **Get** /platform/3/cloud/jobs | 
[**list_cloud_pools**](CloudApi.md#list_cloud_pools) | **Get** /platform/3/cloud/pools | 
[**list_cloud_proxies**](CloudApi.md#list_cloud_proxies) | **Get** /platform/4/cloud/proxies | 
[**list_settings_reporting_eula**](CloudApi.md#list_settings_reporting_eula) | **Get** /platform/3/cloud/settings/reporting-eula | 
[**update_cloud_account**](CloudApi.md#update_cloud_account) | **Put** /platform/4/cloud/accounts/{CloudAccountId} | 
[**update_cloud_job**](CloudApi.md#update_cloud_job) | **Put** /platform/3/cloud/jobs/{CloudJobId} | 
[**update_cloud_pool**](CloudApi.md#update_cloud_pool) | **Put** /platform/3/cloud/pools/{CloudPoolId} | 
[**update_cloud_proxy**](CloudApi.md#update_cloud_proxy) | **Put** /platform/4/cloud/proxies/{CloudProxyId} | 
[**update_cloud_settings**](CloudApi.md#update_cloud_settings) | **Put** /platform/3/cloud/settings | 


# **create_cloud_access_item**
> ::models::Empty create_cloud_access_item(ctx, cloud_access_item)


Add a cluster identifier to access list.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_access_item** | [**CloudAccessItem**](CloudAccessItem.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cloud_account**
> ::models::CreateCloudAccountResponse create_cloud_account(ctx, cloud_account)


Create a new account.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_account** | [**CloudAccountCreateParams**](CloudAccountCreateParams.md)|  | 

### Return type

[**::models::CreateCloudAccountResponse**](CreateCloudAccountResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cloud_job**
> ::models::CreateCloudJobResponse create_cloud_job(ctx, cloud_job)


Create a new job.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_job** | [**CloudJobCreateParams**](CloudJobCreateParams.md)|  | 

### Return type

[**::models::CreateCloudJobResponse**](CreateCloudJobResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cloud_pool**
> ::models::CreateCloudPoolResponse create_cloud_pool(ctx, cloud_pool)


Create a new pool.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_pool** | [**CloudPoolCreateParams**](CloudPoolCreateParams.md)|  | 

### Return type

[**::models::CreateCloudPoolResponse**](CreateCloudPoolResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_cloud_proxy**
> ::models::CreateCloudProxyResponse create_cloud_proxy(ctx, cloud_proxy)


Create a new proxy.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_proxy** | [**CloudProxyCreateParams**](CloudProxyCreateParams.md)|  | 

### Return type

[**::models::CreateCloudProxyResponse**](CreateCloudProxyResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_settings_encryption_key_item**
> ::models::Empty create_settings_encryption_key_item(ctx, settings_encryption_key_item)


Regenerate master encryption key.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_encryption_key_item** | [**Empty**](Empty.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_settings_reporting_eula_item**
> ::models::SettingsReportingEulaItem create_settings_reporting_eula_item(ctx, settings_reporting_eula_item)


Accept telemetry collection EULA.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_reporting_eula_item** | [**SettingsReportingEulaItem**](SettingsReportingEulaItem.md)|  | 

### Return type

[**::models::SettingsReportingEulaItem**](SettingsReportingEulaItem.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_cloud_access_guid**
> delete_cloud_access_guid(ctx, cloud_access_guid)


Delete cloud access.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_access_guid** | **String**| Delete cloud access. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_cloud_account**
> delete_cloud_account(ctx, cloud_account_id, optional)


Delete cloud account.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_account_id** | **String**| Delete cloud account. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cloud_account_id** | **String**| Delete cloud account. | 
 **acknowledge_force_delete** | **String**| A value of 1 acknowledges that the user is deleting data. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_cloud_pool**
> delete_cloud_pool(ctx, cloud_pool_id, optional)


Delete a cloud pool.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_pool_id** | **String**| Delete a cloud pool. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cloud_pool_id** | **String**| Delete a cloud pool. | 
 **acknowledge_force_delete** | **String**| A value of 1 acknowledges that the user is deleting data. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_cloud_proxy**
> delete_cloud_proxy(ctx, cloud_proxy_id)


Delete cloud account.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_proxy_id** | **String**| Delete cloud account. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_settings_reporting_eula**
> delete_settings_reporting_eula(ctx, )


Revoke acceptance of telemetry collection EULA.

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

# **get_cloud_access_guid**
> ::models::CloudAccess get_cloud_access_guid(ctx, cloud_access_guid)


Retrieve cloud access information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_access_guid** | **String**| Retrieve cloud access information. | 

### Return type

[**::models::CloudAccess**](CloudAccess.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cloud_account**
> ::models::CloudAccounts get_cloud_account(ctx, cloud_account_id)


Retrieve cloud account information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_account_id** | **String**| Retrieve cloud account information. | 

### Return type

[**::models::CloudAccounts**](CloudAccounts.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cloud_job**
> ::models::CloudJobs get_cloud_job(ctx, cloud_job_id)


Retrieve cloudpool job information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_job_id** | **String**| Retrieve cloudpool job information. | 

### Return type

[**::models::CloudJobs**](CloudJobs.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cloud_jobs_file**
> ::models::CloudJobsFiles get_cloud_jobs_file(ctx, cloud_jobs_file_id, optional)


Retrieve files associated with a cloudpool job.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_jobs_file_id** | **String**| Retrieve files associated with a cloudpool job. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **cloud_jobs_file_id** | **String**| Retrieve files associated with a cloudpool job. | 
 **sort** | **String**| The field that will be used for sorting. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **batch** | **bool**| If true, only \&quot;limit\&quot; and \&quot;page\&quot; arguments are honored.  Query will return all results, unsorted, as quickly as possible. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **page** | **i32**| Works only when \&quot;batch\&quot; parameter and \&quot;limit\&quot; parameters are specified.  Indicates which the page index of results to be returned | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::CloudJobsFiles**](CloudJobsFiles.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cloud_pool**
> ::models::CloudPools get_cloud_pool(ctx, cloud_pool_id)


Retrieve cloud pool information

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_pool_id** | **String**| Retrieve cloud pool information | 

### Return type

[**::models::CloudPools**](CloudPools.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cloud_proxy**
> ::models::CloudProxies get_cloud_proxy(ctx, cloud_proxy_id)


Retrieve cloud account information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_proxy_id** | **String**| Retrieve cloud account information. | 

### Return type

[**::models::CloudProxies**](CloudProxies.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_cloud_settings**
> ::models::CloudSettings get_cloud_settings(ctx, )


List all cloud settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::CloudSettings**](CloudSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_cloud_access**
> ::models::CloudAccessExtended list_cloud_access(ctx, optional)


List all accessible cluster identifiers.

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

### Return type

[**::models::CloudAccessExtended**](CloudAccessExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_cloud_accounts**
> ::models::CloudAccountsExtended list_cloud_accounts(ctx, optional)


List all accounts.

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

### Return type

[**::models::CloudAccountsExtended**](CloudAccountsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_cloud_jobs**
> ::models::CloudJobsExtended list_cloud_jobs(ctx, optional)


List all cloudpools jobs.

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

### Return type

[**::models::CloudJobsExtended**](CloudJobsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_cloud_pools**
> ::models::CloudPoolsExtended list_cloud_pools(ctx, optional)


List all pools.

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

### Return type

[**::models::CloudPoolsExtended**](CloudPoolsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_cloud_proxies**
> ::models::CloudProxiesExtended list_cloud_proxies(ctx, optional)


List all proxies.

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

### Return type

[**::models::CloudProxiesExtended**](CloudProxiesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_settings_reporting_eula**
> ::models::SettingsReportingEulaItem list_settings_reporting_eula(ctx, )


View telemetry collection EULA acceptance and content URI.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SettingsReportingEulaItem**](SettingsReportingEulaItem.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_cloud_account**
> update_cloud_account(ctx, cloud_account, cloud_account_id)


Modify cloud account.  All fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_account** | [**CloudAccount**](CloudAccount.md)|  | 
  **cloud_account_id** | **String**| Modify cloud account.  All fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_cloud_job**
> update_cloud_job(ctx, cloud_job, cloud_job_id)


Modify a cloud job or operation.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_job** | [**CloudJob**](CloudJob.md)|  | 
  **cloud_job_id** | **String**| Modify a cloud job or operation. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_cloud_pool**
> update_cloud_pool(ctx, cloud_pool, cloud_pool_id)


Modify a cloud pool.  All fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_pool** | [**CloudPool**](CloudPool.md)|  | 
  **cloud_pool_id** | **String**| Modify a cloud pool.  All fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_cloud_proxy**
> update_cloud_proxy(ctx, cloud_proxy, cloud_proxy_id)


Modify cloud account.  All fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_proxy** | [**CloudProxy**](CloudProxy.md)|  | 
  **cloud_proxy_id** | **String**| Modify cloud account.  All fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_cloud_settings**
> update_cloud_settings(ctx, cloud_settings)


Modify one or more settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **cloud_settings** | [**CloudSettingsSettings**](CloudSettingsSettings.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

