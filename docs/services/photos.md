# Google Photos app-created media methods

See the [command guide](../grammar.md) and [request-file format](../file-schemas.md).

| Method | Description |
| --- | --- |
| `albums.add-enrichment` | Adds an enrichment at a specified position in an app created created album. |
| `albums.batch-add-media-items` | Adds one or more app created media items in a user's Google Photos library to an app created album |
| `albums.batch-remove-media-items` | Removes one or more app created media items from a specified app created album |
| `albums.create` | Creates an album in a user's Google Photos library. |
| `albums.get` | Returns the app created album based on the specified `albumId` |
| `albums.list` | Lists all albums created by your app. |
| `albums.patch` | Update the app created album with the specified `id` |
| `media-items.batch-create` | Creates one or more media items in a user's Google Photos library |
| `media-items.batch-get` | Returns the list of app created media items for the specified media item identifiers |
| `media-items.get` | Returns the app created media item for the specified media item identifier. |
| `media-items.list` | List all media items created by your app from a user's Google Photos library. |
| `media-items.patch` | Update the app created media item with the specified `id` |
| `media-items.search` | Searches for app created media items in a user's Google Photos library |

## albums.add-enrichment

```text
Adds an enrichment at a specified position in an app created created album.

Usage: joog [OPTIONS] photos <ACCOUNT> albums.add-enrichment [COMMAND_OPTIONS]

Mandatory parameters:
      --album-id <ALBUM_ID>
          Required. Identifier of the app created album where the enrichment is to be added.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --album-position <JSON>
          Specifies a position in an album.

      --album-position-position <ALBUM_POSITION_POSITION>
          Type of position, for a media or enrichment item. Choices: POSITION_TYPE_UNSPECIFIED, FIRST_IN_ALBUM, LAST_IN_ALBUM, AFTER_MEDIA_ITEM, AFTER_ENRICHMENT_ITEM.

      --album-position-relative-enrichment-item-id <ALBUM_POSITION_RELATIVE_ENRICHMENT_ITEM_ID>
          The enrichment item to which the position is relative to. Only used when position type is AFTER_ENRICHMENT_ITEM.

      --album-position-relative-media-item-id <ALBUM_POSITION_RELATIVE_MEDIA_ITEM_ID>
          The media item to which the position is relative to. Only used when position type is AFTER_MEDIA_ITEM.

      --new-enrichment-item <JSON>
          A new enrichment item to be added to an app created album, used by the `albums.addEnrichment` call.

      --new-enrichment-item-location-enrichment-location-location-name <NEW_ENRICHMENT_ITEM_LOCATION_ENRICHMENT_LOCATION_LOCATION_NAME>
          Name of the location to be displayed.

      --new-enrichment-item-map-enrichment-destination-location-name <NEW_ENRICHMENT_ITEM_MAP_ENRICHMENT_DESTINATION_LOCATION_NAME>
          Name of the location to be displayed.

      --new-enrichment-item-map-enrichment-origin-location-name <NEW_ENRICHMENT_ITEM_MAP_ENRICHMENT_ORIGIN_LOCATION_NAME>
          Name of the location to be displayed.

      --new-enrichment-item-text-enrichment-text <NEW_ENRICHMENT_ITEM_TEXT_ENRICHMENT_TEXT>
          Text for this enrichment item.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## albums.batch-add-media-items

```text
Adds one or more app created media items in a user's Google Photos library to an app created album. The media items and albums must have been created by the developer via the API. Media items are added to the end of the album. If multiple media items are given, they are added in the order specified in this call. Each album can contain up to 20,000 media items. Partial success is not supported. The entire request will fail if an invalid media item or album is specified.

Usage: joog [OPTIONS] photos <ACCOUNT> albums.batch-add-media-items [COMMAND_OPTIONS]

Mandatory parameters:
      --album-id <ALBUM_ID>
          Required. Identifier of the Album that the media items are added to.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --media-item-ids <MEDIA_ITEM_IDS>...
          Required. Identifiers of the MediaItems to be added. The maximum number of media items that can be added in one call is 50.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## albums.batch-remove-media-items

```text
Removes one or more app created media items from a specified app created album. The media items and the album must have been created by the developer via the API. Partial success is not supported. The entire request will fail and no action will be performed on the album if an invalid media item or album is specified.

Usage: joog [OPTIONS] photos <ACCOUNT> albums.batch-remove-media-items [COMMAND_OPTIONS]

Mandatory parameters:
      --album-id <ALBUM_ID>
          Required. Identifier of the Album that the media items are to be removed from.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --media-item-ids <MEDIA_ITEM_IDS>...
          Required. Identifiers of the MediaItems to be removed. Must not contain repeated identifiers and cannot be empty. The maximum number of media items that can be removed in one call is 50.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## albums.create

```text
Creates an album in a user's Google Photos library.

Usage: joog [OPTIONS] photos <ACCOUNT> albums.create [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --title <TITLE>
          Name of the album displayed to the user in their Google Photos account. This string shouldn't be more than 500 characters.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --album <JSON>
          Representation of an album in Google Photos. Albums are containers for media items. If an album has been shared by the application, it contains an extra `shareInfo` property.

      --album-cover-photo-base-url <ALBUM_COVER_PHOTO_BASE_URL>
          [Output only] A URL to the cover photo's bytes. This shouldn't be used as is. Parameters should be appended to this URL before use. See the [developer documentation](https://developers.google.com/photos/library/guides/access-media-items#base-urls) for a complete list of supported parameters. For example, `'=w2048-h1024'` sets the dimensions of the cover photo to have a width of 2048 px and height of 1024 px.

      --album-cover-photo-media-item-id <ALBUM_COVER_PHOTO_MEDIA_ITEM_ID>
          Identifier for the media item associated with the cover photo.

      --album-id <ALBUM_ID>
          Identifier for the album. This is a persistent identifier that can be used between sessions to identify this album.

      --album-is-writeable[=<BOOL>]
          [Output only] True if you can create media items in this album. This field is based on the scopes granted and permissions of the album. If the scopes are changed or permissions of the album are changed, this field is updated.

      --album-media-items-count <ALBUM_MEDIA_ITEMS_COUNT>
          [Output only] The number of media items in the album.

      --album-product-url <ALBUM_PRODUCT_URL>
          [Output only] Google Photos URL for the album. The user needs to be signed in to their Google Photos account to access this link.

      --album-share-info-is-joinable[=<BOOL>]
          True if the album can be joined by users.

      --album-share-info-is-joined[=<BOOL>]
          True if the user is joined to the album. This is always true for the owner of the album.

      --album-share-info-is-owned[=<BOOL>]
          True if the user owns the album.

      --album-share-info-share-token <ALBUM_SHARE_INFO_SHARE_TOKEN>
          A token that is used to join, leave, or retrieve the details of a shared album on behalf of a user who isn't the owner. A `shareToken` is invalidated if the owner turns off link sharing in the Google Photos app, or if the album is unshared.

      --album-share-info-shareable-url <ALBUM_SHARE_INFO_SHAREABLE_URL>
          A link to the shared Google Photos album. Anyone with the link can view the contents of the album, so it should be treated with care. The `shareableUrl` parameter is only returned if the album has link sharing turned on. If a user is already joined to an album that isn't link-shared, they can use the album's [`productUrl`](https://developers.google.com/photos/library/reference/rest/v1/albums#Album) to access it instead. A `shareableUrl` is invalidated if the owner turns off link sharing in the Google Photos app, or if the album is unshared.

      --album-share-info-shared-album-options-is-collaborative[=<BOOL>]
          True if the shared album allows collaborators (users who have joined the album) to add media items to it. Defaults to false.

      --album-share-info-shared-album-options-is-commentable[=<BOOL>]
          True if the shared album allows collaborators (users who have joined the album) to add comments to the album. Defaults to false.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## albums.get

```text
Returns the app created album based on the specified `albumId`. The `albumId` must be the ID of an album created by your app.

Usage: joog [OPTIONS] photos <ACCOUNT> albums.get [COMMAND_OPTIONS]

Mandatory parameters:
      --album-id <ALBUM_ID>
          Required. Identifier of the album to be requested.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --ttl <MINUTES>
          Reuse encrypted response data; 0 disables caching
          
          [default: 0]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## albums.list

```text
Lists all albums created by your app.

Usage: joog [OPTIONS] photos <ACCOUNT> albums.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --ttl <MINUTES>
          Reuse encrypted response data; 0 disables caching
          
          [default: 0]

      --max-pages <N>
          Stop after N pages; by default, fetch all pages

      --exclude-non-app-created-data[=<BOOL>]
          If set, the results exclude media items that were not created by this app. Defaults to false (all albums are returned). This field is ignored if the photoslibrary.readonly.appcreateddata scope is used.

      --page-size <PAGE_SIZE>
          Maximum number of albums to return in the response. Fewer albums might be returned than the specified number. The default `pageSize` is 20, the maximum is 50.

      --page-token <PAGE_TOKEN>
          A continuation token to get the next page of the results. Adding this to the request returns the rows after the `pageToken`. The `pageToken` should be the value returned in the `nextPageToken` parameter in the response to the `listAlbums` request.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## albums.patch

```text
Update the app created album with the specified `id`. Only the `id`, `title` and `cover_photo_media_item_id` fields of the album are read. The album must have been created by the developer via the API.

Usage: joog [OPTIONS] photos <ACCOUNT> albums.patch [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          Identifier for the album. This is a persistent identifier that can be used between sessions to identify this album.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --title <TITLE>
          Name of the album displayed to the user in their Google Photos account. This string shouldn't be more than 500 characters.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --body-id <BODY_ID>
          Identifier for the album. This is a persistent identifier that can be used between sessions to identify this album.

      --cover-photo-base-url <COVER_PHOTO_BASE_URL>
          [Output only] A URL to the cover photo's bytes. This shouldn't be used as is. Parameters should be appended to this URL before use. See the [developer documentation](https://developers.google.com/photos/library/guides/access-media-items#base-urls) for a complete list of supported parameters. For example, `'=w2048-h1024'` sets the dimensions of the cover photo to have a width of 2048 px and height of 1024 px.

      --cover-photo-media-item-id <COVER_PHOTO_MEDIA_ITEM_ID>
          Identifier for the media item associated with the cover photo.

      --is-writeable[=<BOOL>]
          [Output only] True if you can create media items in this album. This field is based on the scopes granted and permissions of the album. If the scopes are changed or permissions of the album are changed, this field is updated.

      --media-items-count <MEDIA_ITEMS_COUNT>
          [Output only] The number of media items in the album.

      --product-url <PRODUCT_URL>
          [Output only] Google Photos URL for the album. The user needs to be signed in to their Google Photos account to access this link.

      --share-info <JSON>
          Information about albums that are shared. This information is only included if you created the album, it is shared and you have the sharing scope.

      --share-info-is-joinable[=<BOOL>]
          True if the album can be joined by users.

      --share-info-is-joined[=<BOOL>]
          True if the user is joined to the album. This is always true for the owner of the album.

      --share-info-is-owned[=<BOOL>]
          True if the user owns the album.

      --share-info-share-token <SHARE_INFO_SHARE_TOKEN>
          A token that is used to join, leave, or retrieve the details of a shared album on behalf of a user who isn't the owner. A `shareToken` is invalidated if the owner turns off link sharing in the Google Photos app, or if the album is unshared.

      --share-info-shareable-url <SHARE_INFO_SHAREABLE_URL>
          A link to the shared Google Photos album. Anyone with the link can view the contents of the album, so it should be treated with care. The `shareableUrl` parameter is only returned if the album has link sharing turned on. If a user is already joined to an album that isn't link-shared, they can use the album's [`productUrl`](https://developers.google.com/photos/library/reference/rest/v1/albums#Album) to access it instead. A `shareableUrl` is invalidated if the owner turns off link sharing in the Google Photos app, or if the album is unshared.

      --share-info-shared-album-options-is-collaborative[=<BOOL>]
          True if the shared album allows collaborators (users who have joined the album) to add media items to it. Defaults to false.

      --share-info-shared-album-options-is-commentable[=<BOOL>]
          True if the shared album allows collaborators (users who have joined the album) to add comments to the album. Defaults to false.

      --update-mask <UPDATE_MASK>
          Required. Indicate what fields in the provided album to update. The only valid values are `title` and `cover_photo_media_item_id`.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## media-items.batch-create

```text
Creates one or more media items in a user's Google Photos library. This is the second step for creating a media item. For details regarding Step 1, uploading the raw bytes to a Google Server, see Uploading media. This call adds the media item to the library. If an app created album `id` is specified, the call adds the media item to the album too. Each album can contain up to 20,000 media items. By default, the media item will be added to the end of the library or album. If an album `id` and position are both defined, the media item is added to the album at the specified position. If the call contains multiple media items, they're added at the specified position.

Usage: joog [OPTIONS] photos <ACCOUNT> media-items.batch-create [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --album-id <ALBUM_ID>
          Identifier of the album where the media items are added. The media items are also added to the user's library. This is an optional field.

      --verbose
          Include advanced parameters in help

      --new-media-items <JSON>
          Required. List of media items to be created. Maximum 50 media items per call.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --album-position <JSON>
          Specifies a position in an album.

      --album-position-position <ALBUM_POSITION_POSITION>
          Type of position, for a media or enrichment item. Choices: POSITION_TYPE_UNSPECIFIED, FIRST_IN_ALBUM, LAST_IN_ALBUM, AFTER_MEDIA_ITEM, AFTER_ENRICHMENT_ITEM.

      --album-position-relative-enrichment-item-id <ALBUM_POSITION_RELATIVE_ENRICHMENT_ITEM_ID>
          The enrichment item to which the position is relative to. Only used when position type is AFTER_ENRICHMENT_ITEM.

      --album-position-relative-media-item-id <ALBUM_POSITION_RELATIVE_MEDIA_ITEM_ID>
          The media item to which the position is relative to. Only used when position type is AFTER_MEDIA_ITEM.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## media-items.batch-get

```text
Returns the list of app created media items for the specified media item identifiers. Items are returned in the same order as the supplied identifiers.

Usage: joog [OPTIONS] photos <ACCOUNT> media-items.batch-get [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --ttl <MINUTES>
          Reuse encrypted response data; 0 disables caching
          
          [default: 0]

      --media-item-ids <MEDIA_ITEM_IDS>...
          Required. Identifiers of the media items to be requested. Must not contain repeated identifiers and cannot be empty. The maximum number of media items that can be retrieved in one call is 50.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## media-items.get

```text
Returns the app created media item for the specified media item identifier.

Usage: joog [OPTIONS] photos <ACCOUNT> media-items.get [COMMAND_OPTIONS]

Mandatory parameters:
      --media-item-id <MEDIA_ITEM_ID>
          Required. Identifier of the media item to be requested.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --ttl <MINUTES>
          Reuse encrypted response data; 0 disables caching
          
          [default: 0]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## media-items.list

```text
List all media items created by your app from a user's Google Photos library.

Usage: joog [OPTIONS] photos <ACCOUNT> media-items.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --ttl <MINUTES>
          Reuse encrypted response data; 0 disables caching
          
          [default: 0]

      --max-pages <N>
          Stop after N pages; by default, fetch all pages

      --page-size <PAGE_SIZE>
          Maximum number of media items to return in the response. Fewer media items might be returned than the specified number. The default `pageSize` is 25, the maximum is 100.

      --page-token <PAGE_TOKEN>
          A continuation token to get the next page of the results. Adding this to the request returns the rows after the `pageToken`. The `pageToken` should be the value returned in the `nextPageToken` parameter in the response to the `listMediaItems` request.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## media-items.patch

```text
Update the app created media item with the specified `id`. Only the `id` and `description` fields of the media item are read. The media item must have been created by the developer via the API.

Usage: joog [OPTIONS] photos <ACCOUNT> media-items.patch [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          Identifier for the media item. This is a persistent identifier that can be used between sessions to identify this media item.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --description <DESCRIPTION>
          Description of the media item. This is shown to the user in the item's info section in the Google Photos app. Must be shorter than 1000 characters. Only include text written by users. Descriptions should add context and help users understand media. Do not include any auto-generated strings such as filenames, tags, and other metadata.

      --mime-type <MIME_TYPE>
          MIME type of the media item. For example, `image/jpeg`.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --body <JSON>
          Inline JSON request body

      --base-url <BASE_URL>
          A URL to the media item's bytes. This shouldn't be used as is. Parameters should be appended to this URL before use. See the [developer documentation](https://developers.google.com/photos/library/guides/access-media-items#base-urls) for a complete list of supported parameters. For example, `'=w2048-h1024'` will set the dimensions of a media item of type photo to have a width of 2048 px and height of 1024 px.

      --body-id <BODY_ID>
          Identifier for the media item. This is a persistent identifier that can be used between sessions to identify this media item.

      --contributor-info <JSON>
          Information about the user who added the media item. Note that this information is included only if the media item is within a shared album created by your app and you have the sharing scope.

      --contributor-info-display-name <CONTRIBUTOR_INFO_DISPLAY_NAME>
          Display name of the contributor.

      --contributor-info-profile-picture-base-url <CONTRIBUTOR_INFO_PROFILE_PICTURE_BASE_URL>
          URL to the profile picture of the contributor.

      --filename <FILENAME>
          Filename of the media item. This is shown to the user in the item's info section in the Google Photos app.

      --media-metadata <JSON>
          Metadata for a media item.

      --media-metadata-creation-time <MEDIA_METADATA_CREATION_TIME>
          Time when the media item was first created (not when it was uploaded to Google Photos).

      --media-metadata-height <MEDIA_METADATA_HEIGHT>
          Original height (in pixels) of the media item.

      --media-metadata-photo-aperture-f-number <MEDIA_METADATA_PHOTO_APERTURE_F_NUMBER>
          Aperture f number of the camera lens with which the photo was taken.

      --media-metadata-photo-camera-make <MEDIA_METADATA_PHOTO_CAMERA_MAKE>
          Brand of the camera with which the photo was taken.

      --media-metadata-photo-camera-model <MEDIA_METADATA_PHOTO_CAMERA_MODEL>
          Model of the camera with which the photo was taken.

      --media-metadata-photo-exposure-time <MEDIA_METADATA_PHOTO_EXPOSURE_TIME>
          Exposure time of the camera aperture when the photo was taken.

      --media-metadata-photo-focal-length <MEDIA_METADATA_PHOTO_FOCAL_LENGTH>
          Focal length of the camera lens with which the photo was taken.

      --media-metadata-photo-iso-equivalent <MEDIA_METADATA_PHOTO_ISO_EQUIVALENT>
          ISO of the camera with which the photo was taken.

      --media-metadata-video-camera-make <MEDIA_METADATA_VIDEO_CAMERA_MAKE>
          Brand of the camera with which the video was taken.

      --media-metadata-video-camera-model <MEDIA_METADATA_VIDEO_CAMERA_MODEL>
          Model of the camera with which the video was taken.

      --media-metadata-video-fps <MEDIA_METADATA_VIDEO_FPS>
          Frame rate of the video.

      --media-metadata-video-status <MEDIA_METADATA_VIDEO_STATUS>
          Processing status of the video. Choices: UNSPECIFIED, PROCESSING, READY, FAILED.

      --media-metadata-width <MEDIA_METADATA_WIDTH>
          Original width (in pixels) of the media item.

      --product-url <PRODUCT_URL>
          Google Photos URL for the media item. This link is available to the user only if they're signed in. When retrieved from an album search, the URL points to the item inside the album.

      --update-mask <UPDATE_MASK>
          Required. Indicate what fields in the provided media item to update. The only valid value is `description`.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## media-items.search

```text
Searches for app created media items in a user's Google Photos library. Only media items and ablums created by your app are returned. If no filters are set, then all app created media items in the user's library are returned. If an app created album is set, all media items in the specified album are returned. If filters are specified, app created media items that match the filters from the user's library are listed. If you set both the album and the filters, the request results in an error.

Input choices:
  Search an album with --album-id, or search with filters. Album searches do not accept filters.

Usage: joog [OPTIONS] photos <ACCOUNT> media-items.search [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --album-id <ALBUM_ID>
          Identifier of an album. If populated, lists all media items in specified album. Can't set in conjunction with any filters.

      --include-archived[=<BOOL>]
          If set, the results include media items that the user has archived. Defaults to false (archived media items aren't included).

      --media-types <MEDIA_TYPES>...
          The types of media items to be included. This field should be populated with only one media type. If you specify multiple media types, it results in an error.

  -h, --help
          Print help (see a summary with '-h')

Optional parameters (advanced):
      --home <PATH>
          Store state in PATH/joog Place before the command.

      --fields <MASK>
          Select response fields

      --timeout <SECONDS>
          Google request timeout in seconds Place before the command.
          
          [default: 30]

      --ttl <MINUTES>
          Reuse encrypted response data; 0 disables caching
          
          [default: 0]

      --max-pages <N>
          Stop after N pages; by default, fetch all pages

      --body <JSON>
          Inline JSON request body

      --filters <JSON>
          Filters that can be applied to a media item search. If multiple filter options are specified, they're treated as AND with each other.

      --filters-content-filter-excluded-content-categories <FILTERS_CONTENT_FILTER_EXCLUDED_CONTENT_CATEGORIES>...
          The set of categories which are not to be included in the media item search results. The items in the set are ORed. There's a maximum of 10 `excludedContentCategories` per request.

      --filters-content-filter-included-content-categories <FILTERS_CONTENT_FILTER_INCLUDED_CONTENT_CATEGORIES>...
          The set of categories to be included in the media item search results. The items in the set are ORed. There's a maximum of 10 `includedContentCategories` per request.

      --filters-date-filter-dates <JSON>
          List of dates that match the media items' creation date. A maximum of 5 dates can be included per request.

      --filters-date-filter-ranges <JSON>
          List of dates ranges that match the media items' creation date. A maximum of 5 dates ranges can be included per request.

      --filters-exclude-non-app-created-data[=<BOOL>]
          If set, the results exclude media items that were not created by this app. Defaults to false (all media items are returned). This field is ignored if the photoslibrary.readonly.appcreateddata scope is used.

      --filters-feature-filter-included-features <FILTERS_FEATURE_FILTER_INCLUDED_FEATURES>...
          The set of features to be included in the media item search results. The items in the set are ORed and may match any of the specified features.

      --order-by <ORDER_BY>
          An optional field to specify the sort order of the search results. The `orderBy` field only works when a dateFilter is used. When this field is not specified, results are displayed newest first, oldest last by their creationTime. Providing `MediaMetadata.creation_time` displays search results in the opposite order, oldest first then newest last. To display results newest first then oldest last, include the `desc` argument as follows: `MediaMetadata.creation_time desc`. The only additional filters that can be used with this parameter are includeArchivedMedia and excludeNonAppCreatedData. No other filters are supported.

      --page-size <PAGE_SIZE>
          Maximum number of media items to return in the response. Fewer media items might be returned than the specified number. The default `pageSize` is 25, the maximum is 100.

      --page-token <PAGE_TOKEN>
          A continuation token to get the next page of the results. Adding this to the request returns the rows after the `pageToken`. The `pageToken` should be the value returned in the `nextPageToken` parameter in the response to the `searchMediaItems` request.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).
