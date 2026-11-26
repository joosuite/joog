# Gmail methods

See the [command guide](../grammar.md) and [request-file format](../file-schemas.md).

| Method | Description |
| --- | --- |
| `drafts.create` | Creates a draft with the `DRAFT` label |
| `drafts.delete` | Immediately and permanently deletes the specified draft |
| `drafts.get` | Gets the specified draft |
| `drafts.list` | Lists the drafts in the user's mailbox |
| `drafts.send` | Sends the specified, existing draft to the recipients in the `To`, `Cc`, and `Bcc` headers |
| `drafts.update` | Replaces a draft's content |
| `history.list` | Lists the history of all changes to the given mailbox |
| `labels.create` | Creates a label |
| `labels.delete` | Immediately and permanently deletes the specified label and removes it from any messages and threads that it's applied to |
| `labels.get` | Gets the specified label |
| `labels.list` | Lists all labels in the user's mailbox |
| `labels.patch` | Patch the specified label |
| `labels.update` | Updates the specified label |
| `messages.attachments.get` | Gets the specified message attachment. |
| `messages.batch-delete` | Deletes many messages by message ID |
| `messages.batch-modify` | Modifies the labels and the Classification Label values on the specified messages |
| `messages.delete` | Immediately and permanently deletes the specified message |
| `messages.get` | Gets the specified message. |
| `messages.import` | Imports a message into only this user's mailbox, with standard email delivery scanning and classification similar to receiving via SMTP |
| `messages.insert` | Directly inserts a message into only this user's mailbox similar to `IMAP APPEND`, bypassing most scanning and classification |
| `messages.list` | Lists the messages in the user's mailbox |
| `messages.modify` | Modifies the labels and the Classification Label values on the specified message |
| `messages.send` | Sends the specified message to the recipients in the `To`, `Cc`, and `Bcc` headers |
| `messages.trash` | Moves the specified message to the trash. |
| `messages.untrash` | Removes the specified message from the trash. |
| `profile` | Gets the current user's Gmail profile. |
| `threads.delete` | Immediately and permanently deletes the specified thread |
| `threads.get` | Gets the specified thread |
| `threads.list` | Lists the threads in the user's mailbox |
| `threads.modify` | Modifies the labels applied to the thread |
| `threads.trash` | Moves the specified thread to the trash |
| `threads.untrash` | Removes the specified thread from the trash |

## drafts.create

```text
Creates a draft with the `DRAFT` label. For more information, see [Create and send draft emails](https://developers.google.com/workspace/gmail/api/guides/drafts).

Input choices:
  Use --text for inline content or --text-file to read a file.
  Use --html for inline content or --html-file to read a file.
  Compose a message with recipients and content, or supply raw MIME. Composition cannot be combined with raw message content or --upload.
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] mail <ACCOUNT> drafts.create [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --attach <PATH>...
          Attach local files as application/octet-stream. Repeat --attach for multiple files.

      --bcc <EMAIL>...
          Bcc email addresses. Repeat --bcc for multiple recipients.

      --cc <EMAIL>...
          Cc email addresses. Repeat --cc for multiple recipients.

      --from <EMAIL>
          Sender email address; defaults to ACCOUNT. Google must allow this sender.

      --html <HTML>
          HTML message body; combines with text as multipart/alternative.

      --html-file <PATH>
          Read the HTML body from a UTF-8 file.

      --reply-to <EMAIL>
          Reply-To email address.

      --subject <TEXT>
          Message subject.

      --text <TEXT>
          Plain text message body.

      --text-file <PATH>
          Read the plain text body from a UTF-8 file.

      --to <EMAIL>...
          Recipient email addresses. Repeat --to for multiple recipients.

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

      --upload <PATH>
          Upload a local file of at most 35 MiB.

      --content-type <MIME>
          Upload MIME type
          
          [default: message/rfc822]

      --body <JSON>
          Inline JSON request body

      --id <ID>
          The immutable ID of the draft.

      --message <JSON>
          An email message.

      --message-classification-label-values <JSON>
          Classification Label values on the message. Available Classification Label schemas can be queried using the Google Drive Labels API. Each classification label ID must be unique. If duplicate IDs are provided, only one will be retained, and the selection is arbitrary. Only used for Google Workspace accounts. There's a limit of 20 Classification Label values per request. If the Classification Label values exceeds the maximum allowed number, the request fails.

      --message-history-id <MESSAGE_HISTORY_ID>
          The ID of the last history record that modified this message.

      --message-id <MESSAGE_ID>
          The immutable ID of the message.

      --message-internal-date <MESSAGE_INTERNAL_DATE>
          The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header.

      --message-label-ids <MESSAGE_LABEL_IDS>...
          List of IDs of labels applied to this message.

      --message-payload-body-attachment-id <MESSAGE_PAYLOAD_BODY_ATTACHMENT_ID>
          When present, contains the ID of an external attachment that can be retrieved in a separate `messages.attachments.get` request. When not present, the entire content of the message part body is contained in the data field.

      --message-payload-body-data <MESSAGE_PAYLOAD_BODY_DATA>
          The body data of a MIME message part as a base64url encoded string. May be empty for MIME container types that have no message body or when the body data is sent as a separate attachment. An attachment ID is present if the body data is contained in a separate attachment.

      --message-payload-body-size <MESSAGE_PAYLOAD_BODY_SIZE>
          Number of bytes for the message part data (encoding notwithstanding).

      --message-payload-filename <MESSAGE_PAYLOAD_FILENAME>
          The filename of the attachment. Only present if this message part represents an attachment.

      --message-payload-headers <JSON>
          List of headers on this message part. For the top-level message part, representing the entire message payload, it will contain the standard RFC 2822 email headers such as `To`, `From`, and `Subject`.

      --message-payload-mime-type <MESSAGE_PAYLOAD_MIME_TYPE>
          The MIME type of the message part.

      --message-payload-part-id <MESSAGE_PAYLOAD_PART_ID>
          The immutable ID of the message part.

      --message-payload-parts <JSON>
          The child MIME message parts of this part. This only applies to container MIME message parts, for example `multipart/*`. For non- container MIME message part types, such as `text/plain`, this field is empty. For more information, see RFC 1521.

      --message-raw <MESSAGE_RAW>
          The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied. @required gmail.users.drafts.create gmail.users.drafts.update

      --message-size-estimate <MESSAGE_SIZE_ESTIMATE>
          Estimated size in bytes of the message.

      --message-snippet <MESSAGE_SNIPPET>
          A short part of the message text.

      --message-thread-id <MESSAGE_THREAD_ID>
          The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## drafts.delete

```text
Immediately and permanently deletes the specified draft. Does not simply trash it. For more information, see [Create and send draft emails](https://developers.google.com/workspace/gmail/api/guides/drafts).

Usage: joog [OPTIONS] mail <ACCOUNT> drafts.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the draft to delete.

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## drafts.get

```text
Gets the specified draft. For more information, see [Create and send draft emails](https://developers.google.com/workspace/gmail/api/guides/drafts).

Usage: joog [OPTIONS] mail <ACCOUNT> drafts.get [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the draft to retrieve.

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

      --format <FORMAT>
          The format to return the draft in. Choices: minimal, full, raw, metadata.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## drafts.list

```text
Lists the drafts in the user's mailbox. For more information, see [Create and send draft emails](https://developers.google.com/workspace/gmail/api/guides/drafts).

Usage: joog [OPTIONS] mail <ACCOUNT> drafts.list [COMMAND_OPTIONS]

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
          Only return draft messages matching the specified query. Supports the same query format as the Gmail search box. For example, `"from:someuser@example.com rfc822msgid: is:unread"`.

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

      --include-spam-trash[=<BOOL>]
          Include drafts from `SPAM` and `TRASH` in the results.

      --max-results <MAX_RESULTS>
          Maximum number of drafts to return. This field defaults to 100. The maximum allowed value for this field is 500.

      --page-token <PAGE_TOKEN>
          Page token to retrieve a specific page of results in the list.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## drafts.send

```text
Sends the specified, existing draft to the recipients in the `To`, `Cc`, and `Bcc` headers. For more information, see [Create and send draft emails](https://developers.google.com/workspace/gmail/api/guides/drafts).

Input choices:
  Use --text for inline content or --text-file to read a file.
  Use --html for inline content or --html-file to read a file.
  Compose a message with recipients and content, or supply raw MIME. Composition cannot be combined with raw message content or --upload.
  When composing a message to send, provide at least one recipient with --to, --cc, or --bcc.
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] mail <ACCOUNT> drafts.send [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The immutable ID of the draft.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --attach <PATH>...
          Attach local files as application/octet-stream. Repeat --attach for multiple files.

      --bcc <EMAIL>...
          Bcc email addresses. Repeat --bcc for multiple recipients.

      --cc <EMAIL>...
          Cc email addresses. Repeat --cc for multiple recipients.

      --from <EMAIL>
          Sender email address; defaults to ACCOUNT. Google must allow this sender.

      --html <HTML>
          HTML message body; combines with text as multipart/alternative.

      --html-file <PATH>
          Read the HTML body from a UTF-8 file.

      --reply-to <EMAIL>
          Reply-To email address.

      --subject <TEXT>
          Message subject.

      --text <TEXT>
          Plain text message body.

      --text-file <PATH>
          Read the plain text body from a UTF-8 file.

      --to <EMAIL>...
          Recipient email addresses. Repeat --to for multiple recipients.

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

      --upload <PATH>
          Upload a local file of at most 35 MiB.

      --content-type <MIME>
          Upload MIME type
          
          [default: message/rfc822]

      --body <JSON>
          Inline JSON request body

      --message <JSON>
          An email message.

      --message-classification-label-values <JSON>
          Classification Label values on the message. Available Classification Label schemas can be queried using the Google Drive Labels API. Each classification label ID must be unique. If duplicate IDs are provided, only one will be retained, and the selection is arbitrary. Only used for Google Workspace accounts. There's a limit of 20 Classification Label values per request. If the Classification Label values exceeds the maximum allowed number, the request fails.

      --message-history-id <MESSAGE_HISTORY_ID>
          The ID of the last history record that modified this message.

      --message-id <MESSAGE_ID>
          The immutable ID of the message.

      --message-internal-date <MESSAGE_INTERNAL_DATE>
          The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header.

      --message-label-ids <MESSAGE_LABEL_IDS>...
          List of IDs of labels applied to this message.

      --message-payload-body-attachment-id <MESSAGE_PAYLOAD_BODY_ATTACHMENT_ID>
          When present, contains the ID of an external attachment that can be retrieved in a separate `messages.attachments.get` request. When not present, the entire content of the message part body is contained in the data field.

      --message-payload-body-data <MESSAGE_PAYLOAD_BODY_DATA>
          The body data of a MIME message part as a base64url encoded string. May be empty for MIME container types that have no message body or when the body data is sent as a separate attachment. An attachment ID is present if the body data is contained in a separate attachment.

      --message-payload-body-size <MESSAGE_PAYLOAD_BODY_SIZE>
          Number of bytes for the message part data (encoding notwithstanding).

      --message-payload-filename <MESSAGE_PAYLOAD_FILENAME>
          The filename of the attachment. Only present if this message part represents an attachment.

      --message-payload-headers <JSON>
          List of headers on this message part. For the top-level message part, representing the entire message payload, it will contain the standard RFC 2822 email headers such as `To`, `From`, and `Subject`.

      --message-payload-mime-type <MESSAGE_PAYLOAD_MIME_TYPE>
          The MIME type of the message part.

      --message-payload-part-id <MESSAGE_PAYLOAD_PART_ID>
          The immutable ID of the message part.

      --message-payload-parts <JSON>
          The child MIME message parts of this part. This only applies to container MIME message parts, for example `multipart/*`. For non- container MIME message part types, such as `text/plain`, this field is empty. For more information, see RFC 1521.

      --message-raw <MESSAGE_RAW>
          The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied. @required gmail.users.drafts.create gmail.users.drafts.update

      --message-size-estimate <MESSAGE_SIZE_ESTIMATE>
          Estimated size in bytes of the message.

      --message-snippet <MESSAGE_SNIPPET>
          A short part of the message text.

      --message-thread-id <MESSAGE_THREAD_ID>
          The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## drafts.update

```text
Replaces a draft's content. For more information, see [Create and send draft emails](https://developers.google.com/workspace/gmail/api/guides/drafts).

Input choices:
  Use --text for inline content or --text-file to read a file.
  Use --html for inline content or --html-file to read a file.
  Compose a message with recipients and content, or supply raw MIME. Composition cannot be combined with raw message content or --upload.
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] mail <ACCOUNT> drafts.update [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the draft to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --attach <PATH>...
          Attach local files as application/octet-stream. Repeat --attach for multiple files.

      --bcc <EMAIL>...
          Bcc email addresses. Repeat --bcc for multiple recipients.

      --cc <EMAIL>...
          Cc email addresses. Repeat --cc for multiple recipients.

      --from <EMAIL>
          Sender email address; defaults to ACCOUNT. Google must allow this sender.

      --html <HTML>
          HTML message body; combines with text as multipart/alternative.

      --html-file <PATH>
          Read the HTML body from a UTF-8 file.

      --reply-to <EMAIL>
          Reply-To email address.

      --subject <TEXT>
          Message subject.

      --text <TEXT>
          Plain text message body.

      --text-file <PATH>
          Read the plain text body from a UTF-8 file.

      --to <EMAIL>...
          Recipient email addresses. Repeat --to for multiple recipients.

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

      --upload <PATH>
          Upload a local file of at most 35 MiB.

      --content-type <MIME>
          Upload MIME type
          
          [default: message/rfc822]

      --body <JSON>
          Inline JSON request body

      --body-id <BODY_ID>
          The immutable ID of the draft.

      --message <JSON>
          An email message.

      --message-classification-label-values <JSON>
          Classification Label values on the message. Available Classification Label schemas can be queried using the Google Drive Labels API. Each classification label ID must be unique. If duplicate IDs are provided, only one will be retained, and the selection is arbitrary. Only used for Google Workspace accounts. There's a limit of 20 Classification Label values per request. If the Classification Label values exceeds the maximum allowed number, the request fails.

      --message-history-id <MESSAGE_HISTORY_ID>
          The ID of the last history record that modified this message.

      --message-id <MESSAGE_ID>
          The immutable ID of the message.

      --message-internal-date <MESSAGE_INTERNAL_DATE>
          The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header.

      --message-label-ids <MESSAGE_LABEL_IDS>...
          List of IDs of labels applied to this message.

      --message-payload-body-attachment-id <MESSAGE_PAYLOAD_BODY_ATTACHMENT_ID>
          When present, contains the ID of an external attachment that can be retrieved in a separate `messages.attachments.get` request. When not present, the entire content of the message part body is contained in the data field.

      --message-payload-body-data <MESSAGE_PAYLOAD_BODY_DATA>
          The body data of a MIME message part as a base64url encoded string. May be empty for MIME container types that have no message body or when the body data is sent as a separate attachment. An attachment ID is present if the body data is contained in a separate attachment.

      --message-payload-body-size <MESSAGE_PAYLOAD_BODY_SIZE>
          Number of bytes for the message part data (encoding notwithstanding).

      --message-payload-filename <MESSAGE_PAYLOAD_FILENAME>
          The filename of the attachment. Only present if this message part represents an attachment.

      --message-payload-headers <JSON>
          List of headers on this message part. For the top-level message part, representing the entire message payload, it will contain the standard RFC 2822 email headers such as `To`, `From`, and `Subject`.

      --message-payload-mime-type <MESSAGE_PAYLOAD_MIME_TYPE>
          The MIME type of the message part.

      --message-payload-part-id <MESSAGE_PAYLOAD_PART_ID>
          The immutable ID of the message part.

      --message-payload-parts <JSON>
          The child MIME message parts of this part. This only applies to container MIME message parts, for example `multipart/*`. For non- container MIME message part types, such as `text/plain`, this field is empty. For more information, see RFC 1521.

      --message-raw <MESSAGE_RAW>
          The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied. @required gmail.users.drafts.create gmail.users.drafts.update

      --message-size-estimate <MESSAGE_SIZE_ESTIMATE>
          Estimated size in bytes of the message.

      --message-snippet <MESSAGE_SNIPPET>
          A short part of the message text.

      --message-thread-id <MESSAGE_THREAD_ID>
          The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## history.list

```text
Lists the history of all changes to the given mailbox. History results are returned in chronological order (increasing `historyId`). For more information, see [Synchronize clients with Gmail](https://developers.google.com/workspace/gmail/api/guides/sync).

Usage: joog [OPTIONS] mail <ACCOUNT> history.list [COMMAND_OPTIONS]

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

      --history-types <HISTORY_TYPES>...
          History types to be returned by the function Choices: messageAdded, messageDeleted, labelAdded, labelRemoved.

      --label-id <LABEL_ID>
          Only return messages with a label matching the ID.

      --max-results <MAX_RESULTS>
          Maximum number of history records to return. This field defaults to 100. The maximum allowed value for this field is 500.

      --page-token <PAGE_TOKEN>
          Page token to retrieve a specific page of results in the list.

      --start-history-id <START_HISTORY_ID>
          Required. Returns history records after the specified `startHistoryId`. The supplied `startHistoryId` should be obtained from the `historyId` of a message, thread, or previous `list` response. History IDs increase chronologically but are not contiguous with random gaps in between valid IDs. Supplying an invalid or out of date `startHistoryId` typically returns an `HTTP 404` error code. A `historyId` is typically valid for at least a week, but in some rare circumstances may be valid for only a few hours. If you receive an `HTTP 404` error response, your application should perform a full sync. If you receive no `nextPageToken` in the response, there are no updates to retrieve and you can store the returned `historyId` for a future request.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## labels.create

```text
Creates a label. For more information, see [Manage labels](https://developers.google.com/workspace/gmail/api/guides/labels).

Usage: joog [OPTIONS] mail <ACCOUNT> labels.create [COMMAND_OPTIONS]

Mandatory parameters:
      --label-list-visibility <LABEL_LIST_VISIBILITY>
          The visibility of the label in the label list in the Gmail web interface. Choices: labelShow, labelShowIfUnread, labelHide.

      --message-list-visibility <MESSAGE_LIST_VISIBILITY>
          The visibility of messages with this label in the message list in the Gmail web interface. Choices: show, hide.

      --name <NAME>
          The display name of the label.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --type <TYPE>
          The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. System labels are internally created and cannot be added, modified, or deleted. System labels may be able to be applied to or removed from messages and threads under some circumstances but this is not guaranteed. For example, users can apply and remove the `INBOX` and `UNREAD` labels from messages and threads, but cannot apply or remove the `DRAFTS` or `SENT` labels from messages or threads. Choices: system, user.

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

      --color <JSON>
          Request body field

      --color-background-color <COLOR_BACKGROUND_COLOR>
          The background color represented as hex string #RRGGBB (ex #000000). This field is required in order to set the color of a label. Only the following predefined set of color values are allowed: \#000000, #434343, #666666, #999999, #cccccc, #efefef, #f3f3f3, #ffffff, \#fb4c2f, #ffad47, #fad165, #16a766, #43d692, #4a86e8, #a479e2, #f691b3, \#f6c5be, #ffe6c7, #fef1d1, #b9e4d0, #c6f3de, #c9daf8, #e4d7f5, #fcdee8, \#efa093, #ffd6a2, #fce8b3, #89d3b2, #a0eac9, #a4c2f4, #d0bcf1, #fbc8d9, \#e66550, #ffbc6b, #fcda83, #44b984, #68dfa9, #6d9eeb, #b694e8, #f7a7c0, \#cc3a21, #eaa041, #f2c960, #149e60, #3dc789, #3c78d8, #8e63ce, #e07798, \#ac2b16, #cf8933, #d5ae49, #0b804b, #2a9c68, #285bac, #653e9b, #b65775, \#822111, #a46a21, #aa8831, #076239, #1a764d, #1c4587, #41236d, #83334c, \#464646, #e7e7e7, #0d3472, #b6cff5, #0d3b44, #98d7e4, #3d188e, #e3d7ff, \#711a36, #fbd3e0, #8a1c0a, #f2b2a8, #7a2e0b, #ffc8af, #7a4706, #ffdeb5, \#594c05, #fbe983, #684e07, #fdedc1, #0b4f30, #b3efd3, #04502e, #a2dcc1, \#c2c2c2, #4986e7, #2da2bb, #b99aff, #994a64, #f691b2, #ff7537, #ffad46, \#662e37, #ebdbde, #cca6ac, #094228, #42d692, #16a765, #757575, #1e53b8, \#007286, #7858c3, #c2185b, #d93025, #54240e, #633e04, #521d28, #202124, \#083018

      --color-text-color <COLOR_TEXT_COLOR>
          The text color of the label, represented as hex string. This field is required in order to set the color of a label. Only the following predefined set of color values are allowed: \#000000, #434343, #666666, #999999, #cccccc, #efefef, #f3f3f3, #ffffff, \#fb4c2f, #ffad47, #fad165, #16a766, #43d692, #4a86e8, #a479e2, #f691b3, \#f6c5be, #ffe6c7, #fef1d1, #b9e4d0, #c6f3de, #c9daf8, #e4d7f5, #fcdee8, \#efa093, #ffd6a2, #fce8b3, #89d3b2, #a0eac9, #a4c2f4, #d0bcf1, #fbc8d9, \#e66550, #ffbc6b, #fcda83, #44b984, #68dfa9, #6d9eeb, #b694e8, #f7a7c0, \#cc3a21, #eaa041, #f2c960, #149e60, #3dc789, #3c78d8, #8e63ce, #e07798, \#ac2b16, #cf8933, #d5ae49, #0b804b, #2a9c68, #285bac, #653e9b, #b65775, \#822111, #a46a21, #aa8831, #076239, #1a764d, #1c4587, #41236d, #83334c, \#464646, #e7e7e7, #0d3472, #b6cff5, #0d3b44, #98d7e4, #3d188e, #e3d7ff, \#711a36, #fbd3e0, #8a1c0a, #f2b2a8, #7a2e0b, #ffc8af, #7a4706, #ffdeb5, \#594c05, #fbe983, #684e07, #fdedc1, #0b4f30, #b3efd3, #04502e, #a2dcc1, \#c2c2c2, #4986e7, #2da2bb, #b99aff, #994a64, #f691b2, #ff7537, #ffad46, \#662e37, #ebdbde, #cca6ac, #094228, #42d692, #16a765, #757575, #1e53b8, \#007286, #7858c3, #c2185b, #d93025, #54240e, #633e04, #521d28, #202124, \#083018

      --id <ID>
          The immutable ID of the label.

      --messages-total <MESSAGES_TOTAL>
          The total number of messages with the label.

      --messages-unread <MESSAGES_UNREAD>
          The number of unread messages with the label.

      --threads-total <THREADS_TOTAL>
          The total number of threads with the label.

      --threads-unread <THREADS_UNREAD>
          The number of unread threads with the label.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## labels.delete

```text
Immediately and permanently deletes the specified label and removes it from any messages and threads that it's applied to. For more information, see [Manage labels](https://developers.google.com/workspace/gmail/api/guides/labels).

Usage: joog [OPTIONS] mail <ACCOUNT> labels.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the label to delete.

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## labels.get

```text
Gets the specified label. For more information, see [Manage labels](https://developers.google.com/workspace/gmail/api/guides/labels).

Usage: joog [OPTIONS] mail <ACCOUNT> labels.get [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the label to retrieve.

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## labels.list

```text
Lists all labels in the user's mailbox. For more information, see [Manage labels](https://developers.google.com/workspace/gmail/api/guides/labels).

Usage: joog [OPTIONS] mail <ACCOUNT> labels.list [COMMAND_OPTIONS]

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## labels.patch

```text
Patch the specified label. For more information, see [Manage labels](https://developers.google.com/workspace/gmail/api/guides/labels).

Usage: joog [OPTIONS] mail <ACCOUNT> labels.patch [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the label to update.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --name <NAME>
          The display name of the label.

      --type <TYPE>
          The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. System labels are internally created and cannot be added, modified, or deleted. System labels may be able to be applied to or removed from messages and threads under some circumstances but this is not guaranteed. For example, users can apply and remove the `INBOX` and `UNREAD` labels from messages and threads, but cannot apply or remove the `DRAFTS` or `SENT` labels from messages or threads. Choices: system, user.

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
          The immutable ID of the label.

      --color <JSON>
          Request body field

      --color-background-color <COLOR_BACKGROUND_COLOR>
          The background color represented as hex string #RRGGBB (ex #000000). This field is required in order to set the color of a label. Only the following predefined set of color values are allowed: \#000000, #434343, #666666, #999999, #cccccc, #efefef, #f3f3f3, #ffffff, \#fb4c2f, #ffad47, #fad165, #16a766, #43d692, #4a86e8, #a479e2, #f691b3, \#f6c5be, #ffe6c7, #fef1d1, #b9e4d0, #c6f3de, #c9daf8, #e4d7f5, #fcdee8, \#efa093, #ffd6a2, #fce8b3, #89d3b2, #a0eac9, #a4c2f4, #d0bcf1, #fbc8d9, \#e66550, #ffbc6b, #fcda83, #44b984, #68dfa9, #6d9eeb, #b694e8, #f7a7c0, \#cc3a21, #eaa041, #f2c960, #149e60, #3dc789, #3c78d8, #8e63ce, #e07798, \#ac2b16, #cf8933, #d5ae49, #0b804b, #2a9c68, #285bac, #653e9b, #b65775, \#822111, #a46a21, #aa8831, #076239, #1a764d, #1c4587, #41236d, #83334c, \#464646, #e7e7e7, #0d3472, #b6cff5, #0d3b44, #98d7e4, #3d188e, #e3d7ff, \#711a36, #fbd3e0, #8a1c0a, #f2b2a8, #7a2e0b, #ffc8af, #7a4706, #ffdeb5, \#594c05, #fbe983, #684e07, #fdedc1, #0b4f30, #b3efd3, #04502e, #a2dcc1, \#c2c2c2, #4986e7, #2da2bb, #b99aff, #994a64, #f691b2, #ff7537, #ffad46, \#662e37, #ebdbde, #cca6ac, #094228, #42d692, #16a765, #757575, #1e53b8, \#007286, #7858c3, #c2185b, #d93025, #54240e, #633e04, #521d28, #202124, \#083018

      --color-text-color <COLOR_TEXT_COLOR>
          The text color of the label, represented as hex string. This field is required in order to set the color of a label. Only the following predefined set of color values are allowed: \#000000, #434343, #666666, #999999, #cccccc, #efefef, #f3f3f3, #ffffff, \#fb4c2f, #ffad47, #fad165, #16a766, #43d692, #4a86e8, #a479e2, #f691b3, \#f6c5be, #ffe6c7, #fef1d1, #b9e4d0, #c6f3de, #c9daf8, #e4d7f5, #fcdee8, \#efa093, #ffd6a2, #fce8b3, #89d3b2, #a0eac9, #a4c2f4, #d0bcf1, #fbc8d9, \#e66550, #ffbc6b, #fcda83, #44b984, #68dfa9, #6d9eeb, #b694e8, #f7a7c0, \#cc3a21, #eaa041, #f2c960, #149e60, #3dc789, #3c78d8, #8e63ce, #e07798, \#ac2b16, #cf8933, #d5ae49, #0b804b, #2a9c68, #285bac, #653e9b, #b65775, \#822111, #a46a21, #aa8831, #076239, #1a764d, #1c4587, #41236d, #83334c, \#464646, #e7e7e7, #0d3472, #b6cff5, #0d3b44, #98d7e4, #3d188e, #e3d7ff, \#711a36, #fbd3e0, #8a1c0a, #f2b2a8, #7a2e0b, #ffc8af, #7a4706, #ffdeb5, \#594c05, #fbe983, #684e07, #fdedc1, #0b4f30, #b3efd3, #04502e, #a2dcc1, \#c2c2c2, #4986e7, #2da2bb, #b99aff, #994a64, #f691b2, #ff7537, #ffad46, \#662e37, #ebdbde, #cca6ac, #094228, #42d692, #16a765, #757575, #1e53b8, \#007286, #7858c3, #c2185b, #d93025, #54240e, #633e04, #521d28, #202124, \#083018

      --label-list-visibility <LABEL_LIST_VISIBILITY>
          The visibility of the label in the label list in the Gmail web interface. Choices: labelShow, labelShowIfUnread, labelHide.

      --message-list-visibility <MESSAGE_LIST_VISIBILITY>
          The visibility of messages with this label in the message list in the Gmail web interface. Choices: show, hide.

      --messages-total <MESSAGES_TOTAL>
          The total number of messages with the label.

      --messages-unread <MESSAGES_UNREAD>
          The number of unread messages with the label.

      --threads-total <THREADS_TOTAL>
          The total number of threads with the label.

      --threads-unread <THREADS_UNREAD>
          The number of unread threads with the label.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## labels.update

```text
Updates the specified label. For more information, see [Manage labels](https://developers.google.com/workspace/gmail/api/guides/labels).

Usage: joog [OPTIONS] mail <ACCOUNT> labels.update [COMMAND_OPTIONS]

Mandatory parameters:
      --body-id <BODY_ID>
          The immutable ID of the label.

      --id <ID>
          The ID of the label to update.

      --label-list-visibility <LABEL_LIST_VISIBILITY>
          The visibility of the label in the label list in the Gmail web interface. Choices: labelShow, labelShowIfUnread, labelHide.

      --message-list-visibility <MESSAGE_LIST_VISIBILITY>
          The visibility of messages with this label in the message list in the Gmail web interface. Choices: show, hide.

      --name <NAME>
          The display name of the label.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --type <TYPE>
          The owner type for the label. User labels are created by the user and can be modified and deleted by the user and can be applied to any message or thread. System labels are internally created and cannot be added, modified, or deleted. System labels may be able to be applied to or removed from messages and threads under some circumstances but this is not guaranteed. For example, users can apply and remove the `INBOX` and `UNREAD` labels from messages and threads, but cannot apply or remove the `DRAFTS` or `SENT` labels from messages or threads. Choices: system, user.

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

      --color <JSON>
          Request body field

      --color-background-color <COLOR_BACKGROUND_COLOR>
          The background color represented as hex string #RRGGBB (ex #000000). This field is required in order to set the color of a label. Only the following predefined set of color values are allowed: \#000000, #434343, #666666, #999999, #cccccc, #efefef, #f3f3f3, #ffffff, \#fb4c2f, #ffad47, #fad165, #16a766, #43d692, #4a86e8, #a479e2, #f691b3, \#f6c5be, #ffe6c7, #fef1d1, #b9e4d0, #c6f3de, #c9daf8, #e4d7f5, #fcdee8, \#efa093, #ffd6a2, #fce8b3, #89d3b2, #a0eac9, #a4c2f4, #d0bcf1, #fbc8d9, \#e66550, #ffbc6b, #fcda83, #44b984, #68dfa9, #6d9eeb, #b694e8, #f7a7c0, \#cc3a21, #eaa041, #f2c960, #149e60, #3dc789, #3c78d8, #8e63ce, #e07798, \#ac2b16, #cf8933, #d5ae49, #0b804b, #2a9c68, #285bac, #653e9b, #b65775, \#822111, #a46a21, #aa8831, #076239, #1a764d, #1c4587, #41236d, #83334c, \#464646, #e7e7e7, #0d3472, #b6cff5, #0d3b44, #98d7e4, #3d188e, #e3d7ff, \#711a36, #fbd3e0, #8a1c0a, #f2b2a8, #7a2e0b, #ffc8af, #7a4706, #ffdeb5, \#594c05, #fbe983, #684e07, #fdedc1, #0b4f30, #b3efd3, #04502e, #a2dcc1, \#c2c2c2, #4986e7, #2da2bb, #b99aff, #994a64, #f691b2, #ff7537, #ffad46, \#662e37, #ebdbde, #cca6ac, #094228, #42d692, #16a765, #757575, #1e53b8, \#007286, #7858c3, #c2185b, #d93025, #54240e, #633e04, #521d28, #202124, \#083018

      --color-text-color <COLOR_TEXT_COLOR>
          The text color of the label, represented as hex string. This field is required in order to set the color of a label. Only the following predefined set of color values are allowed: \#000000, #434343, #666666, #999999, #cccccc, #efefef, #f3f3f3, #ffffff, \#fb4c2f, #ffad47, #fad165, #16a766, #43d692, #4a86e8, #a479e2, #f691b3, \#f6c5be, #ffe6c7, #fef1d1, #b9e4d0, #c6f3de, #c9daf8, #e4d7f5, #fcdee8, \#efa093, #ffd6a2, #fce8b3, #89d3b2, #a0eac9, #a4c2f4, #d0bcf1, #fbc8d9, \#e66550, #ffbc6b, #fcda83, #44b984, #68dfa9, #6d9eeb, #b694e8, #f7a7c0, \#cc3a21, #eaa041, #f2c960, #149e60, #3dc789, #3c78d8, #8e63ce, #e07798, \#ac2b16, #cf8933, #d5ae49, #0b804b, #2a9c68, #285bac, #653e9b, #b65775, \#822111, #a46a21, #aa8831, #076239, #1a764d, #1c4587, #41236d, #83334c, \#464646, #e7e7e7, #0d3472, #b6cff5, #0d3b44, #98d7e4, #3d188e, #e3d7ff, \#711a36, #fbd3e0, #8a1c0a, #f2b2a8, #7a2e0b, #ffc8af, #7a4706, #ffdeb5, \#594c05, #fbe983, #684e07, #fdedc1, #0b4f30, #b3efd3, #04502e, #a2dcc1, \#c2c2c2, #4986e7, #2da2bb, #b99aff, #994a64, #f691b2, #ff7537, #ffad46, \#662e37, #ebdbde, #cca6ac, #094228, #42d692, #16a765, #757575, #1e53b8, \#007286, #7858c3, #c2185b, #d93025, #54240e, #633e04, #521d28, #202124, \#083018

      --messages-total <MESSAGES_TOTAL>
          The total number of messages with the label.

      --messages-unread <MESSAGES_UNREAD>
          The number of unread messages with the label.

      --threads-total <THREADS_TOTAL>
          The total number of threads with the label.

      --threads-unread <THREADS_UNREAD>
          The number of unread threads with the label.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## messages.attachments.get

```text
Gets the specified message attachment.

Usage: joog [OPTIONS] mail <ACCOUNT> messages.attachments.get [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the attachment.

      --message-id <MESSAGE_ID>
          The ID of the message containing the attachment.

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## messages.batch-delete

```text
Deletes many messages by message ID. Provides no guarantees that messages were not already deleted or even existed at all.

Usage: joog [OPTIONS] mail <ACCOUNT> messages.batch-delete [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --ids <IDS>...
          The IDs of the messages to delete.

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## messages.batch-modify

```text
Modifies the labels and the Classification Label values on the specified messages. For administrators modifying messages for users in their organization, requests require authorization with a [service account](https://developers.google.com/identity/protocols/OAuth2ServiceAccount) that has [domain-wide delegation authority](https://developers.google.com/identity/protocols/OAuth2ServiceAccount#delegatingauthority) to impersonate users with the `https://www.googleapis.com/auth/gmail.modify.restricted` scope.

Usage: joog [OPTIONS] mail <ACCOUNT> messages.batch-modify [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --add-label-ids <ADD_LABEL_IDS>...
          A list of label IDs to add to messages.

      --ids <IDS>...
          The IDs of the messages to modify. There is a limit of 1000 ids per request.

      --remove-label-ids <REMOVE_LABEL_IDS>...
          A list of label IDs to remove from messages.

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

      --add-classification-labels <JSON>
          A list of Classification Label values to add. If a Classification Label with the same label ID is already applied to the message, fields with existing field IDs will be updated and fields with new field IDs will be added. There's a limit of 20 Classification Label values per request. If the message is already classified and the final total number of Classification Label values exceeds the maximum allowed number of Classification Label values per message, the modification fails.

      --remove-classification-label-ids <REMOVE_CLASSIFICATION_LABEL_IDS>...
          A list of Classification Label values to remove from messages.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## messages.delete

```text
Immediately and permanently deletes the specified message. This operation cannot be undone. Prefer `messages.trash` instead.

Usage: joog [OPTIONS] mail <ACCOUNT> messages.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the message to delete.

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## messages.get

```text
Gets the specified message.

Usage: joog [OPTIONS] mail <ACCOUNT> messages.get [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the message to retrieve. This ID is usually retrieved using `messages.list`. The ID is also contained in the result when a message is inserted (`messages.insert`) or imported (`messages.import`).

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

      --format <FORMAT>
          The format to return the message in. Choices: minimal, full, raw, metadata.

      --metadata-headers <METADATA_HEADERS>...
          When given and format is `METADATA`, only include headers specified.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## messages.import

```text
Imports a message into only this user's mailbox, with standard email delivery scanning and classification similar to receiving via SMTP. This method doesn't perform SPF checks, so it might not work for some spam messages, such as those attempting to perform domain spoofing. This method does not send a message. Note that the maximum size of the message is 150 MB.

Input choices:
  Use --text for inline content or --text-file to read a file.
  Use --html for inline content or --html-file to read a file.
  Compose a message with recipients and content, or supply raw MIME. Composition cannot be combined with raw message content or --upload.
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] mail <ACCOUNT> messages.import [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --attach <PATH>...
          Attach local files as application/octet-stream. Repeat --attach for multiple files.

      --bcc <EMAIL>...
          Bcc email addresses. Repeat --bcc for multiple recipients.

      --cc <EMAIL>...
          Cc email addresses. Repeat --cc for multiple recipients.

      --from <EMAIL>
          Sender email address; defaults to ACCOUNT. Google must allow this sender.

      --html <HTML>
          HTML message body; combines with text as multipart/alternative.

      --html-file <PATH>
          Read the HTML body from a UTF-8 file.

      --reply-to <EMAIL>
          Reply-To email address.

      --subject <TEXT>
          Message subject.

      --text <TEXT>
          Plain text message body.

      --text-file <PATH>
          Read the plain text body from a UTF-8 file.

      --to <EMAIL>...
          Recipient email addresses. Repeat --to for multiple recipients.

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

      --upload <PATH>
          Upload a local file of at most 150 MiB.

      --content-type <MIME>
          Upload MIME type
          
          [default: message/rfc822]

      --body <JSON>
          Inline JSON request body

      --classification-label-values <JSON>
          Classification Label values on the message. Available Classification Label schemas can be queried using the Google Drive Labels API. Each classification label ID must be unique. If duplicate IDs are provided, only one will be retained, and the selection is arbitrary. Only used for Google Workspace accounts. There's a limit of 20 Classification Label values per request. If the Classification Label values exceeds the maximum allowed number, the request fails.

      --deleted[=<BOOL>]
          Mark the email as permanently deleted (not TRASH) and only visible in Google Vault to a Vault administrator. Only used for Google Workspace accounts.

      --history-id <HISTORY_ID>
          The ID of the last history record that modified this message.

      --id <ID>
          The immutable ID of the message.

      --internal-date <INTERNAL_DATE>
          The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header.

      --internal-date-source <INTERNAL_DATE_SOURCE>
          Source for Gmail's internal date of the message. Choices: receivedTime, dateHeader.

      --label-ids <LABEL_IDS>...
          List of IDs of labels applied to this message.

      --never-mark-spam[=<BOOL>]
          Ignore the Gmail spam classifier decision and never mark this email as SPAM in the mailbox.

      --payload <JSON>
          A single MIME message part.

      --payload-body-attachment-id <PAYLOAD_BODY_ATTACHMENT_ID>
          When present, contains the ID of an external attachment that can be retrieved in a separate `messages.attachments.get` request. When not present, the entire content of the message part body is contained in the data field.

      --payload-body-data <PAYLOAD_BODY_DATA>
          The body data of a MIME message part as a base64url encoded string. May be empty for MIME container types that have no message body or when the body data is sent as a separate attachment. An attachment ID is present if the body data is contained in a separate attachment.

      --payload-body-size <PAYLOAD_BODY_SIZE>
          Number of bytes for the message part data (encoding notwithstanding).

      --payload-filename <PAYLOAD_FILENAME>
          The filename of the attachment. Only present if this message part represents an attachment.

      --payload-headers <JSON>
          List of headers on this message part. For the top-level message part, representing the entire message payload, it will contain the standard RFC 2822 email headers such as `To`, `From`, and `Subject`.

      --payload-mime-type <PAYLOAD_MIME_TYPE>
          The MIME type of the message part.

      --payload-part-id <PAYLOAD_PART_ID>
          The immutable ID of the message part.

      --payload-parts <JSON>
          The child MIME message parts of this part. This only applies to container MIME message parts, for example `multipart/*`. For non- container MIME message part types, such as `text/plain`, this field is empty. For more information, see RFC 1521.

      --process-for-calendar[=<BOOL>]
          Process calendar invites in the email and add any extracted meetings to the Google Calendar for this user.

      --raw <RAW>
          The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied. @required gmail.users.drafts.create gmail.users.drafts.update

      --size-estimate <SIZE_ESTIMATE>
          Estimated size in bytes of the message.

      --snippet <SNIPPET>
          A short part of the message text.

      --thread-id <THREAD_ID>
          The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## messages.insert

```text
Directly inserts a message into only this user's mailbox similar to `IMAP APPEND`, bypassing most scanning and classification. Does not send a message. For more information, see [Create and send email messages](https://developers.google.com/workspace/gmail/api/guides/sending).

Input choices:
  Use --text for inline content or --text-file to read a file.
  Use --html for inline content or --html-file to read a file.
  Compose a message with recipients and content, or supply raw MIME. Composition cannot be combined with raw message content or --upload.
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] mail <ACCOUNT> messages.insert [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --attach <PATH>...
          Attach local files as application/octet-stream. Repeat --attach for multiple files.

      --bcc <EMAIL>...
          Bcc email addresses. Repeat --bcc for multiple recipients.

      --cc <EMAIL>...
          Cc email addresses. Repeat --cc for multiple recipients.

      --from <EMAIL>
          Sender email address; defaults to ACCOUNT. Google must allow this sender.

      --html <HTML>
          HTML message body; combines with text as multipart/alternative.

      --html-file <PATH>
          Read the HTML body from a UTF-8 file.

      --reply-to <EMAIL>
          Reply-To email address.

      --subject <TEXT>
          Message subject.

      --text <TEXT>
          Plain text message body.

      --text-file <PATH>
          Read the plain text body from a UTF-8 file.

      --to <EMAIL>...
          Recipient email addresses. Repeat --to for multiple recipients.

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

      --upload <PATH>
          Upload a local file of at most 150 MiB.

      --content-type <MIME>
          Upload MIME type
          
          [default: message/rfc822]

      --body <JSON>
          Inline JSON request body

      --classification-label-values <JSON>
          Classification Label values on the message. Available Classification Label schemas can be queried using the Google Drive Labels API. Each classification label ID must be unique. If duplicate IDs are provided, only one will be retained, and the selection is arbitrary. Only used for Google Workspace accounts. There's a limit of 20 Classification Label values per request. If the Classification Label values exceeds the maximum allowed number, the request fails.

      --deleted[=<BOOL>]
          Mark the email as permanently deleted (not TRASH) and only visible in Google Vault to a Vault administrator. Only used for Google Workspace accounts.

      --history-id <HISTORY_ID>
          The ID of the last history record that modified this message.

      --id <ID>
          The immutable ID of the message.

      --internal-date <INTERNAL_DATE>
          The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header.

      --internal-date-source <INTERNAL_DATE_SOURCE>
          Source for Gmail's internal date of the message. Choices: receivedTime, dateHeader.

      --label-ids <LABEL_IDS>...
          List of IDs of labels applied to this message.

      --payload <JSON>
          A single MIME message part.

      --payload-body-attachment-id <PAYLOAD_BODY_ATTACHMENT_ID>
          When present, contains the ID of an external attachment that can be retrieved in a separate `messages.attachments.get` request. When not present, the entire content of the message part body is contained in the data field.

      --payload-body-data <PAYLOAD_BODY_DATA>
          The body data of a MIME message part as a base64url encoded string. May be empty for MIME container types that have no message body or when the body data is sent as a separate attachment. An attachment ID is present if the body data is contained in a separate attachment.

      --payload-body-size <PAYLOAD_BODY_SIZE>
          Number of bytes for the message part data (encoding notwithstanding).

      --payload-filename <PAYLOAD_FILENAME>
          The filename of the attachment. Only present if this message part represents an attachment.

      --payload-headers <JSON>
          List of headers on this message part. For the top-level message part, representing the entire message payload, it will contain the standard RFC 2822 email headers such as `To`, `From`, and `Subject`.

      --payload-mime-type <PAYLOAD_MIME_TYPE>
          The MIME type of the message part.

      --payload-part-id <PAYLOAD_PART_ID>
          The immutable ID of the message part.

      --payload-parts <JSON>
          The child MIME message parts of this part. This only applies to container MIME message parts, for example `multipart/*`. For non- container MIME message part types, such as `text/plain`, this field is empty. For more information, see RFC 1521.

      --raw <RAW>
          The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied. @required gmail.users.drafts.create gmail.users.drafts.update

      --size-estimate <SIZE_ESTIMATE>
          Estimated size in bytes of the message.

      --snippet <SNIPPET>
          A short part of the message text.

      --thread-id <THREAD_ID>
          The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## messages.list

```text
Lists the messages in the user's mailbox. For more information, see [List Gmail messages](https://developers.google.com/workspace/gmail/api/guides/list-messages).

Usage: joog [OPTIONS] mail <ACCOUNT> messages.list [COMMAND_OPTIONS]

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
          Only return messages matching the specified query. Supports the same query format as the Gmail search box. For example, `"from:someuser@example.com rfc822msgid: is:unread"`. Parameter cannot be used when accessing the api using the gmail.metadata scope.

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

      --include-spam-trash[=<BOOL>]
          Include messages from `SPAM` and `TRASH` in the results.

      --label-ids <LABEL_IDS>...
          Only return messages with labels that match all of the specified label IDs. Messages in a thread might have labels that other messages in the same thread don't have. To learn more, see [Manage labels on messages and threads](https://developers.google.com/workspace/gmail/api/guides/labels#manage_labels_on_messages_threads).

      --max-results <MAX_RESULTS>
          Maximum number of messages to return. This field defaults to 100. The maximum allowed value for this field is 500.

      --page-token <PAGE_TOKEN>
          Page token to retrieve a specific page of results in the list.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## messages.modify

```text
Modifies the labels and the Classification Label values on the specified message. For administrators modifying message for users in their organization, requests require authorization with a [service account](https://developers.google.com/identity/protocols/OAuth2ServiceAccount) that has [domain-wide delegation authority](https://developers.google.com/identity/protocols/OAuth2ServiceAccount#delegatingauthority) to impersonate users with the `https://www.googleapis.com/auth/gmail.modify.restricted` scope.

Usage: joog [OPTIONS] mail <ACCOUNT> messages.modify [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the message to modify.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --add-label-ids <ADD_LABEL_IDS>...
          A list of IDs of labels to add to this message. You can add up to 100 labels with each update.

      --remove-label-ids <REMOVE_LABEL_IDS>...
          A list IDs of labels to remove from this message. You can remove up to 100 labels with each update.

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

      --add-classification-labels <JSON>
          A list of classification label values to add. If a Classification Label with the same label ID is already applied to the message, fields with existing field IDs will be updated and fields with new field IDs will be added. There's a limit of 20 Classification Label values per request. If the message is already classified and the final total number of Classification Label values exceeds the maximum allowed number of Classification Label values per message, the modification fails.

      --remove-classification-label-ids <REMOVE_CLASSIFICATION_LABEL_IDS>...
          A list of Classification Label values to remove from this message.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## messages.send

```text
Sends the specified message to the recipients in the `To`, `Cc`, and `Bcc` headers. For more information, see [Create and send email messages](https://developers.google.com/workspace/gmail/api/guides/sending).

Input choices:
  Use --text for inline content or --text-file to read a file.
  Use --html for inline content or --html-file to read a file.
  Compose a message with recipients and content, or supply raw MIME. Composition cannot be combined with raw message content or --upload.
  When composing a message to send, provide at least one recipient with --to, --cc, or --bcc.
  --content-type sets the MIME type of a file supplied with --upload.

Usage: joog [OPTIONS] mail <ACCOUNT> messages.send [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --attach <PATH>...
          Attach local files as application/octet-stream. Repeat --attach for multiple files.

      --bcc <EMAIL>...
          Bcc email addresses. Repeat --bcc for multiple recipients.

      --cc <EMAIL>...
          Cc email addresses. Repeat --cc for multiple recipients.

      --from <EMAIL>
          Sender email address; defaults to ACCOUNT. Google must allow this sender.

      --html <HTML>
          HTML message body; combines with text as multipart/alternative.

      --html-file <PATH>
          Read the HTML body from a UTF-8 file.

      --reply-to <EMAIL>
          Reply-To email address.

      --subject <TEXT>
          Message subject.

      --text <TEXT>
          Plain text message body.

      --text-file <PATH>
          Read the plain text body from a UTF-8 file.

      --to <EMAIL>...
          Recipient email addresses. Repeat --to for multiple recipients.

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

      --upload <PATH>
          Upload a local file of at most 35 MiB.

      --content-type <MIME>
          Upload MIME type
          
          [default: message/rfc822]

      --body <JSON>
          Inline JSON request body

      --classification-label-values <JSON>
          Classification Label values on the message. Available Classification Label schemas can be queried using the Google Drive Labels API. Each classification label ID must be unique. If duplicate IDs are provided, only one will be retained, and the selection is arbitrary. Only used for Google Workspace accounts. There's a limit of 20 Classification Label values per request. If the Classification Label values exceeds the maximum allowed number, the request fails.

      --history-id <HISTORY_ID>
          The ID of the last history record that modified this message.

      --id <ID>
          The immutable ID of the message.

      --internal-date <INTERNAL_DATE>
          The internal message creation timestamp (epoch ms), which determines ordering in the inbox. For normal SMTP-received email, this represents the time the message was originally accepted by Google, which is more reliable than the `Date` header. However, for API-migrated mail, it can be configured by client to be based on the `Date` header.

      --label-ids <LABEL_IDS>...
          List of IDs of labels applied to this message.

      --payload <JSON>
          A single MIME message part.

      --payload-body-attachment-id <PAYLOAD_BODY_ATTACHMENT_ID>
          When present, contains the ID of an external attachment that can be retrieved in a separate `messages.attachments.get` request. When not present, the entire content of the message part body is contained in the data field.

      --payload-body-data <PAYLOAD_BODY_DATA>
          The body data of a MIME message part as a base64url encoded string. May be empty for MIME container types that have no message body or when the body data is sent as a separate attachment. An attachment ID is present if the body data is contained in a separate attachment.

      --payload-body-size <PAYLOAD_BODY_SIZE>
          Number of bytes for the message part data (encoding notwithstanding).

      --payload-filename <PAYLOAD_FILENAME>
          The filename of the attachment. Only present if this message part represents an attachment.

      --payload-headers <JSON>
          List of headers on this message part. For the top-level message part, representing the entire message payload, it will contain the standard RFC 2822 email headers such as `To`, `From`, and `Subject`.

      --payload-mime-type <PAYLOAD_MIME_TYPE>
          The MIME type of the message part.

      --payload-part-id <PAYLOAD_PART_ID>
          The immutable ID of the message part.

      --payload-parts <JSON>
          The child MIME message parts of this part. This only applies to container MIME message parts, for example `multipart/*`. For non- container MIME message part types, such as `text/plain`, this field is empty. For more information, see RFC 1521.

      --raw <RAW>
          The entire email message in an RFC 2822 formatted and base64url encoded string. Returned in `messages.get` and `drafts.get` responses when the `format=RAW` parameter is supplied. @required gmail.users.drafts.create gmail.users.drafts.update

      --size-estimate <SIZE_ESTIMATE>
          Estimated size in bytes of the message.

      --snippet <SNIPPET>
          A short part of the message text.

      --thread-id <THREAD_ID>
          The ID of the thread the message belongs to. To add a message or draft to a thread, the following criteria must be met: 1. The requested `threadId` must be specified on the `Message` or `Draft.Message` you supply with your request. 2. The `References` and `In-Reply-To` headers must be set in compliance with the [RFC 2822](https://tools.ietf.org/html/rfc2822) standard. 3. The `Subject` headers must match.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## messages.trash

```text
Moves the specified message to the trash.

Usage: joog [OPTIONS] mail <ACCOUNT> messages.trash [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the message to Trash.

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## messages.untrash

```text
Removes the specified message from the trash.

Usage: joog [OPTIONS] mail <ACCOUNT> messages.untrash [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the message to remove from Trash.

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## profile

```text
Gets the current user's Gmail profile.

Usage: joog [OPTIONS] mail <ACCOUNT> profile [COMMAND_OPTIONS]

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## threads.delete

```text
Immediately and permanently deletes the specified thread. Any messages that belong to the thread are also deleted. This operation cannot be undone. Prefer `threads.trash` instead. For more information, see [Manage threads](https://developers.google.com/workspace/gmail/api/guides/threads).

Usage: joog [OPTIONS] mail <ACCOUNT> threads.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          ID of the Thread to delete.

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## threads.get

```text
Gets the specified thread. For more information, see [Manage threads](https://developers.google.com/workspace/gmail/api/guides/threads).

Usage: joog [OPTIONS] mail <ACCOUNT> threads.get [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the thread to retrieve.

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

      --format <FORMAT>
          The format to return the messages in. Choices: full, metadata, minimal.

      --metadata-headers <METADATA_HEADERS>...
          When given and format is METADATA, only include headers specified.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## threads.list

```text
Lists the threads in the user's mailbox. For more information, see [Manage threads](https://developers.google.com/workspace/gmail/api/guides/threads).

Usage: joog [OPTIONS] mail <ACCOUNT> threads.list [COMMAND_OPTIONS]

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
          Only return threads matching the specified query. Supports the same query format as the Gmail search box. For example, `"from:someuser@example.com rfc822msgid: is:unread"`. Parameter cannot be used when accessing the api using the gmail.metadata scope.

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

      --include-spam-trash[=<BOOL>]
          Include threads from `SPAM` and `TRASH` in the results.

      --label-ids <LABEL_IDS>...
          Only return threads with labels that match all of the specified label IDs.

      --max-results <MAX_RESULTS>
          Maximum number of threads to return. This field defaults to 100. The maximum allowed value for this field is 500.

      --page-token <PAGE_TOKEN>
          Page token to retrieve a specific page of results in the list.

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## threads.modify

```text
Modifies the labels applied to the thread. This applies to all messages in the thread. For more information, see [Manage threads](https://developers.google.com/workspace/gmail/api/guides/threads).

Usage: joog [OPTIONS] mail <ACCOUNT> threads.modify [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the thread to modify.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --add-label-ids <ADD_LABEL_IDS>...
          A list of IDs of labels to add to this thread. You can add up to 100 labels with each update.

      --verbose
          Include advanced parameters in help

      --remove-label-ids <REMOVE_LABEL_IDS>...
          A list of IDs of labels to remove from this thread. You can remove up to 100 labels with each update.

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## threads.trash

```text
Moves the specified thread to the trash. Any messages that belong to the thread are also moved to the trash. For more information, see [Manage threads](https://developers.google.com/workspace/gmail/api/guides/threads).

Usage: joog [OPTIONS] mail <ACCOUNT> threads.trash [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the thread to Trash.

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## threads.untrash

```text
Removes the specified thread from the trash. Any messages that belong to the thread are also removed from the trash. For more information, see [Manage threads](https://developers.google.com/workspace/gmail/api/guides/threads).

Usage: joog [OPTIONS] mail <ACCOUNT> threads.untrash [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          The ID of the thread to remove from Trash.

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

      --user-id <USER_ID>
          The user's email address. The special value `me` can be used to indicate the authenticated user.
          
          [default: me]

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).
