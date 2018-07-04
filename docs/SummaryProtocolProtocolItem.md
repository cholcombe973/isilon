# SummaryProtocolProtocolItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**class** | **String** | The class of the operation. | [default to null]
**_in** | **f32** | Rate of input (in bytes/second) for an operation since the last time isi statistics collected the data. | [default to null]
**in_avg** | **f32** | Average input (received) bytes for an operation, in bytes. | [default to null]
**in_max** | **f32** | Maximum input (received) bytes for an operation, in bytes. | [default to null]
**in_min** | **f32** | Minimum input (received) bytes for an operation, in bytes. | [default to null]
**in_standard_dev** | **f32** | Standard deviation for input (received) bytes for an operation, in bytes. | [default to null]
**node** | **i32** | The node on which the operation was performed. | [optional] [default to null]
**operation** | **String** | The operation performed. | [default to null]
**operation_count** | **i32** | The number of times an operation has been performed. | [default to null]
**operation_rate** | **f32** | The rate (in ops/second) at which an operation has been performed. | [default to null]
**out** | **f32** | Rate of output (in bytes/second) for an operation since the last time isi statistics collected the data. | [default to null]
**out_avg** | **f32** | Average output (sent) bytes for an operation, in bytes. | [default to null]
**out_max** | **f32** | Maximum output (sent) bytes for an operation, in bytes. | [default to null]
**out_min** | **f32** | Minimum output (sent) bytes for an operation, in bytes. | [default to null]
**out_standard_dev** | **f32** | Standard deviation for output (received) bytes for an operation, in bytes. | [default to null]
**protocol** | **String** | The protocol of the operation. | [default to null]
**time** | **i32** | Unix Epoch time in seconds of the request. | [default to null]
**time_avg** | **f32** | The average elapsed time (in microseconds) taken to complete an operation. | [default to null]
**time_max** | **f32** | The maximum elapsed time (in microseconds) taken to complete an operation. | [default to null]
**time_min** | **f32** | The minimum elapsed time (in microseconds) taken to complete an operation. | [default to null]
**time_standard_dev** | **f32** | The standard deviation time (in microseconds) taken to complete an operation. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


