# \AuditApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_audit_topic**](AuditApi.md#create_audit_topic) | **Post** /platform/1/audit/topics | 
[**delete_audit_topic**](AuditApi.md#delete_audit_topic) | **Delete** /platform/1/audit/topics/{AuditTopicId} | 
[**get_audit_progress**](AuditApi.md#get_audit_progress) | **Get** /platform/4/audit/progress | 
[**get_audit_settings**](AuditApi.md#get_audit_settings) | **Get** /platform/3/audit/settings | 
[**get_audit_topic**](AuditApi.md#get_audit_topic) | **Get** /platform/1/audit/topics/{AuditTopicId} | 
[**get_progress_global**](AuditApi.md#get_progress_global) | **Get** /platform/4/audit/progress/global | 
[**get_settings_global**](AuditApi.md#get_settings_global) | **Get** /platform/3/audit/settings/global | 
[**list_audit_topics**](AuditApi.md#list_audit_topics) | **Get** /platform/1/audit/topics | 
[**update_audit_settings**](AuditApi.md#update_audit_settings) | **Put** /platform/3/audit/settings | 
[**update_audit_topic**](AuditApi.md#update_audit_topic) | **Put** /platform/1/audit/topics/{AuditTopicId} | 
[**update_settings_global**](AuditApi.md#update_settings_global) | **Put** /platform/3/audit/settings/global | 


# **create_audit_topic**
> ::models::CreateResponse create_audit_topic(ctx, audit_topic)


Create a new audit topic.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **audit_topic** | [**AuditTopicCreateParams**](AuditTopicCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_audit_topic**
> delete_audit_topic(ctx, audit_topic_id)


Delete the audit topic.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **audit_topic_id** | **String**| Delete the audit topic. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_audit_progress**
> ::models::AuditProgress get_audit_progress(ctx, optional)


View current audit log time.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lnn** | **i32**| lnn of the node. | 

### Return type

[**::models::AuditProgress**](AuditProgress.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_audit_settings**
> ::models::AuditSettings get_audit_settings(ctx, optional)


View per-Access Zone Audit settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **zone** | **String**| Access zone which contains audit settings. | 

### Return type

[**::models::AuditSettings**](AuditSettings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_audit_topic**
> ::models::AuditTopics get_audit_topic(ctx, audit_topic_id)


Retrieve the audit topic information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **audit_topic_id** | **String**| Retrieve the audit topic information. | 

### Return type

[**::models::AuditTopics**](AuditTopics.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_progress_global**
> ::models::ProgressGlobal get_progress_global(ctx, )


View the global audit log time.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::ProgressGlobal**](ProgressGlobal.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_settings_global**
> ::models::SettingsGlobalExtended get_settings_global(ctx, )


View Global Audit settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SettingsGlobalExtended**](SettingsGlobalExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_audit_topics**
> ::models::AuditTopicsExtended list_audit_topics(ctx, )


Retrieve a list of audit topics.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::AuditTopicsExtended**](AuditTopicsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_audit_settings**
> update_audit_settings(ctx, audit_settings, optional)


Modify per-Access Zone Audit settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **audit_settings** | [**AuditSettingsSettings**](AuditSettingsSettings.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **audit_settings** | [**AuditSettingsSettings**](AuditSettingsSettings.md)|  | 
 **zone** | **String**| Access zone which contains audit settings. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_audit_topic**
> update_audit_topic(ctx, audit_topic, audit_topic_id)


Modify the audit topic.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **audit_topic** | [**AuditTopic**](AuditTopic.md)|  | 
  **audit_topic_id** | **String**| Modify the audit topic. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_settings_global**
> update_settings_global(ctx, settings_global)


Modify Global Audit settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_global** | [**SettingsGlobalSettings**](SettingsGlobalSettings.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

