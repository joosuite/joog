# Google Forms methods

See the [command guide](../grammar.md) and [request-file format](../file-schemas.md).

| Method | Description |
| --- | --- |
| `batch-update` | Change the form with a batch of updates. |
| `create` | Create a new form using the title given in the provided form message in the request |
| `get` | Get a form. |
| `responses.get` | Get one response from the form. |
| `responses.list` | List a form's responses. |
| `set-publish-settings` | Updates the publish settings of a form |

## batch-update

```text
Change the form with a batch of updates.

Usage: joog [OPTIONS] forms <ACCOUNT> batch-update [COMMAND_OPTIONS]

Mandatory parameters:
      --form-id <FORM_ID>
          Required. The form ID.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --requests <JSON>
          Required. The update requests of this batch.

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

      --include-form-in-response[=<BOOL>]
          Whether to return an updated version of the model in the response.

      --write-control <JSON>
          Provides control over how write requests are executed.

      --write-control-required-revision-id <WRITE_CONTROL_REQUIRED_REVISION_ID>
          The revision ID of the form that the write request is applied to. If this is not the latest revision of the form, the request is not processed and returns a 400 bad request error.

      --write-control-target-revision-id <WRITE_CONTROL_TARGET_REVISION_ID>
          The target revision ID of the form that the write request is applied to. If changes have occurred after this revision, the changes in this update request are transformed against those changes. This results in a new revision of the form that incorporates both the changes in the request and the intervening changes, with the server resolving conflicting changes. The target revision ID may only be used to write to recent versions of a form. If the target revision is too far behind the latest revision, the request is not processed and returns a 400 (Bad Request Error). The request may be retried after reading the latest version of the form. In most cases a target revision ID remains valid for several minutes after it is read, but for frequently-edited forms this window may be shorter.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## create

```text
Create a new form using the title given in the provided form message in the request. *Important:* Only the form.info.title and form.info.document_title fields are copied to the new form. All other fields including the form description, items and settings are disallowed. To create a new form and add items, you must first call forms.create to create an empty form with a title and (optional) document title, and then call forms.update to add the items.

Usage: joog [OPTIONS] forms <ACCOUNT> create [COMMAND_OPTIONS]

Mandatory parameters:
      --title <TITLE>
          Title.

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

      --info <JSON>
          The general information for a form.

      --info-description <INFO_DESCRIPTION>
          The description of the form.

      --items <JSON>
          Required. A list of the form's items, which can include section headers, questions, embedded media, etc.

      --publish-settings <JSON>
          The publishing settings of a form.

      --publish-settings-publish-state-is-accepting-responses[=<BOOL>]
          Required. Whether the form accepts responses. If `is_published` is set to `false`, this field is forced to `false`.

      --publish-settings-publish-state-is-published[=<BOOL>]
          Required. Whether the form is published and visible to others.

      --settings <JSON>
          A form's settings.

      --settings-email-collection-type <SETTINGS_EMAIL_COLLECTION_TYPE>
          Optional. The setting that determines whether the form collects email addresses from respondents. Choices: EMAIL_COLLECTION_TYPE_UNSPECIFIED, DO_NOT_COLLECT, VERIFIED, RESPONDER_INPUT.

      --settings-quiz-settings-is-quiz[=<BOOL>]
          Whether this form is a quiz or not. When true, responses are graded based on question Grading. Upon setting to false, all question Grading is deleted.

      --unpublished[=<BOOL>]
          Optional. Whether the form is unpublished. If set to `true`, the form doesn't accept responses. If set to `false` or unset, the form is published and accepts responses.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## get

```text
Get a form.

Usage: joog [OPTIONS] forms <ACCOUNT> get [COMMAND_OPTIONS]

Mandatory parameters:
      --form-id <FORM_ID>
          Required. The form ID.

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

## responses.get

```text
Get one response from the form.

Usage: joog [OPTIONS] forms <ACCOUNT> responses.get [COMMAND_OPTIONS]

Mandatory parameters:
      --form-id <FORM_ID>
          Required. The form ID.

      --response-id <RESPONSE_ID>
          Required. The response ID within the form.

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

## responses.list

```text
List a form's responses.

Usage: joog [OPTIONS] forms <ACCOUNT> responses.list [COMMAND_OPTIONS]

Mandatory parameters:
      --form-id <FORM_ID>
          Required. ID of the Form whose responses to list.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --filter <FILTER>
          Which form responses to return. Currently, the only supported filters are: * timestamp > *N* which means to get all form responses submitted after (but not at) timestamp *N*. * timestamp >= *N* which means to get all form responses submitted at and after timestamp *N*. For both supported filters, timestamp must be formatted in RFC3339 UTC "Zulu" format. Examples: "2014-10-02T15:01:23Z" and "2014-10-02T15:01:23.045123456Z".

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
          The maximum number of responses to return. The service may return fewer than this value. If unspecified or zero, at most 5000 responses are returned.

      --page-token <PAGE_TOKEN>
          A page token returned by a previous list response. If this field is set, the form and the values of the filter must be the same as for the original request.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## set-publish-settings

```text
Updates the publish settings of a form. Legacy forms aren't supported because they don't have the `publish_settings` field.

Usage: joog [OPTIONS] forms <ACCOUNT> set-publish-settings [COMMAND_OPTIONS]

Mandatory parameters:
      --form-id <FORM_ID>
          Required. The ID of the form. You can get the id from Form.form_id field.

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

      --publish-settings <JSON>
          The publishing settings of a form.

      --publish-settings-publish-state-is-accepting-responses[=<BOOL>]
          Required. Whether the form accepts responses. If `is_published` is set to `false`, this field is forced to `false`.

      --publish-settings-publish-state-is-published[=<BOOL>]
          Required. Whether the form is published and visible to others.

      --update-mask <UPDATE_MASK>
          Optional. The `publish_settings` fields to update. This field mask accepts the following values: * `publish_state`: Updates or replaces all `publish_state` settings. * `"*"`: Updates or replaces all `publish_settings` fields.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).
