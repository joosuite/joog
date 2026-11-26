# Google Calendar methods

See the [command guide](../grammar.md) and [request-file format](../file-schemas.md).

| Method | Description |
| --- | --- |
| `calendar-list.delete` | Removes a calendar from the user's calendar list. |
| `calendar-list.get` | Returns a calendar from the user's calendar list. |
| `calendar-list.insert` | Inserts an existing calendar into the user's calendar list. |
| `calendar-list.patch` | Updates an existing calendar on the user's calendar list |
| `calendar-list.update` | Updates an existing calendar on the user's calendar list. |
| `calendars.clear` | Clears a primary calendar |
| `calendars.delete` | Deletes a secondary calendar |
| `calendars.get` | Returns metadata for a calendar. |
| `calendars.insert` | Creates a secondary calendar. |
| `calendars.list` | Returns the calendars on the user's calendar list. |
| `calendars.patch` | Updates metadata for a calendar |
| `calendars.transfer-ownership` | Transfers a secondary calendar between users within a Google Workspace organization |
| `calendars.update` | Updates metadata for a calendar. |
| `colors.get` | Returns the color definitions for calendars and events. |
| `events.delete` | Deletes an event. |
| `events.get` | Returns an event based on its Google Calendar ID |
| `events.import` | Imports an event |
| `events.insert` | Creates an event. |
| `events.instances` | Returns instances of the specified recurring event. |
| `events.list` | Returns events on the specified calendar. |
| `events.move` | Moves an event to another calendar, i.e |
| `events.patch` | Updates an event |
| `events.quick-add` | Creates an event based on a simple text string. |
| `events.update` | Updates an event. |
| `events.watch` | Watch for changes to Events resources. |
| `freebusy.query` | Returns free/busy information for a set of calendars. |

## calendar-list.delete

```text
Removes a calendar from the user's calendar list.

Usage: joog [OPTIONS] calendar <ACCOUNT> calendar-list.delete [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

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

## calendar-list.get

```text
Returns a calendar from the user's calendar list.

Usage: joog [OPTIONS] calendar <ACCOUNT> calendar-list.get [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

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

## calendar-list.insert

```text
Inserts an existing calendar into the user's calendar list.

Usage: joog [OPTIONS] calendar <ACCOUNT> calendar-list.insert [COMMAND_OPTIONS]

Mandatory parameters:
      --id <ID>
          Identifier of the calendar.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --description <DESCRIPTION>
          Description of the calendar. Optional. Read-only.

      --summary <SUMMARY>
          Title of the calendar. Read-only.

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

      --access-role <ACCESS_ROLE>
          The effective access role that the authenticated user has on the calendar. Read-only. Possible values are:  
          - "freeBusyReader" - Provides read access to free/busy information. 
          - "reader" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. 
          - "writerWithoutPrivateAccess" - Provides read and write access to the calendar. Private events will appear to users with writerWithoutPrivateAccess access, but event details will be hidden. 
          - "writer" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. 
          - "owner" - Provides manager access to the calendar. This role has all of the permissions of the writer role with the additional ability to see and modify access levels of other users.
          Important: the owner role is different from the calendar's data owner. A calendar has a single data owner, but can have multiple users with owner role.

      --auto-accept-invitations[=<BOOL>]
          Whether this calendar automatically accepts invitations. Only valid for resource calendars. Read-only.

      --background-color <BACKGROUND_COLOR>
          The main color of the calendar in the hexadecimal format "#0088aa". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional.

      --color-id <COLOR_ID>
          The color of the calendar. This is an ID referring to an entry in the calendar section of the colors definition (see the colors endpoint). This property is superseded by the backgroundColor and foregroundColor properties and can be ignored when using these properties. Optional.

      --color-rgb-format[=<BOOL>]
          Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False.

      --conference-properties <JSON>
          Request body field

      --conference-properties-allowed-conference-solution-types <CONFERENCE_PROPERTIES_ALLOWED_CONFERENCE_SOLUTION_TYPES>...
          The types of conference solutions that are supported for this calendar.
          The possible values are:  
          - "eventHangout" 
          - "eventNamedHangout" 
          - "hangoutsMeet"  Optional.

      --data-owner <DATA_OWNER>
          The email of the owner of the calendar. Set only for secondary calendars. Read-only.

      --default-reminders <JSON>
          The default reminders that the authenticated user has for this calendar.

      --deleted[=<BOOL>]
          Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.

      --etag <ETAG>
          ETag of the resource.

      --foreground-color <FOREGROUND_COLOR>
          The foreground color of the calendar in the hexadecimal format "#ffffff". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional.

      --hidden[=<BOOL>]
          Whether the calendar has been hidden from the list. Optional. The attribute is only returned when the calendar is hidden, in which case the value is true.

      --kind <KIND>
          Type of the resource ("calendar#calendarListEntry").

      --location <LOCATION>
          Geographic location of the calendar as free-form text. Optional. Read-only.

      --notification-settings <JSON>
          The notifications that the authenticated user is receiving for this calendar.

      --notification-settings-notifications <JSON>
          The list of notifications set for this calendar.

      --primary[=<BOOL>]
          Whether the calendar is the primary calendar of the authenticated user. Read-only. Optional. The default is False.

      --selected[=<BOOL>]
          Whether the calendar content shows up in the calendar UI. Optional. The default is False.

      --summary-override <SUMMARY_OVERRIDE>
          The summary that the authenticated user has set for this calendar. Optional.

      --time-zone <TIME_ZONE>
          The time zone of the calendar. Optional. Read-only.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## calendar-list.patch

```text
Updates an existing calendar on the user's calendar list. This method supports patch semantics.

Usage: joog [OPTIONS] calendar <ACCOUNT> calendar-list.patch [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

      --description <DESCRIPTION>
          Description of the calendar. Optional. Read-only.

      --summary <SUMMARY>
          Title of the calendar. Read-only.

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

      --access-role <ACCESS_ROLE>
          The effective access role that the authenticated user has on the calendar. Read-only. Possible values are:  
          - "freeBusyReader" - Provides read access to free/busy information. 
          - "reader" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. 
          - "writerWithoutPrivateAccess" - Provides read and write access to the calendar. Private events will appear to users with writerWithoutPrivateAccess access, but event details will be hidden. 
          - "writer" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. 
          - "owner" - Provides manager access to the calendar. This role has all of the permissions of the writer role with the additional ability to see and modify access levels of other users.
          Important: the owner role is different from the calendar's data owner. A calendar has a single data owner, but can have multiple users with owner role.

      --auto-accept-invitations[=<BOOL>]
          Whether this calendar automatically accepts invitations. Only valid for resource calendars. Read-only.

      --background-color <BACKGROUND_COLOR>
          The main color of the calendar in the hexadecimal format "#0088aa". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional.

      --color-id <COLOR_ID>
          The color of the calendar. This is an ID referring to an entry in the calendar section of the colors definition (see the colors endpoint). This property is superseded by the backgroundColor and foregroundColor properties and can be ignored when using these properties. Optional.

      --color-rgb-format[=<BOOL>]
          Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False.

      --conference-properties <JSON>
          Request body field

      --conference-properties-allowed-conference-solution-types <CONFERENCE_PROPERTIES_ALLOWED_CONFERENCE_SOLUTION_TYPES>...
          The types of conference solutions that are supported for this calendar.
          The possible values are:  
          - "eventHangout" 
          - "eventNamedHangout" 
          - "hangoutsMeet"  Optional.

      --data-owner <DATA_OWNER>
          The email of the owner of the calendar. Set only for secondary calendars. Read-only.

      --default-reminders <JSON>
          The default reminders that the authenticated user has for this calendar.

      --deleted[=<BOOL>]
          Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.

      --etag <ETAG>
          ETag of the resource.

      --foreground-color <FOREGROUND_COLOR>
          The foreground color of the calendar in the hexadecimal format "#ffffff". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional.

      --hidden[=<BOOL>]
          Whether the calendar has been hidden from the list. Optional. The attribute is only returned when the calendar is hidden, in which case the value is true.

      --id <ID>
          Identifier of the calendar.

      --kind <KIND>
          Type of the resource ("calendar#calendarListEntry").

      --location <LOCATION>
          Geographic location of the calendar as free-form text. Optional. Read-only.

      --notification-settings <JSON>
          The notifications that the authenticated user is receiving for this calendar.

      --notification-settings-notifications <JSON>
          The list of notifications set for this calendar.

      --primary[=<BOOL>]
          Whether the calendar is the primary calendar of the authenticated user. Read-only. Optional. The default is False.

      --selected[=<BOOL>]
          Whether the calendar content shows up in the calendar UI. Optional. The default is False.

      --summary-override <SUMMARY_OVERRIDE>
          The summary that the authenticated user has set for this calendar. Optional.

      --time-zone <TIME_ZONE>
          The time zone of the calendar. Optional. Read-only.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## calendar-list.update

```text
Updates an existing calendar on the user's calendar list.

Usage: joog [OPTIONS] calendar <ACCOUNT> calendar-list.update [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

      --description <DESCRIPTION>
          Description of the calendar. Optional. Read-only.

      --summary <SUMMARY>
          Title of the calendar. Read-only.

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

      --access-role <ACCESS_ROLE>
          The effective access role that the authenticated user has on the calendar. Read-only. Possible values are:  
          - "freeBusyReader" - Provides read access to free/busy information. 
          - "reader" - Provides read access to the calendar. Private events will appear to users with reader access, but event details will be hidden. 
          - "writerWithoutPrivateAccess" - Provides read and write access to the calendar. Private events will appear to users with writerWithoutPrivateAccess access, but event details will be hidden. 
          - "writer" - Provides read and write access to the calendar. Private events will appear to users with writer access, and event details will be visible. 
          - "owner" - Provides manager access to the calendar. This role has all of the permissions of the writer role with the additional ability to see and modify access levels of other users.
          Important: the owner role is different from the calendar's data owner. A calendar has a single data owner, but can have multiple users with owner role.

      --auto-accept-invitations[=<BOOL>]
          Whether this calendar automatically accepts invitations. Only valid for resource calendars. Read-only.

      --background-color <BACKGROUND_COLOR>
          The main color of the calendar in the hexadecimal format "#0088aa". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional.

      --color-id <COLOR_ID>
          The color of the calendar. This is an ID referring to an entry in the calendar section of the colors definition (see the colors endpoint). This property is superseded by the backgroundColor and foregroundColor properties and can be ignored when using these properties. Optional.

      --color-rgb-format[=<BOOL>]
          Whether to use the foregroundColor and backgroundColor fields to write the calendar colors (RGB). If this feature is used, the index-based colorId field will be set to the best matching option automatically. Optional. The default is False.

      --conference-properties <JSON>
          Request body field

      --conference-properties-allowed-conference-solution-types <CONFERENCE_PROPERTIES_ALLOWED_CONFERENCE_SOLUTION_TYPES>...
          The types of conference solutions that are supported for this calendar.
          The possible values are:  
          - "eventHangout" 
          - "eventNamedHangout" 
          - "hangoutsMeet"  Optional.

      --data-owner <DATA_OWNER>
          The email of the owner of the calendar. Set only for secondary calendars. Read-only.

      --default-reminders <JSON>
          The default reminders that the authenticated user has for this calendar.

      --deleted[=<BOOL>]
          Whether this calendar list entry has been deleted from the calendar list. Read-only. Optional. The default is False.

      --etag <ETAG>
          ETag of the resource.

      --foreground-color <FOREGROUND_COLOR>
          The foreground color of the calendar in the hexadecimal format "#ffffff". This property supersedes the index-based colorId property. To set or change this property, you need to specify colorRgbFormat=true in the parameters of the insert, update and patch methods. Optional.

      --hidden[=<BOOL>]
          Whether the calendar has been hidden from the list. Optional. The attribute is only returned when the calendar is hidden, in which case the value is true.

      --id <ID>
          Identifier of the calendar.

      --kind <KIND>
          Type of the resource ("calendar#calendarListEntry").

      --location <LOCATION>
          Geographic location of the calendar as free-form text. Optional. Read-only.

      --notification-settings <JSON>
          The notifications that the authenticated user is receiving for this calendar.

      --notification-settings-notifications <JSON>
          The list of notifications set for this calendar.

      --primary[=<BOOL>]
          Whether the calendar is the primary calendar of the authenticated user. Read-only. Optional. The default is False.

      --selected[=<BOOL>]
          Whether the calendar content shows up in the calendar UI. Optional. The default is False.

      --summary-override <SUMMARY_OVERRIDE>
          The summary that the authenticated user has set for this calendar. Optional.

      --time-zone <TIME_ZONE>
          The time zone of the calendar. Optional. Read-only.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## calendars.clear

```text
Clears a primary calendar. This operation deletes all events associated with the primary calendar of an account.

Usage: joog [OPTIONS] calendar <ACCOUNT> calendars.clear [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

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

## calendars.delete

```text
Deletes a secondary calendar. Use calendars.clear for clearing all events on primary calendars.

Usage: joog [OPTIONS] calendar <ACCOUNT> calendars.delete [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

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

## calendars.get

```text
Returns metadata for a calendar.

Usage: joog [OPTIONS] calendar <ACCOUNT> calendars.get [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

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

## calendars.insert

```text
Creates a secondary calendar.
The authenticated user for the request is made the data owner of the new calendar.

Note: We recommend to authenticate as the intended data owner of the calendar. You can use domain-wide delegation of authority to allow applications to act on behalf of a specific user. Don't use a service account for authentication. If you use a service account for authentication, the service account is the data owner, which can lead to unexpected behavior. For example, if a service account is the data owner, data ownership cannot be transferred.

Usage: joog [OPTIONS] calendar <ACCOUNT> calendars.insert [COMMAND_OPTIONS]

Mandatory parameters:
      --summary <SUMMARY>
          Title of the calendar.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --description <DESCRIPTION>
          Description of the calendar. Optional.

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

      --auto-accept-invitations[=<BOOL>]
          Whether this calendar automatically accepts invitations. Only valid for resource calendars.

      --conference-properties <JSON>
          Request body field

      --conference-properties-allowed-conference-solution-types <CONFERENCE_PROPERTIES_ALLOWED_CONFERENCE_SOLUTION_TYPES>...
          The types of conference solutions that are supported for this calendar.
          The possible values are:  
          - "eventHangout" 
          - "eventNamedHangout" 
          - "hangoutsMeet"  Optional.

      --data-owner <DATA_OWNER>
          The email of the owner of the calendar. Set only for secondary calendars. Read-only.

      --etag <ETAG>
          ETag of the resource.

      --id <ID>
          Identifier of the calendar. To retrieve IDs call the calendarList.list() method.

      --kind <KIND>
          Type of the resource ("calendar#calendar").

      --label-properties <JSON>
          Request body field

      --label-properties-event-labels <JSON>
          Event labels defined on this calendar. If this is present when updating the calendar, it will replace the existing event labels.
          Extend the list to add a new event label, and remove entities from the list to delete a label from calendar.
          Each calendar can have a maximum of 200 labels.

      --location <LOCATION>
          Geographic location of the calendar as free-form text. Optional.

      --time-zone <TIME_ZONE>
          The time zone of the calendar. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) Optional.

      --working-hours <JSON>
          Represents the information about the user's working hours. Only available for primary calendars of Google Workspace users. To update the working hours, the entire working hours object must be passed. If not specified (or passed as null or an empty object), the existing working hours remain unchanged. To disable working hours or clear the schedule, set workingHoursEnabled to false (and omit availableTimes or pass an empty list).

      --working-hours-available-times <JSON>
          Working hours. Each availability period represents the user's working hours block during one day.

      --working-hours-working-hours-enabled[=<BOOL>]
          Whether the working hours are enabled for this calendar. Cannot be set to true when availableTimes is empty.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## calendars.list

```text
Returns the calendars on the user's calendar list.

Usage: joog [OPTIONS] calendar <ACCOUNT> calendars.list [COMMAND_OPTIONS]

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
          Maximum number of entries returned on one result page. By default the value is 100 entries. The page size can never be larger than 250 entries. Optional.

      --min-access-role <MIN_ACCESS_ROLE>
          The minimum access role for the user in the returned entries. Optional. The default is no restriction. Choices: freeBusyReader, owner, reader, writer, writerWithoutPrivateAccess.

      --page-token <PAGE_TOKEN>
          Token specifying which result page to return. Optional.

      --show-deleted[=<BOOL>]
          Whether to include deleted calendar list entries in the result. Optional. The default is False.

      --show-hidden[=<BOOL>]
          Whether to show hidden entries. Optional. The default is False.

      --show-own-organization-only[=<BOOL>]
          Whether to show only entries for calendars from the organization. This parameter is only applicable to Google Workspace users. Optional. The default is False.

      --sync-token <SYNC_TOKEN>
          Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. If only read-only fields such as calendar properties or ACLs have changed, the entry won't be returned. All entries deleted and hidden since the previous list request will always be in the result set and it is not allowed to set showDeleted neither showHidden to False.
          To ensure client state consistency minAccessRole and showOwnOrganizationOnly query parameters cannot be specified together with nextSyncToken.
          If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
          Learn more about incremental synchronization.
          Optional. The default is to return all entries.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## calendars.patch

```text
Updates metadata for a calendar. This method supports patch semantics.

Usage: joog [OPTIONS] calendar <ACCOUNT> calendars.patch [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

      --description <DESCRIPTION>
          Description of the calendar. Optional.

      --summary <SUMMARY>
          Title of the calendar.

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

      --auto-accept-invitations[=<BOOL>]
          Whether this calendar automatically accepts invitations. Only valid for resource calendars.

      --conference-properties <JSON>
          Request body field

      --conference-properties-allowed-conference-solution-types <CONFERENCE_PROPERTIES_ALLOWED_CONFERENCE_SOLUTION_TYPES>...
          The types of conference solutions that are supported for this calendar.
          The possible values are:  
          - "eventHangout" 
          - "eventNamedHangout" 
          - "hangoutsMeet"  Optional.

      --data-owner <DATA_OWNER>
          The email of the owner of the calendar. Set only for secondary calendars. Read-only.

      --etag <ETAG>
          ETag of the resource.

      --id <ID>
          Identifier of the calendar. To retrieve IDs call the calendarList.list() method.

      --kind <KIND>
          Type of the resource ("calendar#calendar").

      --label-properties <JSON>
          Request body field

      --label-properties-event-labels <JSON>
          Event labels defined on this calendar. If this is present when updating the calendar, it will replace the existing event labels.
          Extend the list to add a new event label, and remove entities from the list to delete a label from calendar.
          Each calendar can have a maximum of 200 labels.

      --location <LOCATION>
          Geographic location of the calendar as free-form text. Optional.

      --time-zone <TIME_ZONE>
          The time zone of the calendar. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) Optional.

      --working-hours <JSON>
          Represents the information about the user's working hours. Only available for primary calendars of Google Workspace users. To update the working hours, the entire working hours object must be passed. If not specified (or passed as null or an empty object), the existing working hours remain unchanged. To disable working hours or clear the schedule, set workingHoursEnabled to false (and omit availableTimes or pass an empty list).

      --working-hours-available-times <JSON>
          Working hours. Each availability period represents the user's working hours block during one day.

      --working-hours-working-hours-enabled[=<BOOL>]
          Whether the working hours are enabled for this calendar. Cannot be set to true when availableTimes is empty.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## calendars.transfer-ownership

```text
Transfers a secondary calendar between users within a Google Workspace organization. Requires user authentication with Manage Calendars administrator privilege, and one of the following authorization scopes: 
- https://www.googleapis.com/auth/calendar 
- https://www.googleapis.com/auth/calendar.calendars In the request, set useAdminAccess to true. The secondary calendar must be active to be transferred. Transferring disabled or deleted calendars isn't supported.

Usage: joog [OPTIONS] calendar <ACCOUNT> calendars.transfer-ownership [COMMAND_OPTIONS]

Mandatory parameters:
      --new-data-owner <NEW_DATA_OWNER>
          The email address of a user who will become the data owner of the calendar.

      --use-admin-access[=<BOOL>]
          When true, the method runs using the user's Google Workspace administrator privileges. The calling user must be a Google Workspace administrator with the Manage Calendars privilege. This method currently only supports admin access, thus only true is accepted for this field.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs, call the calendarList.list method.
          
          [default: primary]

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

## calendars.update

```text
Updates metadata for a calendar.

Usage: joog [OPTIONS] calendar <ACCOUNT> calendars.update [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

      --description <DESCRIPTION>
          Description of the calendar. Optional.

      --summary <SUMMARY>
          Title of the calendar.

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

      --auto-accept-invitations[=<BOOL>]
          Whether this calendar automatically accepts invitations. Only valid for resource calendars.

      --conference-properties <JSON>
          Request body field

      --conference-properties-allowed-conference-solution-types <CONFERENCE_PROPERTIES_ALLOWED_CONFERENCE_SOLUTION_TYPES>...
          The types of conference solutions that are supported for this calendar.
          The possible values are:  
          - "eventHangout" 
          - "eventNamedHangout" 
          - "hangoutsMeet"  Optional.

      --data-owner <DATA_OWNER>
          The email of the owner of the calendar. Set only for secondary calendars. Read-only.

      --etag <ETAG>
          ETag of the resource.

      --id <ID>
          Identifier of the calendar. To retrieve IDs call the calendarList.list() method.

      --kind <KIND>
          Type of the resource ("calendar#calendar").

      --label-properties <JSON>
          Request body field

      --label-properties-event-labels <JSON>
          Event labels defined on this calendar. If this is present when updating the calendar, it will replace the existing event labels.
          Extend the list to add a new event label, and remove entities from the list to delete a label from calendar.
          Each calendar can have a maximum of 200 labels.

      --location <LOCATION>
          Geographic location of the calendar as free-form text. Optional.

      --time-zone <TIME_ZONE>
          The time zone of the calendar. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) Optional.

      --working-hours <JSON>
          Represents the information about the user's working hours. Only available for primary calendars of Google Workspace users. To update the working hours, the entire working hours object must be passed. If not specified (or passed as null or an empty object), the existing working hours remain unchanged. To disable working hours or clear the schedule, set workingHoursEnabled to false (and omit availableTimes or pass an empty list).

      --working-hours-available-times <JSON>
          Working hours. Each availability period represents the user's working hours block during one day.

      --working-hours-working-hours-enabled[=<BOOL>]
          Whether the working hours are enabled for this calendar. Cannot be set to true when availableTimes is empty.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## colors.get

```text
Returns the color definitions for calendars and events.

Usage: joog [OPTIONS] calendar <ACCOUNT> colors.get [COMMAND_OPTIONS]

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

## events.delete

```text
Deletes an event.

Usage: joog [OPTIONS] calendar <ACCOUNT> events.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --event-id <EVENT_ID>
          Event identifier.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --send-updates <SEND_UPDATES>
          Guests who should receive notifications about the deletion of the event. Choices: all, externalOnly, none.

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

      --send-notifications[=<BOOL>]
          Deprecated. Please use sendUpdates instead.
          
          Whether to send notifications about the deletion of the event. Note that some emails might still be sent even if you set the value to false. The default is false.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## events.get

```text
Returns an event based on its Google Calendar ID. To retrieve an event using its iCalendar ID, call the events.list method using the iCalUID parameter.

Usage: joog [OPTIONS] calendar <ACCOUNT> events.get [COMMAND_OPTIONS]

Mandatory parameters:
      --event-id <EVENT_ID>
          Event identifier.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

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

      --always-include-email[=<BOOL>]
          Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).

      --max-attendees <MAX_ATTENDEES>
          The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.

      --time-zone <TIME_ZONE>
          Time zone used in the response. Optional. The default is the time zone of the calendar.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## events.import

```text
Imports an event. This operation is used to add a private copy of an existing event to a calendar. Only events with an eventType of default may be imported.
Deprecated behavior: If a non-default event is imported, its type will be changed to default and any event-type-specific properties it may have will be dropped.

Input choices:
  End is required: use --end-date for an all-day event or --end-time for a timed event.
  When setting originalStartTime, use --original-start-time-date for an all-day event or --original-start-time-date-time for a timed event.
  Start is required: use --start-date for an all-day event or --start-time for a timed event.

Usage: joog [OPTIONS] calendar <ACCOUNT> events.import [COMMAND_OPTIONS]

Mandatory parameters:
      --i-cal-uid <I_CAL_UID>
          Event unique identifier as defined in RFC5545. It is used to uniquely identify events accross calendaring systems and must be supplied when importing events via the import method.
          Note that the iCalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same iCalUIDs. To retrieve an event using its iCalUID, call the events.list method using the iCalUID parameter. To retrieve an event using its id, call the events.get method.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --attendees <JSON>
          The attendees of the event. See the Events with attendees guide for more information on scheduling events with other calendar users. Service accounts need to use domain-wide delegation of authority to populate the attendee list.

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

      --description <DESCRIPTION>
          Description of the event. Can contain HTML. Optional.

      --end-date <END_DATE>
          The date, in the format "yyyy-mm-dd", if this is an all-day event.

      --end-time <END_TIME>
          The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.

      --end-time-zone <END_TIME_ZONE>
          The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

      --start-date <START_DATE>
          The date, in the format "yyyy-mm-dd", if this is an all-day event.

      --start-time <START_TIME>
          The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.

      --start-time-zone <START_TIME_ZONE>
          The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

      --status <STATUS>
          Status of the event. Optional. Possible values are:  
          - "confirmed" - The event is confirmed. This is the default status. 
          - "tentative" - The event is tentatively confirmed. 
          - "cancelled" - The event is cancelled (deleted). The list method returns cancelled events only on incremental sync (when syncToken or updatedMin are specified) or if the showDeleted flag is set to true. The get method always returns them.
          A cancelled status represents two different states depending on the event type:  
          - Cancelled exceptions of an uncancelled recurring event indicate that this instance should no longer be presented to the user. Clients should store these events for the lifetime of the parent recurring event.
          Cancelled exceptions are only guaranteed to have values for the id, recurringEventId and originalStartTime fields populated. The other fields might be empty.  
          - All other cancelled events represent deleted events. Clients should remove their locally synced copies. Such cancelled events will eventually disappear, so do not rely on them being available indefinitely.
          Deleted events are only guaranteed to have the id field populated.   On the organizer's calendar, cancelled events continue to expose event details (summary, location, etc.) so that they can be restored (undeleted). Similarly, the events to which the user was invited and that they manually removed continue to provide details. However, incremental sync requests with showDeleted set to false will not return these details.
          If an event changes its organizer (for example via the move operation) and the original organizer is not on the attendee list, it will leave behind a cancelled event where only the id field is guaranteed to be populated.

      --summary <SUMMARY>
          Title of the event.

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

      --anyone-can-add-self[=<BOOL>]
          Whether anyone can invite themselves to the event (deprecated). Optional. The default is False.

      --attachments <JSON>
          File attachments for the event.
          In order to modify attachments the supportsAttachments request parameter should be set to true.
          There can be at most 25 attachments per event,

      --attendees-omitted[=<BOOL>]
          Whether attendees may have been omitted from the event's representation. When retrieving an event, this may be due to a restriction specified by the maxAttendee query parameter. When updating an event, this can be used to only update the participant's response. Optional. The default is False.

      --birthday-properties <JSON>
          Request body field

      --birthday-properties-contact <BIRTHDAY_PROPERTIES_CONTACT>
          Resource name of the contact this birthday event is linked to. This can be used to fetch contact details from People API. Format: "people/c12345". Read-only.

      --birthday-properties-custom-type-name <BIRTHDAY_PROPERTIES_CUSTOM_TYPE_NAME>
          Custom type label specified for this event. This is populated if birthdayProperties.type is set to "custom". Read-only.

      --birthday-properties-type <BIRTHDAY_PROPERTIES_TYPE>
          Type of birthday or special event. Possible values are:  
          - "anniversary" - An anniversary other than birthday. Always has a contact. 
          - "birthday" - A birthday event. This is the default value. 
          - "custom" - A special date whose label is further specified in the customTypeName field. Always has a contact. 
          - "other" - A special date which does not fall into the other categories, and does not have a custom label. Always has a contact. 
          - "self" - Calendar owner's own birthday. Cannot have a contact.  The Calendar API only supports creating events with the type "birthday". The type cannot be changed after the event is created.

      --color-id <COLOR_ID>
          The color of the event. This is an ID referring to an entry in the event section of the colors definition (see the  colors endpoint). Optional.

      --conference-data <JSON>
          Request body field

      --conference-data-conference-id <CONFERENCE_DATA_CONFERENCE_ID>
          The ID of the conference.
          Can be used by developers to keep track of conferences, should not be displayed to users.
          The ID value is formed differently for each conference solution type:  
          - eventHangout: ID is not set. (This conference type is deprecated.)
          - eventNamedHangout: ID is the name of the Hangout. (This conference type is deprecated.)
          - hangoutsMeet: ID is the 10-letter meeting code, for example aaa-bbbb-ccc.
          - addOn: ID is defined by the third-party provider.  Optional.

      --conference-data-conference-solution-icon-uri <CONFERENCE_DATA_CONFERENCE_SOLUTION_ICON_URI>
          The user-visible icon for this solution.

      --conference-data-conference-solution-key-type <CONFERENCE_DATA_CONFERENCE_SOLUTION_KEY_TYPE>
          The conference solution type.
          If a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.
          The possible values are:  
          - "eventHangout" for Hangouts for consumers (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "eventNamedHangout" for classic Hangouts for Google Workspace users (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "hangoutsMeet" for Google Meet (http://meet.google.com)
          - "addOn" for 3P conference providers

      --conference-data-conference-solution-name <CONFERENCE_DATA_CONFERENCE_SOLUTION_NAME>
          The user-visible name of this solution. Not localized.

      --conference-data-create-request-conference-solution-key-type <CONFERENCE_DATA_CREATE_REQUEST_CONFERENCE_SOLUTION_KEY_TYPE>
          The conference solution type.
          If a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.
          The possible values are:  
          - "eventHangout" for Hangouts for consumers (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "eventNamedHangout" for classic Hangouts for Google Workspace users (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "hangoutsMeet" for Google Meet (http://meet.google.com)
          - "addOn" for 3P conference providers

      --conference-data-create-request-request-id <CONFERENCE_DATA_CREATE_REQUEST_REQUEST_ID>
          The client-generated unique ID for this request.
          Clients should regenerate this ID for every new request. If an ID provided is the same as for the previous request, the request is ignored.

      --conference-data-create-request-status-status-code <CONFERENCE_DATA_CREATE_REQUEST_STATUS_STATUS_CODE>
          The current status of the conference create request. Read-only.
          The possible values are:  
          - "pending": the conference create request is still being processed.
          - "success": the conference create request succeeded, the entry points are populated.
          - "failure": the conference create request failed, there are no entry points.

      --conference-data-entry-points <JSON>
          Information about individual conference entry points, such as URLs or phone numbers.
          All of them must belong to the same conference.
          Either conferenceSolution and at least one entryPoint, or createRequest is required.

      --conference-data-notes <CONFERENCE_DATA_NOTES>
          Additional notes (such as instructions from the domain administrator, legal notices) to display to the user. Can contain HTML. The maximum length is 2048 characters. Optional.

      --conference-data-signature <CONFERENCE_DATA_SIGNATURE>
          The signature of the conference data.
          Generated on server side.
          Unset for a conference with a failed create request.
          Optional for a conference with a pending create request.

      --conference-data-version <CONFERENCE_DATA_VERSION>
          Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0.

      --created <CREATED>
          Creation time of the event (as a RFC3339 timestamp). Read-only.

      --creator <JSON>
          The creator of the event. Read-only.

      --creator-display-name <CREATOR_DISPLAY_NAME>
          The creator's name, if available.

      --creator-email <CREATOR_EMAIL>
          The creator's email address, if available.

      --creator-id <CREATOR_ID>
          The creator's Profile ID, if available.

      --creator-self[=<BOOL>]
          Whether the creator corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.

      --end <JSON>
          Request body field

      --end-time-unspecified[=<BOOL>]
          Whether the end time is actually unspecified. An end time is still provided for compatibility reasons, even if this attribute is set to True. The default is False.

      --etag <ETAG>
          ETag of the resource.

      --event-label-id <EVENT_LABEL_ID>
          The ID of the event label assigned to the event. Optional. This refers to the ID of an entry in the labelProperties.eventLabels property of the calendar (see the Calendars.get endpoint.)
          This property supersedes the index-based colorId property. To set or change this property, you need to specify eventLabelVersion=1 in the parameters of the insert, import, update, and patch methods.
          Setting an empty string, or not setting this field at all, will remove the existing label from the event.

      --event-label-version <EVENT_LABEL_VERSION>
          Version number of the event label feature supported by the API client. Version 0 assumes no event label support and processes the colorId field for color management. Version 1 enables support for event labels, and processes the eventLabelId in the event's body. In this case, the colorId field is ignored. The default is 0.

      --event-type <EVENT_TYPE>
          Specific type of the event. This cannot be modified after the event is created. Possible values are:  
          - "birthday" - A special all-day event with an annual recurrence. 
          - "default" - A regular event or not further specified. 
          - "focusTime" - A focus-time event. 
          - "fromGmail" - An event from Gmail. This type of event cannot be created. 
          - "outOfOffice" - An out-of-office event. 
          - "workingLocation" - A working location event.

      --extended-properties <JSON>
          Extended properties of the event.

      --focus-time-properties <JSON>
          Request body field

      --focus-time-properties-auto-decline-mode <FOCUS_TIME_PROPERTIES_AUTO_DECLINE_MODE>
          Whether to decline meeting invitations which overlap Focus Time events. Valid values are declineNone, meaning that no meeting invitations are declined; declineAllConflictingInvitations, meaning that all conflicting meeting invitations that conflict with the event are declined; and declineOnlyNewConflictingInvitations, meaning that only new conflicting meeting invitations which arrive while the Focus Time event is present are to be declined.

      --focus-time-properties-chat-status <FOCUS_TIME_PROPERTIES_CHAT_STATUS>
          The status to mark the user in Chat and related products. This can be available or doNotDisturb.

      --focus-time-properties-decline-message <FOCUS_TIME_PROPERTIES_DECLINE_MESSAGE>
          Response message to set if an existing event or new invitation is automatically declined by Calendar.

      --gadget <JSON>
          A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.

      --gadget-display <GADGET_DISPLAY>
          The gadget's display mode. Deprecated. Possible values are:  
          - "icon" - The gadget displays next to the event's title in the calendar view. 
          - "chip" - The gadget displays when the event is clicked.

      --gadget-height <GADGET_HEIGHT>
          The gadget's height in pixels. The height must be an integer greater than 0. Optional. Deprecated.

      --gadget-icon-link <GADGET_ICON_LINK>
          The gadget's icon URL. The URL scheme must be HTTPS. Deprecated.

      --gadget-link <GADGET_LINK>
          The gadget's URL. The URL scheme must be HTTPS. Deprecated.

      --gadget-title <GADGET_TITLE>
          The gadget's title. Deprecated.

      --gadget-type <GADGET_TYPE>
          The gadget's type. Deprecated.

      --gadget-width <GADGET_WIDTH>
          The gadget's width in pixels. The width must be an integer greater than 0. Optional. Deprecated.

      --guests-can-invite-others[=<BOOL>]
          Whether attendees other than the organizer can invite others to the event. Optional. The default is True.

      --guests-can-modify[=<BOOL>]
          Whether attendees other than the organizer can modify the event. Optional. The default is False.

      --guests-can-see-other-guests[=<BOOL>]
          Whether attendees other than the organizer can see who the event's attendees are. Optional. The default is True.

      --hangout-link <HANGOUT_LINK>
          An absolute link to the Google Hangout associated with this event. Read-only.

      --html-link <HTML_LINK>
          An absolute link to this event in the Google Calendar Web UI. Read-only.

      --id <ID>
          Opaque identifier of the event. When creating new single or recurring events, you can specify their IDs. Provided IDs must follow these rules:  
          - characters allowed in the ID are those used in base32hex encoding, i.e. lowercase letters a-v and digits 0-9, see section 3.1.2 in RFC2938 
          - the length of the ID must be between 5 and 1024 characters 
          - the ID must be unique per calendar  Due to the globally distributed nature of the system, we cannot guarantee that ID collisions will be detected at event creation time. To minimize the risk of collisions we recommend using an established UUID algorithm such as one described in RFC4122.
          If you do not specify an ID, it will be automatically generated by the server.
          Note that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs.

      --kind <KIND>
          Type of the resource ("calendar#event").

      --location <LOCATION>
          Geographic location of the event as free-form text. Optional.

      --locked[=<BOOL>]
          Whether this is a locked event copy where no changes can be made to the main event fields "summary", "description", "location", "start", "end" or "recurrence". The default is False. Read-Only.

      --organizer <JSON>
          The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.

      --organizer-display-name <ORGANIZER_DISPLAY_NAME>
          The organizer's name, if available.

      --organizer-email <ORGANIZER_EMAIL>
          The organizer's email address, if available. It must be a valid email address as per RFC5322.

      --organizer-id <ORGANIZER_ID>
          The organizer's Profile ID, if available.

      --organizer-self[=<BOOL>]
          Whether the organizer corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.

      --original-start-time <JSON>
          Request body field

      --original-start-time-date <ORIGINAL_START_TIME_DATE>
          The date, in the format "yyyy-mm-dd", if this is an all-day event.

      --original-start-time-date-time <ORIGINAL_START_TIME_DATE_TIME>
          The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.

      --original-start-time-time-zone <ORIGINAL_START_TIME_TIME_ZONE>
          The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

      --out-of-office-properties <JSON>
          Request body field

      --out-of-office-properties-auto-decline-mode <OUT_OF_OFFICE_PROPERTIES_AUTO_DECLINE_MODE>
          Whether to decline meeting invitations which overlap Out of office events. Valid values are declineNone, meaning that no meeting invitations are declined; declineAllConflictingInvitations, meaning that all conflicting meeting invitations that conflict with the event are declined; and declineOnlyNewConflictingInvitations, meaning that only new conflicting meeting invitations which arrive while the Out of office event is present are to be declined.

      --out-of-office-properties-decline-message <OUT_OF_OFFICE_PROPERTIES_DECLINE_MESSAGE>
          Response message to set if an existing event or new invitation is automatically declined by Calendar.

      --private-copy[=<BOOL>]
          If set to True, Event propagation is disabled. Note that it is not the same thing as Private event properties. Optional. Immutable. The default is False.

      --recurrence <RECURRENCE>...
          List of RRULE, EXRULE, RDATE and EXDATE lines for a recurring event, as specified in RFC5545. Note that DTSTART and DTEND lines are not allowed in this field; event start and end times are specified in the start and end fields. This field is omitted for single events or instances of recurring events.

      --recurring-event-id <RECURRING_EVENT_ID>
          For an instance of a recurring event, this is the id of the recurring event to which this instance belongs. Immutable.

      --reminders <JSON>
          Information about the event's reminders for the authenticated user. Note that changing reminders does not also change the updated property of the enclosing event.

      --reminders-overrides <JSON>
          If the event doesn't use the default reminders, this lists the reminders specific to the event, or, if not set, indicates that no reminders are set for this event. The maximum number of override reminders is 5.

      --reminders-use-default[=<BOOL>]
          Whether the default reminders of the calendar apply to the event.

      --sequence <SEQUENCE>
          Sequence number as per iCalendar.

      --source <JSON>
          Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.

      --source-title <SOURCE_TITLE>
          Title of the source; for example a title of a web page or an email subject.

      --source-url <SOURCE_URL>
          URL of the source pointing to a resource. The URL scheme must be HTTP or HTTPS.

      --start <JSON>
          Request body field

      --supports-attachments[=<BOOL>]
          Whether API client performing operation supports event attachments. Optional. The default is False.

      --transparency <TRANSPARENCY>
          Whether the event blocks time on the calendar. Optional. Possible values are:  
          - "opaque" - Default value. The event does block time on the calendar. This is equivalent to setting Show me as to Busy in the Calendar UI. 
          - "transparent" - The event does not block time on the calendar. This is equivalent to setting Show me as to Available in the Calendar UI.

      --updated <UPDATED>
          Last modification time of the main event data (as a RFC3339 timestamp). Updating event reminders will not cause this to change. Read-only.

      --visibility <VISIBILITY>
          Visibility of the event. Optional. Possible values are:  
          - "default" - Uses the default visibility for events on the calendar. This is the default value. 
          - "public" - The event is public and event details are visible to all readers of the calendar. 
          - "private" - The event is private and only event attendees may view event details. 
          - "confidential" - The event is private. This value is provided for compatibility reasons.  
          Note on recurring events: Changing the visibility of a single instance of a recurring event can affect all instances of the series. If the new setting is more restrictive (e.g. from public to private), it is applied to all instances. If the new setting is less restrictive (e.g. from private to public), the change is ignored. To make a recurring event less restrictive, you must update the parent recurring event.

      --working-location-properties <JSON>
          Request body field

      --working-location-properties-custom-location-label <WORKING_LOCATION_PROPERTIES_CUSTOM_LOCATION_LABEL>
          An optional extra label for additional information.

      --working-location-properties-home-office <JSON>
          If present, specifies that the user is working at home.

      --working-location-properties-office-location-building-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_BUILDING_ID>
          An optional building identifier. This should reference a building ID in the organization's Resources database.

      --working-location-properties-office-location-desk-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_DESK_ID>
          An optional desk identifier.

      --working-location-properties-office-location-floor-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_FLOOR_ID>
          An optional floor identifier.

      --working-location-properties-office-location-floor-section-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_FLOOR_SECTION_ID>
          An optional floor section identifier.

      --working-location-properties-office-location-label <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_LABEL>
          The office name that's displayed in Calendar Web and Mobile clients. We recommend you reference a building name in the organization's Resources database.

      --working-location-properties-type <WORKING_LOCATION_PROPERTIES_TYPE>
          Type of the working location. Possible values are:  
          - "homeOffice" - The user is working at home. 
          - "officeLocation" - The user is working from an office. 
          - "customLocation" - The user is working from a custom location.  Any details are specified in a sub-field of the specified name, but this field may be missing if empty. Any other fields are ignored.
          Required when adding working location properties.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## events.insert

```text
Creates an event.

Input choices:
  End is required: use --end-date for an all-day event or --end-time for a timed event.
  When setting originalStartTime, use --original-start-time-date for an all-day event or --original-start-time-date-time for a timed event.
  Start is required: use --start-date for an all-day event or --start-time for a timed event.

Usage: joog [OPTIONS] calendar <ACCOUNT> events.insert [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --attendees <JSON>
          The attendees of the event. See the Events with attendees guide for more information on scheduling events with other calendar users. Service accounts need to use domain-wide delegation of authority to populate the attendee list.

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

      --description <DESCRIPTION>
          Description of the event. Can contain HTML. Optional.

      --end-date <END_DATE>
          The date, in the format "yyyy-mm-dd", if this is an all-day event.

      --end-time <END_TIME>
          The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.

      --end-time-zone <END_TIME_ZONE>
          The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

      --send-updates <SEND_UPDATES>
          Whether to send notifications about the creation of the new event. Note that some emails might still be sent. The default is false. Choices: all, externalOnly, none.

      --start-date <START_DATE>
          The date, in the format "yyyy-mm-dd", if this is an all-day event.

      --start-time <START_TIME>
          The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.

      --start-time-zone <START_TIME_ZONE>
          The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

      --status <STATUS>
          Status of the event. Optional. Possible values are:  
          - "confirmed" - The event is confirmed. This is the default status. 
          - "tentative" - The event is tentatively confirmed. 
          - "cancelled" - The event is cancelled (deleted). The list method returns cancelled events only on incremental sync (when syncToken or updatedMin are specified) or if the showDeleted flag is set to true. The get method always returns them.
          A cancelled status represents two different states depending on the event type:  
          - Cancelled exceptions of an uncancelled recurring event indicate that this instance should no longer be presented to the user. Clients should store these events for the lifetime of the parent recurring event.
          Cancelled exceptions are only guaranteed to have values for the id, recurringEventId and originalStartTime fields populated. The other fields might be empty.  
          - All other cancelled events represent deleted events. Clients should remove their locally synced copies. Such cancelled events will eventually disappear, so do not rely on them being available indefinitely.
          Deleted events are only guaranteed to have the id field populated.   On the organizer's calendar, cancelled events continue to expose event details (summary, location, etc.) so that they can be restored (undeleted). Similarly, the events to which the user was invited and that they manually removed continue to provide details. However, incremental sync requests with showDeleted set to false will not return these details.
          If an event changes its organizer (for example via the move operation) and the original organizer is not on the attendee list, it will leave behind a cancelled event where only the id field is guaranteed to be populated.

      --summary <SUMMARY>
          Title of the event.

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

      --anyone-can-add-self[=<BOOL>]
          Whether anyone can invite themselves to the event (deprecated). Optional. The default is False.

      --attachments <JSON>
          File attachments for the event.
          In order to modify attachments the supportsAttachments request parameter should be set to true.
          There can be at most 25 attachments per event,

      --attendees-omitted[=<BOOL>]
          Whether attendees may have been omitted from the event's representation. When retrieving an event, this may be due to a restriction specified by the maxAttendee query parameter. When updating an event, this can be used to only update the participant's response. Optional. The default is False.

      --birthday-properties <JSON>
          Request body field

      --birthday-properties-contact <BIRTHDAY_PROPERTIES_CONTACT>
          Resource name of the contact this birthday event is linked to. This can be used to fetch contact details from People API. Format: "people/c12345". Read-only.

      --birthday-properties-custom-type-name <BIRTHDAY_PROPERTIES_CUSTOM_TYPE_NAME>
          Custom type label specified for this event. This is populated if birthdayProperties.type is set to "custom". Read-only.

      --birthday-properties-type <BIRTHDAY_PROPERTIES_TYPE>
          Type of birthday or special event. Possible values are:  
          - "anniversary" - An anniversary other than birthday. Always has a contact. 
          - "birthday" - A birthday event. This is the default value. 
          - "custom" - A special date whose label is further specified in the customTypeName field. Always has a contact. 
          - "other" - A special date which does not fall into the other categories, and does not have a custom label. Always has a contact. 
          - "self" - Calendar owner's own birthday. Cannot have a contact.  The Calendar API only supports creating events with the type "birthday". The type cannot be changed after the event is created.

      --color-id <COLOR_ID>
          The color of the event. This is an ID referring to an entry in the event section of the colors definition (see the  colors endpoint). Optional.

      --conference-data <JSON>
          Request body field

      --conference-data-conference-id <CONFERENCE_DATA_CONFERENCE_ID>
          The ID of the conference.
          Can be used by developers to keep track of conferences, should not be displayed to users.
          The ID value is formed differently for each conference solution type:  
          - eventHangout: ID is not set. (This conference type is deprecated.)
          - eventNamedHangout: ID is the name of the Hangout. (This conference type is deprecated.)
          - hangoutsMeet: ID is the 10-letter meeting code, for example aaa-bbbb-ccc.
          - addOn: ID is defined by the third-party provider.  Optional.

      --conference-data-conference-solution-icon-uri <CONFERENCE_DATA_CONFERENCE_SOLUTION_ICON_URI>
          The user-visible icon for this solution.

      --conference-data-conference-solution-key-type <CONFERENCE_DATA_CONFERENCE_SOLUTION_KEY_TYPE>
          The conference solution type.
          If a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.
          The possible values are:  
          - "eventHangout" for Hangouts for consumers (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "eventNamedHangout" for classic Hangouts for Google Workspace users (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "hangoutsMeet" for Google Meet (http://meet.google.com)
          - "addOn" for 3P conference providers

      --conference-data-conference-solution-name <CONFERENCE_DATA_CONFERENCE_SOLUTION_NAME>
          The user-visible name of this solution. Not localized.

      --conference-data-create-request-conference-solution-key-type <CONFERENCE_DATA_CREATE_REQUEST_CONFERENCE_SOLUTION_KEY_TYPE>
          The conference solution type.
          If a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.
          The possible values are:  
          - "eventHangout" for Hangouts for consumers (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "eventNamedHangout" for classic Hangouts for Google Workspace users (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "hangoutsMeet" for Google Meet (http://meet.google.com)
          - "addOn" for 3P conference providers

      --conference-data-create-request-request-id <CONFERENCE_DATA_CREATE_REQUEST_REQUEST_ID>
          The client-generated unique ID for this request.
          Clients should regenerate this ID for every new request. If an ID provided is the same as for the previous request, the request is ignored.

      --conference-data-create-request-status-status-code <CONFERENCE_DATA_CREATE_REQUEST_STATUS_STATUS_CODE>
          The current status of the conference create request. Read-only.
          The possible values are:  
          - "pending": the conference create request is still being processed.
          - "success": the conference create request succeeded, the entry points are populated.
          - "failure": the conference create request failed, there are no entry points.

      --conference-data-entry-points <JSON>
          Information about individual conference entry points, such as URLs or phone numbers.
          All of them must belong to the same conference.
          Either conferenceSolution and at least one entryPoint, or createRequest is required.

      --conference-data-notes <CONFERENCE_DATA_NOTES>
          Additional notes (such as instructions from the domain administrator, legal notices) to display to the user. Can contain HTML. The maximum length is 2048 characters. Optional.

      --conference-data-signature <CONFERENCE_DATA_SIGNATURE>
          The signature of the conference data.
          Generated on server side.
          Unset for a conference with a failed create request.
          Optional for a conference with a pending create request.

      --conference-data-version <CONFERENCE_DATA_VERSION>
          Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0.

      --created <CREATED>
          Creation time of the event (as a RFC3339 timestamp). Read-only.

      --creator <JSON>
          The creator of the event. Read-only.

      --creator-display-name <CREATOR_DISPLAY_NAME>
          The creator's name, if available.

      --creator-email <CREATOR_EMAIL>
          The creator's email address, if available.

      --creator-id <CREATOR_ID>
          The creator's Profile ID, if available.

      --creator-self[=<BOOL>]
          Whether the creator corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.

      --end <JSON>
          Request body field

      --end-time-unspecified[=<BOOL>]
          Whether the end time is actually unspecified. An end time is still provided for compatibility reasons, even if this attribute is set to True. The default is False.

      --etag <ETAG>
          ETag of the resource.

      --event-label-id <EVENT_LABEL_ID>
          The ID of the event label assigned to the event. Optional. This refers to the ID of an entry in the labelProperties.eventLabels property of the calendar (see the Calendars.get endpoint.)
          This property supersedes the index-based colorId property. To set or change this property, you need to specify eventLabelVersion=1 in the parameters of the insert, import, update, and patch methods.
          Setting an empty string, or not setting this field at all, will remove the existing label from the event.

      --event-label-version <EVENT_LABEL_VERSION>
          Version number of the event label feature supported by the API client. Version 0 assumes no event label support and processes the colorId field for color management. Version 1 enables support for event labels, and processes the eventLabelId in the event's body. In this case, the colorId field is ignored. The default is 0.

      --event-type <EVENT_TYPE>
          Specific type of the event. This cannot be modified after the event is created. Possible values are:  
          - "birthday" - A special all-day event with an annual recurrence. 
          - "default" - A regular event or not further specified. 
          - "focusTime" - A focus-time event. 
          - "fromGmail" - An event from Gmail. This type of event cannot be created. 
          - "outOfOffice" - An out-of-office event. 
          - "workingLocation" - A working location event.

      --extended-properties <JSON>
          Extended properties of the event.

      --focus-time-properties <JSON>
          Request body field

      --focus-time-properties-auto-decline-mode <FOCUS_TIME_PROPERTIES_AUTO_DECLINE_MODE>
          Whether to decline meeting invitations which overlap Focus Time events. Valid values are declineNone, meaning that no meeting invitations are declined; declineAllConflictingInvitations, meaning that all conflicting meeting invitations that conflict with the event are declined; and declineOnlyNewConflictingInvitations, meaning that only new conflicting meeting invitations which arrive while the Focus Time event is present are to be declined.

      --focus-time-properties-chat-status <FOCUS_TIME_PROPERTIES_CHAT_STATUS>
          The status to mark the user in Chat and related products. This can be available or doNotDisturb.

      --focus-time-properties-decline-message <FOCUS_TIME_PROPERTIES_DECLINE_MESSAGE>
          Response message to set if an existing event or new invitation is automatically declined by Calendar.

      --gadget <JSON>
          A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.

      --gadget-display <GADGET_DISPLAY>
          The gadget's display mode. Deprecated. Possible values are:  
          - "icon" - The gadget displays next to the event's title in the calendar view. 
          - "chip" - The gadget displays when the event is clicked.

      --gadget-height <GADGET_HEIGHT>
          The gadget's height in pixels. The height must be an integer greater than 0. Optional. Deprecated.

      --gadget-icon-link <GADGET_ICON_LINK>
          The gadget's icon URL. The URL scheme must be HTTPS. Deprecated.

      --gadget-link <GADGET_LINK>
          The gadget's URL. The URL scheme must be HTTPS. Deprecated.

      --gadget-title <GADGET_TITLE>
          The gadget's title. Deprecated.

      --gadget-type <GADGET_TYPE>
          The gadget's type. Deprecated.

      --gadget-width <GADGET_WIDTH>
          The gadget's width in pixels. The width must be an integer greater than 0. Optional. Deprecated.

      --guests-can-invite-others[=<BOOL>]
          Whether attendees other than the organizer can invite others to the event. Optional. The default is True.

      --guests-can-modify[=<BOOL>]
          Whether attendees other than the organizer can modify the event. Optional. The default is False.

      --guests-can-see-other-guests[=<BOOL>]
          Whether attendees other than the organizer can see who the event's attendees are. Optional. The default is True.

      --hangout-link <HANGOUT_LINK>
          An absolute link to the Google Hangout associated with this event. Read-only.

      --html-link <HTML_LINK>
          An absolute link to this event in the Google Calendar Web UI. Read-only.

      --i-cal-uid <I_CAL_UID>
          Event unique identifier as defined in RFC5545. It is used to uniquely identify events accross calendaring systems and must be supplied when importing events via the import method.
          Note that the iCalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same iCalUIDs. To retrieve an event using its iCalUID, call the events.list method using the iCalUID parameter. To retrieve an event using its id, call the events.get method.

      --id <ID>
          Opaque identifier of the event. When creating new single or recurring events, you can specify their IDs. Provided IDs must follow these rules:  
          - characters allowed in the ID are those used in base32hex encoding, i.e. lowercase letters a-v and digits 0-9, see section 3.1.2 in RFC2938 
          - the length of the ID must be between 5 and 1024 characters 
          - the ID must be unique per calendar  Due to the globally distributed nature of the system, we cannot guarantee that ID collisions will be detected at event creation time. To minimize the risk of collisions we recommend using an established UUID algorithm such as one described in RFC4122.
          If you do not specify an ID, it will be automatically generated by the server.
          Note that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs.

      --kind <KIND>
          Type of the resource ("calendar#event").

      --location <LOCATION>
          Geographic location of the event as free-form text. Optional.

      --locked[=<BOOL>]
          Whether this is a locked event copy where no changes can be made to the main event fields "summary", "description", "location", "start", "end" or "recurrence". The default is False. Read-Only.

      --max-attendees <MAX_ATTENDEES>
          The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.

      --organizer <JSON>
          The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.

      --organizer-display-name <ORGANIZER_DISPLAY_NAME>
          The organizer's name, if available.

      --organizer-email <ORGANIZER_EMAIL>
          The organizer's email address, if available. It must be a valid email address as per RFC5322.

      --organizer-id <ORGANIZER_ID>
          The organizer's Profile ID, if available.

      --organizer-self[=<BOOL>]
          Whether the organizer corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.

      --original-start-time <JSON>
          Request body field

      --original-start-time-date <ORIGINAL_START_TIME_DATE>
          The date, in the format "yyyy-mm-dd", if this is an all-day event.

      --original-start-time-date-time <ORIGINAL_START_TIME_DATE_TIME>
          The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.

      --original-start-time-time-zone <ORIGINAL_START_TIME_TIME_ZONE>
          The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

      --out-of-office-properties <JSON>
          Request body field

      --out-of-office-properties-auto-decline-mode <OUT_OF_OFFICE_PROPERTIES_AUTO_DECLINE_MODE>
          Whether to decline meeting invitations which overlap Out of office events. Valid values are declineNone, meaning that no meeting invitations are declined; declineAllConflictingInvitations, meaning that all conflicting meeting invitations that conflict with the event are declined; and declineOnlyNewConflictingInvitations, meaning that only new conflicting meeting invitations which arrive while the Out of office event is present are to be declined.

      --out-of-office-properties-decline-message <OUT_OF_OFFICE_PROPERTIES_DECLINE_MESSAGE>
          Response message to set if an existing event or new invitation is automatically declined by Calendar.

      --private-copy[=<BOOL>]
          If set to True, Event propagation is disabled. Note that it is not the same thing as Private event properties. Optional. Immutable. The default is False.

      --recurrence <RECURRENCE>...
          List of RRULE, EXRULE, RDATE and EXDATE lines for a recurring event, as specified in RFC5545. Note that DTSTART and DTEND lines are not allowed in this field; event start and end times are specified in the start and end fields. This field is omitted for single events or instances of recurring events.

      --recurring-event-id <RECURRING_EVENT_ID>
          For an instance of a recurring event, this is the id of the recurring event to which this instance belongs. Immutable.

      --reminders <JSON>
          Information about the event's reminders for the authenticated user. Note that changing reminders does not also change the updated property of the enclosing event.

      --reminders-overrides <JSON>
          If the event doesn't use the default reminders, this lists the reminders specific to the event, or, if not set, indicates that no reminders are set for this event. The maximum number of override reminders is 5.

      --reminders-use-default[=<BOOL>]
          Whether the default reminders of the calendar apply to the event.

      --send-notifications[=<BOOL>]
          Deprecated. Please use sendUpdates instead.
          
          Whether to send notifications about the creation of the new event. Note that some emails might still be sent even if you set the value to false. The default is false.

      --sequence <SEQUENCE>
          Sequence number as per iCalendar.

      --source <JSON>
          Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.

      --source-title <SOURCE_TITLE>
          Title of the source; for example a title of a web page or an email subject.

      --source-url <SOURCE_URL>
          URL of the source pointing to a resource. The URL scheme must be HTTP or HTTPS.

      --start <JSON>
          Request body field

      --supports-attachments[=<BOOL>]
          Whether API client performing operation supports event attachments. Optional. The default is False.

      --transparency <TRANSPARENCY>
          Whether the event blocks time on the calendar. Optional. Possible values are:  
          - "opaque" - Default value. The event does block time on the calendar. This is equivalent to setting Show me as to Busy in the Calendar UI. 
          - "transparent" - The event does not block time on the calendar. This is equivalent to setting Show me as to Available in the Calendar UI.

      --updated <UPDATED>
          Last modification time of the main event data (as a RFC3339 timestamp). Updating event reminders will not cause this to change. Read-only.

      --visibility <VISIBILITY>
          Visibility of the event. Optional. Possible values are:  
          - "default" - Uses the default visibility for events on the calendar. This is the default value. 
          - "public" - The event is public and event details are visible to all readers of the calendar. 
          - "private" - The event is private and only event attendees may view event details. 
          - "confidential" - The event is private. This value is provided for compatibility reasons.  
          Note on recurring events: Changing the visibility of a single instance of a recurring event can affect all instances of the series. If the new setting is more restrictive (e.g. from public to private), it is applied to all instances. If the new setting is less restrictive (e.g. from private to public), the change is ignored. To make a recurring event less restrictive, you must update the parent recurring event.

      --working-location-properties <JSON>
          Request body field

      --working-location-properties-custom-location-label <WORKING_LOCATION_PROPERTIES_CUSTOM_LOCATION_LABEL>
          An optional extra label for additional information.

      --working-location-properties-home-office <JSON>
          If present, specifies that the user is working at home.

      --working-location-properties-office-location-building-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_BUILDING_ID>
          An optional building identifier. This should reference a building ID in the organization's Resources database.

      --working-location-properties-office-location-desk-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_DESK_ID>
          An optional desk identifier.

      --working-location-properties-office-location-floor-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_FLOOR_ID>
          An optional floor identifier.

      --working-location-properties-office-location-floor-section-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_FLOOR_SECTION_ID>
          An optional floor section identifier.

      --working-location-properties-office-location-label <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_LABEL>
          The office name that's displayed in Calendar Web and Mobile clients. We recommend you reference a building name in the organization's Resources database.

      --working-location-properties-type <WORKING_LOCATION_PROPERTIES_TYPE>
          Type of the working location. Possible values are:  
          - "homeOffice" - The user is working at home. 
          - "officeLocation" - The user is working from an office. 
          - "customLocation" - The user is working from a custom location.  Any details are specified in a sub-field of the specified name, but this field may be missing if empty. Any other fields are ignored.
          Required when adding working location properties.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## events.instances

```text
Returns instances of the specified recurring event.

Usage: joog [OPTIONS] calendar <ACCOUNT> events.instances [COMMAND_OPTIONS]

Mandatory parameters:
      --event-id <EVENT_ID>
          Recurring event identifier.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

      --time-max <TIME_MAX>
          Upper bound (exclusive) for an event's start time to filter by. Optional. The default is not to filter by start time. Must be an RFC3339 timestamp with mandatory time zone offset.

      --time-min <TIME_MIN>
          Lower bound (inclusive) for an event's end time to filter by. Optional. The default is not to filter by end time. Must be an RFC3339 timestamp with mandatory time zone offset.

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

      --always-include-email[=<BOOL>]
          Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).

      --max-attendees <MAX_ATTENDEES>
          The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.

      --max-results <MAX_RESULTS>
          Maximum number of events returned on one result page. By default the value is 250 events. The page size can never be larger than 2500 events. Optional.

      --original-start <ORIGINAL_START>
          The original start time of the instance in the result. Optional.

      --page-token <PAGE_TOKEN>
          Token specifying which result page to return. Optional.

      --show-deleted[=<BOOL>]
          Whether to include deleted events (with status equals "cancelled") in the result. Cancelled instances of recurring events will still be included if singleEvents is False. Optional. The default is False.

      --time-zone <TIME_ZONE>
          Time zone used in the response. Optional. The default is the time zone of the calendar.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## events.list

```text
Returns events on the specified calendar.

Usage: joog [OPTIONS] calendar <ACCOUNT> events.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

      --order-by <ORDER_BY>
          The order of the events returned in the result. Optional. The default is an unspecified, stable order. Choices: startTime, updated.

      --q <Q>
          Free text search terms to find events that match these terms in the following fields:
          
          - summary 
          - description 
          - location 
          - attendee's displayName 
          - attendee's email 
          - organizer's displayName 
          - organizer's email 
          - workingLocationProperties.officeLocation.buildingId 
          - workingLocationProperties.officeLocation.deskId 
          - workingLocationProperties.officeLocation.label 
          - workingLocationProperties.customLocation.label 
          These search terms also match predefined keywords against all display title translations of working location, out-of-office, and focus-time events. For example, searching for "Office" or "Bureau" returns working location events of type officeLocation, whereas searching for "Out of office" or "Abwesend" returns out-of-office events. Optional.

      --time-max <TIME_MAX>
          Upper bound (exclusive) for an event's start time to filter by. Optional. The default is not to filter by start time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMin is set, timeMax must be greater than timeMin.

      --time-min <TIME_MIN>
          Lower bound (exclusive) for an event's end time to filter by. Optional. The default is not to filter by end time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMax is set, timeMin must be smaller than timeMax.

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

      --always-include-email[=<BOOL>]
          Deprecated and ignored.

      --event-types <EVENT_TYPES>...
          Event types to return. Optional. This parameter can be repeated multiple times to return events of different types. If unset, returns all event types. Choices: birthday, default, focusTime, fromGmail, outOfOffice, workingLocation.

      --i-cal-uid <I_CAL_UID>
          Specifies an event ID in the iCalendar format to be provided in the response. Optional. Use this if you want to search for an event by its iCalendar ID.

      --max-attendees <MAX_ATTENDEES>
          The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.

      --max-results <MAX_RESULTS>
          Maximum number of events returned on one result page. The number of events in the resulting page may be less than this value, or none at all, even if there are more events matching the query. Incomplete pages can be detected by a non-empty nextPageToken field in the response. By default the value is 250 events. The page size can never be larger than 2500 events. Optional.

      --page-token <PAGE_TOKEN>
          Token specifying which result page to return. Optional.

      --private-extended-property <PRIVATE_EXTENDED_PROPERTY>...
          Extended properties constraint specified as propertyName=value. Matches only private properties. This parameter might be repeated multiple times to return events that match all given constraints.

      --shared-extended-property <SHARED_EXTENDED_PROPERTY>...
          Extended properties constraint specified as propertyName=value. Matches only shared properties. This parameter might be repeated multiple times to return events that match all given constraints.

      --show-deleted[=<BOOL>]
          Whether to include deleted events (with status equals "cancelled") in the result. Cancelled instances of recurring events (but not the underlying recurring event) will still be included if showDeleted and singleEvents are both False. If showDeleted and singleEvents are both True, only single instances of deleted events (but not the underlying recurring events) are returned. Optional. The default is False.

      --show-hidden-invitations[=<BOOL>]
          Whether to include hidden invitations in the result. Optional. The default is False.

      --single-events[=<BOOL>]
          Whether to expand recurring events into instances and only return single one-off events and instances of recurring events, but not the underlying recurring events themselves. Optional. The default is False.

      --sync-token <SYNC_TOKEN>
          Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All events deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.
          There are several query parameters that cannot be specified together with nextSyncToken to ensure consistency of the client state.
          
          These are: 
          - iCalUID 
          - orderBy 
          - privateExtendedProperty 
          - q 
          - sharedExtendedProperty 
          - timeMin 
          - timeMax 
          - updatedMin All other query parameters should be the same as for the initial synchronization to avoid undefined behavior. If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
          Learn more about incremental synchronization.
          Optional. The default is to return all entries.

      --time-zone <TIME_ZONE>
          Time zone used in the response. Optional. The default is the time zone of the calendar.

      --updated-min <UPDATED_MIN>
          Lower bound for an event's last modification time (as a RFC3339 timestamp) to filter by. When specified, entries deleted since this time will always be included regardless of showDeleted. Optional. The default is not to filter by last modification time.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## events.move

```text
Moves an event to another calendar, i.e. changes an event's organizer. Note that only default events can be moved; birthday, focusTime, fromGmail, outOfOffice and workingLocation events cannot be moved.

Usage: joog [OPTIONS] calendar <ACCOUNT> events.move [COMMAND_OPTIONS]

Mandatory parameters:
      --destination <DESTINATION>
          Calendar identifier of the target calendar where the event is to be moved to.

      --event-id <EVENT_ID>
          Event identifier.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --calendar-id <CALENDAR_ID>
          Calendar identifier of the source calendar where the event currently is on.
          
          [default: primary]

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --send-updates <SEND_UPDATES>
          Guests who should receive notifications about the change of the event's organizer. Choices: all, externalOnly, none.

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

      --send-notifications[=<BOOL>]
          Deprecated. Please use sendUpdates instead.
          
          Whether to send notifications about the change of the event's organizer. Note that some emails might still be sent even if you set the value to false. The default is false.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## events.patch

```text
Updates an event. This method supports patch semantics.

Input choices:
  Use --end-date for an all-day event or --end-time for a timed event; do not supply both.
  Use --original-start-time-date for an all-day event or --original-start-time-date-time for a timed event; do not supply both.
  Use --start-date for an all-day event or --start-time for a timed event; do not supply both.

Usage: joog [OPTIONS] calendar <ACCOUNT> events.patch [COMMAND_OPTIONS]

Mandatory parameters:
      --event-id <EVENT_ID>
          Event identifier.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --attendees <JSON>
          The attendees of the event. See the Events with attendees guide for more information on scheduling events with other calendar users. Service accounts need to use domain-wide delegation of authority to populate the attendee list.

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

      --description <DESCRIPTION>
          Description of the event. Can contain HTML. Optional.

      --end-date <END_DATE>
          The date, in the format "yyyy-mm-dd", if this is an all-day event.

      --end-time <END_TIME>
          The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.

      --end-time-zone <END_TIME_ZONE>
          The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

      --send-updates <SEND_UPDATES>
          Guests who should receive notifications about the event update (for example, title changes, etc.). Choices: all, externalOnly, none.

      --start-date <START_DATE>
          The date, in the format "yyyy-mm-dd", if this is an all-day event.

      --start-time <START_TIME>
          The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.

      --start-time-zone <START_TIME_ZONE>
          The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

      --status <STATUS>
          Status of the event. Optional. Possible values are:  
          - "confirmed" - The event is confirmed. This is the default status. 
          - "tentative" - The event is tentatively confirmed. 
          - "cancelled" - The event is cancelled (deleted). The list method returns cancelled events only on incremental sync (when syncToken or updatedMin are specified) or if the showDeleted flag is set to true. The get method always returns them.
          A cancelled status represents two different states depending on the event type:  
          - Cancelled exceptions of an uncancelled recurring event indicate that this instance should no longer be presented to the user. Clients should store these events for the lifetime of the parent recurring event.
          Cancelled exceptions are only guaranteed to have values for the id, recurringEventId and originalStartTime fields populated. The other fields might be empty.  
          - All other cancelled events represent deleted events. Clients should remove their locally synced copies. Such cancelled events will eventually disappear, so do not rely on them being available indefinitely.
          Deleted events are only guaranteed to have the id field populated.   On the organizer's calendar, cancelled events continue to expose event details (summary, location, etc.) so that they can be restored (undeleted). Similarly, the events to which the user was invited and that they manually removed continue to provide details. However, incremental sync requests with showDeleted set to false will not return these details.
          If an event changes its organizer (for example via the move operation) and the original organizer is not on the attendee list, it will leave behind a cancelled event where only the id field is guaranteed to be populated.

      --summary <SUMMARY>
          Title of the event.

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

      --always-include-email[=<BOOL>]
          Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).

      --anyone-can-add-self[=<BOOL>]
          Whether anyone can invite themselves to the event (deprecated). Optional. The default is False.

      --attachments <JSON>
          File attachments for the event.
          In order to modify attachments the supportsAttachments request parameter should be set to true.
          There can be at most 25 attachments per event,

      --attendees-omitted[=<BOOL>]
          Whether attendees may have been omitted from the event's representation. When retrieving an event, this may be due to a restriction specified by the maxAttendee query parameter. When updating an event, this can be used to only update the participant's response. Optional. The default is False.

      --birthday-properties <JSON>
          Request body field

      --birthday-properties-contact <BIRTHDAY_PROPERTIES_CONTACT>
          Resource name of the contact this birthday event is linked to. This can be used to fetch contact details from People API. Format: "people/c12345". Read-only.

      --birthday-properties-custom-type-name <BIRTHDAY_PROPERTIES_CUSTOM_TYPE_NAME>
          Custom type label specified for this event. This is populated if birthdayProperties.type is set to "custom". Read-only.

      --birthday-properties-type <BIRTHDAY_PROPERTIES_TYPE>
          Type of birthday or special event. Possible values are:  
          - "anniversary" - An anniversary other than birthday. Always has a contact. 
          - "birthday" - A birthday event. This is the default value. 
          - "custom" - A special date whose label is further specified in the customTypeName field. Always has a contact. 
          - "other" - A special date which does not fall into the other categories, and does not have a custom label. Always has a contact. 
          - "self" - Calendar owner's own birthday. Cannot have a contact.  The Calendar API only supports creating events with the type "birthday". The type cannot be changed after the event is created.

      --color-id <COLOR_ID>
          The color of the event. This is an ID referring to an entry in the event section of the colors definition (see the  colors endpoint). Optional.

      --conference-data <JSON>
          Request body field

      --conference-data-conference-id <CONFERENCE_DATA_CONFERENCE_ID>
          The ID of the conference.
          Can be used by developers to keep track of conferences, should not be displayed to users.
          The ID value is formed differently for each conference solution type:  
          - eventHangout: ID is not set. (This conference type is deprecated.)
          - eventNamedHangout: ID is the name of the Hangout. (This conference type is deprecated.)
          - hangoutsMeet: ID is the 10-letter meeting code, for example aaa-bbbb-ccc.
          - addOn: ID is defined by the third-party provider.  Optional.

      --conference-data-conference-solution-icon-uri <CONFERENCE_DATA_CONFERENCE_SOLUTION_ICON_URI>
          The user-visible icon for this solution.

      --conference-data-conference-solution-key-type <CONFERENCE_DATA_CONFERENCE_SOLUTION_KEY_TYPE>
          The conference solution type.
          If a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.
          The possible values are:  
          - "eventHangout" for Hangouts for consumers (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "eventNamedHangout" for classic Hangouts for Google Workspace users (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "hangoutsMeet" for Google Meet (http://meet.google.com)
          - "addOn" for 3P conference providers

      --conference-data-conference-solution-name <CONFERENCE_DATA_CONFERENCE_SOLUTION_NAME>
          The user-visible name of this solution. Not localized.

      --conference-data-create-request-conference-solution-key-type <CONFERENCE_DATA_CREATE_REQUEST_CONFERENCE_SOLUTION_KEY_TYPE>
          The conference solution type.
          If a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.
          The possible values are:  
          - "eventHangout" for Hangouts for consumers (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "eventNamedHangout" for classic Hangouts for Google Workspace users (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "hangoutsMeet" for Google Meet (http://meet.google.com)
          - "addOn" for 3P conference providers

      --conference-data-create-request-request-id <CONFERENCE_DATA_CREATE_REQUEST_REQUEST_ID>
          The client-generated unique ID for this request.
          Clients should regenerate this ID for every new request. If an ID provided is the same as for the previous request, the request is ignored.

      --conference-data-create-request-status-status-code <CONFERENCE_DATA_CREATE_REQUEST_STATUS_STATUS_CODE>
          The current status of the conference create request. Read-only.
          The possible values are:  
          - "pending": the conference create request is still being processed.
          - "success": the conference create request succeeded, the entry points are populated.
          - "failure": the conference create request failed, there are no entry points.

      --conference-data-entry-points <JSON>
          Information about individual conference entry points, such as URLs or phone numbers.
          All of them must belong to the same conference.
          Either conferenceSolution and at least one entryPoint, or createRequest is required.

      --conference-data-notes <CONFERENCE_DATA_NOTES>
          Additional notes (such as instructions from the domain administrator, legal notices) to display to the user. Can contain HTML. The maximum length is 2048 characters. Optional.

      --conference-data-signature <CONFERENCE_DATA_SIGNATURE>
          The signature of the conference data.
          Generated on server side.
          Unset for a conference with a failed create request.
          Optional for a conference with a pending create request.

      --conference-data-version <CONFERENCE_DATA_VERSION>
          Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0.

      --created <CREATED>
          Creation time of the event (as a RFC3339 timestamp). Read-only.

      --creator <JSON>
          The creator of the event. Read-only.

      --creator-display-name <CREATOR_DISPLAY_NAME>
          The creator's name, if available.

      --creator-email <CREATOR_EMAIL>
          The creator's email address, if available.

      --creator-id <CREATOR_ID>
          The creator's Profile ID, if available.

      --creator-self[=<BOOL>]
          Whether the creator corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.

      --end <JSON>
          Request body field

      --end-time-unspecified[=<BOOL>]
          Whether the end time is actually unspecified. An end time is still provided for compatibility reasons, even if this attribute is set to True. The default is False.

      --etag <ETAG>
          ETag of the resource.

      --event-label-id <EVENT_LABEL_ID>
          The ID of the event label assigned to the event. Optional. This refers to the ID of an entry in the labelProperties.eventLabels property of the calendar (see the Calendars.get endpoint.)
          This property supersedes the index-based colorId property. To set or change this property, you need to specify eventLabelVersion=1 in the parameters of the insert, import, update, and patch methods.
          Setting an empty string, or not setting this field at all, will remove the existing label from the event.

      --event-label-version <EVENT_LABEL_VERSION>
          Version number of the event label feature supported by the API client. Version 0 assumes no event label support and processes the colorId field for color management. Version 1 enables support for event labels, and processes the eventLabelId in the event's body. In this case, the colorId field is ignored. The default is 0.

      --event-type <EVENT_TYPE>
          Specific type of the event. This cannot be modified after the event is created. Possible values are:  
          - "birthday" - A special all-day event with an annual recurrence. 
          - "default" - A regular event or not further specified. 
          - "focusTime" - A focus-time event. 
          - "fromGmail" - An event from Gmail. This type of event cannot be created. 
          - "outOfOffice" - An out-of-office event. 
          - "workingLocation" - A working location event.

      --extended-properties <JSON>
          Extended properties of the event.

      --focus-time-properties <JSON>
          Request body field

      --focus-time-properties-auto-decline-mode <FOCUS_TIME_PROPERTIES_AUTO_DECLINE_MODE>
          Whether to decline meeting invitations which overlap Focus Time events. Valid values are declineNone, meaning that no meeting invitations are declined; declineAllConflictingInvitations, meaning that all conflicting meeting invitations that conflict with the event are declined; and declineOnlyNewConflictingInvitations, meaning that only new conflicting meeting invitations which arrive while the Focus Time event is present are to be declined.

      --focus-time-properties-chat-status <FOCUS_TIME_PROPERTIES_CHAT_STATUS>
          The status to mark the user in Chat and related products. This can be available or doNotDisturb.

      --focus-time-properties-decline-message <FOCUS_TIME_PROPERTIES_DECLINE_MESSAGE>
          Response message to set if an existing event or new invitation is automatically declined by Calendar.

      --gadget <JSON>
          A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.

      --gadget-display <GADGET_DISPLAY>
          The gadget's display mode. Deprecated. Possible values are:  
          - "icon" - The gadget displays next to the event's title in the calendar view. 
          - "chip" - The gadget displays when the event is clicked.

      --gadget-height <GADGET_HEIGHT>
          The gadget's height in pixels. The height must be an integer greater than 0. Optional. Deprecated.

      --gadget-icon-link <GADGET_ICON_LINK>
          The gadget's icon URL. The URL scheme must be HTTPS. Deprecated.

      --gadget-link <GADGET_LINK>
          The gadget's URL. The URL scheme must be HTTPS. Deprecated.

      --gadget-title <GADGET_TITLE>
          The gadget's title. Deprecated.

      --gadget-type <GADGET_TYPE>
          The gadget's type. Deprecated.

      --gadget-width <GADGET_WIDTH>
          The gadget's width in pixels. The width must be an integer greater than 0. Optional. Deprecated.

      --guests-can-invite-others[=<BOOL>]
          Whether attendees other than the organizer can invite others to the event. Optional. The default is True.

      --guests-can-modify[=<BOOL>]
          Whether attendees other than the organizer can modify the event. Optional. The default is False.

      --guests-can-see-other-guests[=<BOOL>]
          Whether attendees other than the organizer can see who the event's attendees are. Optional. The default is True.

      --hangout-link <HANGOUT_LINK>
          An absolute link to the Google Hangout associated with this event. Read-only.

      --html-link <HTML_LINK>
          An absolute link to this event in the Google Calendar Web UI. Read-only.

      --i-cal-uid <I_CAL_UID>
          Event unique identifier as defined in RFC5545. It is used to uniquely identify events accross calendaring systems and must be supplied when importing events via the import method.
          Note that the iCalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same iCalUIDs. To retrieve an event using its iCalUID, call the events.list method using the iCalUID parameter. To retrieve an event using its id, call the events.get method.

      --id <ID>
          Opaque identifier of the event. When creating new single or recurring events, you can specify their IDs. Provided IDs must follow these rules:  
          - characters allowed in the ID are those used in base32hex encoding, i.e. lowercase letters a-v and digits 0-9, see section 3.1.2 in RFC2938 
          - the length of the ID must be between 5 and 1024 characters 
          - the ID must be unique per calendar  Due to the globally distributed nature of the system, we cannot guarantee that ID collisions will be detected at event creation time. To minimize the risk of collisions we recommend using an established UUID algorithm such as one described in RFC4122.
          If you do not specify an ID, it will be automatically generated by the server.
          Note that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs.

      --kind <KIND>
          Type of the resource ("calendar#event").

      --location <LOCATION>
          Geographic location of the event as free-form text. Optional.

      --locked[=<BOOL>]
          Whether this is a locked event copy where no changes can be made to the main event fields "summary", "description", "location", "start", "end" or "recurrence". The default is False. Read-Only.

      --max-attendees <MAX_ATTENDEES>
          The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.

      --organizer <JSON>
          The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.

      --organizer-display-name <ORGANIZER_DISPLAY_NAME>
          The organizer's name, if available.

      --organizer-email <ORGANIZER_EMAIL>
          The organizer's email address, if available. It must be a valid email address as per RFC5322.

      --organizer-id <ORGANIZER_ID>
          The organizer's Profile ID, if available.

      --organizer-self[=<BOOL>]
          Whether the organizer corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.

      --original-start-time <JSON>
          Request body field

      --original-start-time-date <ORIGINAL_START_TIME_DATE>
          The date, in the format "yyyy-mm-dd", if this is an all-day event.

      --original-start-time-date-time <ORIGINAL_START_TIME_DATE_TIME>
          The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.

      --original-start-time-time-zone <ORIGINAL_START_TIME_TIME_ZONE>
          The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

      --out-of-office-properties <JSON>
          Request body field

      --out-of-office-properties-auto-decline-mode <OUT_OF_OFFICE_PROPERTIES_AUTO_DECLINE_MODE>
          Whether to decline meeting invitations which overlap Out of office events. Valid values are declineNone, meaning that no meeting invitations are declined; declineAllConflictingInvitations, meaning that all conflicting meeting invitations that conflict with the event are declined; and declineOnlyNewConflictingInvitations, meaning that only new conflicting meeting invitations which arrive while the Out of office event is present are to be declined.

      --out-of-office-properties-decline-message <OUT_OF_OFFICE_PROPERTIES_DECLINE_MESSAGE>
          Response message to set if an existing event or new invitation is automatically declined by Calendar.

      --private-copy[=<BOOL>]
          If set to True, Event propagation is disabled. Note that it is not the same thing as Private event properties. Optional. Immutable. The default is False.

      --recurrence <RECURRENCE>...
          List of RRULE, EXRULE, RDATE and EXDATE lines for a recurring event, as specified in RFC5545. Note that DTSTART and DTEND lines are not allowed in this field; event start and end times are specified in the start and end fields. This field is omitted for single events or instances of recurring events.

      --recurring-event-id <RECURRING_EVENT_ID>
          For an instance of a recurring event, this is the id of the recurring event to which this instance belongs. Immutable.

      --reminders <JSON>
          Information about the event's reminders for the authenticated user. Note that changing reminders does not also change the updated property of the enclosing event.

      --reminders-overrides <JSON>
          If the event doesn't use the default reminders, this lists the reminders specific to the event, or, if not set, indicates that no reminders are set for this event. The maximum number of override reminders is 5.

      --reminders-use-default[=<BOOL>]
          Whether the default reminders of the calendar apply to the event.

      --send-notifications[=<BOOL>]
          Deprecated. Please use sendUpdates instead.
          
          Whether to send notifications about the event update (for example, description changes, etc.). Note that some emails might still be sent even if you set the value to false. The default is false.

      --sequence <SEQUENCE>
          Sequence number as per iCalendar.

      --source <JSON>
          Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.

      --source-title <SOURCE_TITLE>
          Title of the source; for example a title of a web page or an email subject.

      --source-url <SOURCE_URL>
          URL of the source pointing to a resource. The URL scheme must be HTTP or HTTPS.

      --start <JSON>
          Request body field

      --supports-attachments[=<BOOL>]
          Whether API client performing operation supports event attachments. Optional. The default is False.

      --transparency <TRANSPARENCY>
          Whether the event blocks time on the calendar. Optional. Possible values are:  
          - "opaque" - Default value. The event does block time on the calendar. This is equivalent to setting Show me as to Busy in the Calendar UI. 
          - "transparent" - The event does not block time on the calendar. This is equivalent to setting Show me as to Available in the Calendar UI.

      --updated <UPDATED>
          Last modification time of the main event data (as a RFC3339 timestamp). Updating event reminders will not cause this to change. Read-only.

      --visibility <VISIBILITY>
          Visibility of the event. Optional. Possible values are:  
          - "default" - Uses the default visibility for events on the calendar. This is the default value. 
          - "public" - The event is public and event details are visible to all readers of the calendar. 
          - "private" - The event is private and only event attendees may view event details. 
          - "confidential" - The event is private. This value is provided for compatibility reasons.  
          Note on recurring events: Changing the visibility of a single instance of a recurring event can affect all instances of the series. If the new setting is more restrictive (e.g. from public to private), it is applied to all instances. If the new setting is less restrictive (e.g. from private to public), the change is ignored. To make a recurring event less restrictive, you must update the parent recurring event.

      --working-location-properties <JSON>
          Request body field

      --working-location-properties-custom-location-label <WORKING_LOCATION_PROPERTIES_CUSTOM_LOCATION_LABEL>
          An optional extra label for additional information.

      --working-location-properties-home-office <JSON>
          If present, specifies that the user is working at home.

      --working-location-properties-office-location-building-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_BUILDING_ID>
          An optional building identifier. This should reference a building ID in the organization's Resources database.

      --working-location-properties-office-location-desk-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_DESK_ID>
          An optional desk identifier.

      --working-location-properties-office-location-floor-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_FLOOR_ID>
          An optional floor identifier.

      --working-location-properties-office-location-floor-section-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_FLOOR_SECTION_ID>
          An optional floor section identifier.

      --working-location-properties-office-location-label <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_LABEL>
          The office name that's displayed in Calendar Web and Mobile clients. We recommend you reference a building name in the organization's Resources database.

      --working-location-properties-type <WORKING_LOCATION_PROPERTIES_TYPE>
          Type of the working location. Possible values are:  
          - "homeOffice" - The user is working at home. 
          - "officeLocation" - The user is working from an office. 
          - "customLocation" - The user is working from a custom location.  Any details are specified in a sub-field of the specified name, but this field may be missing if empty. Any other fields are ignored.
          Required when adding working location properties.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## events.quick-add

```text
Creates an event based on a simple text string.

Usage: joog [OPTIONS] calendar <ACCOUNT> events.quick-add [COMMAND_OPTIONS]

Mandatory parameters:
      --text <TEXT>
          The text describing the event to be created.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --send-updates <SEND_UPDATES>
          Guests who should receive notifications about the creation of the new event. Choices: all, externalOnly, none.

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

      --send-notifications[=<BOOL>]
          Deprecated. Please use sendUpdates instead.
          
          Whether to send notifications about the creation of the event. Note that some emails might still be sent even if you set the value to false. The default is false.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## events.update

```text
Updates an event.

Input choices:
  End is required: use --end-date for an all-day event or --end-time for a timed event.
  When setting originalStartTime, use --original-start-time-date for an all-day event or --original-start-time-date-time for a timed event.
  Start is required: use --start-date for an all-day event or --start-time for a timed event.

Usage: joog [OPTIONS] calendar <ACCOUNT> events.update [COMMAND_OPTIONS]

Mandatory parameters:
      --event-id <EVENT_ID>
          Event identifier.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --attendees <JSON>
          The attendees of the event. See the Events with attendees guide for more information on scheduling events with other calendar users. Service accounts need to use domain-wide delegation of authority to populate the attendee list.

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

      --description <DESCRIPTION>
          Description of the event. Can contain HTML. Optional.

      --end-date <END_DATE>
          The date, in the format "yyyy-mm-dd", if this is an all-day event.

      --end-time <END_TIME>
          The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.

      --end-time-zone <END_TIME_ZONE>
          The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

      --send-updates <SEND_UPDATES>
          Guests who should receive notifications about the event update (for example, title changes, etc.). Choices: all, externalOnly, none.

      --start-date <START_DATE>
          The date, in the format "yyyy-mm-dd", if this is an all-day event.

      --start-time <START_TIME>
          The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.

      --start-time-zone <START_TIME_ZONE>
          The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

      --status <STATUS>
          Status of the event. Optional. Possible values are:  
          - "confirmed" - The event is confirmed. This is the default status. 
          - "tentative" - The event is tentatively confirmed. 
          - "cancelled" - The event is cancelled (deleted). The list method returns cancelled events only on incremental sync (when syncToken or updatedMin are specified) or if the showDeleted flag is set to true. The get method always returns them.
          A cancelled status represents two different states depending on the event type:  
          - Cancelled exceptions of an uncancelled recurring event indicate that this instance should no longer be presented to the user. Clients should store these events for the lifetime of the parent recurring event.
          Cancelled exceptions are only guaranteed to have values for the id, recurringEventId and originalStartTime fields populated. The other fields might be empty.  
          - All other cancelled events represent deleted events. Clients should remove their locally synced copies. Such cancelled events will eventually disappear, so do not rely on them being available indefinitely.
          Deleted events are only guaranteed to have the id field populated.   On the organizer's calendar, cancelled events continue to expose event details (summary, location, etc.) so that they can be restored (undeleted). Similarly, the events to which the user was invited and that they manually removed continue to provide details. However, incremental sync requests with showDeleted set to false will not return these details.
          If an event changes its organizer (for example via the move operation) and the original organizer is not on the attendee list, it will leave behind a cancelled event where only the id field is guaranteed to be populated.

      --summary <SUMMARY>
          Title of the event.

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

      --always-include-email[=<BOOL>]
          Deprecated and ignored. A value will always be returned in the email field for the organizer, creator and attendees, even if no real email address is available (i.e. a generated, non-working value will be provided).

      --anyone-can-add-self[=<BOOL>]
          Whether anyone can invite themselves to the event (deprecated). Optional. The default is False.

      --attachments <JSON>
          File attachments for the event.
          In order to modify attachments the supportsAttachments request parameter should be set to true.
          There can be at most 25 attachments per event,

      --attendees-omitted[=<BOOL>]
          Whether attendees may have been omitted from the event's representation. When retrieving an event, this may be due to a restriction specified by the maxAttendee query parameter. When updating an event, this can be used to only update the participant's response. Optional. The default is False.

      --birthday-properties <JSON>
          Request body field

      --birthday-properties-contact <BIRTHDAY_PROPERTIES_CONTACT>
          Resource name of the contact this birthday event is linked to. This can be used to fetch contact details from People API. Format: "people/c12345". Read-only.

      --birthday-properties-custom-type-name <BIRTHDAY_PROPERTIES_CUSTOM_TYPE_NAME>
          Custom type label specified for this event. This is populated if birthdayProperties.type is set to "custom". Read-only.

      --birthday-properties-type <BIRTHDAY_PROPERTIES_TYPE>
          Type of birthday or special event. Possible values are:  
          - "anniversary" - An anniversary other than birthday. Always has a contact. 
          - "birthday" - A birthday event. This is the default value. 
          - "custom" - A special date whose label is further specified in the customTypeName field. Always has a contact. 
          - "other" - A special date which does not fall into the other categories, and does not have a custom label. Always has a contact. 
          - "self" - Calendar owner's own birthday. Cannot have a contact.  The Calendar API only supports creating events with the type "birthday". The type cannot be changed after the event is created.

      --color-id <COLOR_ID>
          The color of the event. This is an ID referring to an entry in the event section of the colors definition (see the  colors endpoint). Optional.

      --conference-data <JSON>
          Request body field

      --conference-data-conference-id <CONFERENCE_DATA_CONFERENCE_ID>
          The ID of the conference.
          Can be used by developers to keep track of conferences, should not be displayed to users.
          The ID value is formed differently for each conference solution type:  
          - eventHangout: ID is not set. (This conference type is deprecated.)
          - eventNamedHangout: ID is the name of the Hangout. (This conference type is deprecated.)
          - hangoutsMeet: ID is the 10-letter meeting code, for example aaa-bbbb-ccc.
          - addOn: ID is defined by the third-party provider.  Optional.

      --conference-data-conference-solution-icon-uri <CONFERENCE_DATA_CONFERENCE_SOLUTION_ICON_URI>
          The user-visible icon for this solution.

      --conference-data-conference-solution-key-type <CONFERENCE_DATA_CONFERENCE_SOLUTION_KEY_TYPE>
          The conference solution type.
          If a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.
          The possible values are:  
          - "eventHangout" for Hangouts for consumers (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "eventNamedHangout" for classic Hangouts for Google Workspace users (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "hangoutsMeet" for Google Meet (http://meet.google.com)
          - "addOn" for 3P conference providers

      --conference-data-conference-solution-name <CONFERENCE_DATA_CONFERENCE_SOLUTION_NAME>
          The user-visible name of this solution. Not localized.

      --conference-data-create-request-conference-solution-key-type <CONFERENCE_DATA_CREATE_REQUEST_CONFERENCE_SOLUTION_KEY_TYPE>
          The conference solution type.
          If a client encounters an unfamiliar or empty type, it should still be able to display the entry points. However, it should disallow modifications.
          The possible values are:  
          - "eventHangout" for Hangouts for consumers (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "eventNamedHangout" for classic Hangouts for Google Workspace users (deprecated; existing events may show this conference solution type but new conferences cannot be created)
          - "hangoutsMeet" for Google Meet (http://meet.google.com)
          - "addOn" for 3P conference providers

      --conference-data-create-request-request-id <CONFERENCE_DATA_CREATE_REQUEST_REQUEST_ID>
          The client-generated unique ID for this request.
          Clients should regenerate this ID for every new request. If an ID provided is the same as for the previous request, the request is ignored.

      --conference-data-create-request-status-status-code <CONFERENCE_DATA_CREATE_REQUEST_STATUS_STATUS_CODE>
          The current status of the conference create request. Read-only.
          The possible values are:  
          - "pending": the conference create request is still being processed.
          - "success": the conference create request succeeded, the entry points are populated.
          - "failure": the conference create request failed, there are no entry points.

      --conference-data-entry-points <JSON>
          Information about individual conference entry points, such as URLs or phone numbers.
          All of them must belong to the same conference.
          Either conferenceSolution and at least one entryPoint, or createRequest is required.

      --conference-data-notes <CONFERENCE_DATA_NOTES>
          Additional notes (such as instructions from the domain administrator, legal notices) to display to the user. Can contain HTML. The maximum length is 2048 characters. Optional.

      --conference-data-signature <CONFERENCE_DATA_SIGNATURE>
          The signature of the conference data.
          Generated on server side.
          Unset for a conference with a failed create request.
          Optional for a conference with a pending create request.

      --conference-data-version <CONFERENCE_DATA_VERSION>
          Version number of conference data supported by the API client. Version 0 assumes no conference data support and ignores conference data in the event's body. Version 1 enables support for copying of ConferenceData as well as for creating new conferences using the createRequest field of conferenceData. The default is 0.

      --created <CREATED>
          Creation time of the event (as a RFC3339 timestamp). Read-only.

      --creator <JSON>
          The creator of the event. Read-only.

      --creator-display-name <CREATOR_DISPLAY_NAME>
          The creator's name, if available.

      --creator-email <CREATOR_EMAIL>
          The creator's email address, if available.

      --creator-id <CREATOR_ID>
          The creator's Profile ID, if available.

      --creator-self[=<BOOL>]
          Whether the creator corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.

      --end <JSON>
          Request body field

      --end-time-unspecified[=<BOOL>]
          Whether the end time is actually unspecified. An end time is still provided for compatibility reasons, even if this attribute is set to True. The default is False.

      --etag <ETAG>
          ETag of the resource.

      --event-label-id <EVENT_LABEL_ID>
          The ID of the event label assigned to the event. Optional. This refers to the ID of an entry in the labelProperties.eventLabels property of the calendar (see the Calendars.get endpoint.)
          This property supersedes the index-based colorId property. To set or change this property, you need to specify eventLabelVersion=1 in the parameters of the insert, import, update, and patch methods.
          Setting an empty string, or not setting this field at all, will remove the existing label from the event.

      --event-label-version <EVENT_LABEL_VERSION>
          Version number of the event label feature supported by the API client. Version 0 assumes no event label support and processes the colorId field for color management. Version 1 enables support for event labels, and processes the eventLabelId in the event's body. In this case, the colorId field is ignored. The default is 0.

      --event-type <EVENT_TYPE>
          Specific type of the event. This cannot be modified after the event is created. Possible values are:  
          - "birthday" - A special all-day event with an annual recurrence. 
          - "default" - A regular event or not further specified. 
          - "focusTime" - A focus-time event. 
          - "fromGmail" - An event from Gmail. This type of event cannot be created. 
          - "outOfOffice" - An out-of-office event. 
          - "workingLocation" - A working location event.

      --extended-properties <JSON>
          Extended properties of the event.

      --focus-time-properties <JSON>
          Request body field

      --focus-time-properties-auto-decline-mode <FOCUS_TIME_PROPERTIES_AUTO_DECLINE_MODE>
          Whether to decline meeting invitations which overlap Focus Time events. Valid values are declineNone, meaning that no meeting invitations are declined; declineAllConflictingInvitations, meaning that all conflicting meeting invitations that conflict with the event are declined; and declineOnlyNewConflictingInvitations, meaning that only new conflicting meeting invitations which arrive while the Focus Time event is present are to be declined.

      --focus-time-properties-chat-status <FOCUS_TIME_PROPERTIES_CHAT_STATUS>
          The status to mark the user in Chat and related products. This can be available or doNotDisturb.

      --focus-time-properties-decline-message <FOCUS_TIME_PROPERTIES_DECLINE_MESSAGE>
          Response message to set if an existing event or new invitation is automatically declined by Calendar.

      --gadget <JSON>
          A gadget that extends this event. Gadgets are deprecated; this structure is instead only used for returning birthday calendar metadata.

      --gadget-display <GADGET_DISPLAY>
          The gadget's display mode. Deprecated. Possible values are:  
          - "icon" - The gadget displays next to the event's title in the calendar view. 
          - "chip" - The gadget displays when the event is clicked.

      --gadget-height <GADGET_HEIGHT>
          The gadget's height in pixels. The height must be an integer greater than 0. Optional. Deprecated.

      --gadget-icon-link <GADGET_ICON_LINK>
          The gadget's icon URL. The URL scheme must be HTTPS. Deprecated.

      --gadget-link <GADGET_LINK>
          The gadget's URL. The URL scheme must be HTTPS. Deprecated.

      --gadget-title <GADGET_TITLE>
          The gadget's title. Deprecated.

      --gadget-type <GADGET_TYPE>
          The gadget's type. Deprecated.

      --gadget-width <GADGET_WIDTH>
          The gadget's width in pixels. The width must be an integer greater than 0. Optional. Deprecated.

      --guests-can-invite-others[=<BOOL>]
          Whether attendees other than the organizer can invite others to the event. Optional. The default is True.

      --guests-can-modify[=<BOOL>]
          Whether attendees other than the organizer can modify the event. Optional. The default is False.

      --guests-can-see-other-guests[=<BOOL>]
          Whether attendees other than the organizer can see who the event's attendees are. Optional. The default is True.

      --hangout-link <HANGOUT_LINK>
          An absolute link to the Google Hangout associated with this event. Read-only.

      --html-link <HTML_LINK>
          An absolute link to this event in the Google Calendar Web UI. Read-only.

      --i-cal-uid <I_CAL_UID>
          Event unique identifier as defined in RFC5545. It is used to uniquely identify events accross calendaring systems and must be supplied when importing events via the import method.
          Note that the iCalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same iCalUIDs. To retrieve an event using its iCalUID, call the events.list method using the iCalUID parameter. To retrieve an event using its id, call the events.get method.

      --id <ID>
          Opaque identifier of the event. When creating new single or recurring events, you can specify their IDs. Provided IDs must follow these rules:  
          - characters allowed in the ID are those used in base32hex encoding, i.e. lowercase letters a-v and digits 0-9, see section 3.1.2 in RFC2938 
          - the length of the ID must be between 5 and 1024 characters 
          - the ID must be unique per calendar  Due to the globally distributed nature of the system, we cannot guarantee that ID collisions will be detected at event creation time. To minimize the risk of collisions we recommend using an established UUID algorithm such as one described in RFC4122.
          If you do not specify an ID, it will be automatically generated by the server.
          Note that the icalUID and the id are not identical and only one of them should be supplied at event creation time. One difference in their semantics is that in recurring events, all occurrences of one event have different ids while they all share the same icalUIDs.

      --kind <KIND>
          Type of the resource ("calendar#event").

      --location <LOCATION>
          Geographic location of the event as free-form text. Optional.

      --locked[=<BOOL>]
          Whether this is a locked event copy where no changes can be made to the main event fields "summary", "description", "location", "start", "end" or "recurrence". The default is False. Read-Only.

      --max-attendees <MAX_ATTENDEES>
          The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.

      --organizer <JSON>
          The organizer of the event. If the organizer is also an attendee, this is indicated with a separate entry in attendees with the organizer field set to True. To change the organizer, use the move operation. Read-only, except when importing an event.

      --organizer-display-name <ORGANIZER_DISPLAY_NAME>
          The organizer's name, if available.

      --organizer-email <ORGANIZER_EMAIL>
          The organizer's email address, if available. It must be a valid email address as per RFC5322.

      --organizer-id <ORGANIZER_ID>
          The organizer's Profile ID, if available.

      --organizer-self[=<BOOL>]
          Whether the organizer corresponds to the calendar on which this copy of the event appears. Read-only. The default is False.

      --original-start-time <JSON>
          Request body field

      --original-start-time-date <ORIGINAL_START_TIME_DATE>
          The date, in the format "yyyy-mm-dd", if this is an all-day event.

      --original-start-time-date-time <ORIGINAL_START_TIME_DATE_TIME>
          The time, as a combined date-time value (formatted according to RFC3339). A time zone offset is required unless a time zone is explicitly specified in timeZone.

      --original-start-time-time-zone <ORIGINAL_START_TIME_TIME_ZONE>
          The time zone in which the time is specified. (Formatted as an IANA Time Zone Database name, e.g. "Europe/Zurich".) For recurring events this field is required and specifies the time zone in which the recurrence is expanded. For single events this field is optional and indicates a custom time zone for the event start/end.

      --out-of-office-properties <JSON>
          Request body field

      --out-of-office-properties-auto-decline-mode <OUT_OF_OFFICE_PROPERTIES_AUTO_DECLINE_MODE>
          Whether to decline meeting invitations which overlap Out of office events. Valid values are declineNone, meaning that no meeting invitations are declined; declineAllConflictingInvitations, meaning that all conflicting meeting invitations that conflict with the event are declined; and declineOnlyNewConflictingInvitations, meaning that only new conflicting meeting invitations which arrive while the Out of office event is present are to be declined.

      --out-of-office-properties-decline-message <OUT_OF_OFFICE_PROPERTIES_DECLINE_MESSAGE>
          Response message to set if an existing event or new invitation is automatically declined by Calendar.

      --private-copy[=<BOOL>]
          If set to True, Event propagation is disabled. Note that it is not the same thing as Private event properties. Optional. Immutable. The default is False.

      --recurrence <RECURRENCE>...
          List of RRULE, EXRULE, RDATE and EXDATE lines for a recurring event, as specified in RFC5545. Note that DTSTART and DTEND lines are not allowed in this field; event start and end times are specified in the start and end fields. This field is omitted for single events or instances of recurring events.

      --recurring-event-id <RECURRING_EVENT_ID>
          For an instance of a recurring event, this is the id of the recurring event to which this instance belongs. Immutable.

      --reminders <JSON>
          Information about the event's reminders for the authenticated user. Note that changing reminders does not also change the updated property of the enclosing event.

      --reminders-overrides <JSON>
          If the event doesn't use the default reminders, this lists the reminders specific to the event, or, if not set, indicates that no reminders are set for this event. The maximum number of override reminders is 5.

      --reminders-use-default[=<BOOL>]
          Whether the default reminders of the calendar apply to the event.

      --send-notifications[=<BOOL>]
          Deprecated. Please use sendUpdates instead.
          
          Whether to send notifications about the event update (for example, description changes, etc.). Note that some emails might still be sent even if you set the value to false. The default is false.

      --sequence <SEQUENCE>
          Sequence number as per iCalendar.

      --source <JSON>
          Source from which the event was created. For example, a web page, an email message or any document identifiable by an URL with HTTP or HTTPS scheme. Can only be seen or modified by the creator of the event.

      --source-title <SOURCE_TITLE>
          Title of the source; for example a title of a web page or an email subject.

      --source-url <SOURCE_URL>
          URL of the source pointing to a resource. The URL scheme must be HTTP or HTTPS.

      --start <JSON>
          Request body field

      --supports-attachments[=<BOOL>]
          Whether API client performing operation supports event attachments. Optional. The default is False.

      --transparency <TRANSPARENCY>
          Whether the event blocks time on the calendar. Optional. Possible values are:  
          - "opaque" - Default value. The event does block time on the calendar. This is equivalent to setting Show me as to Busy in the Calendar UI. 
          - "transparent" - The event does not block time on the calendar. This is equivalent to setting Show me as to Available in the Calendar UI.

      --updated <UPDATED>
          Last modification time of the main event data (as a RFC3339 timestamp). Updating event reminders will not cause this to change. Read-only.

      --visibility <VISIBILITY>
          Visibility of the event. Optional. Possible values are:  
          - "default" - Uses the default visibility for events on the calendar. This is the default value. 
          - "public" - The event is public and event details are visible to all readers of the calendar. 
          - "private" - The event is private and only event attendees may view event details. 
          - "confidential" - The event is private. This value is provided for compatibility reasons.  
          Note on recurring events: Changing the visibility of a single instance of a recurring event can affect all instances of the series. If the new setting is more restrictive (e.g. from public to private), it is applied to all instances. If the new setting is less restrictive (e.g. from private to public), the change is ignored. To make a recurring event less restrictive, you must update the parent recurring event.

      --working-location-properties <JSON>
          Request body field

      --working-location-properties-custom-location-label <WORKING_LOCATION_PROPERTIES_CUSTOM_LOCATION_LABEL>
          An optional extra label for additional information.

      --working-location-properties-home-office <JSON>
          If present, specifies that the user is working at home.

      --working-location-properties-office-location-building-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_BUILDING_ID>
          An optional building identifier. This should reference a building ID in the organization's Resources database.

      --working-location-properties-office-location-desk-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_DESK_ID>
          An optional desk identifier.

      --working-location-properties-office-location-floor-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_FLOOR_ID>
          An optional floor identifier.

      --working-location-properties-office-location-floor-section-id <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_FLOOR_SECTION_ID>
          An optional floor section identifier.

      --working-location-properties-office-location-label <WORKING_LOCATION_PROPERTIES_OFFICE_LOCATION_LABEL>
          The office name that's displayed in Calendar Web and Mobile clients. We recommend you reference a building name in the organization's Resources database.

      --working-location-properties-type <WORKING_LOCATION_PROPERTIES_TYPE>
          Type of the working location. Possible values are:  
          - "homeOffice" - The user is working at home. 
          - "officeLocation" - The user is working from an office. 
          - "customLocation" - The user is working from a custom location.  Any details are specified in a sub-field of the specified name, but this field may be missing if empty. Any other fields are ignored.
          Required when adding working location properties.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## events.watch

```text
Watch for changes to Events resources.

Usage: joog [OPTIONS] calendar <ACCOUNT> events.watch [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --calendar-id <CALENDAR_ID>
          Calendar identifier. To retrieve calendar IDs call the calendarList.list method. If you want to access the primary calendar of the currently logged in user, use the "primary" keyword.
          
          [default: primary]

      --order-by <ORDER_BY>
          The order of the events returned in the result. Optional. The default is an unspecified, stable order. Choices: startTime, updated.

      --q <Q>
          Free text search terms to find events that match these terms in the following fields:
          
          - summary 
          - description 
          - location 
          - attendee's displayName 
          - attendee's email 
          - organizer's displayName 
          - organizer's email 
          - workingLocationProperties.officeLocation.buildingId 
          - workingLocationProperties.officeLocation.deskId 
          - workingLocationProperties.officeLocation.label 
          - workingLocationProperties.customLocation.label 
          These search terms also match predefined keywords against all display title translations of working location, out-of-office, and focus-time events. For example, searching for "Office" or "Bureau" returns working location events of type officeLocation, whereas searching for "Out of office" or "Abwesend" returns out-of-office events. Optional.

      --time-max <TIME_MAX>
          Upper bound (exclusive) for an event's start time to filter by. Optional. The default is not to filter by start time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMin is set, timeMax must be greater than timeMin.

      --time-min <TIME_MIN>
          Lower bound (exclusive) for an event's end time to filter by. Optional. The default is not to filter by end time. Must be an RFC3339 timestamp with mandatory time zone offset, for example, 2011-06-03T10:00:00-07:00, 2011-06-03T10:00:00Z. Milliseconds may be provided but are ignored. If timeMax is set, timeMin must be smaller than timeMax.

      --type <TYPE>
          The type of delivery mechanism used for this channel. Valid values are "web_hook" (or "webhook"). Both values refer to a channel where Http requests are used to deliver messages.

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

      --always-include-email[=<BOOL>]
          Deprecated and ignored.

      --body-params <JSON>
          Additional parameters controlling delivery channel behavior. Optional.

      --event-types <EVENT_TYPES>...
          Event types to return. Optional. This parameter can be repeated multiple times to return events of different types. If unset, returns all event types. Choices: birthday, default, focusTime, fromGmail, outOfOffice, workingLocation.

      --expiration <EXPIRATION>
          Date and time of notification channel expiration, expressed as a Unix timestamp, in milliseconds. Optional.

      --i-cal-uid <I_CAL_UID>
          Specifies an event ID in the iCalendar format to be provided in the response. Optional. Use this if you want to search for an event by its iCalendar ID.

      --id <ID>
          A UUID or similar unique string that identifies this channel.

      --kind <KIND>
          Identifies this as a notification channel used to watch for changes to a resource, which is "api#channel".

      --max-attendees <MAX_ATTENDEES>
          The maximum number of attendees to include in the response. If there are more than the specified number of attendees, only the participant is returned. Optional.

      --max-results <MAX_RESULTS>
          Maximum number of events returned on one result page. The number of events in the resulting page may be less than this value, or none at all, even if there are more events matching the query. Incomplete pages can be detected by a non-empty nextPageToken field in the response. By default the value is 250 events. The page size can never be larger than 2500 events. Optional.

      --page-token <PAGE_TOKEN>
          Token specifying which result page to return. Optional.

      --payload[=<BOOL>]
          A Boolean value to indicate whether payload is wanted. Optional.

      --private-extended-property <PRIVATE_EXTENDED_PROPERTY>...
          Extended properties constraint specified as propertyName=value. Matches only private properties. This parameter might be repeated multiple times to return events that match all given constraints.

      --resource-id <RESOURCE_ID>
          An opaque ID that identifies the resource being watched on this channel. Stable across different API versions.

      --resource-uri <RESOURCE_URI>
          A version-specific identifier for the watched resource.

      --shared-extended-property <SHARED_EXTENDED_PROPERTY>...
          Extended properties constraint specified as propertyName=value. Matches only shared properties. This parameter might be repeated multiple times to return events that match all given constraints.

      --show-deleted[=<BOOL>]
          Whether to include deleted events (with status equals "cancelled") in the result. Cancelled instances of recurring events (but not the underlying recurring event) will still be included if showDeleted and singleEvents are both False. If showDeleted and singleEvents are both True, only single instances of deleted events (but not the underlying recurring events) are returned. Optional. The default is False.

      --show-hidden-invitations[=<BOOL>]
          Whether to include hidden invitations in the result. Optional. The default is False.

      --single-events[=<BOOL>]
          Whether to expand recurring events into instances and only return single one-off events and instances of recurring events, but not the underlying recurring events themselves. Optional. The default is False.

      --sync-token <SYNC_TOKEN>
          Token obtained from the nextSyncToken field returned on the last page of results from the previous list request. It makes the result of this list request contain only entries that have changed since then. All events deleted since the previous list request will always be in the result set and it is not allowed to set showDeleted to False.
          There are several query parameters that cannot be specified together with nextSyncToken to ensure consistency of the client state.
          
          These are: 
          - iCalUID 
          - orderBy 
          - privateExtendedProperty 
          - q 
          - sharedExtendedProperty 
          - timeMin 
          - timeMax 
          - updatedMin All other query parameters should be the same as for the initial synchronization to avoid undefined behavior. If the syncToken expires, the server will respond with a 410 GONE response code and the client should clear its storage and perform a full synchronization without any syncToken.
          Learn more about incremental synchronization.
          Optional. The default is to return all entries.

      --time-zone <TIME_ZONE>
          Time zone used in the response. Optional. The default is the time zone of the calendar.

      --token <TOKEN>
          An arbitrary string delivered to the target address with each notification delivered over this channel. Optional.

      --updated-min <UPDATED_MIN>
          Lower bound for an event's last modification time (as a RFC3339 timestamp) to filter by. When specified, entries deleted since this time will always be included regardless of showDeleted. Optional. The default is not to filter by last modification time.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## freebusy.query

```text
Returns free/busy information for a set of calendars.

Usage: joog [OPTIONS] calendar <ACCOUNT> freebusy.query [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --items <JSON>
          List of calendars and/or groups to query.

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

      --body <JSON>
          Inline JSON request body

      --calendar-expansion-max <CALENDAR_EXPANSION_MAX>
          Maximal number of calendars for which FreeBusy information is to be provided. Optional. Maximum value is 50.

      --group-expansion-max <GROUP_EXPANSION_MAX>
          Maximal number of calendar identifiers to be provided for a single group. Optional. An error is returned for a group with more members than this value. Maximum value is 100.

      --time-max <TIME_MAX>
          The end of the interval for the query formatted as per RFC3339.

      --time-min <TIME_MIN>
          The start of the interval for the query formatted as per RFC3339.

      --time-zone <TIME_ZONE>
          Time zone used in the response. Optional. The default is UTC.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).
