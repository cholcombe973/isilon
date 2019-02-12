# ClusterPatchPatchesPatch

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**comment** | **String** | A long comment about the patch. | [optional] [default to null]
**conflicts** | **Vec<String>** | Other patches that this patch conflicts with. | [optional] [default to null]
**dependencies** | **Vec<String>** | Other patches that this patch depends on. | [optional] [default to null]
**description** | **String** | A short description of the patch. | [optional] [default to null]
**files** | [**Vec <crate::models::ClusterPatchPatchesPatchFile>**](ClusterPatchPatchesPatchFile.md) | The files contained in this patch. | [optional] [default to null]
**id** | **String** | A unique identifier for the patch. | [optional] [default to null]
**name** | **String** | The name of the patch. | [optional] [default to null]
**nodes** | **Vec<i32>** | The nodes that this patch is installed on. | [optional] [default to null]
**reboot** | **String** | Describes the reboot requirements | [optional] [default to null]
**services** | [**Vec <crate::models::ClusterPatchPatchesPatchService>**](ClusterPatchPatchesPatchService.md) | The services affected during the patch deployment | [optional] [default to null]
**status** | **String** | The intallation status of this patch on the cluster. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


