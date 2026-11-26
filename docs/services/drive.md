# Google Drive methods

See the [command guide](../grammar.md) and [request-file format](../file-schemas.md).

| Method | Description |
| --- | --- |
| `changes.get-start-page-token` | Gets the starting pageToken for listing future changes |
| `changes.list` | Lists the changes for a user or shared drive |
| `changes.watch` | Subscribes to changes for a user |
| `channels.stop` | Stops watching resources through this channel |
| `comments.create` | Creates a comment on a file |
| `comments.delete` | Deletes a comment |
| `comments.get` | Gets a comment by ID |
| `comments.list` | Lists a file's comments |
| `comments.update` | Updates a comment with patch semantics |
| `drives.create` | Creates a shared drive |
| `drives.delete` | Permanently deletes a shared drive for which the user is an `organizer` |
| `drives.get` | Gets a shared drive's metadata by ID |
| `drives.hide` | Hides a shared drive from the default view |
| `drives.list` | Lists the user's shared drives |
| `drives.unhide` | Restores a shared drive to the default view |
| `drives.update` | Updates the metadata for a shared drive |
| `files.copy` | Creates a copy of a file and applies any requested updates with patch semantics |
| `files.create` | Creates a file |
| `files.delete` | Permanently deletes a file owned by the user without moving it to the trash |
| `files.download` | Downloads the content of a file |
| `files.empty-trash` | Permanently deletes all of the user's trashed files |
| `files.export` | Exports a Google Workspace document to the requested MIME type and returns exported byte content |
| `files.generate-cse-token` | Generates a CSE token which can be used to create or update CSE files. |
| `files.generate-ids` | Generates a set of file IDs which can be provided in create or copy requests |
| `files.get` | Gets a file's metadata or content by ID |
| `files.list` | Lists the user's files |
| `files.list-labels` | Lists the labels on a file |
| `files.modify-labels` | Modifies the set of labels applied to a file |
| `files.update` | Updates a file's metadata, content, or both |
| `operations.get` | Gets the latest state of a long-running operation |
| `permissions.create` | Creates a permission for a file or shared drive |
| `permissions.delete` | Deletes a permission |
| `permissions.get` | Gets a permission by ID |
| `permissions.list` | Lists a file's or shared drive's permissions |
| `permissions.update` | Updates a permission with patch semantics |
| `replies.create` | Creates a reply to a comment |
| `replies.delete` | Deletes a reply |
| `replies.get` | Gets a reply by ID |
| `replies.list` | Lists a comment's replies |
| `replies.update` | Updates a reply with patch semantics |
| `revisions.delete` | Permanently deletes a file version |
| `revisions.get` | Gets a revision's metadata or content by ID |
| `revisions.list` | Lists a file's revisions |
| `revisions.update` | Updates a revision with patch semantics |

## changes.get-start-page-token

```text
Gets the starting pageToken for listing future changes. For more information, see [Retrieve changes](https://developers.google.com/workspace/drive/api/guides/manage-changes).

Usage: joog [OPTIONS] drive <ACCOUNT> changes.get-start-page-token [COMMAND_OPTIONS]

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

      --drive-id <DRIVE_ID>
          The ID of the shared drive for which the starting pageToken for listing future changes from that shared drive will be returned.

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

      --team-drive-id <TEAM_DRIVE_ID>
          Deprecated: Use `driveId` instead.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## changes.list

```text
Lists the changes for a user or shared drive. For more information, see [Retrieve changes](https://developers.google.com/workspace/drive/api/guides/manage-changes).

Usage: joog [OPTIONS] drive <ACCOUNT> changes.list [COMMAND_OPTIONS]

Mandatory parameters:
      --page-token <PAGE_TOKEN>
          The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.

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

      --drive-id <DRIVE_ID>
          The shared drive from which changes will be returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier.

      --include-corpus-removals[=<BOOL>]
          Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file.

      --include-items-from-all-drives[=<BOOL>]
          Whether both My Drive and shared drive items should be included in results.

      --include-labels <INCLUDE_LABELS>
          A comma-separated list of IDs of labels to include in the `labelInfo` part of the response.

      --include-permissions-for-view <INCLUDE_PERMISSIONS_FOR_VIEW>
          Specifies which additional view's permissions to include in the response. Only 'published' is supported.

      --include-removed[=<BOOL>]
          Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access.

      --include-team-drive-items[=<BOOL>]
          Deprecated: Use `includeItemsFromAllDrives` instead.

      --page-size <PAGE_SIZE>
          The maximum number of changes to return. The service may return fewer than this value. If unspecified, at most 100 changes will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000.

      --restrict-to-my-drive[=<BOOL>]
          Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive.

      --spaces <SPACES>
          A comma-separated list of spaces to query within the corpora. Supported values are 'drive' and 'appDataFolder'.

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

      --team-drive-id <TEAM_DRIVE_ID>
          Deprecated: Use `driveId` instead.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## changes.watch

```text
Subscribes to changes for a user. For more information, see [Notifications for resource changes](https://developers.google.com/workspace/drive/api/guides/push).

Usage: joog [OPTIONS] drive <ACCOUNT> changes.watch [COMMAND_OPTIONS]

Mandatory parameters:
      --page-token <PAGE_TOKEN>
          The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response or to the response from the getStartPageToken method.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --type <TYPE>
          The type of delivery mechanism used for this channel. Valid values are "web_hook" or "webhook".

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

      --address <ADDRESS>
          The address where notifications are delivered for this channel.

      --body-params <JSON>
          Additional parameters controlling delivery channel behavior. Optional.

      --drive-id <DRIVE_ID>
          The shared drive from which changes will be returned. If specified the change IDs will be reflective of the shared drive; use the combined drive ID and change ID as an identifier.

      --expiration <EXPIRATION>
          Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.

      --id <ID>
          A UUID or similar unique string that identifies this channel.

      --include-corpus-removals[=<BOOL>]
          Whether changes should include the file resource if the file is still accessible by the user at the time of the request, even when a file was removed from the list of changes and there will be no further change entries for this file.

      --include-items-from-all-drives[=<BOOL>]
          Whether both My Drive and shared drive items should be included in results.

      --include-labels <INCLUDE_LABELS>
          A comma-separated list of IDs of labels to include in the `labelInfo` part of the response.

      --include-permissions-for-view <INCLUDE_PERMISSIONS_FOR_VIEW>
          Specifies which additional view's permissions to include in the response. Only 'published' is supported.

      --include-removed[=<BOOL>]
          Whether to include changes indicating that items have been removed from the list of changes, for example by deletion or loss of access.

      --include-team-drive-items[=<BOOL>]
          Deprecated: Use `includeItemsFromAllDrives` instead.

      --kind <KIND>
          Identifies this as a notification channel used to watch for changes to a resource, which is `api#channel`.

      --page-size <PAGE_SIZE>
          The maximum number of changes to return. The service may return fewer than this value. If unspecified, at most 100 changes will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000.

      --payload[=<BOOL>]
          A Boolean value to indicate whether payload is wanted. Optional.

      --resource-id <RESOURCE_ID>
          An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.

      --resource-uri <RESOURCE_URI>
          A version-specific identifier for the watched resource.

      --restrict-to-my-drive[=<BOOL>]
          Whether to restrict the results to changes inside the My Drive hierarchy. This omits changes to files such as those in the Application Data folder or shared files which have not been added to My Drive.

      --spaces <SPACES>
          A comma-separated list of spaces to query within the corpora. Supported values are 'drive' and 'appDataFolder'.

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

      --team-drive-id <TEAM_DRIVE_ID>
          Deprecated: Use `driveId` instead.

      --token <TOKEN>
          An arbitrary string delivered to the target address with each notification delivered over this channel. Optional.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## channels.stop

```text
Stops watching resources through this channel. For more information, see [Notifications for resource changes](https://developers.google.com/workspace/drive/api/guides/push).

Usage: joog [OPTIONS] drive <ACCOUNT> channels.stop [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --type <TYPE>
          The type of delivery mechanism used for this channel. Valid values are "web_hook" or "webhook".

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

      --address <ADDRESS>
          The address where notifications are delivered for this channel.

      --body-params <JSON>
          Additional parameters controlling delivery channel behavior. Optional.

      --expiration <EXPIRATION>
          Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.

      --id <ID>
          A UUID or similar unique string that identifies this channel.

      --kind <KIND>
          Identifies this as a notification channel used to watch for changes to a resource, which is `api#channel`.

      --payload[=<BOOL>]
          A Boolean value to indicate whether payload is wanted. Optional.

      --resource-id <RESOURCE_ID>
          An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.

      --resource-uri <RESOURCE_URI>
          A version-specific identifier for the watched resource.

      --token <TOKEN>
          An arbitrary string delivered to the target address with each notification delivered over this channel. Optional.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## comments.create

```text
Creates a comment on a file. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments). Required: The `fields` parameter must be set. To return the exact fields you need, see [Return specific fields](https://developers.google.com/workspace/drive/api/guides/fields-parameter).

Usage: joog [OPTIONS] drive <ACCOUNT> comments.create [COMMAND_OPTIONS]

Mandatory parameters:
      --content <CONTENT>
          The plain text content of the comment. This field is used for setting the content, while `htmlContent` should be displayed.

      --file-id <FILE_ID>
          The ID of the file.

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

      --anchor <ANCHOR>
          A region of the document represented as a JSON string. For details on defining anchor properties, refer to [Manage comments and replies](https://developers.google.com/workspace/drive/api/v3/manage-comments).

      --author <JSON>
          Information about a Drive user.

      --created-time <CREATED_TIME>
          The time at which the comment was created (RFC 3339 date-time).

      --modified-time <MODIFIED_TIME>
          The last time the comment or any of its replies was modified (RFC 3339 date-time).

      --quoted-file-content <JSON>
          The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment.

      --quoted-file-content-mime-type <QUOTED_FILE_CONTENT_MIME_TYPE>
          The MIME type of the quoted content.

      --quoted-file-content-value <QUOTED_FILE_CONTENT_VALUE>
          The quoted content itself. This is interpreted as plain text if set through the API.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## comments.delete

```text
Deletes a comment. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments).

Usage: joog [OPTIONS] drive <ACCOUNT> comments.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --comment-id <COMMENT_ID>
          The ID of the comment.

      --file-id <FILE_ID>
          The ID of the file.

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

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## comments.get

```text
Gets a comment by ID. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments). Required: The `fields` parameter must be set. To return the exact fields you need, see [Return specific fields](https://developers.google.com/workspace/drive/api/guides/fields-parameter).

Usage: joog [OPTIONS] drive <ACCOUNT> comments.get [COMMAND_OPTIONS]

Mandatory parameters:
      --comment-id <COMMENT_ID>
          The ID of the comment.

      --file-id <FILE_ID>
          The ID of the file.

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

      --include-deleted[=<BOOL>]
          Whether to return deleted comments. Deleted comments will not include their original content.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## comments.list

```text
Lists a file's comments. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments). Required: The `fields` parameter must be set. To return the exact fields you need, see [Return specific fields](https://developers.google.com/workspace/drive/api/guides/fields-parameter).

Usage: joog [OPTIONS] drive <ACCOUNT> comments.list [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file.

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

      --include-deleted[=<BOOL>]
          Whether to include deleted comments. Deleted comments will not include their original content.

      --page-size <PAGE_SIZE>
          The maximum number of comments to return. The service may return fewer than this value. If unspecified, at most 20 comments will be returned. The maximum value is 100; values above 100 will be coerced to 100.

      --page-token <PAGE_TOKEN>
          The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.

      --start-modified-time <START_MODIFIED_TIME>
          The minimum value of 'modifiedTime' for the result comments (RFC 3339 date-time).

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## comments.update

```text
Updates a comment with patch semantics. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments). Required: The `fields` parameter must be set. To return the exact fields you need, see [Return specific fields](https://developers.google.com/workspace/drive/api/guides/fields-parameter).

Usage: joog [OPTIONS] drive <ACCOUNT> comments.update [COMMAND_OPTIONS]

Mandatory parameters:
      --comment-id <COMMENT_ID>
          The ID of the comment.

      --content <CONTENT>
          The plain text content of the comment. This field is used for setting the content, while `htmlContent` should be displayed.

      --file-id <FILE_ID>
          The ID of the file.

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

      --anchor <ANCHOR>
          A region of the document represented as a JSON string. For details on defining anchor properties, refer to [Manage comments and replies](https://developers.google.com/workspace/drive/api/v3/manage-comments).

      --author <JSON>
          Information about a Drive user.

      --created-time <CREATED_TIME>
          The time at which the comment was created (RFC 3339 date-time).

      --modified-time <MODIFIED_TIME>
          The last time the comment or any of its replies was modified (RFC 3339 date-time).

      --quoted-file-content <JSON>
          The file content to which the comment refers, typically within the anchor region. For a text file, for example, this would be the text at the location of the comment.

      --quoted-file-content-mime-type <QUOTED_FILE_CONTENT_MIME_TYPE>
          The MIME type of the quoted content.

      --quoted-file-content-value <QUOTED_FILE_CONTENT_VALUE>
          The quoted content itself. This is interpreted as plain text if set through the API.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## drives.create

```text
Creates a shared drive. For more information, see [Manage shared drives](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives).

Usage: joog [OPTIONS] drive <ACCOUNT> drives.create [COMMAND_OPTIONS]

Mandatory parameters:
      --request-id <REQUEST_ID>
          Required. An ID, such as a random UUID, which uniquely identifies this user's request for idempotent creation of a shared drive. A repeated request by the same user and with the same request ID will avoid creating duplicates by attempting to create the same shared drive. If the shared drive already exists a 409 error will be returned.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --name <NAME>
          The name of this shared drive.

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

      --background-image-file <JSON>
          An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on `drive.drives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set.

      --background-image-file-id <BACKGROUND_IMAGE_FILE_ID>
          The ID of an image file in Google Drive to use for the background image.

      --background-image-file-width <BACKGROUND_IMAGE_FILE_WIDTH>
          The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high.

      --background-image-file-x-coordinate <BACKGROUND_IMAGE_FILE_X_COORDINATE>
          The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image.

      --background-image-file-y-coordinate <BACKGROUND_IMAGE_FILE_Y_COORDINATE>
          The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image.

      --color-rgb <COLOR_RGB>
          The color of this shared drive as an RGB hex string. It can only be set on a `drive.drives.update` request that does not set `themeId`.

      --created-time <CREATED_TIME>
          The time at which the shared drive was created (RFC 3339 date-time).

      --hidden[=<BOOL>]
          Whether the shared drive is hidden from default view.

      --restrictions <JSON>
          A set of restrictions that apply to this shared drive or items inside this shared drive. Note that restrictions can't be set when creating a shared drive. To add a restriction, first create a shared drive and then use `drives.update` to add restrictions.

      --restrictions-admin-managed-restrictions[=<BOOL>]
          Whether administrative privileges on this shared drive are required to modify restrictions.

      --restrictions-copy-requires-writer-permission[=<BOOL>]
          Whether the options to copy, print, or download files inside this shared drive, should be disabled for readers and commenters. When this restriction is set to `true`, it will override the similarly named field to `true` for any file inside this shared drive.

      --restrictions-domain-users-only[=<BOOL>]
          Whether access to this shared drive and items inside this shared drive is restricted to users of the domain to which this shared drive belongs. This restriction may be overridden by other sharing policies controlled outside of this shared drive.

      --restrictions-download-restriction-restricted-for-readers[=<BOOL>]
          Whether download and copy is restricted for readers.

      --restrictions-download-restriction-restricted-for-writers[=<BOOL>]
          Whether download and copy is restricted for writers. If true, download is also restricted for readers.

      --restrictions-drive-members-only[=<BOOL>]
          Whether access to items inside this shared drive is restricted to its members.

      --restrictions-sharing-folders-requires-organizer-permission[=<BOOL>]
          If true, only users with the organizer role can share folders. If false, users with either the organizer role or the file organizer role can share folders.

      --theme-id <THEME_ID>
          The ID of the theme from which the background image and color will be set. The set of possible `driveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.drives.create` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## drives.delete

```text
Permanently deletes a shared drive for which the user is an `organizer`. The shared drive cannot contain any untrashed items. For more information, see [Manage shared drives](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives).

Usage: joog [OPTIONS] drive <ACCOUNT> drives.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --drive-id <DRIVE_ID>
          The ID of the shared drive.

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

      --allow-item-deletion[=<BOOL>]
          Whether any items inside the shared drive should also be deleted. This option is only supported when `useDomainAdminAccess` is also set to `true`.

      --use-domain-admin-access[=<BOOL>]
          Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## drives.get

```text
Gets a shared drive's metadata by ID. For more information, see [Manage shared drives](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives).

Usage: joog [OPTIONS] drive <ACCOUNT> drives.get [COMMAND_OPTIONS]

Mandatory parameters:
      --drive-id <DRIVE_ID>
          The ID of the shared drive.

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

      --use-domain-admin-access[=<BOOL>]
          Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## drives.hide

```text
Hides a shared drive from the default view. For more information, see [Manage shared drives](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives).

Usage: joog [OPTIONS] drive <ACCOUNT> drives.hide [COMMAND_OPTIONS]

Mandatory parameters:
      --drive-id <DRIVE_ID>
          The ID of the shared drive.

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

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## drives.list

```text
Lists the user's shared drives. This method accepts the `q` parameter, which is a search query combining one or more search terms. For more information, see the [Search for shared drives](https://developers.google.com/workspace/drive/api/guides/search-shareddrives) guide.

Usage: joog [OPTIONS] drive <ACCOUNT> drives.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --q <Q>
          Query string for searching shared drives.

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
          The maximum number of shared drives to return. The service may return fewer than this value. If unspecified, at most 10 shared drives will be returned. The maximum value is 100; values above 100 will be coerced to 100.

      --page-token <PAGE_TOKEN>
          Page token for shared drives.

      --use-domain-admin-access[=<BOOL>]
          Issue the request as a domain administrator; if set to true, then all shared drives of the domain in which the requester is an administrator are returned.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## drives.unhide

```text
Restores a shared drive to the default view. For more information, see [Manage shared drives](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives).

Usage: joog [OPTIONS] drive <ACCOUNT> drives.unhide [COMMAND_OPTIONS]

Mandatory parameters:
      --drive-id <DRIVE_ID>
          The ID of the shared drive.

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

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## drives.update

```text
Updates the metadata for a shared drive. For more information, see [Manage shared drives](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives).

Usage: joog [OPTIONS] drive <ACCOUNT> drives.update [COMMAND_OPTIONS]

Mandatory parameters:
      --drive-id <DRIVE_ID>
          The ID of the shared drive.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --name <NAME>
          The name of this shared drive.

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

      --background-image-file <JSON>
          An image file and cropping parameters from which a background image for this shared drive is set. This is a write only field; it can only be set on `drive.drives.update` requests that don't set `themeId`. When specified, all fields of the `backgroundImageFile` must be set.

      --background-image-file-id <BACKGROUND_IMAGE_FILE_ID>
          The ID of an image file in Google Drive to use for the background image.

      --background-image-file-width <BACKGROUND_IMAGE_FILE_WIDTH>
          The width of the cropped image in the closed range of 0 to 1. This value represents the width of the cropped image divided by the width of the entire image. The height is computed by applying a width to height aspect ratio of 80 to 9. The resulting image must be at least 1280 pixels wide and 144 pixels high.

      --background-image-file-x-coordinate <BACKGROUND_IMAGE_FILE_X_COORDINATE>
          The X coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the horizontal distance from the left side of the entire image to the left side of the cropping area divided by the width of the entire image.

      --background-image-file-y-coordinate <BACKGROUND_IMAGE_FILE_Y_COORDINATE>
          The Y coordinate of the upper left corner of the cropping area in the background image. This is a value in the closed range of 0 to 1. This value represents the vertical distance from the top side of the entire image to the top side of the cropping area divided by the height of the entire image.

      --color-rgb <COLOR_RGB>
          The color of this shared drive as an RGB hex string. It can only be set on a `drive.drives.update` request that does not set `themeId`.

      --created-time <CREATED_TIME>
          The time at which the shared drive was created (RFC 3339 date-time).

      --hidden[=<BOOL>]
          Whether the shared drive is hidden from default view.

      --restrictions <JSON>
          A set of restrictions that apply to this shared drive or items inside this shared drive. Note that restrictions can't be set when creating a shared drive. To add a restriction, first create a shared drive and then use `drives.update` to add restrictions.

      --restrictions-admin-managed-restrictions[=<BOOL>]
          Whether administrative privileges on this shared drive are required to modify restrictions.

      --restrictions-copy-requires-writer-permission[=<BOOL>]
          Whether the options to copy, print, or download files inside this shared drive, should be disabled for readers and commenters. When this restriction is set to `true`, it will override the similarly named field to `true` for any file inside this shared drive.

      --restrictions-domain-users-only[=<BOOL>]
          Whether access to this shared drive and items inside this shared drive is restricted to users of the domain to which this shared drive belongs. This restriction may be overridden by other sharing policies controlled outside of this shared drive.

      --restrictions-download-restriction-restricted-for-readers[=<BOOL>]
          Whether download and copy is restricted for readers.

      --restrictions-download-restriction-restricted-for-writers[=<BOOL>]
          Whether download and copy is restricted for writers. If true, download is also restricted for readers.

      --restrictions-drive-members-only[=<BOOL>]
          Whether access to items inside this shared drive is restricted to its members.

      --restrictions-sharing-folders-requires-organizer-permission[=<BOOL>]
          If true, only users with the organizer role can share folders. If false, users with either the organizer role or the file organizer role can share folders.

      --theme-id <THEME_ID>
          The ID of the theme from which the background image and color will be set. The set of possible `driveThemes` can be retrieved from a `drive.about.get` response. When not specified on a `drive.drives.create` request, a random theme is chosen from which the background image and color are set. This is a write-only field; it can only be set on requests that don't set `colorRgb` or `backgroundImageFile`.

      --use-domain-admin-access[=<BOOL>]
          Issue the request as a domain administrator; if set to true, then the requester will be granted access if they are an administrator of the domain to which the shared drive belongs.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## files.copy

```text
Creates a copy of a file and applies any requested updates with patch semantics. For more information, see [Create and manage files](https://developers.google.com/workspace/drive/api/guides/create-file).

Usage: joog [OPTIONS] drive <ACCOUNT> files.copy [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --description <DESCRIPTION>
          A short description of the file.

      --mime-type <MIME_TYPE>
          The MIME type of the file. Google Drive attempts to automatically detect an appropriate value from uploaded content, if no value is provided. The value cannot be changed unless a new revision is uploaded. If a file is created with a Google Doc MIME type, the uploaded content is imported, if possible. The supported import formats are published in the [`about`](/workspace/drive/api/reference/rest/v3/about) resource.

      --name <NAME>
          The name of the file. This isn't necessarily unique within a folder. Note that for immutable items such as the top-level folders of shared drives, the My Drive root folder, and the Application Data folder, the name is constant.

      --parents <PARENTS>...
          The ID of the parent folder containing the file. A file can only have one parent folder; specifying multiple parents isn't supported. If not specified as part of a create request, the file is placed directly in the user's My Drive folder. If not specified as part of a copy request, the file inherits any discoverable parent of the source file. Update requests must use the `addParents` and `removeParents` parameters to modify the parents list.

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

      --app-properties <JSON>
          A collection of arbitrary key-value pairs which are private to the requesting app.
          Entries with null values are cleared in update and copy requests. These properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties.

      --client-encryption-details <JSON>
          Details about the client-side encryption applied to the file.

      --client-encryption-details-decryption-metadata-aes256-gcm-chunk-size <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_AES256_GCM_CHUNK_SIZE>
          Chunk size used if content was encrypted with the AES 256 GCM Cipher. Possible values are: - default - small

      --client-encryption-details-decryption-metadata-encryption-resource-key-hash <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_ENCRYPTION_RESOURCE_KEY_HASH>
          The URL-safe Base64 encoded HMAC-SHA256 digest of the resource metadata with its DEK (Data Encryption Key); see https://developers.google.com/workspace/cse/reference

      --client-encryption-details-decryption-metadata-jwt <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_JWT>
          The signed JSON Web Token (JWT) which can be used to authorize the requesting user with the Key ACL Service (KACLS). The JWT asserts that the requesting user has at least read permissions on the file.

      --client-encryption-details-decryption-metadata-kacls-id <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_KACLS_ID>
          The ID of the KACLS (Key ACL Service) used to encrypt the file.

      --client-encryption-details-decryption-metadata-kacls-name <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_KACLS_NAME>
          The name of the KACLS (Key ACL Service) used to encrypt the file.

      --client-encryption-details-decryption-metadata-key-format <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_KEY_FORMAT>
          Key format for the unwrapped key. Must be `tinkAesGcmKey`.

      --client-encryption-details-decryption-metadata-wrapped-key <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_WRAPPED_KEY>
          The URL-safe Base64 encoded wrapped key used to encrypt the contents of the file.

      --client-encryption-details-encryption-state <CLIENT_ENCRYPTION_DETAILS_ENCRYPTION_STATE>
          The encryption state of the file. The values expected here are: - encrypted - unencrypted

      --content-hints <JSON>
          Additional information about the content of the file. These fields are never populated in responses.

      --content-hints-indexable-text <CONTENT_HINTS_INDEXABLE_TEXT>
          Text to be indexed for the file to improve fullText queries. This is limited to 128 KB in length and may contain HTML elements.

      --content-hints-thumbnail-image <CONTENT_HINTS_THUMBNAIL_IMAGE>
          The thumbnail data encoded with URL-safe Base64 ([RFC 4648 section 5](https://datatracker.ietf.org/doc/html/rfc4648#section-5)).

      --content-hints-thumbnail-mime-type <CONTENT_HINTS_THUMBNAIL_MIME_TYPE>
          The MIME type of the thumbnail.

      --content-restrictions <JSON>
          Restrictions for accessing the content of the file. Only populated if such a restriction exists.

      --copy-comments[=<BOOL>]
          Whether to copy the open (unresolved) comments associated with the file.

      --copy-requires-writer-permission[=<BOOL>]
          Whether the options to copy, print, or download this file should be disabled for readers and commenters.

      --created-time <CREATED_TIME>
          The time at which the file was created (RFC 3339 date-time).

      --download-restrictions <JSON>
          Download restrictions applied to the file.

      --download-restrictions-effective-download-restriction-with-context-restricted-for-readers[=<BOOL>]
          Whether download and copy is restricted for readers.

      --download-restrictions-effective-download-restriction-with-context-restricted-for-writers[=<BOOL>]
          Whether download and copy is restricted for writers. If true, download is also restricted for readers.

      --download-restrictions-item-download-restriction-restricted-for-readers[=<BOOL>]
          Whether download and copy is restricted for readers.

      --download-restrictions-item-download-restriction-restricted-for-writers[=<BOOL>]
          Whether download and copy is restricted for writers. If true, download is also restricted for readers.

      --enforce-single-parent[=<BOOL>]
          Deprecated: Copying files into multiple folders is no longer supported. Use shortcuts instead.

      --folder-color-rgb <FOLDER_COLOR_RGB>
          The color for a folder or a shortcut to a folder as an RGB hex string. The supported colors are published in the `folderColorPalette` field of the [`about`](/workspace/drive/api/reference/rest/v3/about) resource. If an unsupported color is specified, the closest color in the palette is used instead.

      --id <ID>
          The ID of the file.

      --ignore-default-visibility[=<BOOL>]
          Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders.

      --include-labels <INCLUDE_LABELS>
          A comma-separated list of IDs of labels to include in the `labelInfo` part of the response.

      --include-permissions-for-view <INCLUDE_PERMISSIONS_FOR_VIEW>
          Specifies which additional view's permissions to include in the response. Only `published` is supported.

      --inherited-permissions-disabled[=<BOOL>]
          Whether this file has inherited permissions disabled. Inherited permissions are enabled by default.

      --keep-revision-forever[=<BOOL>]
          Whether to set the `keepForever` field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions.

      --label-info <JSON>
          Label information on the file.

      --last-modifying-user <JSON>
          Information about a Drive user.

      --link-share-metadata <JSON>
          Contains details about the link URLs that clients are using to refer to this item.

      --modified-by-me-time <MODIFIED_BY_ME_TIME>
          The last time the file was modified by the user (RFC 3339 date-time).

      --modified-time <MODIFIED_TIME>
          he last time the file was modified by anyone (RFC 3339 date-time). Note that setting modifiedTime will also update modifiedByMeTime for the user.

      --ocr-language <OCR_LANGUAGE>
          A language hint for OCR processing during image import (ISO 639-1 code).

      --original-filename <ORIGINAL_FILENAME>
          The original filename of the uploaded content if available, or else the original value of the `name` field. This is only available for files with binary content in Google Drive.

      --properties <JSON>
          A collection of arbitrary key-value pairs which are visible to all apps.
          Entries with null values are cleared in update and copy requests.

      --shared-with-me-time <SHARED_WITH_ME_TIME>
          The time at which the file was shared with the user, if applicable (RFC 3339 date-time).

      --sharing-user <JSON>
          Information about a Drive user.

      --shortcut-details <JSON>
          Information about a shortcut file.

      --shortcut-details-target-id <SHORTCUT_DETAILS_TARGET_ID>
          The ID of the file that this shortcut points to. Can only be set on `files.create` requests.

      --starred[=<BOOL>]
          Whether the user has starred the file.

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

      --team-drive-id <TEAM_DRIVE_ID>
          Deprecated: Output only. Use `driveId` instead.

      --trashed[=<BOOL>]
          Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file, but other users can still access the file in the owner's trash until it's permanently deleted.

      --trashed-time <TRASHED_TIME>
          The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives.

      --trashing-user <JSON>
          Information about a Drive user.

      --viewed-by-me-time <VIEWED_BY_ME_TIME>
          The last time the file was viewed by the user (RFC 3339 date-time).

      --viewers-can-copy-content[=<BOOL>]
          Deprecated: Use `copyRequiresWriterPermission` instead.

      --writers-can-share[=<BOOL>]
          Whether users with only `writer` permission can modify the file's permissions. Not populated for items in shared drives.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## files.create

```text
Creates a file. For more information, see [Create and manage files](https://developers.google.com/workspace/drive/api/guides/create-file). This method supports an */upload* URI and accepts uploaded media with the following characteristics: - *Maximum file size:* 5,120 GB - *Accepted Media MIME types:* `*/*` (Specify a valid MIME type, rather than the literal `*/*` value. The literal `*/*` is only used to indicate that any valid MIME type can be uploaded. For more information, see [Google Workspace and Google Drive supported MIME types](https://developers.google.com/workspace/drive/api/guides/mime-types).) For more information on uploading files, see [Upload file data](https://developers.google.com/workspace/drive/api/guides/manage-uploads). Apps creating shortcuts with the `create` method must specify the MIME type `application/vnd.google-apps.shortcut`. Apps should specify a file extension in the `name` property when inserting files with the API. For example, an operation to insert a JPEG file should specify something like `"name": "cat.jpg"` in the metadata. Subsequent `GET` requests include the read-only `fileExtension` property populated with the extension originally specified in the `name` property. When a Google Drive user requests to download a file, or when the file is downloaded through the sync client, Drive builds a full filename (with extension) based on the name. In cases where the extension is missing, Drive attempts to determine the extension based on the file's MIME type.

Input choices:
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] drive <ACCOUNT> files.create [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --upload <PATH>
          Upload a local file of at most 1 GiB.

      --verbose
          Include advanced parameters in help

      --description <DESCRIPTION>
          A short description of the file.

      --mime-type <MIME_TYPE>
          The MIME type of the file. Google Drive attempts to automatically detect an appropriate value from uploaded content, if no value is provided. The value cannot be changed unless a new revision is uploaded. If a file is created with a Google Doc MIME type, the uploaded content is imported, if possible. The supported import formats are published in the [`about`](/workspace/drive/api/reference/rest/v3/about) resource.

      --name <NAME>
          The name of the file. This isn't necessarily unique within a folder. Note that for immutable items such as the top-level folders of shared drives, the My Drive root folder, and the Application Data folder, the name is constant.

      --parents <PARENTS>...
          The ID of the parent folder containing the file. A file can only have one parent folder; specifying multiple parents isn't supported. If not specified as part of a create request, the file is placed directly in the user's My Drive folder. If not specified as part of a copy request, the file inherits any discoverable parent of the source file. Update requests must use the `addParents` and `removeParents` parameters to modify the parents list.

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

      --content-type <MIME>
          Upload MIME type
          
          [default: application/octet-stream]

      --body <JSON>
          Inline JSON request body

      --app-properties <JSON>
          A collection of arbitrary key-value pairs which are private to the requesting app.
          Entries with null values are cleared in update and copy requests. These properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties.

      --client-encryption-details <JSON>
          Details about the client-side encryption applied to the file.

      --client-encryption-details-decryption-metadata-aes256-gcm-chunk-size <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_AES256_GCM_CHUNK_SIZE>
          Chunk size used if content was encrypted with the AES 256 GCM Cipher. Possible values are: - default - small

      --client-encryption-details-decryption-metadata-encryption-resource-key-hash <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_ENCRYPTION_RESOURCE_KEY_HASH>
          The URL-safe Base64 encoded HMAC-SHA256 digest of the resource metadata with its DEK (Data Encryption Key); see https://developers.google.com/workspace/cse/reference

      --client-encryption-details-decryption-metadata-jwt <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_JWT>
          The signed JSON Web Token (JWT) which can be used to authorize the requesting user with the Key ACL Service (KACLS). The JWT asserts that the requesting user has at least read permissions on the file.

      --client-encryption-details-decryption-metadata-kacls-id <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_KACLS_ID>
          The ID of the KACLS (Key ACL Service) used to encrypt the file.

      --client-encryption-details-decryption-metadata-kacls-name <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_KACLS_NAME>
          The name of the KACLS (Key ACL Service) used to encrypt the file.

      --client-encryption-details-decryption-metadata-key-format <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_KEY_FORMAT>
          Key format for the unwrapped key. Must be `tinkAesGcmKey`.

      --client-encryption-details-decryption-metadata-wrapped-key <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_WRAPPED_KEY>
          The URL-safe Base64 encoded wrapped key used to encrypt the contents of the file.

      --client-encryption-details-encryption-state <CLIENT_ENCRYPTION_DETAILS_ENCRYPTION_STATE>
          The encryption state of the file. The values expected here are: - encrypted - unencrypted

      --content-hints <JSON>
          Additional information about the content of the file. These fields are never populated in responses.

      --content-hints-indexable-text <CONTENT_HINTS_INDEXABLE_TEXT>
          Text to be indexed for the file to improve fullText queries. This is limited to 128 KB in length and may contain HTML elements.

      --content-hints-thumbnail-image <CONTENT_HINTS_THUMBNAIL_IMAGE>
          The thumbnail data encoded with URL-safe Base64 ([RFC 4648 section 5](https://datatracker.ietf.org/doc/html/rfc4648#section-5)).

      --content-hints-thumbnail-mime-type <CONTENT_HINTS_THUMBNAIL_MIME_TYPE>
          The MIME type of the thumbnail.

      --content-restrictions <JSON>
          Restrictions for accessing the content of the file. Only populated if such a restriction exists.

      --copy-requires-writer-permission[=<BOOL>]
          Whether the options to copy, print, or download this file should be disabled for readers and commenters.

      --created-time <CREATED_TIME>
          The time at which the file was created (RFC 3339 date-time).

      --download-restrictions <JSON>
          Download restrictions applied to the file.

      --download-restrictions-effective-download-restriction-with-context-restricted-for-readers[=<BOOL>]
          Whether download and copy is restricted for readers.

      --download-restrictions-effective-download-restriction-with-context-restricted-for-writers[=<BOOL>]
          Whether download and copy is restricted for writers. If true, download is also restricted for readers.

      --download-restrictions-item-download-restriction-restricted-for-readers[=<BOOL>]
          Whether download and copy is restricted for readers.

      --download-restrictions-item-download-restriction-restricted-for-writers[=<BOOL>]
          Whether download and copy is restricted for writers. If true, download is also restricted for readers.

      --enforce-single-parent[=<BOOL>]
          Deprecated: Creating files in multiple folders is no longer supported.

      --folder-color-rgb <FOLDER_COLOR_RGB>
          The color for a folder or a shortcut to a folder as an RGB hex string. The supported colors are published in the `folderColorPalette` field of the [`about`](/workspace/drive/api/reference/rest/v3/about) resource. If an unsupported color is specified, the closest color in the palette is used instead.

      --id <ID>
          The ID of the file.

      --ignore-default-visibility[=<BOOL>]
          Whether to ignore the domain's default visibility settings for the created file. Domain administrators can choose to make all uploaded files visible to the domain by default; this parameter bypasses that behavior for the request. Permissions are still inherited from parent folders.

      --include-labels <INCLUDE_LABELS>
          A comma-separated list of IDs of labels to include in the `labelInfo` part of the response.

      --include-permissions-for-view <INCLUDE_PERMISSIONS_FOR_VIEW>
          Specifies which additional view's permissions to include in the response. Only `published` is supported.

      --inherited-permissions-disabled[=<BOOL>]
          Whether this file has inherited permissions disabled. Inherited permissions are enabled by default.

      --keep-revision-forever[=<BOOL>]
          Whether to set the `keepForever` field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions.

      --label-info <JSON>
          Label information on the file.

      --last-modifying-user <JSON>
          Information about a Drive user.

      --link-share-metadata <JSON>
          Contains details about the link URLs that clients are using to refer to this item.

      --modified-by-me-time <MODIFIED_BY_ME_TIME>
          The last time the file was modified by the user (RFC 3339 date-time).

      --modified-time <MODIFIED_TIME>
          he last time the file was modified by anyone (RFC 3339 date-time). Note that setting modifiedTime will also update modifiedByMeTime for the user.

      --ocr-language <OCR_LANGUAGE>
          A language hint for OCR processing during image import (ISO 639-1 code).

      --original-filename <ORIGINAL_FILENAME>
          The original filename of the uploaded content if available, or else the original value of the `name` field. This is only available for files with binary content in Google Drive.

      --properties <JSON>
          A collection of arbitrary key-value pairs which are visible to all apps.
          Entries with null values are cleared in update and copy requests.

      --shared-with-me-time <SHARED_WITH_ME_TIME>
          The time at which the file was shared with the user, if applicable (RFC 3339 date-time).

      --sharing-user <JSON>
          Information about a Drive user.

      --shortcut-details <JSON>
          Information about a shortcut file.

      --shortcut-details-target-id <SHORTCUT_DETAILS_TARGET_ID>
          The ID of the file that this shortcut points to. Can only be set on `files.create` requests.

      --starred[=<BOOL>]
          Whether the user has starred the file.

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

      --team-drive-id <TEAM_DRIVE_ID>
          Deprecated: Output only. Use `driveId` instead.

      --trashed[=<BOOL>]
          Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file, but other users can still access the file in the owner's trash until it's permanently deleted.

      --trashed-time <TRASHED_TIME>
          The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives.

      --trashing-user <JSON>
          Information about a Drive user.

      --use-content-as-indexable-text[=<BOOL>]
          Whether to use the uploaded content as indexable text.

      --viewed-by-me-time <VIEWED_BY_ME_TIME>
          The last time the file was viewed by the user (RFC 3339 date-time).

      --viewers-can-copy-content[=<BOOL>]
          Deprecated: Use `copyRequiresWriterPermission` instead.

      --writers-can-share[=<BOOL>]
          Whether users with only `writer` permission can modify the file's permissions. Not populated for items in shared drives.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## files.delete

```text
Permanently deletes a file owned by the user without moving it to the trash. For more information, see [Trash or delete files and folders](https://developers.google.com/workspace/drive/api/guides/delete). If the file belongs to a shared drive, the user must be an `organizer` on the parent folder. If the target is a folder, all descendants owned by the user are also deleted.

Usage: joog [OPTIONS] drive <ACCOUNT> files.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file.

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

      --enforce-single-parent[=<BOOL>]
          Deprecated: If an item isn't in a shared drive and its last parent is deleted but the item itself isn't, the item will be placed under its owner's root.

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## files.download

```text
Downloads the content of a file. For more information, see [Download and export files](https://developers.google.com/workspace/drive/api/guides/manage-downloads). Operations are valid for 24 hours from the time of creation.

Usage: joog [OPTIONS] drive <ACCOUNT> files.download [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          Required. The ID of the file to download.

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

      --mime-type <MIME_TYPE>
          Optional. The MIME type the file should be downloaded as. This field can only be set when downloading Google Workspace documents. For a list of supported MIME types, see [Export MIME types for Google Workspace documents](/workspace/drive/api/guides/ref-export-formats). If not set, a Google Workspace document is downloaded with a default MIME type. The default MIME type might change in the future.

      --revision-id <REVISION_ID>
          Optional. The revision ID of the file to download. This field can only be set when downloading blob files, Google Docs, and Google Sheets. Returns `INVALID_ARGUMENT` if downloading a specific revision on the file is unsupported.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## files.empty-trash

```text
Permanently deletes all of the user's trashed files. For more information, see [Trash or delete files and folders](https://developers.google.com/workspace/drive/api/guides/delete).

Usage: joog [OPTIONS] drive <ACCOUNT> files.empty-trash [COMMAND_OPTIONS]

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

      --drive-id <DRIVE_ID>
          If set, empties the trash of the provided shared drive.

      --enforce-single-parent[=<BOOL>]
          Deprecated: If an item isn't in a shared drive and its last parent is deleted but the item itself isn't, the item will be placed under its owner's root.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## files.export

```text
Exports a Google Workspace document to the requested MIME type and returns exported byte content. For more information, see [Download and export files](https://developers.google.com/workspace/drive/api/guides/manage-downloads). Note that the exported content is limited to 10 MB.

Usage: joog [OPTIONS] drive <ACCOUNT> files.export [COMMAND_OPTIONS]

Mandatory parameters:
  -o, --output <PATH>
          Save the downloaded content to a new file

      --file-id <FILE_ID>
          The ID of the file.

      --mime-type <MIME_TYPE>
          Required. The MIME type of the format requested for this export. For a list of supported MIME types, see [Export MIME types for Google Workspace documents](/workspace/drive/api/guides/ref-export-formats).

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

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## files.generate-cse-token

```text
Generates a CSE token which can be used to create or update CSE files.

Usage: joog [OPTIONS] drive <ACCOUNT> files.generate-cse-token [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --file-id <FILE_ID>
          The ID of the file for which the JWT should be generated. If not provided, an id will be generated.

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

      --parent <PARENT>
          The ID of the expected parent of the file. Used when generating a JWT for a new CSE file. If specified, the parent will be fetched, and if the parent is a shared drive item, the shared drive's policy will be used to determine the KACLS that should be used. It is invalid to specify both file_id and parent in a single request.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## files.generate-ids

```text
Generates a set of file IDs which can be provided in create or copy requests. For more information, see [Create and manage files](https://developers.google.com/workspace/drive/api/guides/create-file).

Usage: joog [OPTIONS] drive <ACCOUNT> files.generate-ids [COMMAND_OPTIONS]

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

      --count <COUNT>
          The number of IDs to return.

      --space <SPACE>
          The space in which the IDs can be used to create files. Supported values are `drive` and `appDataFolder`. (Default: `drive`.) For more information, see [File organization](https://developers.google.com/workspace/drive/api/guides/about-files#file-organization).

      --type <TYPE>
          The type of items which the IDs can be used for. Supported values are `files` and `shortcuts`. Note that `shortcuts` are only supported in the `drive` `space`. (Default: `files`.) For more information, see [File organization](https://developers.google.com/workspace/drive/api/guides/about-files#file-organization).

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## files.get

```text
Gets a file's metadata or content by ID. For more information, see [Search for files and folders](https://developers.google.com/workspace/drive/api/guides/search-files). If you provide the URL parameter `alt=media`, then the response includes the file contents in the response body. Downloading content with `alt=media` only works if the file is stored in Drive. To download Google Docs, Sheets, and Slides use [`files.export`](https://developers.google.com/workspace/drive/api/reference/rest/v3/files/export) instead. For more information, see [Download and export files](https://developers.google.com/workspace/drive/api/guides/manage-downloads).

Usage: joog [OPTIONS] drive <ACCOUNT> files.get [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

  -o, --output <PATH>
          Save the downloaded content to a new file

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

      --acknowledge-abuse[=<BOOL>]
          Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when the `alt` parameter is set to `media` and the user is the owner of the file or an organizer of the shared drive in which the file resides.

      --include-labels <INCLUDE_LABELS>
          A comma-separated list of IDs of labels to include in the `labelInfo` part of the response.

      --include-permissions-for-view <INCLUDE_PERMISSIONS_FOR_VIEW>
          Specifies which additional view's permissions to include in the response. Only `published` is supported.

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## files.list

```text
Lists the user's files. For more information, see [Search for files and folders](https://developers.google.com/workspace/drive/api/guides/search-files). This method accepts the `q` parameter, which is a search query combining one or more search terms. This method returns *all* files by default, including trashed files. If you don't want trashed files to appear in the list, use the `trashed=false` query parameter to remove trashed files from the results.

Usage: joog [OPTIONS] drive <ACCOUNT> files.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --order-by <ORDER_BY>
          A comma-separated list of sort keys. Valid keys are: * `createdTime`: When the file was created. Avoid using this key for queries on large item collections as it might result in timeouts or other issues. For time-related sorting on large item collections, use `modifiedTime desc` instead. * `folder`: The folder ID. This field is sorted using alphabetical ordering. * `modifiedByMeTime`: The last time the file was modified by the user. * `modifiedTime`: The last time the file was modified by anyone. * `name`: The name of the file. This field is sorted using alphabetical ordering, so 1, 12, 2, 22. * `name_natural`: The name of the file. This field is sorted using natural sort ordering, so 1, 2, 12, 22. * `quotaBytesUsed`: The number of storage quota bytes used by the file. * `recency`: The most recent timestamp from the file's date-time fields. * `sharedWithMeTime`: When the file was shared with the user, if applicable. * `starred`: Whether the user has starred the file. * `viewedByMeTime`: The last time the file was viewed by the user. Each key sorts ascending by default, but can be reversed with the `desc` modifier. Example usage: `?orderBy=folder,modifiedTime desc,name`.

      --q <Q>
          A query for filtering the file results. For supported syntax, see [Search for files and folders](/workspace/drive/api/guides/search-files).

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

      --corpora <CORPORA>
          Specifies a collection of items (files or documents) to which the query applies. Supported items include: * `user` * `domain` * `drive` * `allDrives` Prefer `user` or `drive` to `allDrives` for efficiency. By default, corpora is set to `user`. However, this can change depending on the filter set through the `q` parameter. For more information, see [File organization](https://developers.google.com/workspace/drive/api/guides/about-files#file-organization).

      --corpus <CORPUS>
          Deprecated: The source of files to list. Use `corpora` instead. Choices: domain, user.

      --drive-id <DRIVE_ID>
          ID of the shared drive to search.

      --include-items-from-all-drives[=<BOOL>]
          Whether both My Drive and shared drive items should be included in results.

      --include-labels <INCLUDE_LABELS>
          A comma-separated list of IDs of labels to include in the `labelInfo` part of the response.

      --include-permissions-for-view <INCLUDE_PERMISSIONS_FOR_VIEW>
          Specifies which additional view's permissions to include in the response. Only `published` is supported.

      --include-team-drive-items[=<BOOL>]
          Deprecated: Use `includeItemsFromAllDrives` instead.

      --page-size <PAGE_SIZE>
          The maximum number of files to return. The service may return fewer than this value. If unspecified, at most 100 files will be returned for shared drives, and the entire list of files for non-shared drives. The maximum value is 1000; values above 1000 will be coerced to 1000.

      --page-token <PAGE_TOKEN>
          The token for continuing a previous list request on the next page. This should be set to the value of `nextPageToken` from the previous response.

      --spaces <SPACES>
          A comma-separated list of spaces to query within the corpora. Supported values are `drive` and `appDataFolder`. For more information, see [File organization](https://developers.google.com/workspace/drive/api/guides/about-files#file-organization).

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

      --team-drive-id <TEAM_DRIVE_ID>
          Deprecated: Use `driveId` instead.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## files.list-labels

```text
Lists the labels on a file. For more information, see [List labels on a file](https://developers.google.com/workspace/drive/api/guides/list-labels).

Usage: joog [OPTIONS] drive <ACCOUNT> files.list-labels [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID for the file.

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

      --max-results <MAX_RESULTS>
          The maximum number of labels to return per page. When not set, defaults to 100.

      --page-token <PAGE_TOKEN>
          The token for continuing a previous list request on the next page. This should be set to the value of `nextPageToken` from the previous response.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## files.modify-labels

```text
Modifies the set of labels applied to a file. For more information, see [Set a label field on a file](https://developers.google.com/workspace/drive/api/guides/set-label). Returns a list of the labels that were added or modified.

Usage: joog [OPTIONS] drive <ACCOUNT> files.modify-labels [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file to which the labels belong.

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

      --kind <KIND>
          This is always `"drive#modifyLabelsRequest"`.

      --label-modifications <JSON>
          The list of modifications to apply to the labels on the file.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## files.update

```text
Updates a file's metadata, content, or both. When calling this method, only populate fields in the request that you want to modify. When updating fields, some fields might be changed automatically, such as `modifiedDate`. This method supports patch semantics. This method supports an */upload* URI and accepts uploaded media with the following characteristics: - *Maximum file size:* 5,120 GB - *Accepted Media MIME types:* `*/*` (Specify a valid MIME type, rather than the literal `*/*` value. The literal `*/*` is only used to indicate that any valid MIME type can be uploaded. For more information, see [Google Workspace and Google Drive supported MIME types](https://developers.google.com/workspace/drive/api/guides/mime-types).) For more information on uploading files, see [Upload file data](https://developers.google.com/workspace/drive/api/guides/manage-uploads).

Input choices:
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] drive <ACCOUNT> files.update [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --upload <PATH>
          Upload a local file of at most 1 GiB.

      --verbose
          Include advanced parameters in help

      --description <DESCRIPTION>
          A short description of the file.

      --mime-type <MIME_TYPE>
          The MIME type of the file. Google Drive attempts to automatically detect an appropriate value from uploaded content, if no value is provided. The value cannot be changed unless a new revision is uploaded. If a file is created with a Google Doc MIME type, the uploaded content is imported, if possible. The supported import formats are published in the [`about`](/workspace/drive/api/reference/rest/v3/about) resource.

      --name <NAME>
          The name of the file. This isn't necessarily unique within a folder. Note that for immutable items such as the top-level folders of shared drives, the My Drive root folder, and the Application Data folder, the name is constant.

      --parents <PARENTS>...
          The ID of the parent folder containing the file. A file can only have one parent folder; specifying multiple parents isn't supported. If not specified as part of a create request, the file is placed directly in the user's My Drive folder. If not specified as part of a copy request, the file inherits any discoverable parent of the source file. Update requests must use the `addParents` and `removeParents` parameters to modify the parents list.

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

      --content-type <MIME>
          Upload MIME type
          
          [default: application/octet-stream]

      --body <JSON>
          Inline JSON request body

      --add-parents <ADD_PARENTS>
          A comma-separated list of parent IDs to add.

      --app-properties <JSON>
          A collection of arbitrary key-value pairs which are private to the requesting app.
          Entries with null values are cleared in update and copy requests. These properties can only be retrieved using an authenticated request. An authenticated request uses an access token obtained with a OAuth 2 client ID. You cannot use an API key to retrieve private properties.

      --client-encryption-details <JSON>
          Details about the client-side encryption applied to the file.

      --client-encryption-details-decryption-metadata-aes256-gcm-chunk-size <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_AES256_GCM_CHUNK_SIZE>
          Chunk size used if content was encrypted with the AES 256 GCM Cipher. Possible values are: - default - small

      --client-encryption-details-decryption-metadata-encryption-resource-key-hash <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_ENCRYPTION_RESOURCE_KEY_HASH>
          The URL-safe Base64 encoded HMAC-SHA256 digest of the resource metadata with its DEK (Data Encryption Key); see https://developers.google.com/workspace/cse/reference

      --client-encryption-details-decryption-metadata-jwt <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_JWT>
          The signed JSON Web Token (JWT) which can be used to authorize the requesting user with the Key ACL Service (KACLS). The JWT asserts that the requesting user has at least read permissions on the file.

      --client-encryption-details-decryption-metadata-kacls-id <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_KACLS_ID>
          The ID of the KACLS (Key ACL Service) used to encrypt the file.

      --client-encryption-details-decryption-metadata-kacls-name <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_KACLS_NAME>
          The name of the KACLS (Key ACL Service) used to encrypt the file.

      --client-encryption-details-decryption-metadata-key-format <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_KEY_FORMAT>
          Key format for the unwrapped key. Must be `tinkAesGcmKey`.

      --client-encryption-details-decryption-metadata-wrapped-key <CLIENT_ENCRYPTION_DETAILS_DECRYPTION_METADATA_WRAPPED_KEY>
          The URL-safe Base64 encoded wrapped key used to encrypt the contents of the file.

      --client-encryption-details-encryption-state <CLIENT_ENCRYPTION_DETAILS_ENCRYPTION_STATE>
          The encryption state of the file. The values expected here are: - encrypted - unencrypted

      --content-hints <JSON>
          Additional information about the content of the file. These fields are never populated in responses.

      --content-hints-indexable-text <CONTENT_HINTS_INDEXABLE_TEXT>
          Text to be indexed for the file to improve fullText queries. This is limited to 128 KB in length and may contain HTML elements.

      --content-hints-thumbnail-image <CONTENT_HINTS_THUMBNAIL_IMAGE>
          The thumbnail data encoded with URL-safe Base64 ([RFC 4648 section 5](https://datatracker.ietf.org/doc/html/rfc4648#section-5)).

      --content-hints-thumbnail-mime-type <CONTENT_HINTS_THUMBNAIL_MIME_TYPE>
          The MIME type of the thumbnail.

      --content-restrictions <JSON>
          Restrictions for accessing the content of the file. Only populated if such a restriction exists.

      --copy-requires-writer-permission[=<BOOL>]
          Whether the options to copy, print, or download this file should be disabled for readers and commenters.

      --created-time <CREATED_TIME>
          The time at which the file was created (RFC 3339 date-time).

      --download-restrictions <JSON>
          Download restrictions applied to the file.

      --download-restrictions-effective-download-restriction-with-context-restricted-for-readers[=<BOOL>]
          Whether download and copy is restricted for readers.

      --download-restrictions-effective-download-restriction-with-context-restricted-for-writers[=<BOOL>]
          Whether download and copy is restricted for writers. If true, download is also restricted for readers.

      --download-restrictions-item-download-restriction-restricted-for-readers[=<BOOL>]
          Whether download and copy is restricted for readers.

      --download-restrictions-item-download-restriction-restricted-for-writers[=<BOOL>]
          Whether download and copy is restricted for writers. If true, download is also restricted for readers.

      --enforce-single-parent[=<BOOL>]
          Deprecated: Adding files to multiple folders is no longer supported. Use shortcuts instead.

      --folder-color-rgb <FOLDER_COLOR_RGB>
          The color for a folder or a shortcut to a folder as an RGB hex string. The supported colors are published in the `folderColorPalette` field of the [`about`](/workspace/drive/api/reference/rest/v3/about) resource. If an unsupported color is specified, the closest color in the palette is used instead.

      --id <ID>
          The ID of the file.

      --include-labels <INCLUDE_LABELS>
          A comma-separated list of IDs of labels to include in the `labelInfo` part of the response.

      --include-permissions-for-view <INCLUDE_PERMISSIONS_FOR_VIEW>
          Specifies which additional view's permissions to include in the response. Only `published` is supported.

      --inherited-permissions-disabled[=<BOOL>]
          Whether this file has inherited permissions disabled. Inherited permissions are enabled by default.

      --keep-revision-forever[=<BOOL>]
          Whether to set the `keepForever` field in the new head revision. This is only applicable to files with binary content in Google Drive. Only 200 revisions for the file can be kept forever. If the limit is reached, try deleting pinned revisions.

      --label-info <JSON>
          Label information on the file.

      --last-modifying-user <JSON>
          Information about a Drive user.

      --link-share-metadata <JSON>
          Contains details about the link URLs that clients are using to refer to this item.

      --modified-by-me-time <MODIFIED_BY_ME_TIME>
          The last time the file was modified by the user (RFC 3339 date-time).

      --modified-time <MODIFIED_TIME>
          he last time the file was modified by anyone (RFC 3339 date-time). Note that setting modifiedTime will also update modifiedByMeTime for the user.

      --ocr-language <OCR_LANGUAGE>
          A language hint for OCR processing during image import (ISO 639-1 code).

      --original-filename <ORIGINAL_FILENAME>
          The original filename of the uploaded content if available, or else the original value of the `name` field. This is only available for files with binary content in Google Drive.

      --properties <JSON>
          A collection of arbitrary key-value pairs which are visible to all apps.
          Entries with null values are cleared in update and copy requests.

      --remove-parents <REMOVE_PARENTS>
          A comma-separated list of parent IDs to remove.

      --shared-with-me-time <SHARED_WITH_ME_TIME>
          The time at which the file was shared with the user, if applicable (RFC 3339 date-time).

      --sharing-user <JSON>
          Information about a Drive user.

      --shortcut-details <JSON>
          Information about a shortcut file.

      --shortcut-details-target-id <SHORTCUT_DETAILS_TARGET_ID>
          The ID of the file that this shortcut points to. Can only be set on `files.create` requests.

      --starred[=<BOOL>]
          Whether the user has starred the file.

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

      --team-drive-id <TEAM_DRIVE_ID>
          Deprecated: Output only. Use `driveId` instead.

      --trashed[=<BOOL>]
          Whether the file has been trashed, either explicitly or from a trashed parent folder. Only the owner may trash a file, but other users can still access the file in the owner's trash until it's permanently deleted.

      --trashed-time <TRASHED_TIME>
          The time that the item was trashed (RFC 3339 date-time). Only populated for items in shared drives.

      --trashing-user <JSON>
          Information about a Drive user.

      --use-content-as-indexable-text[=<BOOL>]
          Whether to use the uploaded content as indexable text.

      --viewed-by-me-time <VIEWED_BY_ME_TIME>
          The last time the file was viewed by the user (RFC 3339 date-time).

      --viewers-can-copy-content[=<BOOL>]
          Deprecated: Use `copyRequiresWriterPermission` instead.

      --writers-can-share[=<BOOL>]
          Whether users with only `writer` permission can modify the file's permissions. Not populated for items in shared drives.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## operations.get

```text
Gets the latest state of a long-running operation. Clients can use this method to poll the operation result at intervals as recommended by the API service.

Usage: joog [OPTIONS] drive <ACCOUNT> operations.get [COMMAND_OPTIONS]

Mandatory parameters:
      --name <NAME>
          The name of the operation resource.

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

## permissions.create

```text
Creates a permission for a file or shared drive. For more information, see [Share files, folders, and drives](https://developers.google.com/workspace/drive/api/guides/manage-sharing). **Warning:** Concurrent permission modifications (such as update or delete) on the same file, folder, or shared drive aren't supported across any users or clients; only the last update is applied.

Usage: joog [OPTIONS] drive <ACCOUNT> permissions.create [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file or shared drive.

      --role <ROLE>
          The role granted by this permission. Supported values include: * `owner` * `organizer` * `fileOrganizer` * `writer` * `commenter` * `reader` For more information, see [Roles and permissions](https://developers.google.com/workspace/drive/api/guides/ref-roles).

      --type <TYPE>
          The type of the grantee. Supported values include: * `user` * `group` * `domain` * `anyone` When creating a permission, if `type` is `user` or `group`, you must provide an `emailAddress` for the user or group. If `type` is `domain`, you must provide a `domain`. If `type` is `anyone`, no extra information is required.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --domain <DOMAIN>
          Domain name for a domain permission.

      --email-address <EMAIL_ADDRESS>
          Recipient email address for a user or group permission.

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

      --allow-file-discovery[=<BOOL>]
          Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type `domain` or `anyone`.

      --email-message <EMAIL_MESSAGE>
          A plain text custom message to include in the notification email.

      --enforce-expansive-access[=<BOOL>]
          Deprecated: All requests use the expansive access rules.

      --enforce-single-parent[=<BOOL>]
          Deprecated: See `moveToNewOwnersRoot` for details.

      --expiration-time <EXPIRATION_TIME>
          The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions: - They can only be set on user and group permissions - The time must be in the future - The time cannot be more than a year in the future

      --inherited-permissions-disabled[=<BOOL>]
          When `true`, only organizers, owners, and users with permissions added directly on the item can access it.

      --move-to-new-owners-root[=<BOOL>]
          This parameter only takes effect if the item isn't in a shared drive and the request is attempting to transfer the ownership of the item. If set to `true`, the item is moved to the new owner's My Drive root folder and all prior parents removed. If set to `false`, parents aren't changed.

      --pending-owner[=<BOOL>]
          Whether the account associated with this permission is a pending owner. Only populated for permissions of type `user` for files that aren't in a shared drive.

      --send-notification-email[=<BOOL>]
          Whether to send a notification email when sharing to users or groups. This defaults to `true` for users and groups, and is not allowed for other requests. It must not be disabled for ownership transfers.

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

      --transfer-ownership[=<BOOL>]
          Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect. For more information, see [Transfer file ownership](https://developers.google.com/workspace/drive/api/guides/transfer-file).

      --use-domain-admin-access[=<BOOL>]
          Issue the request as a domain administrator. If set to `true`, and if the following additional conditions are met, the requester is granted access: 1. The file ID parameter refers to a shared drive. 2. The requester is an administrator of the domain to which the shared drive belongs. For more information, see [Manage shared drives as domain administrators](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives#manage-administrators).

      --view <VIEW>
          Indicates the view for this permission. Only populated for permissions that belong to a view. The only supported values are `published` and `metadata`: * `published`: The permission's role is `publishedReader`. * `metadata`: The item is only visible to the `metadata` view because the item has limited access and the scope has at least read access to the parent. The `metadata` view is only supported on folders. For more information, see [Views](https://developers.google.com/workspace/drive/api/guides/ref-roles#views).

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## permissions.delete

```text
Deletes a permission. For more information, see [Share files, folders, and drives](https://developers.google.com/workspace/drive/api/guides/manage-sharing). **Warning:** Concurrent permission modifications (such as update or delete) on the same file, folder, or shared drive aren't supported across any users or clients; only the last update is applied.

Usage: joog [OPTIONS] drive <ACCOUNT> permissions.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file or shared drive.

      --permission-id <PERMISSION_ID>
          The ID of the permission.

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

      --enforce-expansive-access[=<BOOL>]
          Deprecated: All requests use the expansive access rules.

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

      --use-domain-admin-access[=<BOOL>]
          Issue the request as a domain administrator. If set to `true`, and if the following additional conditions are met, the requester is granted access: 1. The file ID parameter refers to a shared drive. 2. The requester is an administrator of the domain to which the shared drive belongs. For more information, see [Manage shared drives as domain administrators](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives#manage-administrators).

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## permissions.get

```text
Gets a permission by ID. For more information, see [Share files, folders, and drives](https://developers.google.com/workspace/drive/api/guides/manage-sharing).

Usage: joog [OPTIONS] drive <ACCOUNT> permissions.get [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file.

      --permission-id <PERMISSION_ID>
          The ID of the permission.

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

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

      --use-domain-admin-access[=<BOOL>]
          Issue the request as a domain administrator. If set to `true`, and if the following additional conditions are met, the requester is granted access: 1. The file ID parameter refers to a shared drive. 2. The requester is an administrator of the domain to which the shared drive belongs. For more information, see [Manage shared drives as domain administrators](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives#manage-administrators).

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## permissions.list

```text
Lists a file's or shared drive's permissions. For more information, see [Share files, folders, and drives](https://developers.google.com/workspace/drive/api/guides/manage-sharing).

Usage: joog [OPTIONS] drive <ACCOUNT> permissions.list [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file or shared drive.

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

      --include-permissions-for-view <INCLUDE_PERMISSIONS_FOR_VIEW>
          Specifies which additional view's permissions to include in the response. Only `published` is supported.

      --page-size <PAGE_SIZE>
          The maximum number of permissions to return. The service may return fewer than this value. If unspecified, at most 100 permissions will be returned for shared drives, and the entire list of permissions for non-shared drives. The maximum value is 100; values above 100 will be coerced to 100.

      --page-token <PAGE_TOKEN>
          The token for continuing a previous list request on the next page. This should be set to the value of `nextPageToken` from the previous response.

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

      --use-domain-admin-access[=<BOOL>]
          Issue the request as a domain administrator. If set to `true`, and if the following additional conditions are met, the requester is granted access: 1. The file ID parameter refers to a shared drive. 2. The requester is an administrator of the domain to which the shared drive belongs. For more information, see [Manage shared drives as domain administrators](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives#manage-administrators).

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## permissions.update

```text
Updates a permission with patch semantics. For more information, see [Share files, folders, and drives](https://developers.google.com/workspace/drive/api/guides/manage-sharing). **Warning:** Concurrent permission modifications (such as update or delete) on the same file, folder, or shared drive aren't supported across any users or clients; only the last update is applied.

Usage: joog [OPTIONS] drive <ACCOUNT> permissions.update [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file or shared drive.

      --permission-id <PERMISSION_ID>
          The ID of the permission.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --role <ROLE>
          The role granted by this permission. Supported values include: * `owner` * `organizer` * `fileOrganizer` * `writer` * `commenter` * `reader` For more information, see [Roles and permissions](https://developers.google.com/workspace/drive/api/guides/ref-roles).

      --type <TYPE>
          The type of the grantee. Supported values include: * `user` * `group` * `domain` * `anyone` When creating a permission, if `type` is `user` or `group`, you must provide an `emailAddress` for the user or group. If `type` is `domain`, you must provide a `domain`. If `type` is `anyone`, no extra information is required.

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

      --allow-file-discovery[=<BOOL>]
          Whether the permission allows the file to be discovered through search. This is only applicable for permissions of type `domain` or `anyone`.

      --enforce-expansive-access[=<BOOL>]
          Deprecated: All requests use the expansive access rules.

      --expiration-time <EXPIRATION_TIME>
          The time at which this permission will expire (RFC 3339 date-time). Expiration times have the following restrictions: - They can only be set on user and group permissions - The time must be in the future - The time cannot be more than a year in the future

      --inherited-permissions-disabled[=<BOOL>]
          When `true`, only organizers, owners, and users with permissions added directly on the item can access it.

      --pending-owner[=<BOOL>]
          Whether the account associated with this permission is a pending owner. Only populated for permissions of type `user` for files that aren't in a shared drive.

      --remove-expiration[=<BOOL>]
          Whether to remove the expiration date.

      --supports-all-drives[=<BOOL>]
          Whether the requesting application supports both My Drives and shared drives.

      --supports-team-drives[=<BOOL>]
          Deprecated: Use `supportsAllDrives` instead.

      --transfer-ownership[=<BOOL>]
          Whether to transfer ownership to the specified user and downgrade the current owner to a writer. This parameter is required as an acknowledgement of the side effect. For more information, see [Transfer file ownership](https://developers.google.com//workspace/drive/api/guides/transfer-file).

      --use-domain-admin-access[=<BOOL>]
          Issue the request as a domain administrator. If set to `true`, and if the following additional conditions are met, the requester is granted access: 1. The file ID parameter refers to a shared drive. 2. The requester is an administrator of the domain to which the shared drive belongs. For more information, see [Manage shared drives as domain administrators](https://developers.google.com/workspace/drive/api/guides/manage-shareddrives#manage-administrators).

      --view <VIEW>
          Indicates the view for this permission. Only populated for permissions that belong to a view. The only supported values are `published` and `metadata`: * `published`: The permission's role is `publishedReader`. * `metadata`: The item is only visible to the `metadata` view because the item has limited access and the scope has at least read access to the parent. The `metadata` view is only supported on folders. For more information, see [Views](https://developers.google.com/workspace/drive/api/guides/ref-roles#views).

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## replies.create

```text
Creates a reply to a comment. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments).

Usage: joog [OPTIONS] drive <ACCOUNT> replies.create [COMMAND_OPTIONS]

Mandatory parameters:
      --comment-id <COMMENT_ID>
          The ID of the comment.

      --file-id <FILE_ID>
          The ID of the file.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --content <CONTENT>
          The plain text content of the reply. This field is used for setting the content, while `htmlContent` should be displayed. This field is required by the `create` method if no `action` value is specified.

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

      --action <ACTION>
          The action the reply performed to the parent comment. The supported values are: * `resolve` * `reopen`

      --author <JSON>
          Information about a Drive user.

      --created-time <CREATED_TIME>
          The time at which the reply was created (RFC 3339 date-time).

      --modified-time <MODIFIED_TIME>
          The last time the reply was modified (RFC 3339 date-time).

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## replies.delete

```text
Deletes a reply. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments).

Usage: joog [OPTIONS] drive <ACCOUNT> replies.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --comment-id <COMMENT_ID>
          The ID of the comment.

      --file-id <FILE_ID>
          The ID of the file.

      --reply-id <REPLY_ID>
          The ID of the reply.

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

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## replies.get

```text
Gets a reply by ID. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments).

Usage: joog [OPTIONS] drive <ACCOUNT> replies.get [COMMAND_OPTIONS]

Mandatory parameters:
      --comment-id <COMMENT_ID>
          The ID of the comment.

      --file-id <FILE_ID>
          The ID of the file.

      --reply-id <REPLY_ID>
          The ID of the reply.

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

      --include-deleted[=<BOOL>]
          Whether to return deleted replies. Deleted replies don't include their original content.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## replies.list

```text
Lists a comment's replies. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments).

Usage: joog [OPTIONS] drive <ACCOUNT> replies.list [COMMAND_OPTIONS]

Mandatory parameters:
      --comment-id <COMMENT_ID>
          The ID of the comment.

      --file-id <FILE_ID>
          The ID of the file.

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

      --include-deleted[=<BOOL>]
          Whether to include deleted replies. Deleted replies don't include their original content.

      --page-size <PAGE_SIZE>
          The maximum number of replies to return. The service may return fewer than this value. If unspecified, at most 20 replies will be returned. The maximum value is 100; values above 100 will be coerced to 100.

      --page-token <PAGE_TOKEN>
          The token for continuing a previous list request on the next page. This should be set to the value of `nextPageToken` from the previous response.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## replies.update

```text
Updates a reply with patch semantics. For more information, see [Manage comments and replies](https://developers.google.com/workspace/drive/api/guides/manage-comments).

Usage: joog [OPTIONS] drive <ACCOUNT> replies.update [COMMAND_OPTIONS]

Mandatory parameters:
      --comment-id <COMMENT_ID>
          The ID of the comment.

      --content <CONTENT>
          The plain text content of the reply. This field is used for setting the content, while `htmlContent` should be displayed. This field is required by the `create` method if no `action` value is specified.

      --file-id <FILE_ID>
          The ID of the file.

      --reply-id <REPLY_ID>
          The ID of the reply.

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

      --action <ACTION>
          The action the reply performed to the parent comment. The supported values are: * `resolve` * `reopen`

      --author <JSON>
          Information about a Drive user.

      --created-time <CREATED_TIME>
          The time at which the reply was created (RFC 3339 date-time).

      --modified-time <MODIFIED_TIME>
          The last time the reply was modified (RFC 3339 date-time).

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## revisions.delete

```text
Permanently deletes a file version. You can only delete revisions for files with binary content in Google Drive, like images or videos. Revisions for other files, like Google Docs or Sheets, and the last remaining file version can't be deleted. For more information, see [Manage file revisions](https://developers.google.com/drive/api/guides/manage-revisions).

Usage: joog [OPTIONS] drive <ACCOUNT> revisions.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file.

      --revision-id <REVISION_ID>
          The ID of the revision.

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

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## revisions.get

```text
Gets a revision's metadata or content by ID. For more information, see [Manage file revisions](https://developers.google.com/workspace/drive/api/guides/manage-revisions).

Usage: joog [OPTIONS] drive <ACCOUNT> revisions.get [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file.

      --revision-id <REVISION_ID>
          The ID of the revision.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

  -o, --output <PATH>
          Save the downloaded content to a new file

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

      --acknowledge-abuse[=<BOOL>]
          Whether the user is acknowledging the risk of downloading known malware or other abusive files. This is only applicable when the `alt` parameter is set to `media` and the user is the owner of the file or an organizer of the shared drive in which the file resides.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## revisions.list

```text
Lists a file's revisions. For more information, see [Manage file revisions](https://developers.google.com/workspace/drive/api/guides/manage-revisions). **Important:** The list of revisions returned by this method might be incomplete for files with a large revision history, including frequently edited Google Docs, Sheets, and Slides. Older revisions might be omitted from the response, meaning the first revision returned may not be the oldest existing revision. The revision history visible in the Workspace editor user interface might be more complete than the list returned by the API.

Usage: joog [OPTIONS] drive <ACCOUNT> revisions.list [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file.

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
          The maximum number of revisions to return. The service may return fewer than this value. If unspecified, at most 200 revisions will be returned. The maximum value is 1000; values above 1000 will be coerced to 1000.

      --page-token <PAGE_TOKEN>
          The token for continuing a previous list request on the next page. This should be set to the value of 'nextPageToken' from the previous response.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## revisions.update

```text
Updates a revision with patch semantics. For more information, see [Manage file revisions](https://developers.google.com/workspace/drive/api/guides/manage-revisions).

Usage: joog [OPTIONS] drive <ACCOUNT> revisions.update [COMMAND_OPTIONS]

Mandatory parameters:
      --file-id <FILE_ID>
          The ID of the file.

      --revision-id <REVISION_ID>
          The ID of the revision.

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

      --keep-forever[=<BOOL>]
          Whether to keep this revision forever, even if it is no longer the head revision. If not set, the revision will be automatically purged 30 days after newer content is uploaded. This can be set on a maximum of 200 revisions for a file. This field is only applicable to files with binary content in Drive.

      --last-modifying-user <JSON>
          Information about a Drive user.

      --modified-time <MODIFIED_TIME>
          The last time the revision was modified (RFC 3339 date-time).

      --publish-auto[=<BOOL>]
          Whether subsequent revisions will be automatically republished. This is only applicable to Docs Editors files.

      --published[=<BOOL>]
          Whether this revision is published. This is only applicable to Docs Editors files.

      --published-outside-domain[=<BOOL>]
          Whether this revision is published outside the domain. This is only applicable to Docs Editors files.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).
