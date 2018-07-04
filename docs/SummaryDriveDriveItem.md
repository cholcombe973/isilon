# SummaryDriveDriveItem

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_latency** | **f32** | The average operation latency. | [default to null]
**access_slow** | **f32** | The rate of slow (long-latency) operations. | [default to null]
**busy** | **f32** | The percentage of time the drive was busy. | [default to null]
**bytes_in** | **f32** | The rate of bytes written. | [default to null]
**bytes_out** | **f32** | The rate of bytes read. | [default to null]
**drive_id** | **String** | Drive ID LNN:bay. | [default to null]
**iosched_latency** | **f32** | The average time spent in the I/O scheduler. | [default to null]
**iosched_queue** | **f32** | The average length of the I/O scheduler queue. | [default to null]
**time** | **i32** | Unix Epoch time in seconds of the request. | [default to null]
**_type** | **String** | The type of the drive. | [default to null]
**used_bytes_percent** | **f32** | The percent of blocks used on the drive. | [default to null]
**used_inodes** | **f32** | The number of inodes allocated on the drive. | [default to null]
**xfer_size_in** | **f32** | The average size of write operations. | [default to null]
**xfer_size_out** | **f32** | The average size of read operations. | [default to null]
**xfers_in** | **f32** | The rate of write operations. | [default to null]
**xfers_out** | **f32** | The rate of read operations. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


