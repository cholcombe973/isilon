# QuotaQuotaThresholdsExtended

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**advisory** | **i32** | Usage bytes at which notifications will be sent but writes will not be denied. | [optional] [default to null]
**hard** | **i32** | Usage bytes at which further writes will be denied. | [optional] [default to null]
**soft** | **i32** | Usage bytes at which notifications will be sent and soft grace time will be started. | [optional] [default to null]
**soft_grace** | **i32** | Time in seconds after which the soft threshold has been hit before writes will be denied. | [optional] [default to null]
**advisory_exceeded** | **bool** | True if the advisory threshold has been hit. | [optional] [default to null]
**advisory_last_exceeded** | **i32** | Time at which advisory threshold was hit. | [optional] [default to null]
**hard_exceeded** | **bool** | True if the hard threshold has been hit. | [optional] [default to null]
**hard_last_exceeded** | **i32** | Time at which hard threshold was hit. | [optional] [default to null]
**soft_exceeded** | **bool** | True if the soft threshold has been hit. | [optional] [default to null]
**soft_last_exceeded** | **i32** | Time at which soft threshold was hit | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


