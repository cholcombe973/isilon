# \LicenseApi

All URIs are relative to *https://YOUR_CLUSTER_HOSTNAME_OR_NODE_IP:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_license_license**](LicenseApi.md#create_license_license) | **Post** /platform/5/license/licenses | 
[**get_license_generate**](LicenseApi.md#get_license_generate) | **Get** /platform/5/license/generate | 
[**get_license_license**](LicenseApi.md#get_license_license) | **Get** /platform/5/license/licenses/{LicenseLicenseId} | 
[**list_license_licenses**](LicenseApi.md#list_license_licenses) | **Get** /platform/5/license/licenses | 


# **create_license_license**
>crate::models::Empty create_license_license(ctx, license_license)


Install a new license file and/or activate evaluation licenses.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **license_license** | [**LicenseLicenseCreateParams**](LicenseLicenseCreateParams.md)|  | 

### Return type

[**::models::Empty**](Empty.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_license_generate**
>crate::models::LicenseGenerate get_license_generate(ctx, optional)


Generate license activation file.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **action** | **String**| enum: license_list_only (default), generate_activation, download_activation. Generate an activation file or return a list of activated licenses. If generating an activation file and no licenses are specified, the default configuration is to generate an activation file with the current set of licensed features. download_activation returns HTTP headers and the same XML content as seen in the response activation. | [default to license_list_only]
 **licenses_to_include** | **String**| Licenses to include in activation file. | 
 **licenses_to_exclude** | **String**| Licenses to omit from activation file. | 
 **only_these_licenses** | **String**| Activate only the defined licenses. This setting overrides all other license activation settings. | 

### Return type

[**::models::LicenseGenerate**](LicenseGenerate.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_license_license**
>crate::models::LicenseLicenses get_license_license(ctx, license_license_id)


Retrieve license information for the feature.

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ctx** | **context.Context** | context containing the authentication | nil if no authentication
  **license_license_id** | **String**| Retrieve license information for the feature. | 

### Return type

[**::models::LicenseLicenses**](LicenseLicenses.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **list_license_licenses**
>crate::models::LicenseLicensesExtended list_license_licenses(ctx, )


Retrieve license information for all licensable products.

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**::models::LicenseLicensesExtended**](LicenseLicensesExtended.md)

### Authorization

[basicAuth](../README.md#basicAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

