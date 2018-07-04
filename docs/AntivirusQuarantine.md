# AntivirusQuarantine

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**file** | **String** | Path of this file, starting with /ifs. | [default to null]
**last_istag** | **String** |  | [optional] [default to null]
**last_scan** | **i32** |  | [optional] [default to null]
**quarantined** | **bool** | If true, this file is quarantined.  If false, the file is not quarantined. | [default to null]
**scan_result** | **String** | The result of the last scan on this file.  This string is usually one of: never_scanned, clean, quarantined, repaired, truncated, infected_no_action_taken, skipped_per_settings.  However, a longer string starting with &#39;unknown_status&#39; and describing the details can also appear in uncommon edge cases. | [default to null]
**scan_status** | **String** | The scanning status of this file.  If &#39;current&#39;, the file was scanned with the most up-to-date virus definitions.  If &#39;not_current&#39;, it has either not been scanned, been modified since the last scan, or the virus definitions are not current. | [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


