# EventChannelParameters

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | **Vec<String>** | Email addresses to send to. | [optional] [default to null]
**batch** | **String** | Batching criterion. | [optional] [default to null]
**batch_period** | **i32** | Period over which batching is to be performed. | [optional] [default to null]
**custom_template** | **String** | Path to custom notification template. | [optional] [default to null]
**send_as** | **String** | Email address to use as from. | [optional] [default to null]
**smtp_host** | **String** | SMTP relay host. | [optional] [default to null]
**smtp_password** | **String** | Password for SMTP authentication - only if smtp_use_auth true. | [optional] [default to null]
**smtp_port** | **i32** | SMTP relay port - optional defaults to 25. | [optional] [default to null]
**smtp_security** | **String** | Encryption protocol to use for SMTP. | [optional] [default to null]
**smtp_use_auth** | **bool** | Use SMTP authentication - optional defaulst to false. | [optional] [default to null]
**smtp_username** | **String** | Username for SMTP authentication - only if smtp_use_auth true. | [optional] [default to null]
**subject** | **String** | Subject for emails. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


