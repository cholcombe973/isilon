# RemotesupportConnectemcConnectemc

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email_customer_on_failure** | **bool** | Email the customer if all transmission methods fail. | [optional] [default to null]
**enabled** | **bool** | Enable ConnectEMC. | [optional] [default to null]
**gateway_access_pools** | **Vec<String>** | List of network pools that are able to connect to the ESRS gateway.  Necessary to enable ConnectEMC. | [optional] [default to null]
**primary_esrs_gateway** | **String** | Primary ESRS Gateway. Necessary to enable ConnectEMC. | [optional] [default to null]
**secondary_esrs_gateway** | **String** | Secondary ESRS Gateway. Used if Primary is unavailable. | [optional] [default to null]
**use_smtp_failover** | **bool** | Use SMPT if primary and secondary gateways are unavailable. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


