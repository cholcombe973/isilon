# LicenseLicenseCreateParams

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**evaluation** | **Vec<String>** | A list of evaluation licenses to enable on the cluster. | [optional] [default to null]
**license_file_content** | **String** | License file string content. The license file is obtained from EMC&#39;s SLC web portal. Do not use with the license_file_path option. | [optional] [default to null]
**license_file_path** | **String** | Path to new license file, must be under /ifs. The license file is obtained from EMC&#39;s SLC web portal. Do not include the path when only enabling evaluation licenses. Do not use with the license_file_content option. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


