# NodeDriveconfigNodeStall

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**clear_time** | **i32** | The amount of time in seconds with no stalls before ignoring previous stalls. | [optional] [default to null]
**diskscrub_stripes** | **i32** | Number of stripes to read during a diskscrub. | [optional] [default to null]
**max_error_frequency** | **i32** | The number of errors during stalled drive disk exercises to cause the drive to be softfailed. | [optional] [default to null]
**max_slow_access** | **i32** | The number of slow accesses during stalled drive disk exercises to cause the drive to be softfailed. | [optional] [default to null]
**max_slow_frequency** | **i32** | The number of slow frequency triggers during stalled drive disk exercises to cause the drive to be softfailed. | [optional] [default to null]
**max_total_stall_time** | **i32** | The maximum amount of time, in seconds, to remain stalled before softfailing the drive. | [optional] [default to null]
**scan_max_ecc_delay** | **i32** | Maximum delay in seconds after an ECC correction during a scan. | [optional] [default to null]
**scan_size** | **i32** | Total bytes of error-free reads to complete a scan. | [optional] [default to null]
**sleep** | **i32** | Delay in seconds between evaluations. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


