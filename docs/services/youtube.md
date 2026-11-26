# YouTube methods

See the [command guide](../grammar.md) and [request-file format](../file-schemas.md).

| Method | Description |
| --- | --- |
| `captions.delete` | Deletes a resource. |
| `captions.download` | Downloads a caption track. |
| `captions.insert` | Inserts a new resource into this collection. |
| `captions.list` | Retrieves a list of resources, possibly filtered. |
| `captions.update` | Updates an existing resource. |
| `channel-banners.insert` | Upload a channel banner image. |
| `channel-sections.delete` | Deletes a resource. |
| `channel-sections.insert` | Inserts a new resource into this collection. |
| `channel-sections.list` | Retrieves a list of resources, possibly filtered. |
| `channel-sections.update` | Updates an existing resource. |
| `channels.list` | Retrieves a list of resources, possibly filtered. |
| `channels.update` | Updates an existing resource. |
| `comment-threads.insert` | Inserts a new resource into this collection. |
| `comment-threads.list` | Retrieves a list of resources, possibly filtered. |
| `comments.delete` | Deletes a resource. |
| `comments.insert` | Inserts a new resource into this collection. |
| `comments.list` | Retrieves a list of resources, possibly filtered. |
| `comments.set-moderation-status` | Sets the moderation status of one or more comments. |
| `comments.update` | Updates an existing resource. |
| `playlist-images.delete` | Deletes a resource. |
| `playlist-images.insert` | Inserts a new resource into this collection. |
| `playlist-images.list` | Retrieves a list of resources, possibly filtered. |
| `playlist-images.update` | Updates an existing resource. |
| `playlist-items.delete` | Deletes a resource. |
| `playlist-items.insert` | Inserts a new resource into this collection. |
| `playlist-items.list` | Retrieves a list of resources, possibly filtered. |
| `playlist-items.update` | Updates an existing resource. |
| `playlists.delete` | Deletes a resource. |
| `playlists.insert` | Inserts a new resource into this collection. |
| `playlists.list` | Retrieves a list of resources, possibly filtered. |
| `playlists.update` | Updates an existing resource. |
| `search.list` | Retrieves a list of search resources |
| `subscriptions.delete` | Deletes a resource. |
| `subscriptions.insert` | Inserts a new resource into this collection. |
| `subscriptions.list` | Retrieves a list of resources, possibly filtered. |
| `super-chat-events.list` | Retrieves a list of resources, possibly filtered. |
| `thumbnails.set` | Set a video's thumbnail from a local image. |
| `video-categories.list` | Retrieves a list of resources, possibly filtered. |
| `video-trainability.get` | Returns the trainability status of a video. |
| `videos.batch-get-stats` | Retrieves a batch of VideoStat resources, possibly filtered |
| `videos.delete` | Deletes a resource. |
| `videos.get-rating` | Retrieves the ratings that the authorized user gave to a list of specified videos. |
| `videos.insert` | Inserts a new resource into this collection. |
| `videos.list` | Retrieves a list of resources, possibly filtered. |
| `videos.rate` | Adds a like or dislike rating to a video or removes a rating from a video. |
| `videos.update` | Updates an existing resource. |
| `watermarks.set` | Upload an image as the channel's video watermark. |
| `watermarks.unset` | Remove the channel's video watermark. |

## captions.delete

```text
Deletes a resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> captions.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          

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

      --on-behalf-of <ON_BEHALF_OF>
          ID of the Google+ Page for the channel that the request is be on behalf of

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## captions.download

```text
Downloads a caption track.

Usage: joog [OPTIONS] youtube <ACCOUNT> captions.download [COMMAND_OPTIONS]

Mandatory parameters:
  -o, --output <PATH>
          Save the downloaded content to a new file

      --id <ID>
          The ID of the caption track to download, required for One Platform.

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

      --on-behalf-of <ON_BEHALF_OF>
          ID of the Google+ Page for the channel that the request is be on behalf of

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --tfmt <TFMT>
          Convert the captions into this format. Supported options are sbv, srt, and vtt.

      --tlang <TLANG>
          tlang is the language code; machine translate the captions into this language.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## captions.insert

```text
Inserts a new resource into this collection.

Input choices:
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] youtube <ACCOUNT> captions.insert [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --upload <PATH>
          Upload a local file of at most 100 MiB.

      --verbose
          Include advanced parameters in help

      --part <PART>...
          The *part* parameter specifies the caption resource parts that the API response will include. Set the parameter value to snippet.
          
          [default: snippet]

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

      --etag <ETAG>
          Etag of this resource.

      --id <ID>
          The ID that YouTube uses to uniquely identify the caption track.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#caption".

      --on-behalf-of <ON_BEHALF_OF>
          ID of the Google+ Page for the channel that the request is be on behalf of

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --snippet <JSON>
          Basic details about a caption track, such as its language and name.

      --snippet-audio-track-type <SNIPPET_AUDIO_TRACK_TYPE>
          The type of audio track associated with the caption track. Choices: unknown, primary, commentary, descriptive.

      --snippet-failure-reason <SNIPPET_FAILURE_REASON>
          The reason that YouTube failed to process the caption track. This property is only present if the state property's value is failed. Choices: unknownFormat, unsupportedFormat, processingFailed.

      --snippet-is-auto-synced[=<BOOL>]
          Indicates whether YouTube synchronized the caption track to the audio track in the video. The value will be true if a sync was explicitly requested when the caption track was uploaded. For example, when calling the captions.insert or captions.update methods, you can set the sync parameter to true to instruct YouTube to sync the uploaded track to the video. If the value is false, YouTube uses the time codes in the uploaded caption track to determine when to display captions.

      --snippet-is-cc[=<BOOL>]
          Indicates whether the track contains closed captions for the deaf and hard of hearing. The default value is false.

      --snippet-is-draft[=<BOOL>]
          Indicates whether the caption track is a draft. If the value is true, then the track is not publicly visible. The default value is false. @mutable youtube.captions.insert youtube.captions.update

      --snippet-is-easy-reader[=<BOOL>]
          Indicates whether caption track is formatted for "easy reader," meaning it is at a third-grade level for language learners. The default value is false.

      --snippet-is-large[=<BOOL>]
          Indicates whether the caption track uses large text for the vision-impaired. The default value is false.

      --snippet-language <SNIPPET_LANGUAGE>
          The language of the caption track. The property value is a BCP-47 language tag.

      --snippet-last-updated <SNIPPET_LAST_UPDATED>
          The date and time when the caption track was last updated.

      --snippet-name <SNIPPET_NAME>
          The name of the caption track. The name is intended to be visible to the user as an option during playback.

      --snippet-status <SNIPPET_STATUS>
          The caption track's status. Choices: serving, syncing, failed.

      --snippet-track-kind <SNIPPET_TRACK_KIND>
          The caption track's type. Choices: standard, ASR, forced.

      --snippet-video-id <SNIPPET_VIDEO_ID>
          The ID that YouTube uses to uniquely identify the video associated with the caption track. @mutable youtube.captions.insert

      --sync[=<BOOL>]
          Extra parameter to allow automatically syncing the uploaded caption/transcript with the audio.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## captions.list

```text
Retrieves a list of resources, possibly filtered.

Usage: joog [OPTIONS] youtube <ACCOUNT> captions.list [COMMAND_OPTIONS]

Mandatory parameters:
      --video-id <VIDEO_ID>
          Returns the captions for the specified video.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --id <ID>...
          Returns the captions with the given IDs for Stubby or Apiary.

      --part <PART>...
          The *part* parameter specifies a comma-separated list of one or more caption resource parts that the API response will include. The part names that you can include in the parameter value are id and snippet.
          
          [default: snippet]

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

      --on-behalf-of <ON_BEHALF_OF>
          ID of the Google+ Page for the channel that the request is on behalf of.

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## captions.update

```text
Updates an existing resource.

Input choices:
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] youtube <ACCOUNT> captions.update [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID that YouTube uses to uniquely identify the caption track.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --upload <PATH>
          Upload a local file of at most 100 MiB.

      --verbose
          Include advanced parameters in help

      --part <PART>...
          The *part* parameter specifies a comma-separated list of one or more caption resource parts that the API response will include. The part names that you can include in the parameter value are id and snippet.
          
          [default: snippet]

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

      --etag <ETAG>
          Etag of this resource.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#caption".

      --on-behalf-of <ON_BEHALF_OF>
          ID of the Google+ Page for the channel that the request is on behalf of.

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --snippet <JSON>
          Basic details about a caption track, such as its language and name.

      --snippet-audio-track-type <SNIPPET_AUDIO_TRACK_TYPE>
          The type of audio track associated with the caption track. Choices: unknown, primary, commentary, descriptive.

      --snippet-failure-reason <SNIPPET_FAILURE_REASON>
          The reason that YouTube failed to process the caption track. This property is only present if the state property's value is failed. Choices: unknownFormat, unsupportedFormat, processingFailed.

      --snippet-is-auto-synced[=<BOOL>]
          Indicates whether YouTube synchronized the caption track to the audio track in the video. The value will be true if a sync was explicitly requested when the caption track was uploaded. For example, when calling the captions.insert or captions.update methods, you can set the sync parameter to true to instruct YouTube to sync the uploaded track to the video. If the value is false, YouTube uses the time codes in the uploaded caption track to determine when to display captions.

      --snippet-is-cc[=<BOOL>]
          Indicates whether the track contains closed captions for the deaf and hard of hearing. The default value is false.

      --snippet-is-draft[=<BOOL>]
          Indicates whether the caption track is a draft. If the value is true, then the track is not publicly visible. The default value is false. @mutable youtube.captions.insert youtube.captions.update

      --snippet-is-easy-reader[=<BOOL>]
          Indicates whether caption track is formatted for "easy reader," meaning it is at a third-grade level for language learners. The default value is false.

      --snippet-is-large[=<BOOL>]
          Indicates whether the caption track uses large text for the vision-impaired. The default value is false.

      --snippet-language <SNIPPET_LANGUAGE>
          The language of the caption track. The property value is a BCP-47 language tag.

      --snippet-last-updated <SNIPPET_LAST_UPDATED>
          The date and time when the caption track was last updated.

      --snippet-name <SNIPPET_NAME>
          The name of the caption track. The name is intended to be visible to the user as an option during playback.

      --snippet-status <SNIPPET_STATUS>
          The caption track's status. Choices: serving, syncing, failed.

      --snippet-track-kind <SNIPPET_TRACK_KIND>
          The caption track's type. Choices: standard, ASR, forced.

      --snippet-video-id <SNIPPET_VIDEO_ID>
          The ID that YouTube uses to uniquely identify the video associated with the caption track. @mutable youtube.captions.insert

      --sync[=<BOOL>]
          Extra parameter to allow automatically syncing the uploaded caption/transcript with the audio.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## channel-banners.insert

```text
Upload a channel banner image.

Input choices:
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] youtube <ACCOUNT> channel-banners.insert [COMMAND_OPTIONS]

Mandatory parameters:
      --upload <PATH>
          Upload a local file of at most 6 MiB.

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

      --content-type <MIME>
          Upload MIME type
          
          [default: application/octet-stream]

      --body <JSON>
          Inline JSON request body

      --channel-id <CHANNEL_ID>
          YouTube selects the signed-in account's channel; this parameter has no effect.

      --etag <ETAG>
          Request body field

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#channelBannerResource".

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --on-behalf-of-content-owner-channel <ON_BEHALF_OF_CONTENT_OWNER_CHANNEL>
          This parameter can only be used in a properly [authorized request](https://developers.google.com/youtube/data/guides/authentication). *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.

      --url <URL>
          The URL of this banner image.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## channel-sections.delete

```text
Deletes a resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> channel-sections.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          

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

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## channel-sections.insert

```text
Inserts a new resource into this collection.

Usage: joog [OPTIONS] youtube <ACCOUNT> channel-sections.insert [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --part <PART>...
          The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. The part names that you can include in the parameter value are snippet and contentDetails.
          
          [default: snippet]

      --title <TITLE>
          The channel section's title for multiple_playlists and multiple_channels.

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

      --content-details <JSON>
          Details about a channelsection, including playlists and channels.

      --content-details-channels <CONTENT_DETAILS_CHANNELS>...
          The channel ids for type multiple_channels.

      --content-details-playlists <CONTENT_DETAILS_PLAYLISTS>...
          The playlist ids for type single_playlist and multiple_playlists. For singlePlaylist, only one playlistId is allowed.

      --etag <ETAG>
          Etag of this resource.

      --id <ID>
          The ID that YouTube uses to uniquely identify the channel section.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#channelSection".

      --localizations <JSON>
          Localizations for different languages

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --on-behalf-of-content-owner-channel <ON_BEHALF_OF_CONTENT_OWNER_CHANNEL>
          This parameter can only be used in a properly [authorized request](https://developers.google.com/youtube/data/guides/authentication). *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.

      --snippet <JSON>
          Basic details about a channel section, including title, style and position.

      --snippet-channel-id <SNIPPET_CHANNEL_ID>
          The ID that YouTube uses to uniquely identify the channel that published the channel section.

      --snippet-default-language <SNIPPET_DEFAULT_LANGUAGE>
          The language of the channel section's default title and description.

      --snippet-localized-title <SNIPPET_LOCALIZED_TITLE>
          The localized strings for channel section's title.

      --snippet-position <SNIPPET_POSITION>
          The position of the channel section in the channel.

      --snippet-style <SNIPPET_STYLE>
          The style of the channel section. Choices: channelsectionStyleUnspecified, horizontalRow, verticalList.

      --snippet-type <SNIPPET_TYPE>
          The type of the channel section. Choices: channelsectionTypeUndefined, singlePlaylist, multiplePlaylists, popularUploads, recentUploads, likes, allPlaylists, likedPlaylists, recentPosts, recentActivity, liveEvents, upcomingEvents, completedEvents, multipleChannels, postedVideos, postedPlaylists, subscriptions.

      --targeting <JSON>
          ChannelSection targeting setting.

      --targeting-countries <TARGETING_COUNTRIES>...
          The country the channel section is targeting.

      --targeting-languages <TARGETING_LANGUAGES>...
          The language the channel section is targeting.

      --targeting-regions <TARGETING_REGIONS>...
          The region the channel section is targeting.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## channel-sections.list

```text
Retrieves a list of resources, possibly filtered.

Usage: joog [OPTIONS] youtube <ACCOUNT> channel-sections.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --channel-id <CHANNEL_ID>
          Return the ChannelSections owned by the specified channel ID.

      --id <ID>...
          Return the ChannelSections with the given IDs for Stubby or Apiary.

      --mine[=<BOOL>]
          Return the ChannelSections owned by the authenticated user.

      --part <PART>...
          The *part* parameter specifies a comma-separated list of one or more channelSection resource properties that the API response will include. The part names that you can include in the parameter value are id, snippet, and contentDetails. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channelSection resource, the snippet property contains other properties, such as a display title for the channelSection. If you set *part=snippet*, the API response will also contain all of those nested properties.
          
          [default: snippet]

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

      --hl <HL>
          Return content in specified language

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## channel-sections.update

```text
Updates an existing resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> channel-sections.update [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --part <PART>...
          The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. The part names that you can include in the parameter value are snippet and contentDetails.
          
          [default: snippet]

      --title <TITLE>
          The channel section's title for multiple_playlists and multiple_channels.

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

      --content-details <JSON>
          Details about a channelsection, including playlists and channels.

      --content-details-channels <CONTENT_DETAILS_CHANNELS>...
          The channel ids for type multiple_channels.

      --content-details-playlists <CONTENT_DETAILS_PLAYLISTS>...
          The playlist ids for type single_playlist and multiple_playlists. For singlePlaylist, only one playlistId is allowed.

      --etag <ETAG>
          Etag of this resource.

      --id <ID>
          The ID that YouTube uses to uniquely identify the channel section.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#channelSection".

      --localizations <JSON>
          Localizations for different languages

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --snippet <JSON>
          Basic details about a channel section, including title, style and position.

      --snippet-channel-id <SNIPPET_CHANNEL_ID>
          The ID that YouTube uses to uniquely identify the channel that published the channel section.

      --snippet-default-language <SNIPPET_DEFAULT_LANGUAGE>
          The language of the channel section's default title and description.

      --snippet-localized-title <SNIPPET_LOCALIZED_TITLE>
          The localized strings for channel section's title.

      --snippet-position <SNIPPET_POSITION>
          The position of the channel section in the channel.

      --snippet-style <SNIPPET_STYLE>
          The style of the channel section. Choices: channelsectionStyleUnspecified, horizontalRow, verticalList.

      --snippet-type <SNIPPET_TYPE>
          The type of the channel section. Choices: channelsectionTypeUndefined, singlePlaylist, multiplePlaylists, popularUploads, recentUploads, likes, allPlaylists, likedPlaylists, recentPosts, recentActivity, liveEvents, upcomingEvents, completedEvents, multipleChannels, postedVideos, postedPlaylists, subscriptions.

      --targeting <JSON>
          ChannelSection targeting setting.

      --targeting-countries <TARGETING_COUNTRIES>...
          The country the channel section is targeting.

      --targeting-languages <TARGETING_LANGUAGES>...
          The language the channel section is targeting.

      --targeting-regions <TARGETING_REGIONS>...
          The region the channel section is targeting.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## channels.list

```text
Retrieves a list of resources, possibly filtered.

Usage: joog [OPTIONS] youtube <ACCOUNT> channels.list [COMMAND_OPTIONS]

Mandatory parameters:
      --part <PART>...
          The *part* parameter specifies a comma-separated list of one or more channel resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a channel resource, the contentDetails property contains other properties, such as the uploads properties. As such, if you set *part=contentDetails*, the API response will also contain all of those nested properties.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --id <ID>...
          Return the channels with the specified IDs.

      --mine[=<BOOL>]
          Return the ids of channels owned by the authenticated user.

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

      --category-id <CATEGORY_ID>
          Return the channels within the specified guide category ID.

      --for-handle <FOR_HANDLE>
          Return the channel associated with a YouTube handle.

      --for-username <FOR_USERNAME>
          Return the channel associated with a YouTube username.

      --hl <HL>
          Stands for "host language". Specifies the localization language of the metadata to be filled into snippet.localized. The field is filled with the default metadata if there is no localization in the specified language. The parameter value must be a language code included in the list returned by the i18nLanguages.list method (e.g. en_US, es_MX).

      --managed-by-me[=<BOOL>]
          Return the channels managed by the authenticated user.

      --max-results <MAX_RESULTS>
          The *maxResults* parameter specifies the maximum number of items that should be returned in the result set.

      --my-subscribers[=<BOOL>]
          Return the channels subscribed to the authenticated user

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --page-token <PAGE_TOKEN>
          The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## channels.update

```text
Updates an existing resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> channels.update [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --description <DESCRIPTION>
          The description of the channel.

      --part <PART>...
          The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. The API currently only allows the parameter value to be set to either brandingSettings or invideoPromotion. (You cannot update both of those parts with a single request.) Note that this method overrides the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies.
          
          [default: snippet]

      --privacy <PRIVACY>
          Privacy status of the channel. Choices: public, unlisted, private.

      --status <JSON>
          JSON template for the status part of a channel.

      --title <TITLE>
          The channel's title.

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

      --audit-details <JSON>
          The auditDetails object encapsulates channel data that is relevant for YouTube Partners during the audit process.

      --audit-details-community-guidelines-good-standing[=<BOOL>]
          Whether or not the channel respects the community guidelines.

      --audit-details-content-id-claims-good-standing[=<BOOL>]
          Whether or not the channel has any unresolved claims.

      --audit-details-copyright-strikes-good-standing[=<BOOL>]
          Whether or not the channel has any copyright strikes.

      --branding-settings <JSON>
          Branding properties of a YouTube channel.

      --branding-settings-channel-country <BRANDING_SETTINGS_CHANNEL_COUNTRY>
          The country of the channel.

      --branding-settings-channel-default-language <BRANDING_SETTINGS_CHANNEL_DEFAULT_LANGUAGE>
          Request body field

      --branding-settings-channel-default-tab <BRANDING_SETTINGS_CHANNEL_DEFAULT_TAB>
          Which content tab users should see when viewing the channel.

      --branding-settings-channel-description <BRANDING_SETTINGS_CHANNEL_DESCRIPTION>
          Specifies the channel description.

      --branding-settings-channel-featured-channels-title <BRANDING_SETTINGS_CHANNEL_FEATURED_CHANNELS_TITLE>
          Title for the featured channels tab.

      --branding-settings-channel-featured-channels-urls <BRANDING_SETTINGS_CHANNEL_FEATURED_CHANNELS_URLS>...
          The list of featured channels.

      --branding-settings-channel-keywords <BRANDING_SETTINGS_CHANNEL_KEYWORDS>
          Lists keywords associated with the channel, comma-separated.

      --branding-settings-channel-moderate-comments[=<BOOL>]
          Whether user-submitted comments left on the channel page need to be approved by the channel owner to be publicly visible.

      --branding-settings-channel-profile-color <BRANDING_SETTINGS_CHANNEL_PROFILE_COLOR>
          A prominent color that can be rendered on this channel page.

      --branding-settings-channel-show-browse-view[=<BOOL>]
          Whether the tab to browse the videos should be displayed.

      --branding-settings-channel-show-related-channels[=<BOOL>]
          Whether related channels should be proposed.

      --branding-settings-channel-title <BRANDING_SETTINGS_CHANNEL_TITLE>
          Specifies the channel title.

      --branding-settings-channel-tracking-analytics-account-id <BRANDING_SETTINGS_CHANNEL_TRACKING_ANALYTICS_ACCOUNT_ID>
          The ID for a Google Analytics account to track and measure traffic to the channels.

      --branding-settings-channel-unsubscribed-trailer <BRANDING_SETTINGS_CHANNEL_UNSUBSCRIBED_TRAILER>
          The trailer of the channel, for users that are not subscribers.

      --branding-settings-hints <JSON>
          Additional experimental branding properties.

      --branding-settings-image-background-image-url-default <BRANDING_SETTINGS_IMAGE_BACKGROUND_IMAGE_URL_DEFAULT>
          Request body field

      --branding-settings-image-background-image-url-localized <JSON>
          Request body field

      --branding-settings-image-banner-external-url <BRANDING_SETTINGS_IMAGE_BANNER_EXTERNAL_URL>
          This is generated when a ChannelBanner.Insert request has succeeded for the given channel.

      --branding-settings-image-banner-image-url <BRANDING_SETTINGS_IMAGE_BANNER_IMAGE_URL>
          Banner image. Desktop size (1060x175).

      --branding-settings-image-banner-mobile-extra-hd-image-url <BRANDING_SETTINGS_IMAGE_BANNER_MOBILE_EXTRA_HD_IMAGE_URL>
          Banner image. Mobile size high resolution (1440x395).

      --branding-settings-image-banner-mobile-hd-image-url <BRANDING_SETTINGS_IMAGE_BANNER_MOBILE_HD_IMAGE_URL>
          Banner image. Mobile size high resolution (1280x360).

      --branding-settings-image-banner-mobile-image-url <BRANDING_SETTINGS_IMAGE_BANNER_MOBILE_IMAGE_URL>
          Banner image. Mobile size (640x175).

      --branding-settings-image-banner-mobile-low-image-url <BRANDING_SETTINGS_IMAGE_BANNER_MOBILE_LOW_IMAGE_URL>
          Banner image. Mobile size low resolution (320x88).

      --branding-settings-image-banner-mobile-medium-hd-image-url <BRANDING_SETTINGS_IMAGE_BANNER_MOBILE_MEDIUM_HD_IMAGE_URL>
          Banner image. Mobile size medium/high resolution (960x263).

      --branding-settings-image-banner-tablet-extra-hd-image-url <BRANDING_SETTINGS_IMAGE_BANNER_TABLET_EXTRA_HD_IMAGE_URL>
          Banner image. Tablet size extra high resolution (2560x424).

      --branding-settings-image-banner-tablet-hd-image-url <BRANDING_SETTINGS_IMAGE_BANNER_TABLET_HD_IMAGE_URL>
          Banner image. Tablet size high resolution (2276x377).

      --branding-settings-image-banner-tablet-image-url <BRANDING_SETTINGS_IMAGE_BANNER_TABLET_IMAGE_URL>
          Banner image. Tablet size (1707x283).

      --branding-settings-image-banner-tablet-low-image-url <BRANDING_SETTINGS_IMAGE_BANNER_TABLET_LOW_IMAGE_URL>
          Banner image. Tablet size low resolution (1138x188).

      --branding-settings-image-banner-tv-high-image-url <BRANDING_SETTINGS_IMAGE_BANNER_TV_HIGH_IMAGE_URL>
          Banner image. TV size high resolution (1920x1080).

      --branding-settings-image-banner-tv-image-url <BRANDING_SETTINGS_IMAGE_BANNER_TV_IMAGE_URL>
          Banner image. TV size extra high resolution (2120x1192).

      --branding-settings-image-banner-tv-low-image-url <BRANDING_SETTINGS_IMAGE_BANNER_TV_LOW_IMAGE_URL>
          Banner image. TV size low resolution (854x480).

      --branding-settings-image-banner-tv-medium-image-url <BRANDING_SETTINGS_IMAGE_BANNER_TV_MEDIUM_IMAGE_URL>
          Banner image. TV size medium resolution (1280x720).

      --branding-settings-image-large-branded-banner-image-imap-script-default <BRANDING_SETTINGS_IMAGE_LARGE_BRANDED_BANNER_IMAGE_IMAP_SCRIPT_DEFAULT>
          Request body field

      --branding-settings-image-large-branded-banner-image-imap-script-localized <JSON>
          Request body field

      --branding-settings-image-large-branded-banner-image-url-default <BRANDING_SETTINGS_IMAGE_LARGE_BRANDED_BANNER_IMAGE_URL_DEFAULT>
          Request body field

      --branding-settings-image-large-branded-banner-image-url-localized <JSON>
          Request body field

      --branding-settings-image-small-branded-banner-image-imap-script-default <BRANDING_SETTINGS_IMAGE_SMALL_BRANDED_BANNER_IMAGE_IMAP_SCRIPT_DEFAULT>
          Request body field

      --branding-settings-image-small-branded-banner-image-imap-script-localized <JSON>
          Request body field

      --branding-settings-image-small-branded-banner-image-url-default <BRANDING_SETTINGS_IMAGE_SMALL_BRANDED_BANNER_IMAGE_URL_DEFAULT>
          Request body field

      --branding-settings-image-small-branded-banner-image-url-localized <JSON>
          Request body field

      --branding-settings-image-tracking-image-url <BRANDING_SETTINGS_IMAGE_TRACKING_IMAGE_URL>
          The URL for a 1px by 1px tracking pixel that can be used to collect statistics for views of the channel or video pages.

      --branding-settings-image-watch-icon-image-url <BRANDING_SETTINGS_IMAGE_WATCH_ICON_IMAGE_URL>
          Request body field

      --branding-settings-watch-background-color <BRANDING_SETTINGS_WATCH_BACKGROUND_COLOR>
          The text color for the video watch page's branded area.

      --branding-settings-watch-featured-playlist-id <BRANDING_SETTINGS_WATCH_FEATURED_PLAYLIST_ID>
          An ID that uniquely identifies a playlist that displays next to the video player.

      --branding-settings-watch-text-color <BRANDING_SETTINGS_WATCH_TEXT_COLOR>
          The background color for the video watch page's branded area.

      --content-details <JSON>
          Details about the content of a channel.

      --content-details-related-playlists-favorites <CONTENT_DETAILS_RELATED_PLAYLISTS_FAVORITES>
          The ID of the playlist that contains the channel"s favorite videos. Use the [`playlistItems.insert`](https://developers.google.com/youtube/data/reference/rest/v3/playlistItems/insert) and [`playlistItems.delete`](https://developers.google.com/youtube/data/reference/rest/v3/playlistItems/delete) to add or remove items from that list.

      --content-details-related-playlists-likes <CONTENT_DETAILS_RELATED_PLAYLISTS_LIKES>
          The ID of the playlist that contains the channel"s liked videos. Use the [`playlistItems.insert`](https://developers.google.com/youtube/data/reference/rest/v3/playlistItems/insert) and [`playlistItems.delete`](https://developers.google.com/youtube/data/reference/rest/v3/playlistItems/delete) to add or remove items from that list.

      --content-details-related-playlists-uploads <CONTENT_DETAILS_RELATED_PLAYLISTS_UPLOADS>
          The ID of the playlist that contains the channel"s uploaded videos. Use the [`videos.insert`](https://developers.google.com/youtube/data/reference/rest/v3/videos/insert) method to upload new videos and the [`videos.delete`](https://developers.google.com/youtube/data/reference/rest/v3/videos/delete) method to delete previously uploaded videos.

      --content-details-related-playlists-watch-history <CONTENT_DETAILS_RELATED_PLAYLISTS_WATCH_HISTORY>
          The ID of the playlist that contains the channel"s watch history. Use the [`playlistItems.insert`](https://developers.google.com/youtube/data/reference/rest/v3/playlistItems/insert) and [`playlistItems.delete`](https://developers.google.com/youtube/data/reference/rest/v3/playlistItems/delete) to add or remove items from that list.

      --content-details-related-playlists-watch-later <CONTENT_DETAILS_RELATED_PLAYLISTS_WATCH_LATER>
          The ID of the playlist that contains the channel"s watch later playlist. Use the [`playlistItems.insert`](https://developers.google.com/youtube/data/reference/rest/v3/playlistItems/insert) and [`playlistItems.delete`](https://developers.google.com/youtube/data/reference/rest/v3/playlistItems/delete) to add or remove items from that list.

      --content-owner-details <JSON>
          The contentOwnerDetails object encapsulates channel data that is relevant for YouTube Partners linked with the channel.

      --content-owner-details-content-owner <CONTENT_OWNER_DETAILS_CONTENT_OWNER>
          The ID of the content owner linked to the channel.

      --content-owner-details-time-linked <CONTENT_OWNER_DETAILS_TIME_LINKED>
          The date and time when the channel was linked to the content owner.

      --conversion-pings <JSON>
          The conversionPings object encapsulates information about conversion pings that need to be respected by the channel.

      --conversion-pings-pings <JSON>
          Pings that the app shall fire (authenticated by biscotti cookie). Each ping has a context, in which the app must fire the ping, and a url identifying the ping.

      --etag <ETAG>
          Etag of this resource.

      --id <ID>
          The ID that YouTube uses to uniquely identify the channel.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#channel".

      --localizations <JSON>
          Localizations for different languages

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          The *onBehalfOfContentOwner* parameter indicates that the authenticated user is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with needs to be linked to the specified YouTube content owner. This parameter must be provided if the request is authenticated with credentials for a CMS content owner user acting on a managed channel. If omitted, the request executes under the authenticated user's direct context and returns an HTTP 403 Forbidden error.

      --snippet <JSON>
          Basic details about a channel, including title, description and thumbnails.

      --snippet-country <SNIPPET_COUNTRY>
          The country of the channel.

      --snippet-custom-url <SNIPPET_CUSTOM_URL>
          The custom url of the channel.

      --snippet-default-language <SNIPPET_DEFAULT_LANGUAGE>
          The language of the channel's default title and description.

      --snippet-localized-description <SNIPPET_LOCALIZED_DESCRIPTION>
          The localized strings for channel's description.

      --snippet-localized-title <SNIPPET_LOCALIZED_TITLE>
          The localized strings for channel's title.

      --snippet-published-at <SNIPPET_PUBLISHED_AT>
          The date and time that the channel was created.

      --snippet-thumbnails-default-height <SNIPPET_THUMBNAILS_DEFAULT_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-default-url <SNIPPET_THUMBNAILS_DEFAULT_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-default-width <SNIPPET_THUMBNAILS_DEFAULT_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-fhd-height <SNIPPET_THUMBNAILS_FHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-fhd-url <SNIPPET_THUMBNAILS_FHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-fhd-width <SNIPPET_THUMBNAILS_FHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-high-height <SNIPPET_THUMBNAILS_HIGH_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-high-url <SNIPPET_THUMBNAILS_HIGH_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-high-width <SNIPPET_THUMBNAILS_HIGH_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-maxres-height <SNIPPET_THUMBNAILS_MAXRES_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-maxres-url <SNIPPET_THUMBNAILS_MAXRES_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-maxres-width <SNIPPET_THUMBNAILS_MAXRES_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-medium-height <SNIPPET_THUMBNAILS_MEDIUM_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-medium-url <SNIPPET_THUMBNAILS_MEDIUM_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-medium-width <SNIPPET_THUMBNAILS_MEDIUM_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-qhd-height <SNIPPET_THUMBNAILS_QHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-qhd-url <SNIPPET_THUMBNAILS_QHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-qhd-width <SNIPPET_THUMBNAILS_QHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-height <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-url <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-shorts-aspect-ratio-width <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-standard-height <SNIPPET_THUMBNAILS_STANDARD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-standard-url <SNIPPET_THUMBNAILS_STANDARD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-standard-width <SNIPPET_THUMBNAILS_STANDARD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-uhd-height <SNIPPET_THUMBNAILS_UHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-uhd-url <SNIPPET_THUMBNAILS_UHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-uhd-width <SNIPPET_THUMBNAILS_UHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --statistics <JSON>
          Statistics about a channel: number of subscribers, number of videos in the channel, etc.

      --statistics-comment-count <STATISTICS_COMMENT_COUNT>
          The number of comments for the channel.

      --statistics-hidden-subscriber-count[=<BOOL>]
          Whether or not the number of subscribers is shown for this user.

      --statistics-subscriber-count <STATISTICS_SUBSCRIBER_COUNT>
          The number of subscribers that the channel has.

      --statistics-video-count <STATISTICS_VIDEO_COUNT>
          The number of videos uploaded to the channel.

      --statistics-view-count <STATISTICS_VIEW_COUNT>
          The sum of the number of times all the videos in all formats have been viewed for a channel. For all video formats (long-form, Live, and Shorts) on a channel, `viewCount` counts views the moment a video begins to play (includes autoplay, hold the pointer over, and click/tap to play). For all video formats (long-form, Live, and Shorts) on a channel, `viewCount` includes co-views when multiple people watch together on a television screen.

      --status-is-channel-monetization-enabled[=<BOOL>]
          Whether the channel is considered ypp monetization enabled. See go/yppornot for more details.

      --status-is-linked[=<BOOL>]
          If true, then the user is linked to either a YouTube username or G+ account. Otherwise, the user doesn't have a public YouTube identity.

      --status-long-uploads-status <STATUS_LONG_UPLOADS_STATUS>
          The long uploads status of this channel. See https://support.google.com/youtube/answer/71673 for more information. Choices: longUploadsUnspecified, allowed, eligible, disallowed.

      --status-made-for-kids[=<BOOL>]
          Request body field

      --status-self-declared-made-for-kids[=<BOOL>]
          Request body field

      --topic-details <JSON>
          Freebase topic information related to the channel.

      --topic-details-topic-categories <TOPIC_DETAILS_TOPIC_CATEGORIES>...
          A list of Wikipedia URLs that describe the channel's content.

      --topic-details-topic-ids <TOPIC_DETAILS_TOPIC_IDS>...
          A list of Freebase topic IDs associated with the channel. You can retrieve information about each topic using the Freebase Topic API.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## comment-threads.insert

```text
Inserts a new resource into this collection.

Usage: joog [OPTIONS] youtube <ACCOUNT> comment-threads.insert [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --part <PART>...
          The *part* parameter identifies the properties that the API response will include. Set the parameter value to snippet. The snippet part has a quota cost of 2 units.
          
          [default: snippet]

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

      --etag <ETAG>
          Etag of this resource.

      --id <ID>
          The ID that YouTube uses to uniquely identify the comment thread.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#commentThread".

      --replies <JSON>
          Comments written in (direct or indirect) reply to the top level comment.

      --replies-comments <JSON>
          A limited number of replies. Unless the number of replies returned equals total_reply_count in the snippet the returned replies are only a subset of the total number of replies.

      --snippet <JSON>
          Basic details about a comment thread.

      --snippet-can-reply[=<BOOL>]
          Whether the current viewer of the thread can reply to it. This is viewer specific - other viewers may see a different value for this field.

      --snippet-channel-id <SNIPPET_CHANNEL_ID>
          The YouTube channel the comments in the thread refer to or the channel with the video the comments refer to. If neither video_id nor post_id is set the comments refer to the channel itself.

      --snippet-is-public[=<BOOL>]
          Whether the thread (and therefore all its comments) is visible to all YouTube users.

      --snippet-top-level-comment-etag <SNIPPET_TOP_LEVEL_COMMENT_ETAG>
          Etag of this resource.

      --snippet-top-level-comment-id <SNIPPET_TOP_LEVEL_COMMENT_ID>
          The ID that YouTube uses to uniquely identify the comment.

      --snippet-top-level-comment-kind <SNIPPET_TOP_LEVEL_COMMENT_KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#comment".

      --snippet-top-level-comment-snippet-author-channel-url <SNIPPET_TOP_LEVEL_COMMENT_SNIPPET_AUTHOR_CHANNEL_URL>
          Link to the author's YouTube channel, if any.

      --snippet-top-level-comment-snippet-author-display-name <SNIPPET_TOP_LEVEL_COMMENT_SNIPPET_AUTHOR_DISPLAY_NAME>
          The name of the user who posted the comment.

      --snippet-top-level-comment-snippet-author-profile-image-url <SNIPPET_TOP_LEVEL_COMMENT_SNIPPET_AUTHOR_PROFILE_IMAGE_URL>
          The URL for the avatar of the user who posted the comment.

      --snippet-top-level-comment-snippet-can-rate[=<BOOL>]
          Whether the current viewer can rate this comment.

      --snippet-top-level-comment-snippet-channel-id <SNIPPET_TOP_LEVEL_COMMENT_SNIPPET_CHANNEL_ID>
          The id of the corresponding YouTube channel. In case of a channel comment this is the channel the comment refers to. In case of a video or post comment it's the video/post's channel.

      --snippet-top-level-comment-snippet-like-count <SNIPPET_TOP_LEVEL_COMMENT_SNIPPET_LIKE_COUNT>
          The total number of likes this comment has received.

      --snippet-top-level-comment-snippet-moderation-status <SNIPPET_TOP_LEVEL_COMMENT_SNIPPET_MODERATION_STATUS>
          The comment's moderation status. Will not be set if the comments were requested through the id filter. Choices: published, heldForReview, likelySpam, rejected.

      --snippet-top-level-comment-snippet-parent-id <SNIPPET_TOP_LEVEL_COMMENT_SNIPPET_PARENT_ID>
          The unique id of the top-level comment, only set for replies.

      --snippet-top-level-comment-snippet-published-at <SNIPPET_TOP_LEVEL_COMMENT_SNIPPET_PUBLISHED_AT>
          The date and time when the comment was originally published.

      --snippet-top-level-comment-snippet-text-display <SNIPPET_TOP_LEVEL_COMMENT_SNIPPET_TEXT_DISPLAY>
          The comment's text. The format is either plain text or HTML dependent on what has been requested. Even the plain text representation may differ from the text originally posted in that it may replace video links with video titles etc.

      --snippet-top-level-comment-snippet-text-original <SNIPPET_TOP_LEVEL_COMMENT_SNIPPET_TEXT_ORIGINAL>
          The comment's original raw text as initially posted or last updated. The original text will only be returned if it is accessible to the viewer, which is only guaranteed if the viewer is the comment's author.

      --snippet-top-level-comment-snippet-updated-at <SNIPPET_TOP_LEVEL_COMMENT_SNIPPET_UPDATED_AT>
          The date and time when the comment was last updated.

      --snippet-top-level-comment-snippet-video-id <SNIPPET_TOP_LEVEL_COMMENT_SNIPPET_VIDEO_ID>
          The ID of the video the comment refers to, if any.

      --snippet-top-level-comment-snippet-viewer-rating <SNIPPET_TOP_LEVEL_COMMENT_SNIPPET_VIEWER_RATING>
          The rating the viewer has given to this comment. For the time being this will never return RATE_TYPE_DISLIKE and instead return RATE_TYPE_NONE. This may change in the future. Choices: none, like, dislike.

      --snippet-total-reply-count <SNIPPET_TOTAL_REPLY_COUNT>
          The total number of replies (not including the top level comment).

      --snippet-video-id <SNIPPET_VIDEO_ID>
          The ID of the video the comments refer to, if any.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## comment-threads.list

```text
Retrieves a list of resources, possibly filtered.

Usage: joog [OPTIONS] youtube <ACCOUNT> comment-threads.list [COMMAND_OPTIONS]

Mandatory parameters:
      --part <PART>...
          The *part* parameter specifies a comma-separated list of one or more commentThread resource properties that the API response will include.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --channel-id <CHANNEL_ID>
          Returns the comment threads for all the channel comments (ie does not include comments left on videos).

      --id <ID>...
          Returns the comment threads with the given IDs for Stubby or Apiary.

      --order <ORDER>
           Choices: orderUnspecified, time, relevance.

      --video-id <VIDEO_ID>
          Returns the comment threads of the specified video.

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

      --all-threads-related-to-channel-id <ALL_THREADS_RELATED_TO_CHANNEL_ID>
          Returns the comment threads of all videos of the channel and the channel comments as well.

      --max-results <MAX_RESULTS>
          The *maxResults* parameter specifies the maximum number of items that should be returned in the result set.

      --moderation-status <MODERATION_STATUS>
          Limits the returned comment threads to those with the specified moderation status. Not compatible with the 'id' filter. Valid values: published, heldForReview, likelySpam. Choices: published, heldForReview, likelySpam, rejected.

      --page-token <PAGE_TOKEN>
          The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.

      --search-terms <SEARCH_TERMS>
          Limits the returned comment threads to those matching the specified key words. Not compatible with the 'id' filter.

      --text-format <TEXT_FORMAT>
          The requested text format for the returned comments. Choices: textFormatUnspecified, html, plainText.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## comments.delete

```text
Deletes a resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> comments.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          

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

## comments.insert

```text
Inserts a new resource into this collection.

Usage: joog [OPTIONS] youtube <ACCOUNT> comments.insert [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --part <PART>...
          The *part* parameter identifies the properties that the API response will include. Set the parameter value to snippet. The snippet part has a quota cost of 2 units.
          
          [default: snippet]

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

      --etag <ETAG>
          Etag of this resource.

      --id <ID>
          The ID that YouTube uses to uniquely identify the comment.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#comment".

      --snippet <JSON>
          Basic details about a comment, such as its author and text.

      --snippet-author-channel-id-value <SNIPPET_AUTHOR_CHANNEL_ID_VALUE>
          The id of the author's YouTube channel.

      --snippet-author-channel-url <SNIPPET_AUTHOR_CHANNEL_URL>
          Link to the author's YouTube channel, if any.

      --snippet-author-display-name <SNIPPET_AUTHOR_DISPLAY_NAME>
          The name of the user who posted the comment.

      --snippet-author-profile-image-url <SNIPPET_AUTHOR_PROFILE_IMAGE_URL>
          The URL for the avatar of the user who posted the comment.

      --snippet-can-rate[=<BOOL>]
          Whether the current viewer can rate this comment.

      --snippet-channel-id <SNIPPET_CHANNEL_ID>
          The id of the corresponding YouTube channel. In case of a channel comment this is the channel the comment refers to. In case of a video or post comment it's the video/post's channel.

      --snippet-like-count <SNIPPET_LIKE_COUNT>
          The total number of likes this comment has received.

      --snippet-moderation-status <SNIPPET_MODERATION_STATUS>
          The comment's moderation status. Will not be set if the comments were requested through the id filter. Choices: published, heldForReview, likelySpam, rejected.

      --snippet-parent-id <SNIPPET_PARENT_ID>
          The unique id of the top-level comment, only set for replies.

      --snippet-published-at <SNIPPET_PUBLISHED_AT>
          The date and time when the comment was originally published.

      --snippet-text-display <SNIPPET_TEXT_DISPLAY>
          The comment's text. The format is either plain text or HTML dependent on what has been requested. Even the plain text representation may differ from the text originally posted in that it may replace video links with video titles etc.

      --snippet-text-original <SNIPPET_TEXT_ORIGINAL>
          The comment's original raw text as initially posted or last updated. The original text will only be returned if it is accessible to the viewer, which is only guaranteed if the viewer is the comment's author.

      --snippet-updated-at <SNIPPET_UPDATED_AT>
          The date and time when the comment was last updated.

      --snippet-video-id <SNIPPET_VIDEO_ID>
          The ID of the video the comment refers to, if any.

      --snippet-viewer-rating <SNIPPET_VIEWER_RATING>
          The rating the viewer has given to this comment. For the time being this will never return RATE_TYPE_DISLIKE and instead return RATE_TYPE_NONE. This may change in the future. Choices: none, like, dislike.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## comments.list

```text
Retrieves a list of resources, possibly filtered.

Input choices:
  When selecting records with --id, omit --max-results. Use --max to limit the returned records.
  When selecting records with --id, omit --page-token. Use --max to limit the returned records.
  When selecting records with --id, omit --max-pages. Use --max to limit the returned records.

Usage: joog [OPTIONS] youtube <ACCOUNT> comments.list [COMMAND_OPTIONS]

Mandatory parameters:
      --part <PART>...
          The *part* parameter specifies a comma-separated list of one or more comment resource properties that the API response will include.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --id <ID>...
          Returns the comments with the given IDs for One Platform.

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
          The *maxResults* parameter specifies the maximum number of items that should be returned in the result set.

      --page-token <PAGE_TOKEN>
          The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.

      --parent-id <PARENT_ID>
          Returns replies to the specified comment. Note, currently YouTube features only one level of replies (ie replies to top level comments). However replies to replies may be supported in the future.

      --text-format <TEXT_FORMAT>
          The requested text format for the returned comments. Choices: textFormatUnspecified, html, plainText.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## comments.set-moderation-status

```text
Sets the moderation status of one or more comments.

Usage: joog [OPTIONS] youtube <ACCOUNT> comments.set-moderation-status [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>...
          Modifies the moderation status of the comments with the given IDs

      --moderation-status <MODERATION_STATUS>
          Specifies the requested moderation status. Note, comments can be in statuses, which are not available through this call. For example, this call does not allow to mark a comment as 'likely spam'. Valid values: 'heldForReview', 'published' or 'rejected'. Choices: published, heldForReview, likelySpam, rejected.

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

      --ban-author[=<BOOL>]
          If set to true the author of the comment gets added to the ban list. This means all future comments of the author will autmomatically be rejected. Only valid in combination with STATUS_REJECTED.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## comments.update

```text
Updates an existing resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> comments.update [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --part <PART>...
          The *part* parameter identifies the properties that the API response will include. You must at least include the snippet part in the parameter value since that part contains all of the properties that the API request can update.
          
          [default: snippet]

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

      --etag <ETAG>
          Etag of this resource.

      --id <ID>
          The ID that YouTube uses to uniquely identify the comment.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#comment".

      --snippet <JSON>
          Basic details about a comment, such as its author and text.

      --snippet-author-channel-id-value <SNIPPET_AUTHOR_CHANNEL_ID_VALUE>
          The id of the author's YouTube channel.

      --snippet-author-channel-url <SNIPPET_AUTHOR_CHANNEL_URL>
          Link to the author's YouTube channel, if any.

      --snippet-author-display-name <SNIPPET_AUTHOR_DISPLAY_NAME>
          The name of the user who posted the comment.

      --snippet-author-profile-image-url <SNIPPET_AUTHOR_PROFILE_IMAGE_URL>
          The URL for the avatar of the user who posted the comment.

      --snippet-can-rate[=<BOOL>]
          Whether the current viewer can rate this comment.

      --snippet-channel-id <SNIPPET_CHANNEL_ID>
          The id of the corresponding YouTube channel. In case of a channel comment this is the channel the comment refers to. In case of a video or post comment it's the video/post's channel.

      --snippet-like-count <SNIPPET_LIKE_COUNT>
          The total number of likes this comment has received.

      --snippet-moderation-status <SNIPPET_MODERATION_STATUS>
          The comment's moderation status. Will not be set if the comments were requested through the id filter. Choices: published, heldForReview, likelySpam, rejected.

      --snippet-parent-id <SNIPPET_PARENT_ID>
          The unique id of the top-level comment, only set for replies.

      --snippet-published-at <SNIPPET_PUBLISHED_AT>
          The date and time when the comment was originally published.

      --snippet-text-display <SNIPPET_TEXT_DISPLAY>
          The comment's text. The format is either plain text or HTML dependent on what has been requested. Even the plain text representation may differ from the text originally posted in that it may replace video links with video titles etc.

      --snippet-text-original <SNIPPET_TEXT_ORIGINAL>
          The comment's original raw text as initially posted or last updated. The original text will only be returned if it is accessible to the viewer, which is only guaranteed if the viewer is the comment's author.

      --snippet-updated-at <SNIPPET_UPDATED_AT>
          The date and time when the comment was last updated.

      --snippet-video-id <SNIPPET_VIDEO_ID>
          The ID of the video the comment refers to, if any.

      --snippet-viewer-rating <SNIPPET_VIEWER_RATING>
          The rating the viewer has given to this comment. For the time being this will never return RATE_TYPE_DISLIKE and instead return RATE_TYPE_NONE. This may change in the future. Choices: none, like, dislike.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## playlist-images.delete

```text
Deletes a resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> playlist-images.delete [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --id <ID>
          Id to identify this image. This is returned from by the List method.

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

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## playlist-images.insert

```text
Inserts a new resource into this collection.

Input choices:
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] youtube <ACCOUNT> playlist-images.insert [COMMAND_OPTIONS]

Mandatory parameters:
      --upload <PATH>
          Upload a local file of at most 50 MiB.

      --snippet-playlist-id <SNIPPET_PLAYLIST_ID>
          The Playlist ID of the playlist this image is associated with.

      --snippet-type <SNIPPET_TYPE>
          The image type. Choices: hero.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --part <PART>...
          The *part* parameter specifies the properties that the API response will include.

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

      --id <ID>
          Identifies this resource (playlist id and image type).

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#playlistImages".

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --on-behalf-of-content-owner-channel <ON_BEHALF_OF_CONTENT_OWNER_CHANNEL>
          This parameter can only be used in a properly [authorized request](https://developers.google.com/youtube/data/guides/authentication). *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.

      --snippet <JSON>
          Playlist image metadata: the playlist, image type, and dimensions.

      --snippet-height <SNIPPET_HEIGHT>
          The image height.

      --snippet-width <SNIPPET_WIDTH>
          The image width.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## playlist-images.list

```text
Retrieves a list of resources, possibly filtered.

Usage: joog [OPTIONS] youtube <ACCOUNT> playlist-images.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --part <PART>...
          The *part* parameter specifies a comma-separated list of one or more playlistImage resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response.

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
          The *maxResults* parameter specifies the maximum number of items that should be returned in the result set.

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --on-behalf-of-content-owner-channel <ON_BEHALF_OF_CONTENT_OWNER_CHANNEL>
          This parameter can only be used in a properly [authorized request](https://developers.google.com/youtube/data/guides/authentication). *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.

      --page-token <PAGE_TOKEN>
          The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.

      --parent <PARENT>
          Return PlaylistImages for this playlist id.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## playlist-images.update

```text
Updates an existing resource.

Input choices:
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] youtube <ACCOUNT> playlist-images.update [COMMAND_OPTIONS]

Mandatory parameters:
      --upload <PATH>
          Upload a local file of at most 50 MiB.

      --id <ID>
          Identifies this resource (playlist id and image type).

      --snippet-playlist-id <SNIPPET_PLAYLIST_ID>
          The Playlist ID of the playlist this image is associated with.

      --snippet-type <SNIPPET_TYPE>
          The image type. Choices: hero.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --part <PART>...
          The *part* parameter specifies the properties that the API response will include.

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

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#playlistImages".

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --snippet <JSON>
          Playlist image metadata: the playlist, image type, and dimensions.

      --snippet-height <SNIPPET_HEIGHT>
          The image height.

      --snippet-width <SNIPPET_WIDTH>
          The image width.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## playlist-items.delete

```text
Deletes a resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> playlist-items.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          

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

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## playlist-items.insert

```text
Inserts a new resource into this collection.

Usage: joog [OPTIONS] youtube <ACCOUNT> playlist-items.insert [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --channel-id <CHANNEL_ID>
          The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.

      --verbose
          Include advanced parameters in help

      --description <DESCRIPTION>
          The item's description.

      --part <PART>...
          The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
          
          [default: snippet]

      --privacy <PRIVACY>
          This resource's privacy status. Choices: public, unlisted, private.

      --status <JSON>
          Information about the playlist item's privacy status.

      --title <TITLE>
          The item's title.

      --video-id <VIDEO_ID>
          The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.

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

      --content-details <JSON>
          Request body field

      --content-details-end-at <CONTENT_DETAILS_END_AT>
          The time, measured in seconds from the start of the video, when the video should stop playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) By default, assume that the video.endTime is the end of the video.

      --content-details-note <CONTENT_DETAILS_NOTE>
          A user-generated note for this item.

      --content-details-start-at <CONTENT_DETAILS_START_AT>
          The time, measured in seconds from the start of the video, when the video should start playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) The default value is 0.

      --content-details-video-id <CONTENT_DETAILS_VIDEO_ID>
          The ID that YouTube uses to uniquely identify a video. To [retrieve the `video` resource](https://developers.google.com/youtube/data/reference/rest/v3/videos/list), set the id query parameter to this value in your API request.

      --content-details-video-published-at <CONTENT_DETAILS_VIDEO_PUBLISHED_AT>
          The date and time that the video was published to YouTube.

      --etag <ETAG>
          Etag of this resource.

      --id <ID>
          The ID that YouTube uses to uniquely identify the playlist item.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#playlistItem".

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --snippet <JSON>
          Basic details about a playlist, including title, description and thumbnails. Basic details of a YouTube Playlist item provided by the author. Next ID: 15

      --snippet-channel-id <SNIPPET_CHANNEL_ID>
          The ID that YouTube uses to uniquely identify the user that added the item to the playlist.

      --snippet-channel-title <SNIPPET_CHANNEL_TITLE>
          Channel title for the channel that the playlist item belongs to.

      --snippet-playlist-id <SNIPPET_PLAYLIST_ID>
          The ID that YouTube uses to uniquely identify thGe playlist that the playlist item is in.

      --snippet-position <SNIPPET_POSITION>
          The order in which the item appears in the playlist. The value uses a zero-based index, so the first item has a position of 0, the second item has a position of 1, and so forth.

      --snippet-published-at <SNIPPET_PUBLISHED_AT>
          The date and time that the item was added to the playlist.

      --snippet-resource-id-kind <SNIPPET_RESOURCE_ID_KIND>
          The type of the API resource.

      --snippet-resource-id-playlist-id <SNIPPET_RESOURCE_ID_PLAYLIST_ID>
          The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.

      --snippet-thumbnails-default-height <SNIPPET_THUMBNAILS_DEFAULT_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-default-url <SNIPPET_THUMBNAILS_DEFAULT_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-default-width <SNIPPET_THUMBNAILS_DEFAULT_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-fhd-height <SNIPPET_THUMBNAILS_FHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-fhd-url <SNIPPET_THUMBNAILS_FHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-fhd-width <SNIPPET_THUMBNAILS_FHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-high-height <SNIPPET_THUMBNAILS_HIGH_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-high-url <SNIPPET_THUMBNAILS_HIGH_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-high-width <SNIPPET_THUMBNAILS_HIGH_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-maxres-height <SNIPPET_THUMBNAILS_MAXRES_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-maxres-url <SNIPPET_THUMBNAILS_MAXRES_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-maxres-width <SNIPPET_THUMBNAILS_MAXRES_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-medium-height <SNIPPET_THUMBNAILS_MEDIUM_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-medium-url <SNIPPET_THUMBNAILS_MEDIUM_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-medium-width <SNIPPET_THUMBNAILS_MEDIUM_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-qhd-height <SNIPPET_THUMBNAILS_QHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-qhd-url <SNIPPET_THUMBNAILS_QHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-qhd-width <SNIPPET_THUMBNAILS_QHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-height <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-url <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-shorts-aspect-ratio-width <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-standard-height <SNIPPET_THUMBNAILS_STANDARD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-standard-url <SNIPPET_THUMBNAILS_STANDARD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-standard-width <SNIPPET_THUMBNAILS_STANDARD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-uhd-height <SNIPPET_THUMBNAILS_UHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-uhd-url <SNIPPET_THUMBNAILS_UHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-uhd-width <SNIPPET_THUMBNAILS_UHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-video-owner-channel-id <SNIPPET_VIDEO_OWNER_CHANNEL_ID>
          Channel id for the channel this video belongs to.

      --snippet-video-owner-channel-title <SNIPPET_VIDEO_OWNER_CHANNEL_TITLE>
          Channel title for the channel this video belongs to.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## playlist-items.list

```text
Retrieves a list of resources, possibly filtered.

Usage: joog [OPTIONS] youtube <ACCOUNT> playlist-items.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --id <ID>...
          

      --part <PART>...
          The *part* parameter specifies a comma-separated list of one or more playlistItem resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlistItem resource, the snippet property contains numerous fields, including the title, description, position, and resourceId properties. As such, if you set *part=snippet*, the API response will contain all of those properties.
          
          [default: snippet]

      --playlist-id <PLAYLIST_ID>
          Return the playlist items within the given playlist.

      --video-id <VIDEO_ID>
          Return the playlist items associated with the given video ID.

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
          The *maxResults* parameter specifies the maximum number of items that should be returned in the result set.

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --page-token <PAGE_TOKEN>
          The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## playlist-items.update

```text
Updates an existing resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> playlist-items.update [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --channel-id <CHANNEL_ID>
          The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.

      --verbose
          Include advanced parameters in help

      --description <DESCRIPTION>
          The item's description.

      --part <PART>...
          The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. Note that this method will override the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies. For example, a playlist item can specify a start time and end time, which identify the times portion of the video that should play when users watch the video in the playlist. If your request is updating a playlist item that sets these values, and the request's part parameter value includes the contentDetails part, the playlist item's start and end times will be updated to whatever value the request body specifies. If the request body does not specify values, the existing start and end times will be removed and replaced with the default settings.
          
          [default: snippet]

      --privacy <PRIVACY>
          This resource's privacy status. Choices: public, unlisted, private.

      --status <JSON>
          Information about the playlist item's privacy status.

      --title <TITLE>
          The item's title.

      --video-id <VIDEO_ID>
          The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.

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

      --content-details <JSON>
          Request body field

      --content-details-end-at <CONTENT_DETAILS_END_AT>
          The time, measured in seconds from the start of the video, when the video should stop playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) By default, assume that the video.endTime is the end of the video.

      --content-details-note <CONTENT_DETAILS_NOTE>
          A user-generated note for this item.

      --content-details-start-at <CONTENT_DETAILS_START_AT>
          The time, measured in seconds from the start of the video, when the video should start playing. (The playlist owner can specify the times when the video should start and stop playing when the video is played in the context of the playlist.) The default value is 0.

      --content-details-video-id <CONTENT_DETAILS_VIDEO_ID>
          The ID that YouTube uses to uniquely identify a video. To [retrieve the `video` resource](https://developers.google.com/youtube/data/reference/rest/v3/videos/list), set the id query parameter to this value in your API request.

      --content-details-video-published-at <CONTENT_DETAILS_VIDEO_PUBLISHED_AT>
          The date and time that the video was published to YouTube.

      --etag <ETAG>
          Etag of this resource.

      --id <ID>
          The ID that YouTube uses to uniquely identify the playlist item.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#playlistItem".

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --snippet <JSON>
          Basic details about a playlist, including title, description and thumbnails. Basic details of a YouTube Playlist item provided by the author. Next ID: 15

      --snippet-channel-id <SNIPPET_CHANNEL_ID>
          The ID that YouTube uses to uniquely identify the user that added the item to the playlist.

      --snippet-channel-title <SNIPPET_CHANNEL_TITLE>
          Channel title for the channel that the playlist item belongs to.

      --snippet-playlist-id <SNIPPET_PLAYLIST_ID>
          The ID that YouTube uses to uniquely identify thGe playlist that the playlist item is in.

      --snippet-position <SNIPPET_POSITION>
          The order in which the item appears in the playlist. The value uses a zero-based index, so the first item has a position of 0, the second item has a position of 1, and so forth.

      --snippet-published-at <SNIPPET_PUBLISHED_AT>
          The date and time that the item was added to the playlist.

      --snippet-resource-id-kind <SNIPPET_RESOURCE_ID_KIND>
          The type of the API resource.

      --snippet-resource-id-playlist-id <SNIPPET_RESOURCE_ID_PLAYLIST_ID>
          The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.

      --snippet-thumbnails-default-height <SNIPPET_THUMBNAILS_DEFAULT_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-default-url <SNIPPET_THUMBNAILS_DEFAULT_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-default-width <SNIPPET_THUMBNAILS_DEFAULT_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-fhd-height <SNIPPET_THUMBNAILS_FHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-fhd-url <SNIPPET_THUMBNAILS_FHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-fhd-width <SNIPPET_THUMBNAILS_FHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-high-height <SNIPPET_THUMBNAILS_HIGH_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-high-url <SNIPPET_THUMBNAILS_HIGH_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-high-width <SNIPPET_THUMBNAILS_HIGH_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-maxres-height <SNIPPET_THUMBNAILS_MAXRES_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-maxres-url <SNIPPET_THUMBNAILS_MAXRES_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-maxres-width <SNIPPET_THUMBNAILS_MAXRES_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-medium-height <SNIPPET_THUMBNAILS_MEDIUM_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-medium-url <SNIPPET_THUMBNAILS_MEDIUM_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-medium-width <SNIPPET_THUMBNAILS_MEDIUM_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-qhd-height <SNIPPET_THUMBNAILS_QHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-qhd-url <SNIPPET_THUMBNAILS_QHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-qhd-width <SNIPPET_THUMBNAILS_QHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-height <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-url <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-shorts-aspect-ratio-width <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-standard-height <SNIPPET_THUMBNAILS_STANDARD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-standard-url <SNIPPET_THUMBNAILS_STANDARD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-standard-width <SNIPPET_THUMBNAILS_STANDARD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-uhd-height <SNIPPET_THUMBNAILS_UHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-uhd-url <SNIPPET_THUMBNAILS_UHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-uhd-width <SNIPPET_THUMBNAILS_UHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-video-owner-channel-id <SNIPPET_VIDEO_OWNER_CHANNEL_ID>
          Channel id for the channel this video belongs to.

      --snippet-video-owner-channel-title <SNIPPET_VIDEO_OWNER_CHANNEL_TITLE>
          Channel title for the channel this video belongs to.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## playlists.delete

```text
Deletes a resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> playlists.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          

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

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## playlists.insert

```text
Inserts a new resource into this collection.

Usage: joog [OPTIONS] youtube <ACCOUNT> playlists.insert [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --description <DESCRIPTION>
          The playlist's description.

      --part <PART>...
          The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
          
          [default: snippet]

      --privacy <PRIVACY>
          The playlist's privacy status. Choices: public, unlisted, private.

      --status <JSON>
          Request body field

      --tags <TAGS>...
          Keyword tags associated with the playlist.

      --title <TITLE>
          The playlist's title.

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

      --content-details <JSON>
          Request body field

      --content-details-item-count <CONTENT_DETAILS_ITEM_COUNT>
          The number of videos in the playlist.

      --etag <ETAG>
          Etag of this resource.

      --id <ID>
          The ID that YouTube uses to uniquely identify the playlist.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#playlist".

      --localizations <JSON>
          Localizations for different languages

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --on-behalf-of-content-owner-channel <ON_BEHALF_OF_CONTENT_OWNER_CHANNEL>
          This parameter can only be used in a properly [authorized request](https://developers.google.com/youtube/data/guides/authentication). *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.

      --player <JSON>
          Request body field

      --player-embed-html <PLAYER_EMBED_HTML>
          An  tag that embeds a player that will play the playlist.

      --snippet <JSON>
          Basic details about a playlist, including title, description and thumbnails.

      --snippet-channel-id <SNIPPET_CHANNEL_ID>
          The ID that YouTube uses to uniquely identify the channel that published the playlist.

      --snippet-channel-title <SNIPPET_CHANNEL_TITLE>
          The channel title of the channel that the video belongs to.

      --snippet-default-language <SNIPPET_DEFAULT_LANGUAGE>
          The language of the playlist's default title and description.

      --snippet-localized-description <SNIPPET_LOCALIZED_DESCRIPTION>
          The localized strings for playlist's description.

      --snippet-localized-title <SNIPPET_LOCALIZED_TITLE>
          The localized strings for playlist's title.

      --snippet-published-at <SNIPPET_PUBLISHED_AT>
          The date and time that the playlist was created.

      --snippet-thumbnail-video-id <SNIPPET_THUMBNAIL_VIDEO_ID>
          Note: if the playlist has a custom thumbnail, this field will not be populated. The video id selected by the user that will be used as the thumbnail of this playlist. This field defaults to the first publicly viewable video in the playlist, if: 1. The user has never selected a video to be the thumbnail of the playlist. 2. The user selects a video to be the thumbnail, and then removes that video from the playlist. 3. The user selects a non-owned video to be the thumbnail, but that video becomes private, or gets deleted.

      --snippet-thumbnails-default-height <SNIPPET_THUMBNAILS_DEFAULT_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-default-url <SNIPPET_THUMBNAILS_DEFAULT_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-default-width <SNIPPET_THUMBNAILS_DEFAULT_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-fhd-height <SNIPPET_THUMBNAILS_FHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-fhd-url <SNIPPET_THUMBNAILS_FHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-fhd-width <SNIPPET_THUMBNAILS_FHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-high-height <SNIPPET_THUMBNAILS_HIGH_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-high-url <SNIPPET_THUMBNAILS_HIGH_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-high-width <SNIPPET_THUMBNAILS_HIGH_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-maxres-height <SNIPPET_THUMBNAILS_MAXRES_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-maxres-url <SNIPPET_THUMBNAILS_MAXRES_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-maxres-width <SNIPPET_THUMBNAILS_MAXRES_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-medium-height <SNIPPET_THUMBNAILS_MEDIUM_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-medium-url <SNIPPET_THUMBNAILS_MEDIUM_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-medium-width <SNIPPET_THUMBNAILS_MEDIUM_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-qhd-height <SNIPPET_THUMBNAILS_QHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-qhd-url <SNIPPET_THUMBNAILS_QHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-qhd-width <SNIPPET_THUMBNAILS_QHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-height <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-url <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-shorts-aspect-ratio-width <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-standard-height <SNIPPET_THUMBNAILS_STANDARD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-standard-url <SNIPPET_THUMBNAILS_STANDARD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-standard-width <SNIPPET_THUMBNAILS_STANDARD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-uhd-height <SNIPPET_THUMBNAILS_UHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-uhd-url <SNIPPET_THUMBNAILS_UHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-uhd-width <SNIPPET_THUMBNAILS_UHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --status-podcast-status <STATUS_PODCAST_STATUS>
          The playlist's podcast status. Choices: enabled, disabled.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## playlists.list

```text
Retrieves a list of resources, possibly filtered.

Usage: joog [OPTIONS] youtube <ACCOUNT> playlists.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --channel-id <CHANNEL_ID>
          Return the playlists owned by the specified channel ID.

      --id <ID>...
          Return the playlists with the given IDs for Stubby or Apiary.

      --mine[=<BOOL>]
          Return the playlists owned by the authenticated user.

      --part <PART>...
          The *part* parameter specifies a comma-separated list of one or more playlist resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a playlist resource, the snippet property contains properties like author, title, description, tags, and timeCreated. As such, if you set *part=snippet*, the API response will contain all of those properties.
          
          [default: snippet]

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

      --hl <HL>
          Return content in specified language

      --max-results <MAX_RESULTS>
          The *maxResults* parameter specifies the maximum number of items that should be returned in the result set.

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --on-behalf-of-content-owner-channel <ON_BEHALF_OF_CONTENT_OWNER_CHANNEL>
          This parameter can only be used in a properly [authorized request](https://developers.google.com/youtube/data/guides/authentication). *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.

      --page-token <PAGE_TOKEN>
          The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## playlists.update

```text
Updates an existing resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> playlists.update [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --description <DESCRIPTION>
          The playlist's description.

      --part <PART>...
          The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. Note that this method will override the existing values for mutable properties that are contained in any parts that the request body specifies. For example, a playlist's description is contained in the snippet part, which must be included in the request body. If the request does not specify a value for the snippet.description property, the playlist's existing description will be deleted.
          
          [default: snippet]

      --privacy <PRIVACY>
          The playlist's privacy status. Choices: public, unlisted, private.

      --status <JSON>
          Request body field

      --tags <TAGS>...
          Keyword tags associated with the playlist.

      --title <TITLE>
          The playlist's title.

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

      --content-details <JSON>
          Request body field

      --content-details-item-count <CONTENT_DETAILS_ITEM_COUNT>
          The number of videos in the playlist.

      --etag <ETAG>
          Etag of this resource.

      --id <ID>
          The ID that YouTube uses to uniquely identify the playlist.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#playlist".

      --localizations <JSON>
          Localizations for different languages

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --player <JSON>
          Request body field

      --player-embed-html <PLAYER_EMBED_HTML>
          An  tag that embeds a player that will play the playlist.

      --snippet <JSON>
          Basic details about a playlist, including title, description and thumbnails.

      --snippet-channel-id <SNIPPET_CHANNEL_ID>
          The ID that YouTube uses to uniquely identify the channel that published the playlist.

      --snippet-channel-title <SNIPPET_CHANNEL_TITLE>
          The channel title of the channel that the video belongs to.

      --snippet-default-language <SNIPPET_DEFAULT_LANGUAGE>
          The language of the playlist's default title and description.

      --snippet-localized-description <SNIPPET_LOCALIZED_DESCRIPTION>
          The localized strings for playlist's description.

      --snippet-localized-title <SNIPPET_LOCALIZED_TITLE>
          The localized strings for playlist's title.

      --snippet-published-at <SNIPPET_PUBLISHED_AT>
          The date and time that the playlist was created.

      --snippet-thumbnail-video-id <SNIPPET_THUMBNAIL_VIDEO_ID>
          Note: if the playlist has a custom thumbnail, this field will not be populated. The video id selected by the user that will be used as the thumbnail of this playlist. This field defaults to the first publicly viewable video in the playlist, if: 1. The user has never selected a video to be the thumbnail of the playlist. 2. The user selects a video to be the thumbnail, and then removes that video from the playlist. 3. The user selects a non-owned video to be the thumbnail, but that video becomes private, or gets deleted.

      --snippet-thumbnails-default-height <SNIPPET_THUMBNAILS_DEFAULT_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-default-url <SNIPPET_THUMBNAILS_DEFAULT_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-default-width <SNIPPET_THUMBNAILS_DEFAULT_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-fhd-height <SNIPPET_THUMBNAILS_FHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-fhd-url <SNIPPET_THUMBNAILS_FHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-fhd-width <SNIPPET_THUMBNAILS_FHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-high-height <SNIPPET_THUMBNAILS_HIGH_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-high-url <SNIPPET_THUMBNAILS_HIGH_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-high-width <SNIPPET_THUMBNAILS_HIGH_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-maxres-height <SNIPPET_THUMBNAILS_MAXRES_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-maxres-url <SNIPPET_THUMBNAILS_MAXRES_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-maxres-width <SNIPPET_THUMBNAILS_MAXRES_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-medium-height <SNIPPET_THUMBNAILS_MEDIUM_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-medium-url <SNIPPET_THUMBNAILS_MEDIUM_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-medium-width <SNIPPET_THUMBNAILS_MEDIUM_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-qhd-height <SNIPPET_THUMBNAILS_QHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-qhd-url <SNIPPET_THUMBNAILS_QHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-qhd-width <SNIPPET_THUMBNAILS_QHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-height <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-url <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-shorts-aspect-ratio-width <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-standard-height <SNIPPET_THUMBNAILS_STANDARD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-standard-url <SNIPPET_THUMBNAILS_STANDARD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-standard-width <SNIPPET_THUMBNAILS_STANDARD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-uhd-height <SNIPPET_THUMBNAILS_UHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-uhd-url <SNIPPET_THUMBNAILS_UHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-uhd-width <SNIPPET_THUMBNAILS_UHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --status-podcast-status <STATUS_PODCAST_STATUS>
          The playlist's podcast status. Choices: enabled, disabled.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## search.list

```text
Retrieves a list of search resources

Usage: joog [OPTIONS] youtube <ACCOUNT> search.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --channel-id <CHANNEL_ID>
          Filter on resources belonging to this channelId. (Force TAP rebuild)

      --order <ORDER>
          Sort order of the results. Choices: searchSortUnspecified, date, rating, viewCount, relevance, title, videoCount.

      --part <PART>...
          The *part* parameter specifies a comma-separated list of one or more search resource properties that the API response will include. Set the parameter value to snippet.
          
          [default: snippet]

      --q <Q>
          Textual search terms to match.

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

      --channel-type <CHANNEL_TYPE>
          Add a filter on the channel search. Choices: channelTypeUnspecified, any, show.

      --event-type <EVENT_TYPE>
          Filter on the livestream status of the videos. Choices: none, upcoming, live, completed.

      --for-content-owner[=<BOOL>]
          Search owned by a content owner.

      --for-developer[=<BOOL>]
          Restrict the search to only retrieve videos uploaded using the project id of the authenticated user.

      --for-mine[=<BOOL>]
          Search for the private videos of the authenticated user.

      --location <LOCATION>
          Filter on location of the video

      --location-radius <LOCATION_RADIUS>
          Filter on distance from the location (specified above).

      --max-results <MAX_RESULTS>
          The *maxResults* parameter specifies the maximum number of items that should be returned in the result set.

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --page-token <PAGE_TOKEN>
          The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.

      --published-after <PUBLISHED_AFTER>
          Filter on resources published after this date.

      --published-before <PUBLISHED_BEFORE>
          Filter on resources published before this date.

      --region-code <REGION_CODE>
          Display the content as seen by viewers in this country.

      --relevance-language <RELEVANCE_LANGUAGE>
          Return results relevant to this language.

      --safe-search <SAFE_SEARCH>
          Indicates whether the search results should include restricted content as well as standard content. Choices: safeSearchSettingUnspecified, none, moderate, strict.

      --topic-id <TOPIC_ID>
          Restrict results to a particular topic.

      --type <TYPE>...
          Restrict results to a particular set of resource types from One Platform.

      --video-caption <VIDEO_CAPTION>
          Filter on the presence of captions on the videos. Choices: videoCaptionUnspecified, any, closedCaption, none.

      --video-category-id <VIDEO_CATEGORY_ID>
          Filter on videos in a specific category.

      --video-definition <VIDEO_DEFINITION>
          Filter on the definition of the videos. Choices: any, standard, high.

      --video-dimension <VIDEO_DIMENSION>
          Filter on 3d videos. Choices: any, 2d, 3d.

      --video-duration <VIDEO_DURATION>
          Filter on the duration of the videos. Choices: videoDurationUnspecified, any, short, medium, long.

      --video-embeddable <VIDEO_EMBEDDABLE>
          Filter on embeddable videos. Choices: videoEmbeddableUnspecified, any, true.

      --video-license <VIDEO_LICENSE>
          Filter on the license of the videos. Choices: any, youtube, creativeCommon.

      --video-paid-product-placement <VIDEO_PAID_PRODUCT_PLACEMENT>
           Choices: videoPaidProductPlacementUnspecified, any, true.

      --video-syndicated <VIDEO_SYNDICATED>
          Filter on syndicated videos. Choices: videoSyndicatedUnspecified, any, true.

      --video-type <VIDEO_TYPE>
          Filter on videos of a specific type. Choices: videoTypeUnspecified, any, movie, episode.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## subscriptions.delete

```text
Deletes a resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> subscriptions.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          

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

## subscriptions.insert

```text
Inserts a new resource into this collection.

Usage: joog [OPTIONS] youtube <ACCOUNT> subscriptions.insert [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --channel-id <CHANNEL_ID>
          The ID that YouTube uses to uniquely identify the referred resource, if that resource is a channel. This property is only present if the resourceId.kind value is youtube#channel.

      --verbose
          Include advanced parameters in help

      --description <DESCRIPTION>
          The subscription's details.

      --part <PART>...
          The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include.
          
          [default: snippet]

      --title <TITLE>
          The subscription's title.

      --video-id <VIDEO_ID>
          The ID that YouTube uses to uniquely identify the referred resource, if that resource is a video. This property is only present if the resourceId.kind value is youtube#video.

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

      --content-details <JSON>
          Details about the content to witch a subscription refers.

      --content-details-activity-type <CONTENT_DETAILS_ACTIVITY_TYPE>
          The type of activity this subscription is for (only uploads, everything). Choices: subscriptionActivityTypeUnspecified, all, uploads.

      --content-details-new-item-count <CONTENT_DETAILS_NEW_ITEM_COUNT>
          The number of new items in the subscription since its content was last read.

      --content-details-total-item-count <CONTENT_DETAILS_TOTAL_ITEM_COUNT>
          The approximate number of items that the subscription points to.

      --etag <ETAG>
          Etag of this resource.

      --id <ID>
          The ID that YouTube uses to uniquely identify the subscription.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#subscription".

      --snippet <JSON>
          Basic details about a subscription, including title, description and thumbnails of the subscribed item.

      --snippet-channel-id <SNIPPET_CHANNEL_ID>
          The ID that YouTube uses to uniquely identify the subscriber's channel.

      --snippet-published-at <SNIPPET_PUBLISHED_AT>
          The date and time that the subscription was created.

      --snippet-resource-id-kind <SNIPPET_RESOURCE_ID_KIND>
          The type of the API resource.

      --snippet-resource-id-playlist-id <SNIPPET_RESOURCE_ID_PLAYLIST_ID>
          The ID that YouTube uses to uniquely identify the referred resource, if that resource is a playlist. This property is only present if the resourceId.kind value is youtube#playlist.

      --snippet-thumbnails-default-height <SNIPPET_THUMBNAILS_DEFAULT_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-default-url <SNIPPET_THUMBNAILS_DEFAULT_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-default-width <SNIPPET_THUMBNAILS_DEFAULT_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-fhd-height <SNIPPET_THUMBNAILS_FHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-fhd-url <SNIPPET_THUMBNAILS_FHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-fhd-width <SNIPPET_THUMBNAILS_FHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-high-height <SNIPPET_THUMBNAILS_HIGH_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-high-url <SNIPPET_THUMBNAILS_HIGH_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-high-width <SNIPPET_THUMBNAILS_HIGH_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-maxres-height <SNIPPET_THUMBNAILS_MAXRES_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-maxres-url <SNIPPET_THUMBNAILS_MAXRES_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-maxres-width <SNIPPET_THUMBNAILS_MAXRES_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-medium-height <SNIPPET_THUMBNAILS_MEDIUM_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-medium-url <SNIPPET_THUMBNAILS_MEDIUM_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-medium-width <SNIPPET_THUMBNAILS_MEDIUM_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-qhd-height <SNIPPET_THUMBNAILS_QHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-qhd-url <SNIPPET_THUMBNAILS_QHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-qhd-width <SNIPPET_THUMBNAILS_QHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-height <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-url <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-shorts-aspect-ratio-width <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-standard-height <SNIPPET_THUMBNAILS_STANDARD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-standard-url <SNIPPET_THUMBNAILS_STANDARD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-standard-width <SNIPPET_THUMBNAILS_STANDARD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-uhd-height <SNIPPET_THUMBNAILS_UHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-uhd-url <SNIPPET_THUMBNAILS_UHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-uhd-width <SNIPPET_THUMBNAILS_UHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --subscriber-snippet <JSON>
          Basic details about a subscription's subscriber including title, description, channel ID and thumbnails.

      --subscriber-snippet-channel-id <SUBSCRIBER_SNIPPET_CHANNEL_ID>
          The channel ID of the subscriber.

      --subscriber-snippet-description <SUBSCRIBER_SNIPPET_DESCRIPTION>
          The description of the subscriber.

      --subscriber-snippet-thumbnails-default-height <SUBSCRIBER_SNIPPET_THUMBNAILS_DEFAULT_HEIGHT>
          (Optional) Height of the thumbnail image.

      --subscriber-snippet-thumbnails-default-url <SUBSCRIBER_SNIPPET_THUMBNAILS_DEFAULT_URL>
          The thumbnail image's URL.

      --subscriber-snippet-thumbnails-default-width <SUBSCRIBER_SNIPPET_THUMBNAILS_DEFAULT_WIDTH>
          (Optional) Width of the thumbnail image.

      --subscriber-snippet-thumbnails-fhd-height <SUBSCRIBER_SNIPPET_THUMBNAILS_FHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --subscriber-snippet-thumbnails-fhd-url <SUBSCRIBER_SNIPPET_THUMBNAILS_FHD_URL>
          The thumbnail image's URL.

      --subscriber-snippet-thumbnails-fhd-width <SUBSCRIBER_SNIPPET_THUMBNAILS_FHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --subscriber-snippet-thumbnails-high-height <SUBSCRIBER_SNIPPET_THUMBNAILS_HIGH_HEIGHT>
          (Optional) Height of the thumbnail image.

      --subscriber-snippet-thumbnails-high-url <SUBSCRIBER_SNIPPET_THUMBNAILS_HIGH_URL>
          The thumbnail image's URL.

      --subscriber-snippet-thumbnails-high-width <SUBSCRIBER_SNIPPET_THUMBNAILS_HIGH_WIDTH>
          (Optional) Width of the thumbnail image.

      --subscriber-snippet-thumbnails-maxres-height <SUBSCRIBER_SNIPPET_THUMBNAILS_MAXRES_HEIGHT>
          (Optional) Height of the thumbnail image.

      --subscriber-snippet-thumbnails-maxres-url <SUBSCRIBER_SNIPPET_THUMBNAILS_MAXRES_URL>
          The thumbnail image's URL.

      --subscriber-snippet-thumbnails-maxres-width <SUBSCRIBER_SNIPPET_THUMBNAILS_MAXRES_WIDTH>
          (Optional) Width of the thumbnail image.

      --subscriber-snippet-thumbnails-medium-height <SUBSCRIBER_SNIPPET_THUMBNAILS_MEDIUM_HEIGHT>
          (Optional) Height of the thumbnail image.

      --subscriber-snippet-thumbnails-medium-url <SUBSCRIBER_SNIPPET_THUMBNAILS_MEDIUM_URL>
          The thumbnail image's URL.

      --subscriber-snippet-thumbnails-medium-width <SUBSCRIBER_SNIPPET_THUMBNAILS_MEDIUM_WIDTH>
          (Optional) Width of the thumbnail image.

      --subscriber-snippet-thumbnails-qhd-height <SUBSCRIBER_SNIPPET_THUMBNAILS_QHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --subscriber-snippet-thumbnails-qhd-url <SUBSCRIBER_SNIPPET_THUMBNAILS_QHD_URL>
          The thumbnail image's URL.

      --subscriber-snippet-thumbnails-qhd-width <SUBSCRIBER_SNIPPET_THUMBNAILS_QHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --subscriber-snippet-thumbnails-shorts-aspect-ratio-height <SUBSCRIBER_SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_HEIGHT>
          (Optional) Height of the thumbnail image.

      --subscriber-snippet-thumbnails-shorts-aspect-ratio-url <SUBSCRIBER_SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_URL>
          The thumbnail image's URL.

      --subscriber-snippet-thumbnails-shorts-aspect-ratio-width <SUBSCRIBER_SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_WIDTH>
          (Optional) Width of the thumbnail image.

      --subscriber-snippet-thumbnails-standard-height <SUBSCRIBER_SNIPPET_THUMBNAILS_STANDARD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --subscriber-snippet-thumbnails-standard-url <SUBSCRIBER_SNIPPET_THUMBNAILS_STANDARD_URL>
          The thumbnail image's URL.

      --subscriber-snippet-thumbnails-standard-width <SUBSCRIBER_SNIPPET_THUMBNAILS_STANDARD_WIDTH>
          (Optional) Width of the thumbnail image.

      --subscriber-snippet-thumbnails-uhd-height <SUBSCRIBER_SNIPPET_THUMBNAILS_UHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --subscriber-snippet-thumbnails-uhd-url <SUBSCRIBER_SNIPPET_THUMBNAILS_UHD_URL>
          The thumbnail image's URL.

      --subscriber-snippet-thumbnails-uhd-width <SUBSCRIBER_SNIPPET_THUMBNAILS_UHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --subscriber-snippet-title <SUBSCRIBER_SNIPPET_TITLE>
          The title of the subscriber.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## subscriptions.list

```text
Retrieves a list of resources, possibly filtered.

Usage: joog [OPTIONS] youtube <ACCOUNT> subscriptions.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --channel-id <CHANNEL_ID>
          Return the subscriptions of the given channel owner.

      --id <ID>...
          Return the subscriptions with the given IDs for Stubby or Apiary.

      --mine[=<BOOL>]
          Flag for returning the subscriptions of the authenticated user.

      --order <ORDER>
          The order of the returned subscriptions Choices: subscriptionOrderUnspecified, relevance, unread, alphabetical.

      --part <PART>...
          The *part* parameter specifies a comma-separated list of one or more subscription resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a subscription resource, the snippet property contains other properties, such as a display title for the subscription. If you set *part=snippet*, the API response will also contain all of those nested properties.
          
          [default: snippet]

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

      --for-channel-id <FOR_CHANNEL_ID>
          Return the subscriptions to the subset of these channels that the authenticated user is subscribed to.

      --max-results <MAX_RESULTS>
          The *maxResults* parameter specifies the maximum number of items that should be returned in the result set.

      --my-recent-subscribers[=<BOOL>]
          

      --my-subscribers[=<BOOL>]
          Return the subscribers of the given channel owner.

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --on-behalf-of-content-owner-channel <ON_BEHALF_OF_CONTENT_OWNER_CHANNEL>
          This parameter can only be used in a properly [authorized request](https://developers.google.com/youtube/data/guides/authentication). *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.

      --page-token <PAGE_TOKEN>
          The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## super-chat-events.list

```text
Retrieves a list of resources, possibly filtered.

Usage: joog [OPTIONS] youtube <ACCOUNT> super-chat-events.list [COMMAND_OPTIONS]

Mandatory parameters:
      --part <PART>...
          The *part* parameter specifies the superChatEvent resource parts that the API response will include. This parameter is currently not supported.

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

      --hl <HL>
          Return rendered funding amounts in specified language.

      --max-results <MAX_RESULTS>
          The *maxResults* parameter specifies the maximum number of items that should be returned in the result set.

      --page-token <PAGE_TOKEN>
          The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## thumbnails.set

```text
Set a video's thumbnail from a local image.

Input choices:
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] youtube <ACCOUNT> thumbnails.set [COMMAND_OPTIONS]

Mandatory parameters:
      --video-id <VIDEO_ID>
          Returns the Thumbnail with the given video IDs for Stubby or Apiary.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --upload <PATH>
          Upload a local file of at most 50 MiB.

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

      --content-type <MIME>
          Upload MIME type
          
          [default: application/octet-stream]

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## video-categories.list

```text
Retrieves a list of resources, possibly filtered.

Usage: joog [OPTIONS] youtube <ACCOUNT> video-categories.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --id <ID>...
          Returns the video categories with the given IDs for Stubby or Apiary.

      --part <PART>...
          The *part* parameter specifies the videoCategory resource properties that the API response will include. Set the parameter value to snippet.
          
          [default: snippet]

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

      --hl <HL>
          

      --region-code <REGION_CODE>
          

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## video-trainability.get

```text
Returns the trainability status of a video.

Usage: joog [OPTIONS] youtube <ACCOUNT> video-trainability.get [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --id <ID>
          The ID of the video to retrieve.

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

## videos.batch-get-stats

```text
Retrieves a batch of VideoStat resources, possibly filtered. BatchGetStats is intentionally not atomic to provide a better user experience.

Usage: joog [OPTIONS] youtube <ACCOUNT> videos.batch-get-stats [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --id <ID>...
          Required. Return videos with the given ids. The number of IDs specified cannot exceed 50.

      --verbose
          Include advanced parameters in help

      --part <PART>...
          Required. The `**part**` parameter specifies a comma-separated list of one or more `videoStat` resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a `videoStat` resource, the `statistics` property contains `view_count` and `like_count`. As such, if you set `**part=snippet**`, the API response will contain all of those properties.

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

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          Optional. **Note:** This parameter is intended exclusively for YouTube content partners. The `onBehalfOfContentOwner` parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## videos.delete

```text
Deletes a resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> videos.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          

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

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## videos.get-rating

```text
Retrieves the ratings that the authorized user gave to a list of specified videos.

Usage: joog [OPTIONS] youtube <ACCOUNT> videos.get-rating [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>...
          

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

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## videos.insert

```text
Inserts a new resource into this collection.

Input choices:
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] youtube <ACCOUNT> videos.insert [COMMAND_OPTIONS]

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
          The video's description. @mutable youtube.videos.insert youtube.videos.update

      --part <PART>...
          The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. Note that not all parts contain properties that can be set when inserting or updating a video. For example, the statistics object encapsulates statistics that YouTube calculates for a video and does not contain values that you can set or modify. If the parameter value specifies a part that does not contain mutable values, that part will still be included in the API response.
          
          [default: snippet]

      --privacy <PRIVACY>
          The video's privacy status. In a `videos.insert` request, if you don't specify a value for this property, the uploaded video's privacy status defaults to `public`. Videos uploaded from unverified API projects are not restricted to private viewing mode. Choices: public, unlisted, private.

      --status <JSON>
          Basic details about a video category, such as its localized title. Next Id: 20

      --tags <TAGS>...
          A list of keyword tags associated with the video. Tags may contain spaces.

      --title <TITLE>
          The video's title. @mutable youtube.videos.insert youtube.videos.update

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

      --age-gating <JSON>
          Request body field

      --age-gating-alcohol-content[=<BOOL>]
          Indicates whether or not the video has alcoholic beverage content. Only users of legal purchasing age in a particular country, as identified by ICAP, can view the content.

      --age-gating-restricted[=<BOOL>]
          Age-restricted trailers. For redband trailers and adult-rated video-games. Only users aged 18+ can view the content. The the field is true the content is restricted to viewers aged 18+. Otherwise The field won't be present.

      --age-gating-video-game-rating <AGE_GATING_VIDEO_GAME_RATING>
          Video game rating, if any. Choices: anyone, m15Plus, m16Plus, m17Plus.

      --auto-levels[=<BOOL>]
          Should auto-levels be applied to the upload.

      --brand-partner <JSON>
          Details about the brand partner linked to the video for Creator Initiated Linking (CIL). Next ID: 6

      --brand-partner-channel-handle <BRAND_PARTNER_CHANNEL_HANDLE>
          Required. Channel handle, must begin with "@"

      --brand-partner-channel-id <BRAND_PARTNER_CHANNEL_ID>
          Required. External Channel ID, must begin with "UC"

      --content-details <JSON>
          Details about the content of a YouTube Video.

      --content-details-caption <CONTENT_DETAILS_CAPTION>
          The value of captions indicates whether the video has captions or not. Choices: true, false.

      --content-details-content-rating-acb-rating <CONTENT_DETAILS_CONTENT_RATING_ACB_RATING>
          The video's Australian Classification Board (ACB) or Australian Communications and Media Authority (ACMA) rating. ACMA ratings are used to classify children's television programming. Choices: acbUnspecified, acbE, acbP, acbC, acbG, acbPg, acbM, acbMa15plus, acbR18plus, acbUnrated.

      --content-details-content-rating-agcom-rating <CONTENT_DETAILS_CONTENT_RATING_AGCOM_RATING>
          The video's rating from Italy's Autorità per le Garanzie nelle Comunicazioni (AGCOM). Choices: agcomUnspecified, agcomT, agcomVm14, agcomVm18, agcomUnrated.

      --content-details-content-rating-anatel-rating <CONTENT_DETAILS_CONTENT_RATING_ANATEL_RATING>
          The video's Anatel (Asociación Nacional de Televisión) rating for Chilean television. Choices: anatelUnspecified, anatelF, anatelI, anatelI7, anatelI10, anatelI12, anatelR, anatelA, anatelUnrated.

      --content-details-content-rating-bbfc-rating <CONTENT_DETAILS_CONTENT_RATING_BBFC_RATING>
          The video's British Board of Film Classification (BBFC) rating. Choices: bbfcUnspecified, bbfcU, bbfcPg, bbfc12a, bbfc12, bbfc15, bbfc18, bbfcR18, bbfcUnrated.

      --content-details-content-rating-bfvc-rating <CONTENT_DETAILS_CONTENT_RATING_BFVC_RATING>
          The video's rating from Thailand's Board of Film and Video Censors. Choices: bfvcUnspecified, bfvcG, bfvcE, bfvc13, bfvc15, bfvc18, bfvc20, bfvcB, bfvcUnrated.

      --content-details-content-rating-bmukk-rating <CONTENT_DETAILS_CONTENT_RATING_BMUKK_RATING>
          The video's rating from the Austrian Board of Media Classification (Bundesministerium für Unterricht, Kunst und Kultur). Choices: bmukkUnspecified, bmukkAa, bmukk6, bmukk8, bmukk10, bmukk12, bmukk14, bmukk16, bmukkUnrated.

      --content-details-content-rating-catv-rating <CONTENT_DETAILS_CONTENT_RATING_CATV_RATING>
          Rating system for Canadian TV - Canadian TV Classification System The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian English-language broadcasts. For more information, see the Canadian Broadcast Standards Council website. Choices: catvUnspecified, catvC, catvC8, catvG, catvPg, catv14plus, catv18plus, catvUnrated, catvE.

      --content-details-content-rating-catvfr-rating <CONTENT_DETAILS_CONTENT_RATING_CATVFR_RATING>
          The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian French-language broadcasts. For more information, see the Canadian Broadcast Standards Council website. Choices: catvfrUnspecified, catvfrG, catvfr8plus, catvfr13plus, catvfr16plus, catvfr18plus, catvfrUnrated, catvfrE.

      --content-details-content-rating-cbfc-rating <CONTENT_DETAILS_CONTENT_RATING_CBFC_RATING>
          The video's Central Board of Film Certification (CBFC - India) rating. Choices: cbfcUnspecified, cbfcU, cbfcUA, cbfcUA7plus, cbfcUA13plus, cbfcUA16plus, cbfcA, cbfcS, cbfcUnrated.

      --content-details-content-rating-ccc-rating <CONTENT_DETAILS_CONTENT_RATING_CCC_RATING>
          The video's Consejo de Calificación Cinematográfica (Chile) rating. Choices: cccUnspecified, cccTe, ccc6, ccc14, ccc18, ccc18v, ccc18s, cccUnrated.

      --content-details-content-rating-cce-rating <CONTENT_DETAILS_CONTENT_RATING_CCE_RATING>
          The video's rating from Portugal's Comissão de Classificação de Espect´culos. Choices: cceUnspecified, cceM4, cceM6, cceM12, cceM16, cceM18, cceUnrated, cceM14.

      --content-details-content-rating-chfilm-rating <CONTENT_DETAILS_CONTENT_RATING_CHFILM_RATING>
          The video's rating in Switzerland. Choices: chfilmUnspecified, chfilm0, chfilm6, chfilm12, chfilm16, chfilm18, chfilmUnrated.

      --content-details-content-rating-chvrs-rating <CONTENT_DETAILS_CONTENT_RATING_CHVRS_RATING>
          The video's Canadian Home Video Rating System (CHVRS) rating. Choices: chvrsUnspecified, chvrsG, chvrsPg, chvrs14a, chvrs18a, chvrsR, chvrsE, chvrsUnrated.

      --content-details-content-rating-cicf-rating <CONTENT_DETAILS_CONTENT_RATING_CICF_RATING>
          The video's rating from the Commission de Contrôle des Films (Belgium). Choices: cicfUnspecified, cicfE, cicfKtEa, cicfKntEna, cicfUnrated.

      --content-details-content-rating-cna-rating <CONTENT_DETAILS_CONTENT_RATING_CNA_RATING>
          The video's rating from Romania's CONSILIUL NATIONAL AL AUDIOVIZUALULUI (CNA). Choices: cnaUnspecified, cnaAp, cna12, cna15, cna18, cna18plus, cnaUnrated.

      --content-details-content-rating-cnc-rating <CONTENT_DETAILS_CONTENT_RATING_CNC_RATING>
          Rating system in France - Commission de classification cinematographique Choices: cncUnspecified, cncT, cnc10, cnc12, cnc16, cnc18, cncE, cncInterdiction, cncUnrated.

      --content-details-content-rating-csa-rating <CONTENT_DETAILS_CONTENT_RATING_CSA_RATING>
          The video's rating from France's Conseil supérieur de l’audiovisuel, which rates broadcast content. Choices: csaUnspecified, csaT, csa10, csa12, csa16, csa18, csaInterdiction, csaUnrated.

      --content-details-content-rating-cscf-rating <CONTENT_DETAILS_CONTENT_RATING_CSCF_RATING>
          The video's rating from Luxembourg's Commission de surveillance de la classification des films (CSCF). Choices: cscfUnspecified, cscfAl, cscfA, cscf6, cscf9, cscf12, cscf16, cscf18, cscfUnrated.

      --content-details-content-rating-czfilm-rating <CONTENT_DETAILS_CONTENT_RATING_CZFILM_RATING>
          The video's rating in the Czech Republic. Choices: czfilmUnspecified, czfilmU, czfilm12, czfilm14, czfilm18, czfilmUnrated.

      --content-details-content-rating-djctq-rating <CONTENT_DETAILS_CONTENT_RATING_DJCTQ_RATING>
          The video's Departamento de Justiça, Classificação, Qualificação e Títulos (DJCQT - Brazil) rating. Choices: djctqUnspecified, djctqL, djctq10, djctq12, djctq14, djctq16, djctq18, djctqEr, djctqL10, djctqL12, djctqL14, djctqL16, djctqL18, djctq1012, djctq1014, djctq1016, djctq1018, djctq1214, djctq1216, djctq1218, djctq1416, djctq1418, djctq1618, djctqUnrated.

      --content-details-content-rating-djctq-rating-reasons <CONTENT_DETAILS_CONTENT_RATING_DJCTQ_RATING_REASONS>...
          Reasons that explain why the video received its DJCQT (Brazil) rating.

      --content-details-content-rating-ecbmct-rating <CONTENT_DETAILS_CONTENT_RATING_ECBMCT_RATING>
          Rating system in Turkey - Evaluation and Classification Board of the Ministry of Culture and Tourism Choices: ecbmctUnspecified, ecbmctG, ecbmct7a, ecbmct7plus, ecbmct13a, ecbmct13plus, ecbmct15a, ecbmct15plus, ecbmct18plus, ecbmctUnrated.

      --content-details-content-rating-eefilm-rating <CONTENT_DETAILS_CONTENT_RATING_EEFILM_RATING>
          The video's rating in Estonia. Choices: eefilmUnspecified, eefilmPere, eefilmL, eefilmMs6, eefilmK6, eefilmMs12, eefilmK12, eefilmK14, eefilmK16, eefilmUnrated.

      --content-details-content-rating-egfilm-rating <CONTENT_DETAILS_CONTENT_RATING_EGFILM_RATING>
          The video's rating in Egypt. Choices: egfilmUnspecified, egfilmGn, egfilm18, egfilmBn, egfilmUnrated.

      --content-details-content-rating-eirin-rating <CONTENT_DETAILS_CONTENT_RATING_EIRIN_RATING>
          The video's Eirin (映倫) rating. Eirin is the Japanese rating system. Choices: eirinUnspecified, eirinG, eirinPg12, eirinR15plus, eirinR18plus, eirinUnrated.

      --content-details-content-rating-fcbm-rating <CONTENT_DETAILS_CONTENT_RATING_FCBM_RATING>
          The video's rating from Malaysia's Film Censorship Board. Choices: fcbmUnspecified, fcbmU, fcbmPg13, fcbmP13, fcbm18, fcbm18sx, fcbm18pa, fcbm18sg, fcbm18pl, fcbmUnrated.

      --content-details-content-rating-fco-rating <CONTENT_DETAILS_CONTENT_RATING_FCO_RATING>
          The video's rating from Hong Kong's Office for Film, Newspaper and Article Administration. Choices: fcoUnspecified, fcoI, fcoIia, fcoIib, fcoIi, fcoIii, fcoUnrated.

      --content-details-content-rating-fmoc-rating <CONTENT_DETAILS_CONTENT_RATING_FMOC_RATING>
          This property has been deprecated. Use the contentDetails.contentRating.cncRating instead. Choices: fmocUnspecified, fmocU, fmoc10, fmoc12, fmoc16, fmoc18, fmocE, fmocUnrated.

      --content-details-content-rating-fpb-rating <CONTENT_DETAILS_CONTENT_RATING_FPB_RATING>
          The video's rating from South Africa's Film and Publication Board. Choices: fpbUnspecified, fpbA, fpbPg, fpb79Pg, fpb1012Pg, fpb13, fpb16, fpb18, fpbX18, fpbXx, fpbUnrated, fpb10.

      --content-details-content-rating-fpb-rating-reasons <CONTENT_DETAILS_CONTENT_RATING_FPB_RATING_REASONS>...
          Reasons that explain why the video received its FPB (South Africa) rating.

      --content-details-content-rating-fsk-rating <CONTENT_DETAILS_CONTENT_RATING_FSK_RATING>
          The video's Freiwillige Selbstkontrolle der Filmwirtschaft (FSK - Germany) rating. Choices: fskUnspecified, fsk0, fsk6, fsk12, fsk16, fsk18, fskUnrated.

      --content-details-content-rating-grfilm-rating <CONTENT_DETAILS_CONTENT_RATING_GRFILM_RATING>
          The video's rating in Greece. Choices: grfilmUnspecified, grfilmK, grfilmE, grfilmK12, grfilmK13, grfilmK15, grfilmK17, grfilmK18, grfilmUnrated.

      --content-details-content-rating-icaa-rating <CONTENT_DETAILS_CONTENT_RATING_ICAA_RATING>
          The video's Instituto de la Cinematografía y de las Artes Audiovisuales (ICAA - Spain) rating. Choices: icaaUnspecified, icaaApta, icaa7, icaa12, icaa13, icaa16, icaa18, icaaX, icaaUnrated.

      --content-details-content-rating-ifco-rating <CONTENT_DETAILS_CONTENT_RATING_IFCO_RATING>
          The video's Irish Film Classification Office (IFCO - Ireland) rating. See the IFCO website for more information. Choices: ifcoUnspecified, ifcoG, ifcoPg, ifco12, ifco12a, ifco15, ifco15a, ifco16, ifco18, ifcoUnrated.

      --content-details-content-rating-ilfilm-rating <CONTENT_DETAILS_CONTENT_RATING_ILFILM_RATING>
          The video's rating in Israel. Choices: ilfilmUnspecified, ilfilmAa, ilfilm12, ilfilm14, ilfilm16, ilfilm18, ilfilmUnrated.

      --content-details-content-rating-incaa-rating <CONTENT_DETAILS_CONTENT_RATING_INCAA_RATING>
          The video's INCAA (Instituto Nacional de Cine y Artes Audiovisuales - Argentina) rating. Choices: incaaUnspecified, incaaAtp, incaaSam13, incaaSam16, incaaSam18, incaaC, incaaUnrated.

      --content-details-content-rating-kfcb-rating <CONTENT_DETAILS_CONTENT_RATING_KFCB_RATING>
          The video's rating from the Kenya Film Classification Board. Choices: kfcbUnspecified, kfcbG, kfcbPg, kfcb16plus, kfcbR, kfcbUnrated.

      --content-details-content-rating-kijkwijzer-rating <CONTENT_DETAILS_CONTENT_RATING_KIJKWIJZER_RATING>
          The video's NICAM/Kijkwijzer rating from the Nederlands Instituut voor de Classificatie van Audiovisuele Media (Netherlands). Choices: kijkwijzerUnspecified, kijkwijzerAl, kijkwijzer6, kijkwijzer9, kijkwijzer12, kijkwijzer16, kijkwijzer18, kijkwijzerUnrated.

      --content-details-content-rating-kmrb-rating <CONTENT_DETAILS_CONTENT_RATING_KMRB_RATING>
          The video's Korea Media Rating Board (영상물등급위원회) rating. The KMRB rates videos in South Korea. Choices: kmrbUnspecified, kmrbAll, kmrb12plus, kmrb15plus, kmrbTeenr, kmrbR, kmrbUnrated.

      --content-details-content-rating-lsf-rating <CONTENT_DETAILS_CONTENT_RATING_LSF_RATING>
          The video's rating from Indonesia's Lembaga Sensor Film. Choices: lsfUnspecified, lsfSu, lsfA, lsfBo, lsf13, lsfR, lsf17, lsfD, lsf21, lsfUnrated.

      --content-details-content-rating-mccaa-rating <CONTENT_DETAILS_CONTENT_RATING_MCCAA_RATING>
          The video's rating from Malta's Film Age-Classification Board. Choices: mccaaUnspecified, mccaaU, mccaaPg, mccaa12a, mccaa12, mccaa14, mccaa15, mccaa16, mccaa18, mccaaUnrated.

      --content-details-content-rating-mccyp-rating <CONTENT_DETAILS_CONTENT_RATING_MCCYP_RATING>
          The video's rating from the Danish Film Institute's (Det Danske Filminstitut) Media Council for Children and Young People. Choices: mccypUnspecified, mccypA, mccyp7, mccyp11, mccyp15, mccypUnrated.

      --content-details-content-rating-mcst-rating <CONTENT_DETAILS_CONTENT_RATING_MCST_RATING>
          The video's rating system for Vietnam - MCST Choices: mcstUnspecified, mcstP, mcst0, mcstC13, mcstC16, mcst16plus, mcstC18, mcstGPg, mcstUnrated.

      --content-details-content-rating-mda-rating <CONTENT_DETAILS_CONTENT_RATING_MDA_RATING>
          The video's rating from Singapore's Media Development Authority (MDA) and, specifically, it's Board of Film Censors (BFC). Choices: mdaUnspecified, mdaG, mdaPg, mdaPg13, mdaNc16, mdaM18, mdaR21, mdaUnrated.

      --content-details-content-rating-medietilsynet-rating <CONTENT_DETAILS_CONTENT_RATING_MEDIETILSYNET_RATING>
          The video's rating from Medietilsynet, the Norwegian Media Authority. Choices: medietilsynetUnspecified, medietilsynetA, medietilsynet6, medietilsynet7, medietilsynet9, medietilsynet11, medietilsynet12, medietilsynet15, medietilsynet18, medietilsynetUnrated.

      --content-details-content-rating-meku-rating <CONTENT_DETAILS_CONTENT_RATING_MEKU_RATING>
          The video's rating from Finland's Kansallinen Audiovisuaalinen Instituutti (National Audiovisual Institute). Choices: mekuUnspecified, mekuS, meku7, meku12, meku16, meku18, mekuUnrated.

      --content-details-content-rating-mena-mpaa-rating <CONTENT_DETAILS_CONTENT_RATING_MENA_MPAA_RATING>
          MENA content rating, using the MPAA rating categories. Choices: menaMpaaUnspecified, menaMpaaG, menaMpaaPg, menaMpaaPg13, menaMpaaR, menaMpaaUnrated.

      --content-details-content-rating-mibac-rating <CONTENT_DETAILS_CONTENT_RATING_MIBAC_RATING>
          The video's rating from the Ministero dei Beni e delle Attività Culturali e del Turismo (Italy). Choices: mibacUnspecified, mibacT, mibacVap, mibacVm6, mibacVm12, mibacVm14, mibacVm16, mibacVm18, mibacUnrated.

      --content-details-content-rating-moc-rating <CONTENT_DETAILS_CONTENT_RATING_MOC_RATING>
          The video's Ministerio de Cultura (Colombia) rating. Choices: mocUnspecified, mocE, mocT, moc7, moc12, moc15, moc18, mocX, mocBanned, mocUnrated.

      --content-details-content-rating-moctw-rating <CONTENT_DETAILS_CONTENT_RATING_MOCTW_RATING>
          The video's rating from Taiwan's Ministry of Culture (文化部). Choices: moctwUnspecified, moctwG, moctwP, moctwPg, moctwR, moctwUnrated, moctwR12, moctwR15.

      --content-details-content-rating-mpaa-rating <CONTENT_DETAILS_CONTENT_RATING_MPAA_RATING>
          The video's Motion Picture Association of America (MPAA) rating. Choices: mpaaUnspecified, mpaaG, mpaaPg, mpaaPg13, mpaaR, mpaaNc17, mpaaX, mpaaUnrated.

      --content-details-content-rating-mpaat-rating <CONTENT_DETAILS_CONTENT_RATING_MPAAT_RATING>
          The rating system for trailer, DVD, and Ad in the US. See http://movielabs.com/md/ratings/v2.3/html/US_MPAAT_Ratings.html. Choices: mpaatUnspecified, mpaatGb, mpaatRb.

      --content-details-content-rating-mtrcb-rating <CONTENT_DETAILS_CONTENT_RATING_MTRCB_RATING>
          The video's rating from the Movie and Television Review and Classification Board (Philippines). Choices: mtrcbUnspecified, mtrcbG, mtrcbPg, mtrcbR13, mtrcbR16, mtrcbR18, mtrcbX, mtrcbUnrated.

      --content-details-content-rating-nbc-rating <CONTENT_DETAILS_CONTENT_RATING_NBC_RATING>
          The video's rating from the Maldives National Bureau of Classification. Choices: nbcUnspecified, nbcG, nbcPg, nbc12plus, nbc15plus, nbc18plus, nbc18plusr, nbcPu, nbcUnrated.

      --content-details-content-rating-nbcpl-rating <CONTENT_DETAILS_CONTENT_RATING_NBCPL_RATING>
          The video's rating in Poland. Choices: nbcplUnspecified, nbcplI, nbcplIi, nbcplIii, nbcplIv, nbcpl18plus, nbcplUnrated.

      --content-details-content-rating-nfrc-rating <CONTENT_DETAILS_CONTENT_RATING_NFRC_RATING>
          The video's rating from the Bulgarian National Film Center. Choices: nfrcUnspecified, nfrcA, nfrcB, nfrcC, nfrcD, nfrcX, nfrcUnrated.

      --content-details-content-rating-nfvcb-rating <CONTENT_DETAILS_CONTENT_RATING_NFVCB_RATING>
          The video's rating from Nigeria's National Film and Video Censors Board. Choices: nfvcbUnspecified, nfvcbG, nfvcbPg, nfvcb12, nfvcb12a, nfvcb15, nfvcb18, nfvcbRe, nfvcbUnrated.

      --content-details-content-rating-nkclv-rating <CONTENT_DETAILS_CONTENT_RATING_NKCLV_RATING>
          The video's rating from the Nacionãlais Kino centrs (National Film Centre of Latvia). Choices: nkclvUnspecified, nkclvU, nkclv7plus, nkclv12plus, nkclv16plus, nkclv18plus, nkclvUnrated.

      --content-details-content-rating-nmc-rating <CONTENT_DETAILS_CONTENT_RATING_NMC_RATING>
          The National Media Council ratings system for United Arab Emirates. Choices: nmcUnspecified, nmcG, nmcPg, nmcPg13, nmcPg15, nmc15plus, nmc18plus, nmc18tc, nmcUnrated.

      --content-details-content-rating-oflc-rating <CONTENT_DETAILS_CONTENT_RATING_OFLC_RATING>
          The video's Office of Film and Literature Classification (OFLC - New Zealand) rating. Choices: oflcUnspecified, oflcG, oflcPg, oflcM, oflcR13, oflcR15, oflcR16, oflcR18, oflcUnrated, oflcRp13, oflcRp16, oflcRp18.

      --content-details-content-rating-pefilm-rating <CONTENT_DETAILS_CONTENT_RATING_PEFILM_RATING>
          The video's rating in Peru. Choices: pefilmUnspecified, pefilmPt, pefilmPg, pefilm14, pefilm18, pefilmUnrated.

      --content-details-content-rating-rcnof-rating <CONTENT_DETAILS_CONTENT_RATING_RCNOF_RATING>
          The video's rating from the Hungarian Nemzeti Filmiroda, the Rating Committee of the National Office of Film. Choices: rcnofUnspecified, rcnofI, rcnofIi, rcnofIii, rcnofIv, rcnofV, rcnofVi, rcnofUnrated.

      --content-details-content-rating-resorteviolencia-rating <CONTENT_DETAILS_CONTENT_RATING_RESORTEVIOLENCIA_RATING>
          The video's rating in Venezuela. Choices: resorteviolenciaUnspecified, resorteviolenciaA, resorteviolenciaB, resorteviolenciaC, resorteviolenciaD, resorteviolenciaE, resorteviolenciaUnrated.

      --content-details-content-rating-rtc-rating <CONTENT_DETAILS_CONTENT_RATING_RTC_RATING>
          The video's General Directorate of Radio, Television and Cinematography (Mexico) rating. Choices: rtcUnspecified, rtcAa, rtcA, rtcB, rtcB15, rtcC, rtcD, rtcUnrated.

      --content-details-content-rating-rte-rating <CONTENT_DETAILS_CONTENT_RATING_RTE_RATING>
          The video's rating from Ireland's Raidió Teilifís Éireann. Choices: rteUnspecified, rteGa, rteCh, rtePs, rteMa, rteUnrated.

      --content-details-content-rating-russia-rating <CONTENT_DETAILS_CONTENT_RATING_RUSSIA_RATING>
          The video's National Film Registry of the Russian Federation (MKRF - Russia) rating. Choices: russiaUnspecified, russia0, russia6, russia12, russia16, russia18, russiaUnrated.

      --content-details-content-rating-skfilm-rating <CONTENT_DETAILS_CONTENT_RATING_SKFILM_RATING>
          The video's rating in Slovakia. Choices: skfilmUnspecified, skfilmG, skfilmP2, skfilmP5, skfilmP8, skfilmUnrated.

      --content-details-content-rating-smais-rating <CONTENT_DETAILS_CONTENT_RATING_SMAIS_RATING>
          The video's rating in Iceland. Choices: smaisUnspecified, smaisL, smais7, smais12, smais14, smais16, smais18, smaisUnrated.

      --content-details-content-rating-smsa-rating <CONTENT_DETAILS_CONTENT_RATING_SMSA_RATING>
          The video's rating from Statens medieråd (Sweden's National Media Council). Choices: smsaUnspecified, smsaA, smsa7, smsa11, smsa15, smsaUnrated.

      --content-details-content-rating-tvpg-rating <CONTENT_DETAILS_CONTENT_RATING_TVPG_RATING>
          The video's TV Parental Guidelines (TVPG) rating. Choices: tvpgUnspecified, tvpgY, tvpgY7, tvpgY7Fv, tvpgG, tvpgPg, pg14, tvpgMa, tvpgUnrated.

      --content-details-content-rating-yt-rating <CONTENT_DETAILS_CONTENT_RATING_YT_RATING>
          A rating that YouTube uses to identify age-restricted content. Choices: ytUnspecified, ytAgeRestricted.

      --content-details-country-restriction-allowed[=<BOOL>]
          The value of allowed indicates whether the access to the policy is allowed or denied by default.

      --content-details-country-restriction-exception <CONTENT_DETAILS_COUNTRY_RESTRICTION_EXCEPTION>...
          A list of region codes that identify countries where the default policy do not apply.

      --content-details-definition <CONTENT_DETAILS_DEFINITION>
          The value of definition indicates whether the video is available in high definition or only in standard definition. Choices: sd, hd.

      --content-details-dimension <CONTENT_DETAILS_DIMENSION>
          The value of dimension indicates whether the video is available in 3D or in 2D.

      --content-details-duration <CONTENT_DETAILS_DURATION>
          The length of the video. The tag value is an ISO 8601 duration in the format PT#M#S, in which the letters PT indicate that the value specifies a period of time, and the letters M and S refer to length in minutes and seconds, respectively. The # characters preceding the M and S letters are both integers that specify the number of minutes (or seconds) of the video. For example, a value of PT15M51S indicates that the video is 15 minutes and 51 seconds long.

      --content-details-has-custom-thumbnail[=<BOOL>]
          Indicates whether the video uploader has provided a custom thumbnail image for the video. This property is only visible to the video uploader.

      --content-details-licensed-content[=<BOOL>]
          The value of is_license_content indicates whether the video is licensed content.

      --content-details-projection <CONTENT_DETAILS_PROJECTION>
          Specifies the projection format of the video. Choices: rectangular, 360.

      --content-details-region-restriction-allowed <CONTENT_DETAILS_REGION_RESTRICTION_ALLOWED>...
          A list of region codes that identify countries where the video is viewable. If this property is present and a country is not listed in its value, then the video is blocked from appearing in that country. If this property is present and contains an empty list, the video is blocked in all countries.

      --content-details-region-restriction-blocked <CONTENT_DETAILS_REGION_RESTRICTION_BLOCKED>...
          A list of region codes that identify countries where the video is blocked. If this property is present and a country is not listed in its value, then the video is viewable in that country. If this property is present and contains an empty list, the video is viewable in all countries.

      --etag <ETAG>
          Etag of this resource.

      --file-details <JSON>
          Describes original video file properties, including technical details about audio and video streams, but also metadata information like content length, digitization time, or geotagging information.

      --file-details-audio-streams <JSON>
          A list of audio streams contained in the uploaded video file. Each item in the list contains detailed metadata about an audio stream.

      --file-details-bitrate-bps <FILE_DETAILS_BITRATE_BPS>
          The uploaded video file's combined (video and audio) bitrate in bits per second.

      --file-details-container <FILE_DETAILS_CONTAINER>
          The uploaded video file's container format.

      --file-details-creation-time <FILE_DETAILS_CREATION_TIME>
          The date and time when the uploaded video file was created. The value is specified in ISO 8601 format. Currently, the following ISO 8601 formats are supported: - Date only: YYYY-MM-DD - Naive time: YYYY-MM-DDTHH:MM:SS - Time with timezone: YYYY-MM-DDTHH:MM:SS+HH:MM

      --file-details-duration-ms <FILE_DETAILS_DURATION_MS>
          The length of the uploaded video in milliseconds.

      --file-details-file-name <FILE_DETAILS_FILE_NAME>
          The uploaded file's name. This field is present whether a video file or another type of file was uploaded.

      --file-details-file-size <FILE_DETAILS_FILE_SIZE>
          The uploaded file's size in bytes. This field is present whether a video file or another type of file was uploaded.

      --file-details-file-type <FILE_DETAILS_FILE_TYPE>
          The uploaded file's type as detected by YouTube's video processing engine. Currently, YouTube only processes video files, but this field is present whether a video file or another type of file was uploaded. Choices: video, audio, image, archive, document, project, other.

      --file-details-video-streams <JSON>
          A list of video streams contained in the uploaded video file. Each item in the list contains detailed metadata about a video stream.

      --id <ID>
          The ID that YouTube uses to uniquely identify the video.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#video".

      --live-streaming-details <JSON>
          Details about the live streaming metadata.

      --live-streaming-details-active-live-chat-id <LIVE_STREAMING_DETAILS_ACTIVE_LIVE_CHAT_ID>
          The ID of the currently active live chat attached to this video. This field is filled only if the video is a currently live broadcast that has live chat. Once the broadcast transitions to complete this field will be removed and the live chat closed down. For persistent broadcasts that live chat id will no longer be tied to this video but rather to the new video being displayed at the persistent page.

      --live-streaming-details-actual-end-time <LIVE_STREAMING_DETAILS_ACTUAL_END_TIME>
          The time that the broadcast actually ended. This value will not be available until the broadcast is over.

      --live-streaming-details-actual-start-time <LIVE_STREAMING_DETAILS_ACTUAL_START_TIME>
          The time that the broadcast actually started. This value will not be available until the broadcast begins.

      --live-streaming-details-concurrent-viewers <LIVE_STREAMING_DETAILS_CONCURRENT_VIEWERS>
          The number of viewers watching the broadcast at the time of the request. This count includes co-viewers when multiple people watch together on a television screen. The property and its value will be present if the broadcast has current viewers and the broadcast owner has not hidden the viewcount for the video. Note that YouTube stops tracking the number of concurrent viewers for a broadcast when the broadcast ends. So, this property would not identify the number of viewers watching an archived video of a live broadcast that already ended. The concurrent viewer counts that the YouTube Data API returns might differ from the processed, despammed concurrent viewer counts available through YouTube Analytics. Learn more about live streaming metrics in the [YouTube Help Center](https://support.google.com/youtube/answer/2853833).

      --live-streaming-details-scheduled-end-time <LIVE_STREAMING_DETAILS_SCHEDULED_END_TIME>
          The time that the broadcast is scheduled to end. If the value is empty or the property is not present, then the broadcast is scheduled to continue indefinitely.

      --live-streaming-details-scheduled-start-time <LIVE_STREAMING_DETAILS_SCHEDULED_START_TIME>
          The time that the broadcast is scheduled to begin.

      --localizations <JSON>
          The localizations object contains localized versions of the basic details about the video, such as its title and description.

      --monetization-details <JSON>
          Details about monetization of a YouTube Video.

      --monetization-details-access-allowed[=<BOOL>]
          The value of allowed indicates whether the access to the policy is allowed or denied by default.

      --monetization-details-access-exception <MONETIZATION_DETAILS_ACCESS_EXCEPTION>...
          A list of region codes that identify countries where the default policy do not apply.

      --notify-subscribers[=<BOOL>]
          Notify the channel subscribers about the new video. As default, the notification is enabled.

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --on-behalf-of-content-owner-channel <ON_BEHALF_OF_CONTENT_OWNER_CHANNEL>
          This parameter can only be used in a properly [authorized request](https://developers.google.com/youtube/data/guides/authentication). *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwnerChannel* parameter specifies the YouTube channel ID of the channel to which a video is being added. This parameter is required when a request specifies a value for the onBehalfOfContentOwner parameter, and it can only be used in conjunction with that parameter. In addition, the request must be authorized using a CMS account that is linked to the content owner that the onBehalfOfContentOwner parameter specifies. Finally, the channel that the onBehalfOfContentOwnerChannel parameter value specifies must be linked to the content owner that the onBehalfOfContentOwner parameter specifies. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and perform actions on behalf of the channel specified in the parameter value, without having to provide authentication credentials for each separate channel.

      --paid-product-placement-details <JSON>
          Details about paid content, such as paid product placement, sponsorships or endorsement, contained in a YouTube video and a method to inform viewers of paid promotion. This data can only be retrieved by the video owner.

      --paid-product-placement-details-has-paid-product-placement[=<BOOL>]
          This boolean represents whether the video contains Paid Product Placement, Studio equivalent: https://screenshot.googleplex.com/4Me79DE6AfT2ktp.png

      --player <JSON>
          Player to be used for a video playback.

      --player-embed-height <PLAYER_EMBED_HEIGHT>
          Request body field

      --player-embed-html <PLAYER_EMBED_HTML>
          An  tag that embeds a player that will play the video.

      --player-embed-width <PLAYER_EMBED_WIDTH>
          The embed width

      --processing-details <JSON>
          Describes processing status and progress and availability of some other Video resource parts.

      --processing-details-editor-suggestions-availability <PROCESSING_DETAILS_EDITOR_SUGGESTIONS_AVAILABILITY>
          This value indicates whether video editing suggestions, which might improve video quality or the playback experience, are available for the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.

      --processing-details-file-details-availability <PROCESSING_DETAILS_FILE_DETAILS_AVAILABILITY>
          This value indicates whether file details are available for the uploaded video. You can retrieve a video's file details by requesting the fileDetails part in your videos.list() request.

      --processing-details-processing-failure-reason <PROCESSING_DETAILS_PROCESSING_FAILURE_REASON>
          The reason that YouTube failed to process the video. This property will only have a value if the processingStatus property's value is failed. Choices: uploadFailed, transcodeFailed, streamingFailed, other.

      --processing-details-processing-issues-availability <PROCESSING_DETAILS_PROCESSING_ISSUES_AVAILABILITY>
          This value indicates whether the video processing engine has generated suggestions that might improve YouTube's ability to process the the video, warnings that explain video processing problems, or errors that cause video processing problems. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.

      --processing-details-processing-progress-parts-processed <PROCESSING_DETAILS_PROCESSING_PROGRESS_PARTS_PROCESSED>
          The number of parts of the video that YouTube has already processed. You can estimate the percentage of the video that YouTube has already processed by calculating: 100 * parts_processed / parts_total Note that since the estimated number of parts could increase without a corresponding increase in the number of parts that have already been processed, it is possible that the calculated progress could periodically decrease while YouTube processes a video.

      --processing-details-processing-progress-parts-total <PROCESSING_DETAILS_PROCESSING_PROGRESS_PARTS_TOTAL>
          An estimate of the total number of parts that need to be processed for the video. The number may be updated with more precise estimates while YouTube processes the video.

      --processing-details-processing-progress-time-left-ms <PROCESSING_DETAILS_PROCESSING_PROGRESS_TIME_LEFT_MS>
          An estimate of the amount of time, in millseconds, that YouTube needs to finish processing the video.

      --processing-details-processing-status <PROCESSING_DETAILS_PROCESSING_STATUS>
          The video's processing status. This value indicates whether YouTube was able to process the video or if the video is still being processed. Choices: processing, succeeded, failed, terminated.

      --processing-details-tag-suggestions-availability <PROCESSING_DETAILS_TAG_SUGGESTIONS_AVAILABILITY>
          This value indicates whether keyword (tag) suggestions are available for the video. Tags can be added to a video's metadata to make it easier for other users to find the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.

      --processing-details-thumbnails-availability <PROCESSING_DETAILS_THUMBNAILS_AVAILABILITY>
          This value indicates whether thumbnail images have been generated for the video.

      --project-details <JSON>
          Deprecated. This field is no longer populated.

      --recording-details <JSON>
          Recording information associated with the video.

      --recording-details-location-altitude <RECORDING_DETAILS_LOCATION_ALTITUDE>
          Altitude above the reference ellipsoid, in meters.

      --recording-details-location-description <RECORDING_DETAILS_LOCATION_DESCRIPTION>
          The text description of the location where the video was recorded.

      --recording-details-location-latitude <RECORDING_DETAILS_LOCATION_LATITUDE>
          Latitude in degrees.

      --recording-details-location-longitude <RECORDING_DETAILS_LOCATION_LONGITUDE>
          Longitude in degrees.

      --recording-details-recording-date <RECORDING_DETAILS_RECORDING_DATE>
          The date and time when the video was recorded.

      --snippet <JSON>
          Basic details about a video, including title, description, uploader, thumbnails and category.

      --snippet-category-id <SNIPPET_CATEGORY_ID>
          The YouTube [video category](https://developers.google.com/youtube/data/reference/rest/v3/videoCategories/list) associated with the video.

      --snippet-channel-id <SNIPPET_CHANNEL_ID>
          The ID that YouTube uses to uniquely identify the channel that the video was uploaded to.

      --snippet-channel-title <SNIPPET_CHANNEL_TITLE>
          Channel title for the channel that the video belongs to.

      --snippet-default-audio-language <SNIPPET_DEFAULT_AUDIO_LANGUAGE>
          The default_audio_language property specifies the language spoken in the video's default audio track.

      --snippet-default-language <SNIPPET_DEFAULT_LANGUAGE>
          The language of the videos's default snippet.

      --snippet-live-broadcast-content <SNIPPET_LIVE_BROADCAST_CONTENT>
          Indicates if the video is an upcoming/active live broadcast. Or it's "none" if the video is not an upcoming/active live broadcast. Choices: none, upcoming, live, completed.

      --snippet-localized-description <SNIPPET_LOCALIZED_DESCRIPTION>
          Localized version of the video's description.

      --snippet-localized-title <SNIPPET_LOCALIZED_TITLE>
          Localized version of the video's title.

      --snippet-published-at <SNIPPET_PUBLISHED_AT>
          The date and time when the video was uploaded.

      --snippet-thumbnails-default-height <SNIPPET_THUMBNAILS_DEFAULT_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-default-url <SNIPPET_THUMBNAILS_DEFAULT_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-default-width <SNIPPET_THUMBNAILS_DEFAULT_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-fhd-height <SNIPPET_THUMBNAILS_FHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-fhd-url <SNIPPET_THUMBNAILS_FHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-fhd-width <SNIPPET_THUMBNAILS_FHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-high-height <SNIPPET_THUMBNAILS_HIGH_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-high-url <SNIPPET_THUMBNAILS_HIGH_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-high-width <SNIPPET_THUMBNAILS_HIGH_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-maxres-height <SNIPPET_THUMBNAILS_MAXRES_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-maxres-url <SNIPPET_THUMBNAILS_MAXRES_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-maxres-width <SNIPPET_THUMBNAILS_MAXRES_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-medium-height <SNIPPET_THUMBNAILS_MEDIUM_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-medium-url <SNIPPET_THUMBNAILS_MEDIUM_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-medium-width <SNIPPET_THUMBNAILS_MEDIUM_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-qhd-height <SNIPPET_THUMBNAILS_QHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-qhd-url <SNIPPET_THUMBNAILS_QHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-qhd-width <SNIPPET_THUMBNAILS_QHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-height <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-url <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-shorts-aspect-ratio-width <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-standard-height <SNIPPET_THUMBNAILS_STANDARD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-standard-url <SNIPPET_THUMBNAILS_STANDARD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-standard-width <SNIPPET_THUMBNAILS_STANDARD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-uhd-height <SNIPPET_THUMBNAILS_UHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-uhd-url <SNIPPET_THUMBNAILS_UHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-uhd-width <SNIPPET_THUMBNAILS_UHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --stabilize[=<BOOL>]
          Should stabilize be applied to the upload.

      --statistics <JSON>
          Statistics about the video, such as the number of times the video was viewed or liked.

      --statistics-comment-count <STATISTICS_COMMENT_COUNT>
          The number of comments for the video.

      --statistics-dislike-count <STATISTICS_DISLIKE_COUNT>
          The number of users who have indicated that they disliked the video by giving it a negative rating.

      --statistics-favorite-count <STATISTICS_FAVORITE_COUNT>
          The number of users who currently have the video marked as a favorite video.

      --statistics-like-count <STATISTICS_LIKE_COUNT>
          The number of users who have indicated that they liked the video by giving it a positive rating.

      --statistics-view-count <STATISTICS_VIEW_COUNT>
          The number of times the video has been viewed. For all video formats (long-form, Live, and Shorts), `viewCount` counts views the moment a video begins to play (includes autoplay, hold the pointer over, and click/tap to play). For all video formats (long-form, Live, and Shorts), `viewCount` includes co-views when multiple people watch together on a television screen.

      --status-contains-synthetic-media[=<BOOL>]
          Indicates if the video contains altered or synthetic media.

      --status-embeddable[=<BOOL>]
          This value indicates if the video can be embedded on another website. @mutable youtube.videos.insert youtube.videos.update

      --status-failure-reason <STATUS_FAILURE_REASON>
          This value explains why a video failed to upload. This property is only present if the `uploadStatus` property indicates that the upload failed. Choices: conversion, invalidFile, emptyFile, tooSmall, codec, uploadAborted.

      --status-license <STATUS_LICENSE>
          The video's license. @mutable youtube.videos.insert youtube.videos.update Choices: youtube, creativeCommon.

      --status-made-for-kids[=<BOOL>]
          Request body field

      --status-public-stats-viewable[=<BOOL>]
          This value indicates if the extended video statistics on the watch page can be viewed by everyone. Note that the view count, likes, etc will still be visible if this is disabled. @mutable youtube.videos.insert youtube.videos.update

      --status-publish-at <STATUS_PUBLISH_AT>
          The date and time when the video is scheduled to publish. It can be set only if the privacy status of the video is private..

      --status-rejection-reason <STATUS_REJECTION_REASON>
          This value explains why YouTube rejected an uploaded video. This property is only present if the `uploadStatus` property indicates that the upload was rejected. Choices: copyright, inappropriate, duplicate, termsOfUse, uploaderAccountSuspended, length, claim, uploaderAccountClosed, trademark, legal.

      --status-self-declared-made-for-kids[=<BOOL>]
          Request body field

      --status-upload-status <STATUS_UPLOAD_STATUS>
          The status of the uploaded video. Choices: uploaded, processed, failed, rejected, deleted.

      --suggestions <JSON>
          Specifies suggestions on how to improve video content, including encoding hints, tag suggestions, and editor suggestions.

      --suggestions-editor-suggestions <SUGGESTIONS_EDITOR_SUGGESTIONS>...
          A list of video editing operations that might improve the video quality or playback experience of the uploaded video.

      --suggestions-processing-errors <SUGGESTIONS_PROCESSING_ERRORS>...
          A list of errors that will prevent YouTube from successfully processing the uploaded video video. These errors indicate that, regardless of the video's current processing status, eventually, that status will almost certainly be failed.

      --suggestions-processing-hints <SUGGESTIONS_PROCESSING_HINTS>...
          A list of suggestions that may improve YouTube's ability to process the video.

      --suggestions-processing-warnings <SUGGESTIONS_PROCESSING_WARNINGS>...
          A list of reasons why YouTube may have difficulty transcoding the uploaded video or that might result in an erroneous transcoding. These warnings are generated before YouTube actually processes the uploaded video file. In addition, they identify issues that are unlikely to cause the video processing to fail but that might cause problems such as sync issues, video artifacts, or a missing audio track.

      --suggestions-tag-suggestions <JSON>
          A list of keyword tags that could be added to the video's metadata to increase the likelihood that users will locate your video when searching or browsing on YouTube.

      --topic-details <JSON>
          Freebase topic information related to the video.

      --topic-details-relevant-topic-ids <TOPIC_DETAILS_RELEVANT_TOPIC_IDS>...
          Similar to topic_id, except that these topics are merely relevant to the video. These are topics that may be mentioned in, or appear in the video. You can retrieve information about each topic using Freebase Topic API.

      --topic-details-topic-categories <TOPIC_DETAILS_TOPIC_CATEGORIES>...
          A list of Wikipedia URLs that provide a high-level description of the video's content.

      --topic-details-topic-ids <TOPIC_DETAILS_TOPIC_IDS>...
          A list of Freebase topic IDs that are centrally associated with the video. These are topics that are centrally featured in the video, and it can be said that the video is mainly about each of these. You can retrieve information about each topic using the Freebase Topic API.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## videos.list

```text
Retrieves a list of resources, possibly filtered.

Input choices:
  When selecting records with --id, omit --max-results. Use --max to limit the returned records.
  When selecting records with --id, omit --page-token. Use --max to limit the returned records.
  When selecting records with --id, omit --max-pages. Use --max to limit the returned records.

Usage: joog [OPTIONS] youtube <ACCOUNT> videos.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --id <ID>...
          Return videos with the given ids.

      --part <PART>...
          The *part* parameter specifies a comma-separated list of one or more video resource properties that the API response will include. If the parameter identifies a property that contains child properties, the child properties will be included in the response. For example, in a video resource, the snippet property contains the channelId, title, description, tags, and categoryId properties. As such, if you set *part=snippet*, the API response will contain all of those properties.
          
          [default: snippet]

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

      --chart <CHART>
          Return the videos that are in the specified chart. Choices: chartUnspecified, mostPopular.

      --hl <HL>
          Stands for "host language". Specifies the localization language of the metadata to be filled into snippet.localized. The field is filled with the default metadata if there is no localization in the specified language. The parameter value must be a language code included in the list returned by the i18nLanguages.list method (e.g. en_US, es_MX).

      --locale <LOCALE>
          

      --max-height <MAX_HEIGHT>
          

      --max-results <MAX_RESULTS>
          The *maxResults* parameter specifies the maximum number of items that should be returned in the result set. *Note:* This parameter is supported for use in conjunction with the [`myRating`](https://developers.google.com/youtube/data/reference/rest/v3/videos/list#myRating) and [`chart`](https://developers.google.com/youtube/data/reference/rest/v3/videos/list#chart) parameters, but it is not supported for use in conjunction with the [`id`](https://developers.google.com/youtube/data/reference/rest/v3/videos/list#id) parameter.

      --max-width <MAX_WIDTH>
          Return the player with maximum height specified in

      --my-rating <MY_RATING>
          Return videos liked/disliked by the authenticated user. Does not support RateType.RATED_TYPE_NONE. Choices: none, like, dislike.

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --page-token <PAGE_TOKEN>
          The *pageToken* parameter identifies a specific page in the result set that should be returned. In an API response, the nextPageToken and prevPageToken properties identify other pages that could be retrieved. *Note:* This parameter is supported for use in conjunction with the [`myRating`](https://developers.google.com/youtube/data/reference/rest/v3/videos/list#myRating) and [`chart`](https://developers.google.com/youtube/data/reference/rest/v3/videos/list#chart) parameters, but it is not supported for use in conjunction with the [`id`](https://developers.google.com/youtube/data/reference/rest/v3/videos/list#id) parameter.

      --region-code <REGION_CODE>
          Use a chart that is specific to the specified region

      --video-category-id <VIDEO_CATEGORY_ID>
          Use chart that is specific to the specified video category

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## videos.rate

```text
Adds a like or dislike rating to a video or removes a rating from a video.

Usage: joog [OPTIONS] youtube <ACCOUNT> videos.rate [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          

      --rating <RATING>
           Choices: none, like, dislike.

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

## videos.update

```text
Updates an existing resource.

Usage: joog [OPTIONS] youtube <ACCOUNT> videos.update [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID that YouTube uses to uniquely identify the video.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --description <DESCRIPTION>
          The video's description. @mutable youtube.videos.insert youtube.videos.update

      --part <PART>...
          The *part* parameter serves two purposes in this operation. It identifies the properties that the write operation will set as well as the properties that the API response will include. Note that this method will override the existing values for all of the mutable properties that are contained in any parts that the parameter value specifies. For example, a video's privacy setting is contained in the status part. As such, if your request is updating a private video, and the request's part parameter value includes the status part, the video's privacy setting will be updated to whatever value the request body specifies. If the request body does not specify a value, the existing privacy setting will be removed and the video will revert to the default privacy setting. In addition, not all parts contain properties that can be set when inserting or updating a video. For example, the statistics object encapsulates statistics that YouTube calculates for a video and does not contain values that you can set or modify. If the parameter value specifies a part that does not contain mutable values, that part will still be included in the API response.
          
          [default: snippet]

      --privacy <PRIVACY>
          The video's privacy status. In a `videos.insert` request, if you don't specify a value for this property, the uploaded video's privacy status defaults to `public`. Videos uploaded from unverified API projects are not restricted to private viewing mode. Choices: public, unlisted, private.

      --status <JSON>
          Basic details about a video category, such as its localized title. Next Id: 20

      --tags <TAGS>...
          A list of keyword tags associated with the video. Tags may contain spaces.

      --title <TITLE>
          The video's title. @mutable youtube.videos.insert youtube.videos.update

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

      --age-gating <JSON>
          Request body field

      --age-gating-alcohol-content[=<BOOL>]
          Indicates whether or not the video has alcoholic beverage content. Only users of legal purchasing age in a particular country, as identified by ICAP, can view the content.

      --age-gating-restricted[=<BOOL>]
          Age-restricted trailers. For redband trailers and adult-rated video-games. Only users aged 18+ can view the content. The the field is true the content is restricted to viewers aged 18+. Otherwise The field won't be present.

      --age-gating-video-game-rating <AGE_GATING_VIDEO_GAME_RATING>
          Video game rating, if any. Choices: anyone, m15Plus, m16Plus, m17Plus.

      --brand-partner <JSON>
          Details about the brand partner linked to the video for Creator Initiated Linking (CIL). Next ID: 6

      --brand-partner-channel-handle <BRAND_PARTNER_CHANNEL_HANDLE>
          Required. Channel handle, must begin with "@"

      --brand-partner-channel-id <BRAND_PARTNER_CHANNEL_ID>
          Required. External Channel ID, must begin with "UC"

      --content-details <JSON>
          Details about the content of a YouTube Video.

      --content-details-caption <CONTENT_DETAILS_CAPTION>
          The value of captions indicates whether the video has captions or not. Choices: true, false.

      --content-details-content-rating-acb-rating <CONTENT_DETAILS_CONTENT_RATING_ACB_RATING>
          The video's Australian Classification Board (ACB) or Australian Communications and Media Authority (ACMA) rating. ACMA ratings are used to classify children's television programming. Choices: acbUnspecified, acbE, acbP, acbC, acbG, acbPg, acbM, acbMa15plus, acbR18plus, acbUnrated.

      --content-details-content-rating-agcom-rating <CONTENT_DETAILS_CONTENT_RATING_AGCOM_RATING>
          The video's rating from Italy's Autorità per le Garanzie nelle Comunicazioni (AGCOM). Choices: agcomUnspecified, agcomT, agcomVm14, agcomVm18, agcomUnrated.

      --content-details-content-rating-anatel-rating <CONTENT_DETAILS_CONTENT_RATING_ANATEL_RATING>
          The video's Anatel (Asociación Nacional de Televisión) rating for Chilean television. Choices: anatelUnspecified, anatelF, anatelI, anatelI7, anatelI10, anatelI12, anatelR, anatelA, anatelUnrated.

      --content-details-content-rating-bbfc-rating <CONTENT_DETAILS_CONTENT_RATING_BBFC_RATING>
          The video's British Board of Film Classification (BBFC) rating. Choices: bbfcUnspecified, bbfcU, bbfcPg, bbfc12a, bbfc12, bbfc15, bbfc18, bbfcR18, bbfcUnrated.

      --content-details-content-rating-bfvc-rating <CONTENT_DETAILS_CONTENT_RATING_BFVC_RATING>
          The video's rating from Thailand's Board of Film and Video Censors. Choices: bfvcUnspecified, bfvcG, bfvcE, bfvc13, bfvc15, bfvc18, bfvc20, bfvcB, bfvcUnrated.

      --content-details-content-rating-bmukk-rating <CONTENT_DETAILS_CONTENT_RATING_BMUKK_RATING>
          The video's rating from the Austrian Board of Media Classification (Bundesministerium für Unterricht, Kunst und Kultur). Choices: bmukkUnspecified, bmukkAa, bmukk6, bmukk8, bmukk10, bmukk12, bmukk14, bmukk16, bmukkUnrated.

      --content-details-content-rating-catv-rating <CONTENT_DETAILS_CONTENT_RATING_CATV_RATING>
          Rating system for Canadian TV - Canadian TV Classification System The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian English-language broadcasts. For more information, see the Canadian Broadcast Standards Council website. Choices: catvUnspecified, catvC, catvC8, catvG, catvPg, catv14plus, catv18plus, catvUnrated, catvE.

      --content-details-content-rating-catvfr-rating <CONTENT_DETAILS_CONTENT_RATING_CATVFR_RATING>
          The video's rating from the Canadian Radio-Television and Telecommunications Commission (CRTC) for Canadian French-language broadcasts. For more information, see the Canadian Broadcast Standards Council website. Choices: catvfrUnspecified, catvfrG, catvfr8plus, catvfr13plus, catvfr16plus, catvfr18plus, catvfrUnrated, catvfrE.

      --content-details-content-rating-cbfc-rating <CONTENT_DETAILS_CONTENT_RATING_CBFC_RATING>
          The video's Central Board of Film Certification (CBFC - India) rating. Choices: cbfcUnspecified, cbfcU, cbfcUA, cbfcUA7plus, cbfcUA13plus, cbfcUA16plus, cbfcA, cbfcS, cbfcUnrated.

      --content-details-content-rating-ccc-rating <CONTENT_DETAILS_CONTENT_RATING_CCC_RATING>
          The video's Consejo de Calificación Cinematográfica (Chile) rating. Choices: cccUnspecified, cccTe, ccc6, ccc14, ccc18, ccc18v, ccc18s, cccUnrated.

      --content-details-content-rating-cce-rating <CONTENT_DETAILS_CONTENT_RATING_CCE_RATING>
          The video's rating from Portugal's Comissão de Classificação de Espect´culos. Choices: cceUnspecified, cceM4, cceM6, cceM12, cceM16, cceM18, cceUnrated, cceM14.

      --content-details-content-rating-chfilm-rating <CONTENT_DETAILS_CONTENT_RATING_CHFILM_RATING>
          The video's rating in Switzerland. Choices: chfilmUnspecified, chfilm0, chfilm6, chfilm12, chfilm16, chfilm18, chfilmUnrated.

      --content-details-content-rating-chvrs-rating <CONTENT_DETAILS_CONTENT_RATING_CHVRS_RATING>
          The video's Canadian Home Video Rating System (CHVRS) rating. Choices: chvrsUnspecified, chvrsG, chvrsPg, chvrs14a, chvrs18a, chvrsR, chvrsE, chvrsUnrated.

      --content-details-content-rating-cicf-rating <CONTENT_DETAILS_CONTENT_RATING_CICF_RATING>
          The video's rating from the Commission de Contrôle des Films (Belgium). Choices: cicfUnspecified, cicfE, cicfKtEa, cicfKntEna, cicfUnrated.

      --content-details-content-rating-cna-rating <CONTENT_DETAILS_CONTENT_RATING_CNA_RATING>
          The video's rating from Romania's CONSILIUL NATIONAL AL AUDIOVIZUALULUI (CNA). Choices: cnaUnspecified, cnaAp, cna12, cna15, cna18, cna18plus, cnaUnrated.

      --content-details-content-rating-cnc-rating <CONTENT_DETAILS_CONTENT_RATING_CNC_RATING>
          Rating system in France - Commission de classification cinematographique Choices: cncUnspecified, cncT, cnc10, cnc12, cnc16, cnc18, cncE, cncInterdiction, cncUnrated.

      --content-details-content-rating-csa-rating <CONTENT_DETAILS_CONTENT_RATING_CSA_RATING>
          The video's rating from France's Conseil supérieur de l’audiovisuel, which rates broadcast content. Choices: csaUnspecified, csaT, csa10, csa12, csa16, csa18, csaInterdiction, csaUnrated.

      --content-details-content-rating-cscf-rating <CONTENT_DETAILS_CONTENT_RATING_CSCF_RATING>
          The video's rating from Luxembourg's Commission de surveillance de la classification des films (CSCF). Choices: cscfUnspecified, cscfAl, cscfA, cscf6, cscf9, cscf12, cscf16, cscf18, cscfUnrated.

      --content-details-content-rating-czfilm-rating <CONTENT_DETAILS_CONTENT_RATING_CZFILM_RATING>
          The video's rating in the Czech Republic. Choices: czfilmUnspecified, czfilmU, czfilm12, czfilm14, czfilm18, czfilmUnrated.

      --content-details-content-rating-djctq-rating <CONTENT_DETAILS_CONTENT_RATING_DJCTQ_RATING>
          The video's Departamento de Justiça, Classificação, Qualificação e Títulos (DJCQT - Brazil) rating. Choices: djctqUnspecified, djctqL, djctq10, djctq12, djctq14, djctq16, djctq18, djctqEr, djctqL10, djctqL12, djctqL14, djctqL16, djctqL18, djctq1012, djctq1014, djctq1016, djctq1018, djctq1214, djctq1216, djctq1218, djctq1416, djctq1418, djctq1618, djctqUnrated.

      --content-details-content-rating-djctq-rating-reasons <CONTENT_DETAILS_CONTENT_RATING_DJCTQ_RATING_REASONS>...
          Reasons that explain why the video received its DJCQT (Brazil) rating.

      --content-details-content-rating-ecbmct-rating <CONTENT_DETAILS_CONTENT_RATING_ECBMCT_RATING>
          Rating system in Turkey - Evaluation and Classification Board of the Ministry of Culture and Tourism Choices: ecbmctUnspecified, ecbmctG, ecbmct7a, ecbmct7plus, ecbmct13a, ecbmct13plus, ecbmct15a, ecbmct15plus, ecbmct18plus, ecbmctUnrated.

      --content-details-content-rating-eefilm-rating <CONTENT_DETAILS_CONTENT_RATING_EEFILM_RATING>
          The video's rating in Estonia. Choices: eefilmUnspecified, eefilmPere, eefilmL, eefilmMs6, eefilmK6, eefilmMs12, eefilmK12, eefilmK14, eefilmK16, eefilmUnrated.

      --content-details-content-rating-egfilm-rating <CONTENT_DETAILS_CONTENT_RATING_EGFILM_RATING>
          The video's rating in Egypt. Choices: egfilmUnspecified, egfilmGn, egfilm18, egfilmBn, egfilmUnrated.

      --content-details-content-rating-eirin-rating <CONTENT_DETAILS_CONTENT_RATING_EIRIN_RATING>
          The video's Eirin (映倫) rating. Eirin is the Japanese rating system. Choices: eirinUnspecified, eirinG, eirinPg12, eirinR15plus, eirinR18plus, eirinUnrated.

      --content-details-content-rating-fcbm-rating <CONTENT_DETAILS_CONTENT_RATING_FCBM_RATING>
          The video's rating from Malaysia's Film Censorship Board. Choices: fcbmUnspecified, fcbmU, fcbmPg13, fcbmP13, fcbm18, fcbm18sx, fcbm18pa, fcbm18sg, fcbm18pl, fcbmUnrated.

      --content-details-content-rating-fco-rating <CONTENT_DETAILS_CONTENT_RATING_FCO_RATING>
          The video's rating from Hong Kong's Office for Film, Newspaper and Article Administration. Choices: fcoUnspecified, fcoI, fcoIia, fcoIib, fcoIi, fcoIii, fcoUnrated.

      --content-details-content-rating-fmoc-rating <CONTENT_DETAILS_CONTENT_RATING_FMOC_RATING>
          This property has been deprecated. Use the contentDetails.contentRating.cncRating instead. Choices: fmocUnspecified, fmocU, fmoc10, fmoc12, fmoc16, fmoc18, fmocE, fmocUnrated.

      --content-details-content-rating-fpb-rating <CONTENT_DETAILS_CONTENT_RATING_FPB_RATING>
          The video's rating from South Africa's Film and Publication Board. Choices: fpbUnspecified, fpbA, fpbPg, fpb79Pg, fpb1012Pg, fpb13, fpb16, fpb18, fpbX18, fpbXx, fpbUnrated, fpb10.

      --content-details-content-rating-fpb-rating-reasons <CONTENT_DETAILS_CONTENT_RATING_FPB_RATING_REASONS>...
          Reasons that explain why the video received its FPB (South Africa) rating.

      --content-details-content-rating-fsk-rating <CONTENT_DETAILS_CONTENT_RATING_FSK_RATING>
          The video's Freiwillige Selbstkontrolle der Filmwirtschaft (FSK - Germany) rating. Choices: fskUnspecified, fsk0, fsk6, fsk12, fsk16, fsk18, fskUnrated.

      --content-details-content-rating-grfilm-rating <CONTENT_DETAILS_CONTENT_RATING_GRFILM_RATING>
          The video's rating in Greece. Choices: grfilmUnspecified, grfilmK, grfilmE, grfilmK12, grfilmK13, grfilmK15, grfilmK17, grfilmK18, grfilmUnrated.

      --content-details-content-rating-icaa-rating <CONTENT_DETAILS_CONTENT_RATING_ICAA_RATING>
          The video's Instituto de la Cinematografía y de las Artes Audiovisuales (ICAA - Spain) rating. Choices: icaaUnspecified, icaaApta, icaa7, icaa12, icaa13, icaa16, icaa18, icaaX, icaaUnrated.

      --content-details-content-rating-ifco-rating <CONTENT_DETAILS_CONTENT_RATING_IFCO_RATING>
          The video's Irish Film Classification Office (IFCO - Ireland) rating. See the IFCO website for more information. Choices: ifcoUnspecified, ifcoG, ifcoPg, ifco12, ifco12a, ifco15, ifco15a, ifco16, ifco18, ifcoUnrated.

      --content-details-content-rating-ilfilm-rating <CONTENT_DETAILS_CONTENT_RATING_ILFILM_RATING>
          The video's rating in Israel. Choices: ilfilmUnspecified, ilfilmAa, ilfilm12, ilfilm14, ilfilm16, ilfilm18, ilfilmUnrated.

      --content-details-content-rating-incaa-rating <CONTENT_DETAILS_CONTENT_RATING_INCAA_RATING>
          The video's INCAA (Instituto Nacional de Cine y Artes Audiovisuales - Argentina) rating. Choices: incaaUnspecified, incaaAtp, incaaSam13, incaaSam16, incaaSam18, incaaC, incaaUnrated.

      --content-details-content-rating-kfcb-rating <CONTENT_DETAILS_CONTENT_RATING_KFCB_RATING>
          The video's rating from the Kenya Film Classification Board. Choices: kfcbUnspecified, kfcbG, kfcbPg, kfcb16plus, kfcbR, kfcbUnrated.

      --content-details-content-rating-kijkwijzer-rating <CONTENT_DETAILS_CONTENT_RATING_KIJKWIJZER_RATING>
          The video's NICAM/Kijkwijzer rating from the Nederlands Instituut voor de Classificatie van Audiovisuele Media (Netherlands). Choices: kijkwijzerUnspecified, kijkwijzerAl, kijkwijzer6, kijkwijzer9, kijkwijzer12, kijkwijzer16, kijkwijzer18, kijkwijzerUnrated.

      --content-details-content-rating-kmrb-rating <CONTENT_DETAILS_CONTENT_RATING_KMRB_RATING>
          The video's Korea Media Rating Board (영상물등급위원회) rating. The KMRB rates videos in South Korea. Choices: kmrbUnspecified, kmrbAll, kmrb12plus, kmrb15plus, kmrbTeenr, kmrbR, kmrbUnrated.

      --content-details-content-rating-lsf-rating <CONTENT_DETAILS_CONTENT_RATING_LSF_RATING>
          The video's rating from Indonesia's Lembaga Sensor Film. Choices: lsfUnspecified, lsfSu, lsfA, lsfBo, lsf13, lsfR, lsf17, lsfD, lsf21, lsfUnrated.

      --content-details-content-rating-mccaa-rating <CONTENT_DETAILS_CONTENT_RATING_MCCAA_RATING>
          The video's rating from Malta's Film Age-Classification Board. Choices: mccaaUnspecified, mccaaU, mccaaPg, mccaa12a, mccaa12, mccaa14, mccaa15, mccaa16, mccaa18, mccaaUnrated.

      --content-details-content-rating-mccyp-rating <CONTENT_DETAILS_CONTENT_RATING_MCCYP_RATING>
          The video's rating from the Danish Film Institute's (Det Danske Filminstitut) Media Council for Children and Young People. Choices: mccypUnspecified, mccypA, mccyp7, mccyp11, mccyp15, mccypUnrated.

      --content-details-content-rating-mcst-rating <CONTENT_DETAILS_CONTENT_RATING_MCST_RATING>
          The video's rating system for Vietnam - MCST Choices: mcstUnspecified, mcstP, mcst0, mcstC13, mcstC16, mcst16plus, mcstC18, mcstGPg, mcstUnrated.

      --content-details-content-rating-mda-rating <CONTENT_DETAILS_CONTENT_RATING_MDA_RATING>
          The video's rating from Singapore's Media Development Authority (MDA) and, specifically, it's Board of Film Censors (BFC). Choices: mdaUnspecified, mdaG, mdaPg, mdaPg13, mdaNc16, mdaM18, mdaR21, mdaUnrated.

      --content-details-content-rating-medietilsynet-rating <CONTENT_DETAILS_CONTENT_RATING_MEDIETILSYNET_RATING>
          The video's rating from Medietilsynet, the Norwegian Media Authority. Choices: medietilsynetUnspecified, medietilsynetA, medietilsynet6, medietilsynet7, medietilsynet9, medietilsynet11, medietilsynet12, medietilsynet15, medietilsynet18, medietilsynetUnrated.

      --content-details-content-rating-meku-rating <CONTENT_DETAILS_CONTENT_RATING_MEKU_RATING>
          The video's rating from Finland's Kansallinen Audiovisuaalinen Instituutti (National Audiovisual Institute). Choices: mekuUnspecified, mekuS, meku7, meku12, meku16, meku18, mekuUnrated.

      --content-details-content-rating-mena-mpaa-rating <CONTENT_DETAILS_CONTENT_RATING_MENA_MPAA_RATING>
          MENA content rating, using the MPAA rating categories. Choices: menaMpaaUnspecified, menaMpaaG, menaMpaaPg, menaMpaaPg13, menaMpaaR, menaMpaaUnrated.

      --content-details-content-rating-mibac-rating <CONTENT_DETAILS_CONTENT_RATING_MIBAC_RATING>
          The video's rating from the Ministero dei Beni e delle Attività Culturali e del Turismo (Italy). Choices: mibacUnspecified, mibacT, mibacVap, mibacVm6, mibacVm12, mibacVm14, mibacVm16, mibacVm18, mibacUnrated.

      --content-details-content-rating-moc-rating <CONTENT_DETAILS_CONTENT_RATING_MOC_RATING>
          The video's Ministerio de Cultura (Colombia) rating. Choices: mocUnspecified, mocE, mocT, moc7, moc12, moc15, moc18, mocX, mocBanned, mocUnrated.

      --content-details-content-rating-moctw-rating <CONTENT_DETAILS_CONTENT_RATING_MOCTW_RATING>
          The video's rating from Taiwan's Ministry of Culture (文化部). Choices: moctwUnspecified, moctwG, moctwP, moctwPg, moctwR, moctwUnrated, moctwR12, moctwR15.

      --content-details-content-rating-mpaa-rating <CONTENT_DETAILS_CONTENT_RATING_MPAA_RATING>
          The video's Motion Picture Association of America (MPAA) rating. Choices: mpaaUnspecified, mpaaG, mpaaPg, mpaaPg13, mpaaR, mpaaNc17, mpaaX, mpaaUnrated.

      --content-details-content-rating-mpaat-rating <CONTENT_DETAILS_CONTENT_RATING_MPAAT_RATING>
          The rating system for trailer, DVD, and Ad in the US. See http://movielabs.com/md/ratings/v2.3/html/US_MPAAT_Ratings.html. Choices: mpaatUnspecified, mpaatGb, mpaatRb.

      --content-details-content-rating-mtrcb-rating <CONTENT_DETAILS_CONTENT_RATING_MTRCB_RATING>
          The video's rating from the Movie and Television Review and Classification Board (Philippines). Choices: mtrcbUnspecified, mtrcbG, mtrcbPg, mtrcbR13, mtrcbR16, mtrcbR18, mtrcbX, mtrcbUnrated.

      --content-details-content-rating-nbc-rating <CONTENT_DETAILS_CONTENT_RATING_NBC_RATING>
          The video's rating from the Maldives National Bureau of Classification. Choices: nbcUnspecified, nbcG, nbcPg, nbc12plus, nbc15plus, nbc18plus, nbc18plusr, nbcPu, nbcUnrated.

      --content-details-content-rating-nbcpl-rating <CONTENT_DETAILS_CONTENT_RATING_NBCPL_RATING>
          The video's rating in Poland. Choices: nbcplUnspecified, nbcplI, nbcplIi, nbcplIii, nbcplIv, nbcpl18plus, nbcplUnrated.

      --content-details-content-rating-nfrc-rating <CONTENT_DETAILS_CONTENT_RATING_NFRC_RATING>
          The video's rating from the Bulgarian National Film Center. Choices: nfrcUnspecified, nfrcA, nfrcB, nfrcC, nfrcD, nfrcX, nfrcUnrated.

      --content-details-content-rating-nfvcb-rating <CONTENT_DETAILS_CONTENT_RATING_NFVCB_RATING>
          The video's rating from Nigeria's National Film and Video Censors Board. Choices: nfvcbUnspecified, nfvcbG, nfvcbPg, nfvcb12, nfvcb12a, nfvcb15, nfvcb18, nfvcbRe, nfvcbUnrated.

      --content-details-content-rating-nkclv-rating <CONTENT_DETAILS_CONTENT_RATING_NKCLV_RATING>
          The video's rating from the Nacionãlais Kino centrs (National Film Centre of Latvia). Choices: nkclvUnspecified, nkclvU, nkclv7plus, nkclv12plus, nkclv16plus, nkclv18plus, nkclvUnrated.

      --content-details-content-rating-nmc-rating <CONTENT_DETAILS_CONTENT_RATING_NMC_RATING>
          The National Media Council ratings system for United Arab Emirates. Choices: nmcUnspecified, nmcG, nmcPg, nmcPg13, nmcPg15, nmc15plus, nmc18plus, nmc18tc, nmcUnrated.

      --content-details-content-rating-oflc-rating <CONTENT_DETAILS_CONTENT_RATING_OFLC_RATING>
          The video's Office of Film and Literature Classification (OFLC - New Zealand) rating. Choices: oflcUnspecified, oflcG, oflcPg, oflcM, oflcR13, oflcR15, oflcR16, oflcR18, oflcUnrated, oflcRp13, oflcRp16, oflcRp18.

      --content-details-content-rating-pefilm-rating <CONTENT_DETAILS_CONTENT_RATING_PEFILM_RATING>
          The video's rating in Peru. Choices: pefilmUnspecified, pefilmPt, pefilmPg, pefilm14, pefilm18, pefilmUnrated.

      --content-details-content-rating-rcnof-rating <CONTENT_DETAILS_CONTENT_RATING_RCNOF_RATING>
          The video's rating from the Hungarian Nemzeti Filmiroda, the Rating Committee of the National Office of Film. Choices: rcnofUnspecified, rcnofI, rcnofIi, rcnofIii, rcnofIv, rcnofV, rcnofVi, rcnofUnrated.

      --content-details-content-rating-resorteviolencia-rating <CONTENT_DETAILS_CONTENT_RATING_RESORTEVIOLENCIA_RATING>
          The video's rating in Venezuela. Choices: resorteviolenciaUnspecified, resorteviolenciaA, resorteviolenciaB, resorteviolenciaC, resorteviolenciaD, resorteviolenciaE, resorteviolenciaUnrated.

      --content-details-content-rating-rtc-rating <CONTENT_DETAILS_CONTENT_RATING_RTC_RATING>
          The video's General Directorate of Radio, Television and Cinematography (Mexico) rating. Choices: rtcUnspecified, rtcAa, rtcA, rtcB, rtcB15, rtcC, rtcD, rtcUnrated.

      --content-details-content-rating-rte-rating <CONTENT_DETAILS_CONTENT_RATING_RTE_RATING>
          The video's rating from Ireland's Raidió Teilifís Éireann. Choices: rteUnspecified, rteGa, rteCh, rtePs, rteMa, rteUnrated.

      --content-details-content-rating-russia-rating <CONTENT_DETAILS_CONTENT_RATING_RUSSIA_RATING>
          The video's National Film Registry of the Russian Federation (MKRF - Russia) rating. Choices: russiaUnspecified, russia0, russia6, russia12, russia16, russia18, russiaUnrated.

      --content-details-content-rating-skfilm-rating <CONTENT_DETAILS_CONTENT_RATING_SKFILM_RATING>
          The video's rating in Slovakia. Choices: skfilmUnspecified, skfilmG, skfilmP2, skfilmP5, skfilmP8, skfilmUnrated.

      --content-details-content-rating-smais-rating <CONTENT_DETAILS_CONTENT_RATING_SMAIS_RATING>
          The video's rating in Iceland. Choices: smaisUnspecified, smaisL, smais7, smais12, smais14, smais16, smais18, smaisUnrated.

      --content-details-content-rating-smsa-rating <CONTENT_DETAILS_CONTENT_RATING_SMSA_RATING>
          The video's rating from Statens medieråd (Sweden's National Media Council). Choices: smsaUnspecified, smsaA, smsa7, smsa11, smsa15, smsaUnrated.

      --content-details-content-rating-tvpg-rating <CONTENT_DETAILS_CONTENT_RATING_TVPG_RATING>
          The video's TV Parental Guidelines (TVPG) rating. Choices: tvpgUnspecified, tvpgY, tvpgY7, tvpgY7Fv, tvpgG, tvpgPg, pg14, tvpgMa, tvpgUnrated.

      --content-details-content-rating-yt-rating <CONTENT_DETAILS_CONTENT_RATING_YT_RATING>
          A rating that YouTube uses to identify age-restricted content. Choices: ytUnspecified, ytAgeRestricted.

      --content-details-country-restriction-allowed[=<BOOL>]
          The value of allowed indicates whether the access to the policy is allowed or denied by default.

      --content-details-country-restriction-exception <CONTENT_DETAILS_COUNTRY_RESTRICTION_EXCEPTION>...
          A list of region codes that identify countries where the default policy do not apply.

      --content-details-definition <CONTENT_DETAILS_DEFINITION>
          The value of definition indicates whether the video is available in high definition or only in standard definition. Choices: sd, hd.

      --content-details-dimension <CONTENT_DETAILS_DIMENSION>
          The value of dimension indicates whether the video is available in 3D or in 2D.

      --content-details-duration <CONTENT_DETAILS_DURATION>
          The length of the video. The tag value is an ISO 8601 duration in the format PT#M#S, in which the letters PT indicate that the value specifies a period of time, and the letters M and S refer to length in minutes and seconds, respectively. The # characters preceding the M and S letters are both integers that specify the number of minutes (or seconds) of the video. For example, a value of PT15M51S indicates that the video is 15 minutes and 51 seconds long.

      --content-details-has-custom-thumbnail[=<BOOL>]
          Indicates whether the video uploader has provided a custom thumbnail image for the video. This property is only visible to the video uploader.

      --content-details-licensed-content[=<BOOL>]
          The value of is_license_content indicates whether the video is licensed content.

      --content-details-projection <CONTENT_DETAILS_PROJECTION>
          Specifies the projection format of the video. Choices: rectangular, 360.

      --content-details-region-restriction-allowed <CONTENT_DETAILS_REGION_RESTRICTION_ALLOWED>...
          A list of region codes that identify countries where the video is viewable. If this property is present and a country is not listed in its value, then the video is blocked from appearing in that country. If this property is present and contains an empty list, the video is blocked in all countries.

      --content-details-region-restriction-blocked <CONTENT_DETAILS_REGION_RESTRICTION_BLOCKED>...
          A list of region codes that identify countries where the video is blocked. If this property is present and a country is not listed in its value, then the video is viewable in that country. If this property is present and contains an empty list, the video is viewable in all countries.

      --etag <ETAG>
          Etag of this resource.

      --file-details <JSON>
          Describes original video file properties, including technical details about audio and video streams, but also metadata information like content length, digitization time, or geotagging information.

      --file-details-audio-streams <JSON>
          A list of audio streams contained in the uploaded video file. Each item in the list contains detailed metadata about an audio stream.

      --file-details-bitrate-bps <FILE_DETAILS_BITRATE_BPS>
          The uploaded video file's combined (video and audio) bitrate in bits per second.

      --file-details-container <FILE_DETAILS_CONTAINER>
          The uploaded video file's container format.

      --file-details-creation-time <FILE_DETAILS_CREATION_TIME>
          The date and time when the uploaded video file was created. The value is specified in ISO 8601 format. Currently, the following ISO 8601 formats are supported: - Date only: YYYY-MM-DD - Naive time: YYYY-MM-DDTHH:MM:SS - Time with timezone: YYYY-MM-DDTHH:MM:SS+HH:MM

      --file-details-duration-ms <FILE_DETAILS_DURATION_MS>
          The length of the uploaded video in milliseconds.

      --file-details-file-name <FILE_DETAILS_FILE_NAME>
          The uploaded file's name. This field is present whether a video file or another type of file was uploaded.

      --file-details-file-size <FILE_DETAILS_FILE_SIZE>
          The uploaded file's size in bytes. This field is present whether a video file or another type of file was uploaded.

      --file-details-file-type <FILE_DETAILS_FILE_TYPE>
          The uploaded file's type as detected by YouTube's video processing engine. Currently, YouTube only processes video files, but this field is present whether a video file or another type of file was uploaded. Choices: video, audio, image, archive, document, project, other.

      --file-details-video-streams <JSON>
          A list of video streams contained in the uploaded video file. Each item in the list contains detailed metadata about a video stream.

      --kind <KIND>
          Identifies what kind of resource this is. Value: the fixed string "youtube#video".

      --live-streaming-details <JSON>
          Details about the live streaming metadata.

      --live-streaming-details-active-live-chat-id <LIVE_STREAMING_DETAILS_ACTIVE_LIVE_CHAT_ID>
          The ID of the currently active live chat attached to this video. This field is filled only if the video is a currently live broadcast that has live chat. Once the broadcast transitions to complete this field will be removed and the live chat closed down. For persistent broadcasts that live chat id will no longer be tied to this video but rather to the new video being displayed at the persistent page.

      --live-streaming-details-actual-end-time <LIVE_STREAMING_DETAILS_ACTUAL_END_TIME>
          The time that the broadcast actually ended. This value will not be available until the broadcast is over.

      --live-streaming-details-actual-start-time <LIVE_STREAMING_DETAILS_ACTUAL_START_TIME>
          The time that the broadcast actually started. This value will not be available until the broadcast begins.

      --live-streaming-details-concurrent-viewers <LIVE_STREAMING_DETAILS_CONCURRENT_VIEWERS>
          The number of viewers watching the broadcast at the time of the request. This count includes co-viewers when multiple people watch together on a television screen. The property and its value will be present if the broadcast has current viewers and the broadcast owner has not hidden the viewcount for the video. Note that YouTube stops tracking the number of concurrent viewers for a broadcast when the broadcast ends. So, this property would not identify the number of viewers watching an archived video of a live broadcast that already ended. The concurrent viewer counts that the YouTube Data API returns might differ from the processed, despammed concurrent viewer counts available through YouTube Analytics. Learn more about live streaming metrics in the [YouTube Help Center](https://support.google.com/youtube/answer/2853833).

      --live-streaming-details-scheduled-end-time <LIVE_STREAMING_DETAILS_SCHEDULED_END_TIME>
          The time that the broadcast is scheduled to end. If the value is empty or the property is not present, then the broadcast is scheduled to continue indefinitely.

      --live-streaming-details-scheduled-start-time <LIVE_STREAMING_DETAILS_SCHEDULED_START_TIME>
          The time that the broadcast is scheduled to begin.

      --localizations <JSON>
          The localizations object contains localized versions of the basic details about the video, such as its title and description.

      --monetization-details <JSON>
          Details about monetization of a YouTube Video.

      --monetization-details-access-allowed[=<BOOL>]
          The value of allowed indicates whether the access to the policy is allowed or denied by default.

      --monetization-details-access-exception <MONETIZATION_DETAILS_ACCESS_EXCEPTION>...
          A list of region codes that identify countries where the default policy do not apply.

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The actual CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --paid-product-placement-details <JSON>
          Details about paid content, such as paid product placement, sponsorships or endorsement, contained in a YouTube video and a method to inform viewers of paid promotion. This data can only be retrieved by the video owner.

      --paid-product-placement-details-has-paid-product-placement[=<BOOL>]
          This boolean represents whether the video contains Paid Product Placement, Studio equivalent: https://screenshot.googleplex.com/4Me79DE6AfT2ktp.png

      --player <JSON>
          Player to be used for a video playback.

      --player-embed-height <PLAYER_EMBED_HEIGHT>
          Request body field

      --player-embed-html <PLAYER_EMBED_HTML>
          An  tag that embeds a player that will play the video.

      --player-embed-width <PLAYER_EMBED_WIDTH>
          The embed width

      --processing-details <JSON>
          Describes processing status and progress and availability of some other Video resource parts.

      --processing-details-editor-suggestions-availability <PROCESSING_DETAILS_EDITOR_SUGGESTIONS_AVAILABILITY>
          This value indicates whether video editing suggestions, which might improve video quality or the playback experience, are available for the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.

      --processing-details-file-details-availability <PROCESSING_DETAILS_FILE_DETAILS_AVAILABILITY>
          This value indicates whether file details are available for the uploaded video. You can retrieve a video's file details by requesting the fileDetails part in your videos.list() request.

      --processing-details-processing-failure-reason <PROCESSING_DETAILS_PROCESSING_FAILURE_REASON>
          The reason that YouTube failed to process the video. This property will only have a value if the processingStatus property's value is failed. Choices: uploadFailed, transcodeFailed, streamingFailed, other.

      --processing-details-processing-issues-availability <PROCESSING_DETAILS_PROCESSING_ISSUES_AVAILABILITY>
          This value indicates whether the video processing engine has generated suggestions that might improve YouTube's ability to process the the video, warnings that explain video processing problems, or errors that cause video processing problems. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.

      --processing-details-processing-progress-parts-processed <PROCESSING_DETAILS_PROCESSING_PROGRESS_PARTS_PROCESSED>
          The number of parts of the video that YouTube has already processed. You can estimate the percentage of the video that YouTube has already processed by calculating: 100 * parts_processed / parts_total Note that since the estimated number of parts could increase without a corresponding increase in the number of parts that have already been processed, it is possible that the calculated progress could periodically decrease while YouTube processes a video.

      --processing-details-processing-progress-parts-total <PROCESSING_DETAILS_PROCESSING_PROGRESS_PARTS_TOTAL>
          An estimate of the total number of parts that need to be processed for the video. The number may be updated with more precise estimates while YouTube processes the video.

      --processing-details-processing-progress-time-left-ms <PROCESSING_DETAILS_PROCESSING_PROGRESS_TIME_LEFT_MS>
          An estimate of the amount of time, in millseconds, that YouTube needs to finish processing the video.

      --processing-details-processing-status <PROCESSING_DETAILS_PROCESSING_STATUS>
          The video's processing status. This value indicates whether YouTube was able to process the video or if the video is still being processed. Choices: processing, succeeded, failed, terminated.

      --processing-details-tag-suggestions-availability <PROCESSING_DETAILS_TAG_SUGGESTIONS_AVAILABILITY>
          This value indicates whether keyword (tag) suggestions are available for the video. Tags can be added to a video's metadata to make it easier for other users to find the video. You can retrieve these suggestions by requesting the suggestions part in your videos.list() request.

      --processing-details-thumbnails-availability <PROCESSING_DETAILS_THUMBNAILS_AVAILABILITY>
          This value indicates whether thumbnail images have been generated for the video.

      --project-details <JSON>
          Deprecated. This field is no longer populated.

      --recording-details <JSON>
          Recording information associated with the video.

      --recording-details-location-altitude <RECORDING_DETAILS_LOCATION_ALTITUDE>
          Altitude above the reference ellipsoid, in meters.

      --recording-details-location-description <RECORDING_DETAILS_LOCATION_DESCRIPTION>
          The text description of the location where the video was recorded.

      --recording-details-location-latitude <RECORDING_DETAILS_LOCATION_LATITUDE>
          Latitude in degrees.

      --recording-details-location-longitude <RECORDING_DETAILS_LOCATION_LONGITUDE>
          Longitude in degrees.

      --recording-details-recording-date <RECORDING_DETAILS_RECORDING_DATE>
          The date and time when the video was recorded.

      --snippet <JSON>
          Basic details about a video, including title, description, uploader, thumbnails and category.

      --snippet-category-id <SNIPPET_CATEGORY_ID>
          The YouTube [video category](https://developers.google.com/youtube/data/reference/rest/v3/videoCategories/list) associated with the video.

      --snippet-channel-id <SNIPPET_CHANNEL_ID>
          The ID that YouTube uses to uniquely identify the channel that the video was uploaded to.

      --snippet-channel-title <SNIPPET_CHANNEL_TITLE>
          Channel title for the channel that the video belongs to.

      --snippet-default-audio-language <SNIPPET_DEFAULT_AUDIO_LANGUAGE>
          The default_audio_language property specifies the language spoken in the video's default audio track.

      --snippet-default-language <SNIPPET_DEFAULT_LANGUAGE>
          The language of the videos's default snippet.

      --snippet-live-broadcast-content <SNIPPET_LIVE_BROADCAST_CONTENT>
          Indicates if the video is an upcoming/active live broadcast. Or it's "none" if the video is not an upcoming/active live broadcast. Choices: none, upcoming, live, completed.

      --snippet-localized-description <SNIPPET_LOCALIZED_DESCRIPTION>
          Localized version of the video's description.

      --snippet-localized-title <SNIPPET_LOCALIZED_TITLE>
          Localized version of the video's title.

      --snippet-published-at <SNIPPET_PUBLISHED_AT>
          The date and time when the video was uploaded.

      --snippet-thumbnails-default-height <SNIPPET_THUMBNAILS_DEFAULT_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-default-url <SNIPPET_THUMBNAILS_DEFAULT_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-default-width <SNIPPET_THUMBNAILS_DEFAULT_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-fhd-height <SNIPPET_THUMBNAILS_FHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-fhd-url <SNIPPET_THUMBNAILS_FHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-fhd-width <SNIPPET_THUMBNAILS_FHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-high-height <SNIPPET_THUMBNAILS_HIGH_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-high-url <SNIPPET_THUMBNAILS_HIGH_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-high-width <SNIPPET_THUMBNAILS_HIGH_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-maxres-height <SNIPPET_THUMBNAILS_MAXRES_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-maxres-url <SNIPPET_THUMBNAILS_MAXRES_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-maxres-width <SNIPPET_THUMBNAILS_MAXRES_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-medium-height <SNIPPET_THUMBNAILS_MEDIUM_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-medium-url <SNIPPET_THUMBNAILS_MEDIUM_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-medium-width <SNIPPET_THUMBNAILS_MEDIUM_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-qhd-height <SNIPPET_THUMBNAILS_QHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-qhd-url <SNIPPET_THUMBNAILS_QHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-qhd-width <SNIPPET_THUMBNAILS_QHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-height <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-shorts-aspect-ratio-url <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-shorts-aspect-ratio-width <SNIPPET_THUMBNAILS_SHORTS_ASPECT_RATIO_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-standard-height <SNIPPET_THUMBNAILS_STANDARD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-standard-url <SNIPPET_THUMBNAILS_STANDARD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-standard-width <SNIPPET_THUMBNAILS_STANDARD_WIDTH>
          (Optional) Width of the thumbnail image.

      --snippet-thumbnails-uhd-height <SNIPPET_THUMBNAILS_UHD_HEIGHT>
          (Optional) Height of the thumbnail image.

      --snippet-thumbnails-uhd-url <SNIPPET_THUMBNAILS_UHD_URL>
          The thumbnail image's URL.

      --snippet-thumbnails-uhd-width <SNIPPET_THUMBNAILS_UHD_WIDTH>
          (Optional) Width of the thumbnail image.

      --statistics <JSON>
          Statistics about the video, such as the number of times the video was viewed or liked.

      --statistics-comment-count <STATISTICS_COMMENT_COUNT>
          The number of comments for the video.

      --statistics-dislike-count <STATISTICS_DISLIKE_COUNT>
          The number of users who have indicated that they disliked the video by giving it a negative rating.

      --statistics-favorite-count <STATISTICS_FAVORITE_COUNT>
          The number of users who currently have the video marked as a favorite video.

      --statistics-like-count <STATISTICS_LIKE_COUNT>
          The number of users who have indicated that they liked the video by giving it a positive rating.

      --statistics-view-count <STATISTICS_VIEW_COUNT>
          The number of times the video has been viewed. For all video formats (long-form, Live, and Shorts), `viewCount` counts views the moment a video begins to play (includes autoplay, hold the pointer over, and click/tap to play). For all video formats (long-form, Live, and Shorts), `viewCount` includes co-views when multiple people watch together on a television screen.

      --status-contains-synthetic-media[=<BOOL>]
          Indicates if the video contains altered or synthetic media.

      --status-embeddable[=<BOOL>]
          This value indicates if the video can be embedded on another website. @mutable youtube.videos.insert youtube.videos.update

      --status-failure-reason <STATUS_FAILURE_REASON>
          This value explains why a video failed to upload. This property is only present if the `uploadStatus` property indicates that the upload failed. Choices: conversion, invalidFile, emptyFile, tooSmall, codec, uploadAborted.

      --status-license <STATUS_LICENSE>
          The video's license. @mutable youtube.videos.insert youtube.videos.update Choices: youtube, creativeCommon.

      --status-made-for-kids[=<BOOL>]
          Request body field

      --status-public-stats-viewable[=<BOOL>]
          This value indicates if the extended video statistics on the watch page can be viewed by everyone. Note that the view count, likes, etc will still be visible if this is disabled. @mutable youtube.videos.insert youtube.videos.update

      --status-publish-at <STATUS_PUBLISH_AT>
          The date and time when the video is scheduled to publish. It can be set only if the privacy status of the video is private..

      --status-rejection-reason <STATUS_REJECTION_REASON>
          This value explains why YouTube rejected an uploaded video. This property is only present if the `uploadStatus` property indicates that the upload was rejected. Choices: copyright, inappropriate, duplicate, termsOfUse, uploaderAccountSuspended, length, claim, uploaderAccountClosed, trademark, legal.

      --status-self-declared-made-for-kids[=<BOOL>]
          Request body field

      --status-upload-status <STATUS_UPLOAD_STATUS>
          The status of the uploaded video. Choices: uploaded, processed, failed, rejected, deleted.

      --suggestions <JSON>
          Specifies suggestions on how to improve video content, including encoding hints, tag suggestions, and editor suggestions.

      --suggestions-editor-suggestions <SUGGESTIONS_EDITOR_SUGGESTIONS>...
          A list of video editing operations that might improve the video quality or playback experience of the uploaded video.

      --suggestions-processing-errors <SUGGESTIONS_PROCESSING_ERRORS>...
          A list of errors that will prevent YouTube from successfully processing the uploaded video video. These errors indicate that, regardless of the video's current processing status, eventually, that status will almost certainly be failed.

      --suggestions-processing-hints <SUGGESTIONS_PROCESSING_HINTS>...
          A list of suggestions that may improve YouTube's ability to process the video.

      --suggestions-processing-warnings <SUGGESTIONS_PROCESSING_WARNINGS>...
          A list of reasons why YouTube may have difficulty transcoding the uploaded video or that might result in an erroneous transcoding. These warnings are generated before YouTube actually processes the uploaded video file. In addition, they identify issues that are unlikely to cause the video processing to fail but that might cause problems such as sync issues, video artifacts, or a missing audio track.

      --suggestions-tag-suggestions <JSON>
          A list of keyword tags that could be added to the video's metadata to increase the likelihood that users will locate your video when searching or browsing on YouTube.

      --topic-details <JSON>
          Freebase topic information related to the video.

      --topic-details-relevant-topic-ids <TOPIC_DETAILS_RELEVANT_TOPIC_IDS>...
          Similar to topic_id, except that these topics are merely relevant to the video. These are topics that may be mentioned in, or appear in the video. You can retrieve information about each topic using Freebase Topic API.

      --topic-details-topic-categories <TOPIC_DETAILS_TOPIC_CATEGORIES>...
          A list of Wikipedia URLs that provide a high-level description of the video's content.

      --topic-details-topic-ids <TOPIC_DETAILS_TOPIC_IDS>...
          A list of Freebase topic IDs that are centrally associated with the video. These are topics that are centrally featured in the video, and it can be said that the video is mainly about each of these. You can retrieve information about each topic using the Freebase Topic API.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## watermarks.set

```text
Upload an image as the channel's video watermark.

Input choices:
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] youtube <ACCOUNT> watermarks.set [COMMAND_OPTIONS]

Mandatory parameters:
      --upload <PATH>
          Upload a local file of at most 10 MiB.

      --channel-id <CHANNEL_ID>
          ID of the channel whose video watermark you want to change.

      --timing-offset-ms <TIMING_OFFSET_MS>
          Defines the time at which the promotion will appear. Depending on the value of type the value of the offsetMs field will represent a time offset from the start or from the end of the video, expressed in milliseconds.

      --timing-type <TIMING_TYPE>
          Describes a timing type. If the value is offsetFromStart, then the offsetMs field represents an offset from the start of the video. If the value is offsetFromEnd, then the offsetMs field represents an offset from the end of the video. Choices: offsetFromStart, offsetFromEnd.

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

      --content-type <MIME>
          Upload MIME type
          
          [default: application/octet-stream]

      --body <JSON>
          Inline JSON request body

      --image-bytes <IMAGE_BYTES>
          Encoded image data returned by YouTube. Upload a local image with --upload.

      --image-url <IMAGE_URL>
          Watermark image URL returned by YouTube. Upload a local image with --upload.

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

      --position <JSON>
          Describes the spatial position of a visual widget inside a video. It is a union of various position types, out of which only will be set one.

      --position-corner-position <POSITION_CORNER_POSITION>
          Describes in which corner of the video the visual widget will appear. Choices: topLeft, topRight, bottomLeft, bottomRight.

      --position-type <POSITION_TYPE>
          Defines the position type. Choices: corner.

      --target-channel-id <TARGET_CHANNEL_ID>
          The channel to which this branding links. If not present it defaults to the current channel.

      --timing <JSON>
          Describes a temporal position of a visual widget inside a video.

      --timing-duration-ms <TIMING_DURATION_MS>
          Defines the duration in milliseconds for which the promotion should be displayed. If missing, the client should use the default.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## watermarks.unset

```text
Remove the channel's video watermark.

Usage: joog [OPTIONS] youtube <ACCOUNT> watermarks.unset [COMMAND_OPTIONS]

Mandatory parameters:
      --channel-id <CHANNEL_ID>
          ID of the channel whose video watermark you want to change.

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

      --on-behalf-of-content-owner <ON_BEHALF_OF_CONTENT_OWNER>
          *Note:* This parameter is intended exclusively for YouTube content partners. The *onBehalfOfContentOwner* parameter indicates that the request's authorization credentials identify a YouTube CMS user who is acting on behalf of the content owner specified in the parameter value. This parameter is intended for YouTube content partners that own and manage many different YouTube channels. It allows content owners to authenticate once and get access to all their video and channel data, without having to provide authentication credentials for each individual channel. The CMS account that the user authenticates with must be linked to the specified YouTube content owner.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).
