# \FilesystemApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_settings_access_time**](FilesystemApi.md#get_settings_access_time) | **Get** /platform/1/filesystem/settings/access-time | 
[**get_settings_character_encodings**](FilesystemApi.md#get_settings_character_encodings) | **Get** /platform/1/filesystem/settings/character-encodings | 
[**update_settings_access_time**](FilesystemApi.md#update_settings_access_time) | **Put** /platform/1/filesystem/settings/access-time | 
[**update_settings_character_encodings**](FilesystemApi.md#update_settings_character_encodings) | **Put** /platform/1/filesystem/settings/character-encodings | 


# **get_settings_access_time**
> ::models::SettingsAccessTime get_settings_access_time(ctx, )


Retrieve settings for access time.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SettingsAccessTime**](SettingsAccessTime.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_settings_character_encodings**
> ::models::SettingsCharacterEncodings get_settings_character_encodings(ctx, )


Retrieve current cluster character encoding settings.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::SettingsCharacterEncodings**](SettingsCharacterEncodings.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_settings_access_time**
> update_settings_access_time(ctx, settings_access_time)


Set settings for access time.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_access_time** | [**SettingsAccessTimeExtended**](SettingsAccessTimeExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **update_settings_character_encodings**
> update_settings_character_encodings(ctx, settings_character_encodings)


Set current character encoding.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **settings_character_encodings** | [**SettingsCharacterEncodingsExtended**](SettingsCharacterEncodingsExtended.md)|  | 

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

