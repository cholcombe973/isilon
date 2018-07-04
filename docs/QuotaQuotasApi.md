# \QuotaQuotasApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_quota_notification**](QuotaQuotasApi.md#create_quota_notification) | **Post** /platform/1/quota/quotas/{Qid}/notifications | 
[**delete_quota_notification**](QuotaQuotasApi.md#delete_quota_notification) | **Delete** /platform/1/quota/quotas/{Qid}/notifications/{QuotaNotificationId} | 
[**delete_quota_notifications**](QuotaQuotasApi.md#delete_quota_notifications) | **Delete** /platform/1/quota/quotas/{Qid}/notifications | 
[**get_quota_notification**](QuotaQuotasApi.md#get_quota_notification) | **Get** /platform/1/quota/quotas/{Qid}/notifications/{QuotaNotificationId} | 
[**list_quota_notifications**](QuotaQuotasApi.md#list_quota_notifications) | **Get** /platform/1/quota/quotas/{Qid}/notifications | 
[**update_quota_notification**](QuotaQuotasApi.md#update_quota_notification) | **Put** /platform/1/quota/quotas/{Qid}/notifications/{QuotaNotificationId} | 
[**update_quota_notifications**](QuotaQuotasApi.md#update_quota_notifications) | **Put** /platform/1/quota/quotas/{Qid}/notifications | 


# **create_quota_notification**
> ::models::CreateResponse create_quota_notification(ctx, quota_notification, qid)


Create a new notification rule specific to this quota.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **quota_notification** | [**QuotaNotificationCreateParams**](QuotaNotificationCreateParams.md)|  | 
  **qid** | **String**|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_quota_notification**
> delete_quota_notification(ctx, quota_notification_id, qid)


Delete the notification rule.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **quota_notification_id** | **String**| Delete the notification rule. | 
  **qid** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_quota_notifications**
> delete_quota_notifications(ctx, qid)


Delete all quota specific rules. The quota will then use the global rules.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **qid** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_quota_notification**
> ::models::QuotaNotifications get_quota_notification(ctx, quota_notification_id, qid)


Retrieve notification rule information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **quota_notification_id** | **String**| Retrieve notification rule information. | 
  **qid** | **String**|  | 

### Return type

[**::models::QuotaNotifications**](QuotaNotifications.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_quota_notifications**
> ::models::QuotaNotificationsExtended list_quota_notifications(ctx, qid)


List all rules.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **qid** | **String**|  | 

### Return type

[**::models::QuotaNotificationsExtended**](QuotaNotificationsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_quota_notification**
> update_quota_notification(ctx, quota_notification, quota_notification_id, qid)


Modify notification rule. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **quota_notification** | [**QuotaNotification**](QuotaNotification.md)|  | 
  **quota_notification_id** | **String**| Modify notification rule. All input fields are optional, but one or more must be supplied. | 
  **qid** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_quota_notifications**
> update_quota_notifications(ctx, quota_notifications, qid)


This method creates an empty set of rules so that the global rules are not used. The input must be an empty JSON object.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **quota_notifications** | [**Empty**](Empty.md)|  | 
  **qid** | **String**|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

