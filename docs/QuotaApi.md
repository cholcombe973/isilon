# \QuotaApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_quota_quota**](QuotaApi.md#create_quota_quota) | **Post** /platform/1/quota/quotas | 
[**create_quota_report**](QuotaApi.md#create_quota_report) | **Post** /platform/1/quota/reports | 
[**create_settings_mapping**](QuotaApi.md#create_settings_mapping) | **Post** /platform/1/quota/settings/mappings | 
[**create_settings_notification**](QuotaApi.md#create_settings_notification) | **Post** /platform/1/quota/settings/notifications | 
[**delete_quota_quota**](QuotaApi.md#delete_quota_quota) | **Delete** /platform/1/quota/quotas/{QuotaQuotaId} | 
[**delete_quota_quotas**](QuotaApi.md#delete_quota_quotas) | **Delete** /platform/1/quota/quotas | 
[**delete_quota_report**](QuotaApi.md#delete_quota_report) | **Delete** /platform/1/quota/reports/{QuotaReportId} | 
[**delete_settings_mapping**](QuotaApi.md#delete_settings_mapping) | **Delete** /platform/1/quota/settings/mappings/{SettingsMappingId} | 
[**delete_settings_mappings**](QuotaApi.md#delete_settings_mappings) | **Delete** /platform/1/quota/settings/mappings | 
[**delete_settings_notification**](QuotaApi.md#delete_settings_notification) | **Delete** /platform/1/quota/settings/notifications/{SettingsNotificationId} | 
[**delete_settings_notifications**](QuotaApi.md#delete_settings_notifications) | **Delete** /platform/1/quota/settings/notifications | 
[**get_quota_license**](QuotaApi.md#get_quota_license) | **Get** /platform/5/quota/license | 
[**get_quota_quota**](QuotaApi.md#get_quota_quota) | **Get** /platform/1/quota/quotas/{QuotaQuotaId} | 
[**get_quota_quotas_summary**](QuotaApi.md#get_quota_quotas_summary) | **Get** /platform/1/quota/quotas-summary | 
[**get_quota_report**](QuotaApi.md#get_quota_report) | **Get** /platform/1/quota/reports/{QuotaReportId} | 
[**get_settings_mapping**](QuotaApi.md#get_settings_mapping) | **Get** /platform/1/quota/settings/mappings/{SettingsMappingId} | 
[**get_settings_notification**](QuotaApi.md#get_settings_notification) | **Get** /platform/1/quota/settings/notifications/{SettingsNotificationId} | 
[**get_settings_reports**](QuotaApi.md#get_settings_reports) | **Get** /platform/1/quota/settings/reports | 
[**list_quota_quotas**](QuotaApi.md#list_quota_quotas) | **Get** /platform/1/quota/quotas | 
[**list_quota_reports**](QuotaApi.md#list_quota_reports) | **Get** /platform/1/quota/reports | 
[**list_settings_mappings**](QuotaApi.md#list_settings_mappings) | **Get** /platform/1/quota/settings/mappings | 
[**list_settings_notifications**](QuotaApi.md#list_settings_notifications) | **Get** /platform/1/quota/settings/notifications | 
[**update_quota_quota**](QuotaApi.md#update_quota_quota) | **Put** /platform/1/quota/quotas/{QuotaQuotaId} | 
[**update_settings_mapping**](QuotaApi.md#update_settings_mapping) | **Put** /platform/1/quota/settings/mappings/{SettingsMappingId} | 
[**update_settings_notification**](QuotaApi.md#update_settings_notification) | **Put** /platform/1/quota/settings/notifications/{SettingsNotificationId} | 
[**update_settings_reports**](QuotaApi.md#update_settings_reports) | **Put** /platform/1/quota/settings/reports | 


# **create_quota_quota**
> ::models::CreateResponse create_quota_quota(ctx, quota_quota, optional)


Create a new quota.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **quota_quota** | [**QuotaQuotaCreateParams**](QuotaQuotaCreateParams.md)|  | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **quota_quota** | [**QuotaQuotaCreateParams**](QuotaQuotaCreateParams.md)|  | 
 **zone** | **String**| Optional named zone to use for user and group resolution. | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_quota_report**
> ::models::CreateQuotaReportResponse create_quota_report(ctx, quota_report)


Create a new report. The type of this report is 'manual'; it is also sometimes called 'live' or 'ad-hoc'.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **quota_report** | [**Empty**](Empty.md)|  | 

### Return type

[**::models::CreateQuotaReportResponse**](CreateQuotaReportResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_settings_mapping**
> ::models::CreateResponse create_settings_mapping(ctx, settings_mapping)


Create a new rule. The new rule must not conflict with an existing rule (e.g. match both the type and domain fields).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_mapping** | [**SettingsMappingExtendedExtended**](SettingsMappingExtendedExtended.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **create_settings_notification**
> ::models::CreateResponse create_settings_notification(ctx, settings_notification)


Create a new global notification rule.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_notification** | [**QuotaNotificationCreateParams**](QuotaNotificationCreateParams.md)|  | 

### Return type

[**::models::CreateResponse**](CreateResponse.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_quota_quota**
> delete_quota_quota(ctx, quota_quota_id)


Delete the quota.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **quota_quota_id** | **String**| Delete the quota. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_quota_quotas**
> delete_quota_quotas(ctx, optional)


Delete all or matching quotas.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **enforced** | **bool**| Only delete quotas with this enforcement (non-accounting). | 
 **include_snapshots** | **bool**| Only delete quotas with this setting for include_snapshots. | 
 **zone** | **String**| Optional named zone to use for user and group resolution. | 
 **recurse_path_children** | **bool**| If used with the path argument, delete all quotas at that path or any descendent sub-directory. | 
 **recurse_path_parents** | **bool**| If used with the path argument, delete all quotas at that path or any parent directory. | 
 **persona** | **String**| Only delete user or group quotas matching this persona (must be used with the corresponding type argument).  Format is &lt;PERSONA_TYPE&gt;:&lt;string/integer&gt;, where PERSONA_TYPE is one of USER, GROUP, SID, ID, or GID. | 
 **path** | **String**| Only delete quotas matching this path (see also recurse_path_*). | 
 **_type** | **String**| Only delete quotas matching this type. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_quota_report**
> delete_quota_report(ctx, quota_report_id)


Delete the report.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **quota_report_id** | **String**| Delete the report. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_settings_mapping**
> delete_settings_mapping(ctx, settings_mapping_id)


Delete the mapping.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_mapping_id** | **String**| Delete the mapping. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_settings_mappings**
> delete_settings_mappings(ctx, )


Delete all rules.

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

# **delete_settings_notification**
> delete_settings_notification(ctx, settings_notification_id)


Delete the notification rule.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_notification_id** | **String**| Delete the notification rule. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **delete_settings_notifications**
> delete_settings_notifications(ctx, )


Delete all rules.

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

# **get_quota_license**
> ::models::LicenseLicense get_quota_license(ctx, )


Retrieve license information.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::LicenseLicense**](LicenseLicense.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_quota_quota**
> ::models::QuotaQuotas get_quota_quota(ctx, quota_quota_id, optional)


Retrieve quota information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **quota_quota_id** | **String**| Retrieve quota information. | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **quota_quota_id** | **String**| Retrieve quota information. | 
 **resolve_names** | **bool**| If true, resolve group and user names in personas. | 
 **zone** | **String**| Optional named zone to use for user and group resolution. | 

### Return type

[**::models::QuotaQuotas**](QuotaQuotas.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_quota_quotas_summary**
> ::models::QuotaQuotasSummary get_quota_quotas_summary(ctx, )


Return summary information about quotas.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::QuotaQuotasSummary**](QuotaQuotasSummary.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_quota_report**
> ::models::ReportAbout get_quota_report(ctx, quota_report_id, optional)


Retrieve report data (XML) or contents (meta-data).

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **quota_report_id** | **String**| Retrieve report data (XML) or contents (meta-data). | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **quota_report_id** | **String**| Retrieve report data (XML) or contents (meta-data). | 
 **contents** | **bool**| Display JSON meta-data contents instead of report data. | 

### Return type

[**::models::ReportAbout**](ReportAbout.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_settings_mapping**
> ::models::SettingsMappings get_settings_mapping(ctx, settings_mapping_id)


Retrieve the mapping information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_mapping_id** | **String**| Retrieve the mapping information. | 

### Return type

[**::models::SettingsMappings**](SettingsMappings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_settings_notification**
> ::models::QuotaNotifications get_settings_notification(ctx, settings_notification_id)


Retrieve notification rule information.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_notification_id** | **String**| Retrieve notification rule information. | 

### Return type

[**::models::QuotaNotifications**](QuotaNotifications.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_settings_reports**
> ::models::SettingsReports get_settings_reports(ctx, )


List all settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SettingsReports**](SettingsReports.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_quota_quotas**
> ::models::QuotaQuotasExtended list_quota_quotas(ctx, optional)


List all or matching quotas. Can also be used to retrieve quota state from existing reports. For any query argument not supplied, the default behavior is return all.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **enforced** | **bool**| Only list quotas with this enforcement (non-accounting). | 
 **include_snapshots** | **bool**| Only list quotas with this setting for include_snapshots. | 
 **zone** | **String**| Optional named zone to use for user and group resolution. | 
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **recurse_path_children** | **bool**| If used with the path argument, match all quotas at that path or any descendent sub-directory. | 
 **resolve_names** | **bool**| If true, resolve group and user names in personas. | 
 **recurse_path_parents** | **bool**| If used with the path argument, match all quotas at that path or any parent directory. | 
 **persona** | **String**| Only list user or group quotas matching this persona (must be used with the corresponding type argument).  Format is &lt;PERSONA_TYPE&gt;:&lt;string/integer&gt;, where PERSONA_TYPE is one of USER, GROUP, SID, ID, or GID. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **exceeded** | **bool**| Set to true to only list quotas which have exceeded one or more of their thresholds. | 
 **path** | **String**| Only list quotas matching this path (see also recurse_path_*). | 
 **_type** | **String**| Only list quotas matching this type. | 
 **report_id** | **String**| Use the named report as a source rather than the live quotas. See the /q/quota/reports resource for a list of valid reports. | 

### Return type

[**::models::QuotaQuotasExtended**](QuotaQuotasExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_quota_reports**
> ::models::QuotaReports list_quota_reports(ctx, optional)


List all or matching reports.

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
 **resume** | **String**| Continue returning results from previous call using this token (token should come from the previous call, resume cannot be used with other options). | 
 **generated** | **String**| Only list reports matching this source. | 
 **limit** | **i32**| Return no more than this many results at once (see resume). | 
 **_type** | **String**| Only list reports matching this type. | 
 **dir** | **String**| The direction of the sort. | 

### Return type

[**::models::QuotaReports**](QuotaReports.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_settings_mappings**
> ::models::SettingsMappings list_settings_mappings(ctx, )


List all rules.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SettingsMappings**](SettingsMappings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_settings_notifications**
> ::models::QuotaNotificationsExtended list_settings_notifications(ctx, )


List all rules.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::QuotaNotificationsExtended**](QuotaNotificationsExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_quota_quota**
> update_quota_quota(ctx, quota_quota, quota_quota_id)


Modify quota. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **quota_quota** | [**QuotaQuota**](QuotaQuota.md)|  | 
  **quota_quota_id** | **String**| Modify quota. All input fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_settings_mapping**
> update_settings_mapping(ctx, settings_mapping, settings_mapping_id)


Modify the mapping.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_mapping** | [**SettingsMappingExtended**](SettingsMappingExtended.md)|  | 
  **settings_mapping_id** | **String**| Modify the mapping. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_settings_notification**
> update_settings_notification(ctx, settings_notification, settings_notification_id)


Modify notification rule. All input fields are optional, but one or more must be supplied.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_notification** | [**QuotaNotification**](QuotaNotification.md)|  | 
  **settings_notification_id** | **String**| Modify notification rule. All input fields are optional, but one or more must be supplied. | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_settings_reports**
> update_settings_reports(ctx, settings_reports)


Modify one or more settings.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_reports** | [**SettingsReportsExtended**](SettingsReportsExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

