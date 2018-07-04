# NamespaceObject

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_time** | **String** | Specifies the date when the object was last accessed in HTTP date/time format. | [optional] [default to null]
**atime_val** | **i32** | Specifies the time when the object was last accessed in UNIX Epoch format. | [optional] [default to null]
**block_size** | **i32** | Specifies the block size of the object. | [optional] [default to null]
**blocks** | **i32** | Specifies the number of blocks that compose the object. | [optional] [default to null]
**btime_val** | **i32** | Specifies the time when the object data was created in UNIX Epoch format. | [optional] [default to null]
**change_time** | **String** | Specifies the date when the object was last changed (including data and metadata changes) in HTTP date/time format. | [optional] [default to null]
**create_time** | **String** | Specifies the date when the object data was created in HTTP date/time format. | [optional] [default to null]
**ctime_val** | **i32** | Specifies the time when the object was last changed (including data and metadata changes) in UNIX Epoch format. | [optional] [default to null]
**gid** | **i32** | Specifies the GID for the owner. | [optional] [default to null]
**group** | **String** | Specifies the group name for the owner of the object. | [optional] [default to null]
**id** | **i32** | Specifies the object ID, which is also the INODE number. | [optional] [default to null]
**is_hidden** | **bool** | Specifies whether the file is hidden or not. | [optional] [default to null]
**last_modified** | **String** | Specifies the time when the object data was last modified in HTTP date/time format. | [optional] [default to null]
**mode** | **String** | Specifies the UNIX mode octal number. | [optional] [default to null]
**mtime_val** | **i32** | Specifies the time when the object data was last modified in UNIX Epoch format. | [optional] [default to null]
**name** | **String** | Specifies the name of the object. | [optional] [default to null]
**nlink** | **i32** | Specifies the number of hard links to the object. | [optional] [default to null]
**owner** | **String** | Specifies the user name for the owner of the object. | [optional] [default to null]
**size** | **i32** | Specifies the size of the object in bytes. | [optional] [default to null]
**stub** | **bool** |  | [optional] [default to null]
**_type** | **String** | Specifies the object type, which can be one of the following values: container, object, pipe, character_device, block_device, symbolic_link, socket, or whiteout_file. | [optional] [default to null]
**uid** | **i32** | Specifies the UID for the owner. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


