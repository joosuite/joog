# Google Tasks methods

See the [command guide](../grammar.md) and [request-file format](../file-schemas.md).

| Method | Description |
| --- | --- |
| `tasklists.delete` | Deletes the authenticated user's specified task list |
| `tasklists.get` | Returns the authenticated user's specified task list. |
| `tasklists.insert` | Creates a new task list and adds it to the authenticated user's task lists |
| `tasklists.list` | Returns all the authenticated user's task lists |
| `tasklists.patch` | Updates the authenticated user's specified task list |
| `tasklists.update` | Updates the authenticated user's specified task list. |
| `tasks.clear` | Clears all completed tasks from the specified task list |
| `tasks.delete` | Deletes the specified task from the task list |
| `tasks.get` | Returns the specified task. |
| `tasks.insert` | Creates a new task on the specified task list |
| `tasks.list` | Returns all tasks in the specified task list |
| `tasks.move` | Moves the specified task to another position in the destination task list |
| `tasks.patch` | Updates the specified task |
| `tasks.update` | Updates the specified task. |

## tasklists.delete

```text
Deletes the authenticated user's specified task list. If the list contains assigned tasks, both the assigned tasks and the original tasks in the assignment surface (Docs, Chat Spaces) are deleted. The user's default task list cannot be deleted.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasklists.delete [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --tasklist <TASKLIST>
          Task list identifier. The default task list cannot be deleted.
          
          [default: @default]

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

## tasklists.get

```text
Returns the authenticated user's specified task list.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasklists.get [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --tasklist <TASKLIST>
          Task list identifier. Use "@default" to get the authenticated user's default task list.
          
          [default: @default]

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

## tasklists.insert

```text
Creates a new task list and adds it to the authenticated user's task lists. A user can have up to 2000 lists at a time.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasklists.insert [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --title <TITLE>
          Title of the task list. Maximum length allowed: 1024 characters.

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
          ETag of the resource.

      --id <ID>
          Task list identifier. In requests, the authenticated user's default task list can also be referred to as "@default".

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## tasklists.list

```text
Returns all the authenticated user's task lists. A user can have up to 2000 lists at a time.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasklists.list [COMMAND_OPTIONS]

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
          Optional. Maximum number of task lists returned on one page. The default is 1000 (max allowed: 1000).

      --page-token <PAGE_TOKEN>
          Optional. Token specifying the result page to return.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## tasklists.patch

```text
Updates the authenticated user's specified task list. This method supports patch semantics.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasklists.patch [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --tasklist <TASKLIST>
          Task list identifier.
          
          [default: @default]

      --title <TITLE>
          Title of the task list. Maximum length allowed: 1024 characters.

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
          ETag of the resource.

      --id <ID>
          Task list identifier. In requests, the authenticated user's default task list can also be referred to as "@default".

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## tasklists.update

```text
Updates the authenticated user's specified task list.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasklists.update [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --tasklist <TASKLIST>
          Task list identifier.
          
          [default: @default]

      --title <TITLE>
          Title of the task list. Maximum length allowed: 1024 characters.

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
          ETag of the resource.

      --id <ID>
          Task list identifier. In requests, the authenticated user's default task list can also be referred to as "@default".

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## tasks.clear

```text
Clears all completed tasks from the specified task list. The affected tasks will be marked as 'hidden' and no longer be returned by default when retrieving all tasks for a task list.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasks.clear [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --tasklist <TASKLIST>
          Task list identifier.
          
          [default: @default]

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

## tasks.delete

```text
Deletes the specified task from the task list. If the task is assigned, both the assigned task and the original task (in Docs, Chat Spaces) are deleted. To delete the assigned task only, navigate to the assignment surface and unassign the task from there.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasks.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --task <TASK>
          Task identifier.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --tasklist <TASKLIST>
          Task list identifier.
          
          [default: @default]

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

## tasks.get

```text
Returns the specified task.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasks.get [COMMAND_OPTIONS]

Mandatory parameters:
      --task <TASK>
          Task identifier.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --tasklist <TASKLIST>
          Task list identifier.
          
          [default: @default]

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

## tasks.insert

```text
Creates a new task on the specified task list. Tasks assigned from Docs or Chat Spaces cannot be inserted from Tasks Public API; they can only be created by assigning them from Docs or Chat Spaces. A user can have up to 20,000 non-hidden tasks per list and up to 100,000 tasks in total at a time.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasks.insert [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --due <DUE>
          Scheduled date for the task (as an RFC 3339 timestamp). Optional. This represents the day that the task should be done, or that the task is visible on the calendar grid. It doesn't represent the deadline of the task. Only date information is recorded; the time portion of the timestamp is discarded when setting this field. It isn't possible to read or write the time that a task is scheduled for using the API.

      --notes <NOTES>
          Notes describing the task. Tasks assigned from Google Docs cannot have notes. Optional. Maximum length allowed: 8192 characters.

      --status <STATUS>
          Status of the task. This is either "needsAction" or "completed".

      --tasklist <TASKLIST>
          Task list identifier.
          
          [default: @default]

      --title <TITLE>
          Title of the task. Maximum length allowed: 1024 characters.

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

      --assignment-info <JSON>
          Information about the source of the task assignment (Document, Chat Space).

      --completed <COMPLETED>
          Completion date of the task (as a RFC 3339 timestamp). This field is omitted if the task has not been completed.

      --deleted[=<BOOL>]
          Flag indicating whether the task has been deleted. For assigned tasks this field is read-only. They can only be deleted by calling tasks.delete, in which case both the assigned task and the original task (in Docs or Chat Spaces) are deleted. To delete the assigned task only, navigate to the assignment surface and unassign the task from there. The default is False.

      --etag <ETAG>
          ETag of the resource.

      --hidden[=<BOOL>]
          Flag indicating whether the task is hidden. This is the case if the task had been marked completed when the task list was last cleared. The default is False. This field is read-only.

      --id <ID>
          Task identifier.

      --parent <PARENT>
          Optional. Parent task identifier. If the task is created at the top level, this parameter is omitted. An assigned task cannot be a parent task, nor can it have a parent. Setting the parent to an assigned task results in failure of the request.

      --previous <PREVIOUS>
          Optional. Previous sibling task identifier. If the task is created at the first position among its siblings, this parameter is omitted.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## tasks.list

```text
Returns all tasks in the specified task list. Doesn't return assigned tasks by default (from Docs, Chat Spaces). A user can have up to 20,000 non-hidden tasks per list and up to 100,000 tasks in total at a time.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasks.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --show-completed[=<BOOL>]
          Optional. Flag indicating whether completed tasks are returned in the result. Note that showHidden must also be True to show tasks completed in first party clients, such as the web UI and Google's mobile apps. The default is True.

      --tasklist <TASKLIST>
          Task list identifier.
          
          [default: @default]

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

      --completed-max <COMPLETED_MAX>
          Optional. Upper bound for a task's completion date (as a RFC 3339 timestamp) to filter by. The default is not to filter by completion date.

      --completed-min <COMPLETED_MIN>
          Optional. Lower bound for a task's completion date (as a RFC 3339 timestamp) to filter by. The default is not to filter by completion date.

      --due-max <DUE_MAX>
          Optional. Upper bound for a task's due date (as a RFC 3339 timestamp) to filter by. The default is not to filter by due date.

      --due-min <DUE_MIN>
          Optional. Lower bound for a task's due date (as a RFC 3339 timestamp) to filter by. The default is not to filter by due date.

      --max-results <MAX_RESULTS>
          Optional. Maximum number of tasks returned on one page. The default is 20 (max allowed: 100).

      --page-token <PAGE_TOKEN>
          Optional. Token specifying the result page to return.

      --show-assigned[=<BOOL>]
          Optional. Flag indicating whether tasks assigned to the current user are returned in the result. The default is False.

      --show-deleted[=<BOOL>]
          Optional. Flag indicating whether deleted tasks are returned in the result. The default is False.

      --show-hidden[=<BOOL>]
          Optional. Flag indicating whether hidden tasks are returned in the result. The default is False.

      --updated-min <UPDATED_MIN>
          Optional. Lower bound for a task's last modification time (as a RFC 3339 timestamp) to filter by. The default is not to filter by last modification time.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## tasks.move

```text
Moves the specified task to another position in the destination task list. If the destination list is not specified, the task is moved within its current list. This can include putting it as a child task under a new parent and/or move it to a different position among its sibling tasks. A user can have up to 2,000 subtasks per task.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasks.move [COMMAND_OPTIONS]

Mandatory parameters:
      --task <TASK>
          Task identifier.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --tasklist <TASKLIST>
          Task list identifier.
          
          [default: @default]

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

      --destination-tasklist <DESTINATION_TASKLIST>
          Optional. Destination task list identifier. If set, the task is moved from tasklist to the destinationTasklist list. Otherwise the task is moved within its current list.

      --parent <PARENT>
          Optional. New parent task identifier. If the task is moved to the top level, this parameter is omitted. The task set as parent must exist in the task list and can not be hidden. Exceptions: 1. Assigned and repeating tasks cannot be set as parent tasks (have subtasks), or be moved under a parent task (become subtasks). 2. Tasks that are both completed and hidden cannot be nested, so the parent field must be empty.

      --previous <PREVIOUS>
          Optional. New previous sibling task identifier. If the task is moved to the first position among its siblings, this parameter is omitted. The task set as previous must exist in the task list and can not be hidden. Exceptions: 1. Tasks that are both completed and hidden can only be moved to position 0, so the previous field must be empty.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## tasks.patch

```text
Updates the specified task. This method supports patch semantics.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasks.patch [COMMAND_OPTIONS]

Mandatory parameters:
      --task <TASK>
          Task identifier.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --due <DUE>
          Scheduled date for the task (as an RFC 3339 timestamp). Optional. This represents the day that the task should be done, or that the task is visible on the calendar grid. It doesn't represent the deadline of the task. Only date information is recorded; the time portion of the timestamp is discarded when setting this field. It isn't possible to read or write the time that a task is scheduled for using the API.

      --notes <NOTES>
          Notes describing the task. Tasks assigned from Google Docs cannot have notes. Optional. Maximum length allowed: 8192 characters.

      --status <STATUS>
          Status of the task. This is either "needsAction" or "completed".

      --tasklist <TASKLIST>
          Task list identifier.
          
          [default: @default]

      --title <TITLE>
          Title of the task. Maximum length allowed: 1024 characters.

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

      --assignment-info <JSON>
          Information about the source of the task assignment (Document, Chat Space).

      --completed <COMPLETED>
          Completion date of the task (as a RFC 3339 timestamp). This field is omitted if the task has not been completed.

      --deleted[=<BOOL>]
          Flag indicating whether the task has been deleted. For assigned tasks this field is read-only. They can only be deleted by calling tasks.delete, in which case both the assigned task and the original task (in Docs or Chat Spaces) are deleted. To delete the assigned task only, navigate to the assignment surface and unassign the task from there. The default is False.

      --etag <ETAG>
          ETag of the resource.

      --hidden[=<BOOL>]
          Flag indicating whether the task is hidden. This is the case if the task had been marked completed when the task list was last cleared. The default is False. This field is read-only.

      --id <ID>
          Task identifier.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## tasks.update

```text
Updates the specified task.

Usage: joog [OPTIONS] tasks <ACCOUNT> tasks.update [COMMAND_OPTIONS]

Mandatory parameters:
      --task <TASK>
          Task identifier.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --verbose
          Include advanced parameters in help

      --due <DUE>
          Scheduled date for the task (as an RFC 3339 timestamp). Optional. This represents the day that the task should be done, or that the task is visible on the calendar grid. It doesn't represent the deadline of the task. Only date information is recorded; the time portion of the timestamp is discarded when setting this field. It isn't possible to read or write the time that a task is scheduled for using the API.

      --notes <NOTES>
          Notes describing the task. Tasks assigned from Google Docs cannot have notes. Optional. Maximum length allowed: 8192 characters.

      --status <STATUS>
          Status of the task. This is either "needsAction" or "completed".

      --tasklist <TASKLIST>
          Task list identifier.
          
          [default: @default]

      --title <TITLE>
          Title of the task. Maximum length allowed: 1024 characters.

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

      --assignment-info <JSON>
          Information about the source of the task assignment (Document, Chat Space).

      --completed <COMPLETED>
          Completion date of the task (as a RFC 3339 timestamp). This field is omitted if the task has not been completed.

      --deleted[=<BOOL>]
          Flag indicating whether the task has been deleted. For assigned tasks this field is read-only. They can only be deleted by calling tasks.delete, in which case both the assigned task and the original task (in Docs or Chat Spaces) are deleted. To delete the assigned task only, navigate to the assignment surface and unassign the task from there. The default is False.

      --etag <ETAG>
          ETag of the resource.

      --hidden[=<BOOL>]
          Flag indicating whether the task is hidden. This is the case if the task had been marked completed when the task list was last cleared. The default is False. This field is read-only.

      --id <ID>
          Task identifier.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).
