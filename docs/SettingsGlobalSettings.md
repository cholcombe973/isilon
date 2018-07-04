# SettingsGlobalSettings

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**audited_zones** | **Vec<String>** | Specifies zones that are audited when the protocol_auditing_enabled property is enabled. | [optional] [default to null]
**cee_log_time** | **String** | Specifies that events past a certain date are forwarded by the audit CEE forwarder. Format these events as follows: &#39;Topic@YYYY-MM-DD HH:MM:SS&#39;. | [optional] [default to null]
**cee_server_uris** | **Vec<String>** | Specifies a list of Common Event Enabler (CEE) server URIs. Protocol audit logs are sent to these URIs for external processing. | [optional] [default to null]
**config_auditing_enabled** | **bool** | Specifies whether logging for API configuration changes are enabled. | [optional] [default to null]
**config_syslog_enabled** | **bool** | Specifies whether configuration audit syslog messages are forwarded. | [optional] [default to null]
**hostname** | **String** | Specifies the hostname that is reported in protocol events from this cluster. | [optional] [default to null]
**protocol_auditing_enabled** | **bool** | Specifies if logging for the I/O stream is enabled. | [optional] [default to null]
**syslog_log_time** | **String** | Specifies that events past a specified date are forwarded by the audit syslog forwarder. Format these events as follows: &#39;Topic@YYYY-MM-DD HH:MM:SS&#39; format | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


