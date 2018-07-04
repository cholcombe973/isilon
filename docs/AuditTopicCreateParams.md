# AuditTopicCreateParams

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**max_cached_messages** | **i32** | Specifies the maximum number of messages that can be sent and received at the same time. Messages that are sent and received at the same time can be lost if a system crash occurs. You can prevent message loss by setting this property to 0, which sets audit logs to synchronous. | [optional] [default to null]
**name** | **String** | Specifies the name of the audit topic. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


