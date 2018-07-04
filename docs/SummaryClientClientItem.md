# SummaryClientClientItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class** | **String** | The class of the operation. | [default to null]
**_in** | **f32** | Rate of input (in bytes/second) for an operation since the last time isi statistics collected the data. | [default to null]
**in_avg** | **f32** | Average input (received) bytes for an operation, in bytes. | [default to null]
**in_max** | **f32** | Maximum input (received) bytes for an operation, in bytes. | [default to null]
**in_min** | **f32** | Minimum input (received) bytes for an operation, in bytes. | [default to null]
**local_addr** | **String** | The IP address (in dotted-quad form) of the host receiving the operation request. | [default to null]
**local_name** | **String** | The resolved text name of the LocalAddr, if resolution can be performed. | [default to null]
**node** | **i32** | The node on which the operation was performed. | [optional] [default to null]
**num_operations** | **i32** | The number of times an operation has been performed. | [default to null]
**operation_rate** | **f32** | The rate (in ops/second) at which an operation has been performed. | [default to null]
**out** | **f32** | Rate of output (in bytes/second) for an operation since the last time isi statistics collected the data. | [default to null]
**out_avg** | **f32** | Average output (sent) bytes for an operation, in bytes. | [default to null]
**out_max** | **f32** | Maximum output (sent) bytes for an operation, in bytes. | [default to null]
**out_min** | **f32** | Minimum output (sent) bytes for an operation, in bytes. | [default to null]
**protocol** | **String** | The protocol of the operation. | [default to null]
**remote_addr** | **String** | The IP address (in dotted-quad form) of the host sending the operation request. | [default to null]
**remote_name** | **String** | The resolved text name of the RemoteAddr, if resolution can be performed. | [default to null]
**time** | **i32** | Unix Epoch time in seconds of the request. | [default to null]
**time_avg** | **f32** | The average elapsed time (in microseconds) taken to complete an operation. | [default to null]
**time_max** | **f32** | The maximum elapsed time (in microseconds) taken to complete an operation. | [default to null]
**time_min** | **f32** | The minimum elapsed time (in microseconds) taken to complete an operation. | [default to null]
**user** | [***::models::AuthAccessAccessItemFileGroup**](AuthAccessAccessItemFileGroup.md) | Specifies properties for a persona, which consists of either a &#39;type&#39; and a &#39;name&#39; or an &#39;ID&#39;. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


