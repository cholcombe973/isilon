# QuotaNotificationCreateParams

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**action_alert** | **bool** | Send alert when rule matches. | [optional] [default to null]
**action_email_address** | **String** | Email a specific email address when rule matches. | [optional] [default to null]
**action_email_owner** | **bool** | Email quota domain owner when rule matches. | [optional] [default to null]
**email_template** | **String** | Path of optional /ifs template file used for email actions. | [optional] [default to null]
**holdoff** | **i32** | Time to wait between detections for rules triggered by user actions. | [optional] [default to null]
**schedule** | **String** | Schedule for rules that repeatedly notify. | [optional] [default to null]
**condition** | **String** | The condition detected. | [default to null]
**threshold** | **String** | The quota threshold detected. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


