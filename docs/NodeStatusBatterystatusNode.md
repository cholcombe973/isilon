# NodeStatusBatterystatusNode

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error** | **String** | Error message, if the HTTP status returned from this node was not 200. | [optional] [default to null]
**id** | **i32** | Node ID of the node reporting this information. | [optional] [default to null]
**last_test_time1** | **String** | The last battery test time for battery 1. | [optional] [default to null]
**last_test_time2** | **String** | The last battery test time for battery 2. | [optional] [default to null]
**lnn** | **i32** | Logical node number of the node reporting this information. | [optional] [default to null]
**next_test_time1** | **String** | The next checkup for battery 1. | [optional] [default to null]
**next_test_time2** | **String** | The next checkup for battery 2. | [optional] [default to null]
**present** | **bool** | Node has battery status. | [optional] [default to null]
**result1** | **String** | The result of the last battery test for battery 1. | [optional] [default to null]
**result2** | **String** | The result of the last battery test for battery 2. | [optional] [default to null]
**status** | **i32** | Status of the HTTP response from this node if not 200.  If 200, this field does not appear. | [optional] [default to null]
**status1** | **String** | The status of battery 1. | [optional] [default to null]
**status2** | **String** | The status of battery 2. | [optional] [default to null]
**supported** | **bool** | Node supports battery status. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


