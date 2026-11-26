# Google Contacts methods

See the [command guide](../grammar.md) and [request-file format](../file-schemas.md).

| Method | Description |
| --- | --- |
| `contact-groups.batch-get` | Get a list of contact groups owned by the authenticated user by specifying a list of contact group resource names. |
| `contact-groups.create` | Create a new contact group owned by the authenticated user |
| `contact-groups.delete` | Delete an existing contact group owned by the authenticated user by specifying a contact group resource name |
| `contact-groups.get` | Get a specific contact group owned by the authenticated user by specifying a contact group resource name. |
| `contact-groups.list` | List all contact groups owned by the authenticated user |
| `contact-groups.members.modify` | Modify the members of a contact group owned by the authenticated user |
| `contact-groups.update` | Update the name of an existing contact group owned by the authenticated user |
| `other-contacts.copy-other-contact-to-my-contacts-group` | Copies an "Other contact" to a new contact in the user's "myContacts" group Mutate requests for the same user should be sent sequentially to avoid increased latency and failures. |
| `other-contacts.list` | List all "Other contacts", that is contacts that are not in a contact group |
| `other-contacts.search` | Provides a list of contacts in the authenticated user's other contacts that matches the search query |
| `people.batch-create-contacts` | Create a batch of new contacts and return the PersonResponses for the newly Mutate requests for the same user should be sent sequentially to avoid increased latency and failures. |
| `people.batch-delete-contacts` | Delete a batch of contacts |
| `people.batch-update-contacts` | Update a batch of contacts and return a map of resource names to PersonResponses for the updated contacts |
| `people.connections.list` | Provides a list of the authenticated user's contacts |
| `people.create-contact` | Create a new contact and return the person resource for that contact |
| `people.delete-contact` | Delete a contact person |
| `people.delete-contact-photo` | Delete a contact's photo |
| `people.get` | Provides information about a person by specifying a resource name |
| `people.get-batch-get` | Provides information about a list of specific people by specifying a list of requested resource names |
| `people.list-directory-people` | Provides a list of domain profiles and domain contacts in the authenticated user's domain directory |
| `people.search-contacts` | Provides a list of contacts in the authenticated user's grouped contacts that matches the search query |
| `people.search-directory-people` | Provides a list of domain profiles and domain contacts in the authenticated user's domain directory that match the search query. |
| `people.update-contact` | Update contact data for an existing contact person |
| `people.update-contact-photo` | Update a contact's photo |

## contact-groups.batch-get

```text
Get a list of contact groups owned by the authenticated user by specifying a list of contact group resource names.

Usage: joog [OPTIONS] contacts <ACCOUNT> contact-groups.batch-get [COMMAND_OPTIONS]

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

      --group-fields <GROUP_FIELDS>
          Optional. A field mask to restrict which fields on the group are returned. Defaults to `metadata`, `groupType`, `memberCount`, and `name` if not set or set to empty. Valid fields are: * clientData * groupType * memberCount * metadata * name

      --max-members <MAX_MEMBERS>
          Optional. Specifies the maximum number of members to return for each group. Defaults to 0 if not set, which will return zero members.

      --resource-names <RESOURCE_NAMES>...
          Required. The resource names of the contact groups to get. There is a maximum of 200 resource names.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## contact-groups.create

```text
Create a new contact group owned by the authenticated user. Created contact group names must be unique to the users contact groups. Attempting to create a group with a duplicate name will return a HTTP 409 error. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

Usage: joog [OPTIONS] contacts <ACCOUNT> contact-groups.create [COMMAND_OPTIONS]

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

      --contact-group <JSON>
          A contact group.

      --contact-group-client-data <JSON>
          The group's client data.

      --contact-group-etag <CONTACT_GROUP_ETAG>
          The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the resource. Used for web cache validation.

      --contact-group-name <CONTACT_GROUP_NAME>
          The contact group name set by the group owner or a system provided name for system groups. For [`contactGroups.create`](/people/api/rest/v1/contactGroups/create) or [`contactGroups.update`](/people/api/rest/v1/contactGroups/update) the name must be unique to the users contact groups. Attempting to create a group with a duplicate name will return a HTTP 409 error.

      --contact-group-resource-name <CONTACT_GROUP_RESOURCE_NAME>
          The resource name for the contact group, assigned by the server. An ASCII string, in the form of `contactGroups/{contact_group_id}`.

      --read-group-fields <READ_GROUP_FIELDS>
          Optional. A field mask to restrict which fields on the group are returned. Defaults to `metadata`, `groupType`, and `name` if not set or set to empty. Valid fields are: * clientData * groupType * metadata * name

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## contact-groups.delete

```text
Delete an existing contact group owned by the authenticated user by specifying a contact group resource name. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

Usage: joog [OPTIONS] contacts <ACCOUNT> contact-groups.delete [COMMAND_OPTIONS]

Mandatory parameters:
      --resource-name <RESOURCE_NAME>
          Required. The resource name of the contact group to delete.

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

      --delete-contacts[=<BOOL>]
          Optional. Set to true to also delete the contacts in the specified group.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## contact-groups.get

```text
Get a specific contact group owned by the authenticated user by specifying a contact group resource name.

Usage: joog [OPTIONS] contacts <ACCOUNT> contact-groups.get [COMMAND_OPTIONS]

Mandatory parameters:
      --resource-name <RESOURCE_NAME>
          Required. The resource name of the contact group to get.

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

      --group-fields <GROUP_FIELDS>
          Optional. A field mask to restrict which fields on the group are returned. Defaults to `metadata`, `groupType`, `memberCount`, and `name` if not set or set to empty. Valid fields are: * clientData * groupType * memberCount * metadata * name

      --max-members <MAX_MEMBERS>
          Optional. Specifies the maximum number of members to return. Defaults to 0 if not set, which will return zero members.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## contact-groups.list

```text
List all contact groups owned by the authenticated user. Members of the contact groups are not populated.

Usage: joog [OPTIONS] contacts <ACCOUNT> contact-groups.list [COMMAND_OPTIONS]

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

      --group-fields <GROUP_FIELDS>
          Optional. A field mask to restrict which fields on the group are returned. Defaults to `metadata`, `groupType`, `memberCount`, and `name` if not set or set to empty. Valid fields are: * clientData * groupType * memberCount * metadata * name

      --page-size <PAGE_SIZE>
          Optional. The maximum number of resources to return. Valid values are between 1 and 1000, inclusive. Defaults to 30 if not set or set to 0.

      --page-token <PAGE_TOKEN>
          Optional. The next_page_token value returned from a previous call to [ListContactGroups](/people/api/rest/v1/contactgroups/list). Requests the next page of resources.

      --sync-token <SYNC_TOKEN>
          Optional. A sync token, returned by a previous call to `contactgroups.list`. Only resources changed since the sync token was created will be returned.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## contact-groups.members.modify

```text
Modify the members of a contact group owned by the authenticated user. The only system contact groups that can have members added are `contactGroups/myContacts` and `contactGroups/starred`. Other system contact groups are deprecated and can only have contacts removed.

Usage: joog [OPTIONS] contacts <ACCOUNT> contact-groups.members.modify [COMMAND_OPTIONS]

Mandatory parameters:
      --resource-name <RESOURCE_NAME>
          Required. The resource name of the contact group to modify.

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

      --resource-names-to-add <RESOURCE_NAMES_TO_ADD>...
          Optional. The resource names of the contact people to add in the form of `people/{person_id}`. The total number of resource names in `resource_names_to_add` and `resource_names_to_remove` must be less than or equal to 1000.

      --resource-names-to-remove <RESOURCE_NAMES_TO_REMOVE>...
          Optional. The resource names of the contact people to remove in the form of `people/{person_id}`. The total number of resource names in `resource_names_to_add` and `resource_names_to_remove` must be less than or equal to 1000.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## contact-groups.update

```text
Update the name of an existing contact group owned by the authenticated user. Updated contact group names must be unique to the users contact groups. Attempting to create a group with a duplicate name will return a HTTP 409 error. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

Usage: joog [OPTIONS] contacts <ACCOUNT> contact-groups.update [COMMAND_OPTIONS]

Mandatory parameters:
      --resource-name <RESOURCE_NAME>
          The resource name for the contact group, assigned by the server. An ASCII string, in the form of `contactGroups/{contact_group_id}`.

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

      --contact-group <JSON>
          A contact group.

      --contact-group-client-data <JSON>
          The group's client data.

      --contact-group-etag <CONTACT_GROUP_ETAG>
          The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the resource. Used for web cache validation.

      --contact-group-name <CONTACT_GROUP_NAME>
          The contact group name set by the group owner or a system provided name for system groups. For [`contactGroups.create`](/people/api/rest/v1/contactGroups/create) or [`contactGroups.update`](/people/api/rest/v1/contactGroups/update) the name must be unique to the users contact groups. Attempting to create a group with a duplicate name will return a HTTP 409 error.

      --contact-group-resource-name <CONTACT_GROUP_RESOURCE_NAME>
          The resource name for the contact group, assigned by the server. An ASCII string, in the form of `contactGroups/{contact_group_id}`.

      --read-group-fields <READ_GROUP_FIELDS>
          Optional. A field mask to restrict which fields on the group are returned. Defaults to `metadata`, `groupType`, and `name` if not set or set to empty. Valid fields are: * clientData * groupType * memberCount * metadata * name

      --update-group-fields <UPDATE_GROUP_FIELDS>
          Optional. A field mask to restrict which fields on the group are updated. Multiple fields can be specified by separating them with commas. Defaults to `name` if not set or set to empty. Updated fields are replaced. Valid values are: * clientData * name

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## other-contacts.copy-other-contact-to-my-contacts-group

```text
Copies an "Other contact" to a new contact in the user's "myContacts" group Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

Usage: joog [OPTIONS] contacts <ACCOUNT> other-contacts.copy-other-contact-to-my-contacts-group [COMMAND_OPTIONS]

Mandatory parameters:
      --resource-name <RESOURCE_NAME>
          Required. The resource name of the "Other contact" to copy.

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

      --copy-mask <COPY_MASK>
          Required. A field mask to restrict which fields are copied into the new contact. Valid values are: * emailAddresses * names * phoneNumbers

      --read-mask <READ_MASK>
          Optional. A field mask to restrict which fields on the person are returned. Multiple fields can be specified by separating them with commas. Defaults to the copy mask with metadata and membership fields if not set. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

      --sources <SOURCES>...
          Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## other-contacts.list

```text
List all "Other contacts", that is contacts that are not in a contact group. "Other contacts" are typically auto created contacts from interactions. Sync tokens expire 7 days after the full sync. A request with an expired sync token will get an error with an [google.rpc.ErrorInfo](https://cloud.google.com/apis/design/errors#error_info) with reason "EXPIRED_SYNC_TOKEN". In the case of such an error clients should make a full sync request without a `sync_token`. The first page of a full sync request has an additional quota. If the quota is exceeded, a 429 error will be returned. This quota is fixed and can not be increased. When the `sync_token` is specified, resources deleted since the last sync will be returned as a person with `PersonMetadata.deleted` set to true. When the `page_token` or `sync_token` is specified, all other request parameters must match the first call. Writes may have a propagation delay of several minutes for sync requests. Incremental syncs are not intended for read-after-write use cases. See example usage at [List the user's other contacts that have changed](/people/v1/other-contacts#list_the_users_other_contacts_that_have_changed).

Usage: joog [OPTIONS] contacts <ACCOUNT> other-contacts.list [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --read-mask <READ_MASK>
          Required. A field mask to restrict which fields on each person are returned. Multiple fields can be specified by separating them with commas. What values are valid depend on what ReadSourceType is used. If READ_SOURCE_TYPE_CONTACT is used, valid values are: * emailAddresses * metadata * names * phoneNumbers * photos If READ_SOURCE_TYPE_PROFILE is used, valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

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
          Optional. The number of "Other contacts" to include in the response. Valid values are between 1 and 1000, inclusive. Defaults to 100 if not set or set to 0.

      --page-token <PAGE_TOKEN>
          Optional. A page token, received from a previous response `next_page_token`. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `otherContacts.list` must match the first call that provided the page token.

      --request-sync-token[=<BOOL>]
          Optional. Whether the response should return `next_sync_token` on the last page of results. It can be used to get incremental changes since the last request by setting it on the request `sync_token`. More details about sync behavior at `otherContacts.list`.

      --sources <SOURCES>...
          Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT if not set. Possible values for this field are: * READ_SOURCE_TYPE_CONTACT * READ_SOURCE_TYPE_CONTACT,READ_SOURCE_TYPE_PROFILE Specifying READ_SOURCE_TYPE_PROFILE without specifying READ_SOURCE_TYPE_CONTACT is not permitted. Choices: READ_SOURCE_TYPE_UNSPECIFIED, READ_SOURCE_TYPE_PROFILE, READ_SOURCE_TYPE_CONTACT, READ_SOURCE_TYPE_DOMAIN_CONTACT, READ_SOURCE_TYPE_OTHER_CONTACT.

      --sync-token <SYNC_TOKEN>
          Optional. A sync token, received from a previous response `next_sync_token` Provide this to retrieve only the resources changed since the last request. When syncing, all other parameters provided to `otherContacts.list` must match the first call that provided the sync token. More details about sync behavior at `otherContacts.list`.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## other-contacts.search

```text
Provides a list of contacts in the authenticated user's other contacts that matches the search query. The query matches on a contact's `names`, `emailAddresses`, and `phoneNumbers` fields that are from the OTHER_CONTACT source. **IMPORTANT**: Before searching, clients should send a warmup request with an empty query to update the cache. See https://developers.google.com/people/v1/other-contacts#search_the_users_other_contacts

Usage: joog [OPTIONS] contacts <ACCOUNT> other-contacts.search [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --query <QUERY>
          Required. The plain-text query for the request. The query is used to match prefix phrases of the fields on a person. For example, a person with name "foo name" matches queries such as "f", "fo", "foo", "foo n", "nam", etc., but not "oo n".

      --read-mask <READ_MASK>
          Required. A field mask to restrict which fields on each person are returned. Multiple fields can be specified by separating them with commas. Valid values are: * emailAddresses * metadata * names * phoneNumbers

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

      --page-size <PAGE_SIZE>
          Optional. The number of results to return. Defaults to 10 if field is not set, or set to 0. Values greater than 30 will be capped to 30.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## people.batch-create-contacts

```text
Create a batch of new contacts and return the PersonResponses for the newly Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

Usage: joog [OPTIONS] contacts <ACCOUNT> people.batch-create-contacts [COMMAND_OPTIONS]

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

      --contacts <JSON>
          Required. The contact to create. Allows up to 200 contacts in a single request.

      --read-mask <READ_MASK>
          Required. A field mask to restrict which fields on each person are returned in the response. Multiple fields can be specified by separating them with commas. If read mask is left empty, the post-mutate-get is skipped and no data will be returned in the response. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

      --sources <SOURCES>...
          Optional. A mask of what source types to return in the post mutate read. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## people.batch-delete-contacts

```text
Delete a batch of contacts. Any non-contact data will not be deleted. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

Usage: joog [OPTIONS] contacts <ACCOUNT> people.batch-delete-contacts [COMMAND_OPTIONS]

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

      --resource-names <RESOURCE_NAMES>...
          Required. The resource names of the contact to delete. It's repeatable. Allows up to 500 resource names in a single request.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## people.batch-update-contacts

```text
Update a batch of contacts and return a map of resource names to PersonResponses for the updated contacts. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

Usage: joog [OPTIONS] contacts <ACCOUNT> people.batch-update-contacts [COMMAND_OPTIONS]

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

      --contacts <JSON>
          Required. A map of resource names to the person data to be updated. Allows up to 200 contacts in a single request.

      --read-mask <READ_MASK>
          Required. A field mask to restrict which fields on each person are returned. Multiple fields can be specified by separating them with commas. If read mask is left empty, the post-mutate-get is skipped and no data will be returned in the response. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

      --sources <SOURCES>...
          Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set.

      --update-mask <UPDATE_MASK>
          Required. A field mask to restrict which fields on the person are updated. Multiple fields can be specified by separating them with commas. All specified fields will be replaced, or cleared if left empty for each person. Valid values are: * addresses * biographies * birthdays * calendarUrls * clientData * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * relations * sipAddresses * urls * userDefined

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## people.connections.list

```text
Provides a list of the authenticated user's contacts. Sync tokens expire 7 days after the full sync. A request with an expired sync token will get an error with an [google.rpc.ErrorInfo](https://cloud.google.com/apis/design/errors#error_info) with reason "EXPIRED_SYNC_TOKEN". In the case of such an error clients should make a full sync request without a `sync_token`. The first page of a full sync request has an additional quota. If the quota is exceeded, a 429 error will be returned. This quota is fixed and can not be increased. When the `sync_token` is specified, resources deleted since the last sync will be returned as a person with `PersonMetadata.deleted` set to true. When the `page_token` or `sync_token` is specified, all other request parameters must match the first call. Writes may have a propagation delay of several minutes for sync requests. Incremental syncs are not intended for read-after-write use cases. See example usage at [List the user's contacts that have changed](/people/v1/contacts#list_the_users_contacts_that_have_changed).

Usage: joog [OPTIONS] contacts <ACCOUNT> people.connections.list [COMMAND_OPTIONS]

Mandatory parameters:
      --resource-name <RESOURCE_NAME>
          Required. The resource name to return connections for. Only `people/me` is valid.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --person-fields <PERSON_FIELDS>
          Required. A field mask to restrict which fields on each person are returned. Multiple fields can be specified by separating them with commas. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

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
          Optional. The number of connections to include in the response. Valid values are between 1 and 1000, inclusive. Defaults to 100 if not set or set to 0.

      --page-token <PAGE_TOKEN>
          Optional. A page token, received from a previous response `next_page_token`. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `people.connections.list` must match the first call that provided the page token.

      --request-mask-include-field <REQUEST_MASK_INCLUDE_FIELD>
          Required. Comma-separated list of person fields to be included in the response. Each path should start with `person.`: for example, `person.names` or `person.photos`.

      --request-sync-token[=<BOOL>]
          Optional. Whether the response should return `next_sync_token` on the last page of results. It can be used to get incremental changes since the last request by setting it on the request `sync_token`. More details about sync behavior at `people.connections.list`.

      --sort-order <SORT_ORDER>
          Optional. The order in which the connections should be sorted. Defaults to `LAST_MODIFIED_ASCENDING`. Choices: LAST_MODIFIED_ASCENDING, LAST_MODIFIED_DESCENDING, FIRST_NAME_ASCENDING, LAST_NAME_ASCENDING.

      --sources <SOURCES>...
          Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set. Choices: READ_SOURCE_TYPE_UNSPECIFIED, READ_SOURCE_TYPE_PROFILE, READ_SOURCE_TYPE_CONTACT, READ_SOURCE_TYPE_DOMAIN_CONTACT, READ_SOURCE_TYPE_OTHER_CONTACT.

      --sync-token <SYNC_TOKEN>
          Optional. A sync token, received from a previous response `next_sync_token` Provide this to retrieve only the resources changed since the last request. When syncing, all other parameters provided to `people.connections.list` must match the first call that provided the sync token. More details about sync behavior at `people.connections.list`.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## people.create-contact

```text
Create a new contact and return the person resource for that contact. The request returns a 400 error if more than one field is specified on a field that is a singleton for contact sources: * biographies * birthdays * genders * names Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

Usage: joog [OPTIONS] contacts <ACCOUNT> people.create-contact [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --addresses <JSON>
          The person's street addresses.

      --verbose
          Include advanced parameters in help

      --birthdays <JSON>
          The person's birthdays. This field is a singleton for contact sources.

      --email-addresses <JSON>
          The person's email addresses. For `people.connections.list` and `otherContacts.list` the number of email addresses is limited to 100. If a Person has more email addresses the entire set can be obtained by calling GetPeople.

      --memberships <JSON>
          The person's group memberships.

      --names <JSON>
          The person's names. This field is a singleton for contact sources.

      --organizations <JSON>
          The person's past or current organizations.

      --person-fields <PERSON_FIELDS>
          Required. A field mask to restrict which fields on each person are returned. Multiple fields can be specified by separating them with commas. Defaults to all fields if not set. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

      --phone-numbers <JSON>
          The person's phone numbers. For `people.connections.list` and `otherContacts.list` the number of phone numbers is limited to 100. If a Person has more phone numbers the entire set can be obtained by calling GetPeople.

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

      --biographies <JSON>
          The person's biographies. This field is a singleton for contact sources.

      --bragging-rights <JSON>
          **DEPRECATED**: No data will be returned The person's bragging rights.

      --calendar-urls <JSON>
          The person's calendar URLs.

      --client-data <JSON>
          The person's client data.

      --etag <ETAG>
          The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the resource. Used for web cache validation.

      --events <JSON>
          The person's events.

      --external-ids <JSON>
          The person's external IDs.

      --file-ases <JSON>
          The person's file-ases.

      --genders <JSON>
          The person's genders. This field is a singleton for contact sources.

      --im-clients <JSON>
          The person's instant messaging clients.

      --interests <JSON>
          The person's interests.

      --locales <JSON>
          The person's locale preferences.

      --locations <JSON>
          The person's locations.

      --metadata <JSON>
          The metadata about a person.

      --metadata-sources <JSON>
          The sources of data for the person.

      --misc-keywords <JSON>
          The person's miscellaneous keywords.

      --nicknames <JSON>
          The person's nicknames.

      --occupations <JSON>
          The person's occupations.

      --relations <JSON>
          The person's relations.

      --residences <JSON>
          **DEPRECATED**: (Please use `person.locations` instead) The person's residences.

      --resource-name <RESOURCE_NAME>
          The resource name for the person, assigned by the server. An ASCII string in the form of `people/{person_id}`.

      --sip-addresses <JSON>
          The person's SIP addresses.

      --skills <JSON>
          The person's skills.

      --sources <SOURCES>...
          Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set. Choices: READ_SOURCE_TYPE_UNSPECIFIED, READ_SOURCE_TYPE_PROFILE, READ_SOURCE_TYPE_CONTACT, READ_SOURCE_TYPE_DOMAIN_CONTACT, READ_SOURCE_TYPE_OTHER_CONTACT.

      --urls <JSON>
          The person's associated URLs.

      --user-defined <JSON>
          The person's user defined data.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## people.delete-contact

```text
Delete a contact person. Any non-contact data will not be deleted. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

Usage: joog [OPTIONS] contacts <ACCOUNT> people.delete-contact [COMMAND_OPTIONS]

Mandatory parameters:
      --resource-name <RESOURCE_NAME>
          Required. The resource name of the contact to delete.

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

## people.delete-contact-photo

```text
Delete a contact's photo. Mutate requests for the same user should be done sequentially to avoid // lock contention.

Usage: joog [OPTIONS] contacts <ACCOUNT> people.delete-contact-photo [COMMAND_OPTIONS]

Mandatory parameters:
      --resource-name <RESOURCE_NAME>
          Required. The resource name of the contact whose photo will be deleted.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --person-fields <PERSON_FIELDS>
          Optional. A field mask to restrict which fields on the person are returned. Multiple fields can be specified by separating them with commas. Defaults to empty if not set, which will skip the post mutate get. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

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

      --sources <SOURCES>...
          Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set. Choices: READ_SOURCE_TYPE_UNSPECIFIED, READ_SOURCE_TYPE_PROFILE, READ_SOURCE_TYPE_CONTACT, READ_SOURCE_TYPE_DOMAIN_CONTACT, READ_SOURCE_TYPE_OTHER_CONTACT.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## people.get

```text
Provides information about a person by specifying a resource name. Use `people/me` to indicate the authenticated user. The request returns a 400 error if 'personFields' is not specified.

Usage: joog [OPTIONS] contacts <ACCOUNT> people.get [COMMAND_OPTIONS]

Mandatory parameters:
      --resource-name <RESOURCE_NAME>
          Required. The resource name of the person to provide information about. - To get information about the authenticated user, specify `people/me`. - To get information about a google account, specify `people/{account_id}`. - To get information about a contact, specify the resource name that identifies the contact as returned by `people.connections.list`.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --person-fields <PERSON_FIELDS>
          Required. A field mask to restrict which fields on the person are returned. Multiple fields can be specified by separating them with commas. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

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

      --request-mask-include-field <REQUEST_MASK_INCLUDE_FIELD>
          Required. Comma-separated list of person fields to be included in the response. Each path should start with `person.`: for example, `person.names` or `person.photos`.

      --sources <SOURCES>...
          Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_PROFILE and READ_SOURCE_TYPE_CONTACT if not set. Choices: READ_SOURCE_TYPE_UNSPECIFIED, READ_SOURCE_TYPE_PROFILE, READ_SOURCE_TYPE_CONTACT, READ_SOURCE_TYPE_DOMAIN_CONTACT, READ_SOURCE_TYPE_OTHER_CONTACT.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## people.get-batch-get

```text
Provides information about a list of specific people by specifying a list of requested resource names. Use `people/me` to indicate the authenticated user. The request returns a 400 error if 'personFields' is not specified.

Usage: joog [OPTIONS] contacts <ACCOUNT> people.get-batch-get [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --person-fields <PERSON_FIELDS>
          Required. A field mask to restrict which fields on each person are returned. Multiple fields can be specified by separating them with commas. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

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

      --request-mask-include-field <REQUEST_MASK_INCLUDE_FIELD>
          Required. Comma-separated list of person fields to be included in the response. Each path should start with `person.`: for example, `person.names` or `person.photos`.

      --resource-names <RESOURCE_NAMES>...
          Required. The resource names of the people to provide information about. It's repeatable. The URL query parameter should be resourceNames=&resourceNames=&... - To get information about the authenticated user, specify `people/me`. - To get information about a google account, specify `people/{account_id}`. - To get information about a contact, specify the resource name that identifies the contact as returned by `people.connections.list`. There is a maximum of 200 resource names.

      --sources <SOURCES>...
          Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set. Choices: READ_SOURCE_TYPE_UNSPECIFIED, READ_SOURCE_TYPE_PROFILE, READ_SOURCE_TYPE_CONTACT, READ_SOURCE_TYPE_DOMAIN_CONTACT, READ_SOURCE_TYPE_OTHER_CONTACT.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## people.list-directory-people

```text
Provides a list of domain profiles and domain contacts in the authenticated user's domain directory. When the `sync_token` is specified, resources deleted since the last sync will be returned as a person with `PersonMetadata.deleted` set to true. When the `page_token` or `sync_token` is specified, all other request parameters must match the first call. Writes may have a propagation delay of several minutes for sync requests. Incremental syncs are not intended for read-after-write use cases. See example usage at [List the directory people that have changed](/people/v1/directory#list_the_directory_people_that_have_changed).

Usage: joog [OPTIONS] contacts <ACCOUNT> people.list-directory-people [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --read-mask <READ_MASK>
          Required. A field mask to restrict which fields on each person are returned. Multiple fields can be specified by separating them with commas. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

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

      --merge-sources <MERGE_SOURCES>...
          Optional. Additional data to merge into the directory sources if they are connected through verified join keys such as email addresses or phone numbers. Choices: DIRECTORY_MERGE_SOURCE_TYPE_UNSPECIFIED, DIRECTORY_MERGE_SOURCE_TYPE_CONTACT.

      --page-size <PAGE_SIZE>
          Optional. The number of people to include in the response. Valid values are between 1 and 1000, inclusive. Defaults to 100 if not set or set to 0.

      --page-token <PAGE_TOKEN>
          Optional. A page token, received from a previous response `next_page_token`. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `people.listDirectoryPeople` must match the first call that provided the page token.

      --request-sync-token[=<BOOL>]
          Optional. Whether the response should return `next_sync_token`. It can be used to get incremental changes since the last request by setting it on the request `sync_token`. More details about sync behavior at `people.listDirectoryPeople`.

      --sources <SOURCES>...
          Required. Directory sources to return. Choices: DIRECTORY_SOURCE_TYPE_UNSPECIFIED, DIRECTORY_SOURCE_TYPE_DOMAIN_CONTACT, DIRECTORY_SOURCE_TYPE_DOMAIN_PROFILE.

      --sync-token <SYNC_TOKEN>
          Optional. A sync token, received from a previous response `next_sync_token` Provide this to retrieve only the resources changed since the last request. When syncing, all other parameters provided to `people.listDirectoryPeople` must match the first call that provided the sync token. More details about sync behavior at `people.listDirectoryPeople`.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## people.search-contacts

```text
Provides a list of contacts in the authenticated user's grouped contacts that matches the search query. The query matches on a contact's `names`, `nickNames`, `emailAddresses`, `phoneNumbers`, and `organizations` fields that are from the CONTACT source. **IMPORTANT**: Before searching, clients should send a warmup request with an empty query to update the cache. See https://developers.google.com/people/v1/contacts#search_the_users_contacts

Usage: joog [OPTIONS] contacts <ACCOUNT> people.search-contacts [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --query <QUERY>
          Required. The plain-text query for the request. The query is used to match prefix phrases of the fields on a person. For example, a person with name "foo name" matches queries such as "f", "fo", "foo", "foo n", "nam", etc., but not "oo n".

      --read-mask <READ_MASK>
          Required. A field mask to restrict which fields on each person are returned. Multiple fields can be specified by separating them with commas. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

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

      --page-size <PAGE_SIZE>
          Optional. The number of results to return. Defaults to 10 if field is not set, or set to 0. Values greater than 30 will be capped to 30.

      --sources <SOURCES>...
          Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT if not set. Choices: READ_SOURCE_TYPE_UNSPECIFIED, READ_SOURCE_TYPE_PROFILE, READ_SOURCE_TYPE_CONTACT, READ_SOURCE_TYPE_DOMAIN_CONTACT, READ_SOURCE_TYPE_OTHER_CONTACT.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## people.search-directory-people

```text
Provides a list of domain profiles and domain contacts in the authenticated user's domain directory that match the search query.

Usage: joog [OPTIONS] contacts <ACCOUNT> people.search-directory-people [COMMAND_OPTIONS]

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --max <N>
          Return at most N results; by default, return all results

      --verbose
          Include advanced parameters in help

      --query <QUERY>
          Required. Prefix query that matches fields in the person. Does NOT use the read_mask for determining what fields to match.

      --read-mask <READ_MASK>
          Required. A field mask to restrict which fields on each person are returned. Multiple fields can be specified by separating them with commas. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

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

      --merge-sources <MERGE_SOURCES>...
          Optional. Additional data to merge into the directory sources if they are connected through verified join keys such as email addresses or phone numbers. Choices: DIRECTORY_MERGE_SOURCE_TYPE_UNSPECIFIED, DIRECTORY_MERGE_SOURCE_TYPE_CONTACT.

      --page-size <PAGE_SIZE>
          Optional. The number of people to include in the response. Valid values are between 1 and 500, inclusive. Defaults to 100 if not set or set to 0.

      --page-token <PAGE_TOKEN>
          Optional. A page token, received from a previous response `next_page_token`. Provide this to retrieve the subsequent page. When paginating, all other parameters provided to `SearchDirectoryPeople` must match the first call that provided the page token.

      --sources <SOURCES>...
          Required. Directory sources to return. Choices: DIRECTORY_SOURCE_TYPE_UNSPECIFIED, DIRECTORY_SOURCE_TYPE_DOMAIN_CONTACT, DIRECTORY_SOURCE_TYPE_DOMAIN_PROFILE.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## people.update-contact

```text
Update contact data for an existing contact person. Any non-contact data will not be modified. Any non-contact data in the person to update will be ignored. All fields specified in the `update_mask` will be replaced. The server returns a 400 error if `person.metadata.sources` is not specified for the contact to be updated or if there is no contact source. The server returns a 400 error with reason `"failedPrecondition"` if `person.metadata.sources.etag` is different than the contact's etag, which indicates the contact has changed since its data was read. Clients should get the latest person and merge their updates into the latest person. If making sequential updates to the same person, the etag from the `updateContact` response should be used to avoid failures. The server returns a 400 error if `memberships` are being updated and there are no contact group memberships specified on the person. The server returns a 400 error if more than one field is specified on a field that is a singleton for contact sources: * biographies * birthdays * genders * names Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

Input choices:
  Provide --metadata as a JSON object, or set its named fields.

Usage: joog [OPTIONS] contacts <ACCOUNT> people.update-contact [COMMAND_OPTIONS]

Mandatory parameters:
      --resource-name <RESOURCE_NAME>
          The resource name for the person, assigned by the server. An ASCII string in the form of `people/{person_id}`.

Optional parameters:
      --file <PATH>
          Read command parameters from a JSON file

      --json
          Output JSON instead of human-readable text Place before the command.

      --addresses <JSON>
          The person's street addresses.

      --verbose
          Include advanced parameters in help

      --birthdays <JSON>
          The person's birthdays. This field is a singleton for contact sources.

      --email-addresses <JSON>
          The person's email addresses. For `people.connections.list` and `otherContacts.list` the number of email addresses is limited to 100. If a Person has more email addresses the entire set can be obtained by calling GetPeople.

      --memberships <JSON>
          The person's group memberships.

      --names <JSON>
          The person's names. This field is a singleton for contact sources.

      --organizations <JSON>
          The person's past or current organizations.

      --person-fields <PERSON_FIELDS>
          Optional. A field mask to restrict which fields on each person are returned. Multiple fields can be specified by separating them with commas. Defaults to all fields if not set. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

      --phone-numbers <JSON>
          The person's phone numbers. For `people.connections.list` and `otherContacts.list` the number of phone numbers is limited to 100. If a Person has more phone numbers the entire set can be obtained by calling GetPeople.

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

      --biographies <JSON>
          The person's biographies. This field is a singleton for contact sources.

      --body-resource-name <BODY_RESOURCE_NAME>
          The resource name for the person, assigned by the server. An ASCII string in the form of `people/{person_id}`.

      --bragging-rights <JSON>
          **DEPRECATED**: No data will be returned The person's bragging rights.

      --calendar-urls <JSON>
          The person's calendar URLs.

      --client-data <JSON>
          The person's client data.

      --etag <ETAG>
          The [HTTP entity tag](https://en.wikipedia.org/wiki/HTTP_ETag) of the resource. Used for web cache validation.

      --events <JSON>
          The person's events.

      --external-ids <JSON>
          The person's external IDs.

      --file-ases <JSON>
          The person's file-ases.

      --genders <JSON>
          The person's genders. This field is a singleton for contact sources.

      --im-clients <JSON>
          The person's instant messaging clients.

      --interests <JSON>
          The person's interests.

      --locales <JSON>
          The person's locale preferences.

      --locations <JSON>
          The person's locations.

      --metadata <JSON>
          The metadata about a person.

      --metadata-sources <JSON>
          The sources of data for the person.

      --misc-keywords <JSON>
          The person's miscellaneous keywords.

      --nicknames <JSON>
          The person's nicknames.

      --occupations <JSON>
          The person's occupations.

      --relations <JSON>
          The person's relations.

      --residences <JSON>
          **DEPRECATED**: (Please use `person.locations` instead) The person's residences.

      --sip-addresses <JSON>
          The person's SIP addresses.

      --skills <JSON>
          The person's skills.

      --sources <SOURCES>...
          Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set. Choices: READ_SOURCE_TYPE_UNSPECIFIED, READ_SOURCE_TYPE_PROFILE, READ_SOURCE_TYPE_CONTACT, READ_SOURCE_TYPE_DOMAIN_CONTACT, READ_SOURCE_TYPE_OTHER_CONTACT.

      --update-person-fields <UPDATE_PERSON_FIELDS>
          Required. A field mask to restrict which fields on the person are updated. Multiple fields can be specified by separating them with commas. All updated fields will be replaced. Valid values are: * addresses * biographies * birthdays * calendarUrls * clientData * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * relations * sipAddresses * urls * userDefined

      --urls <JSON>
          The person's associated URLs.

      --user-defined <JSON>
          The person's user defined data.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

## people.update-contact-photo

```text
Update a contact's photo. Mutate requests for the same user should be sent sequentially to avoid increased latency and failures.

Usage: joog [OPTIONS] contacts <ACCOUNT> people.update-contact-photo [COMMAND_OPTIONS]

Mandatory parameters:
      --resource-name <RESOURCE_NAME>
          Required. Person resource name

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

      --person-fields <PERSON_FIELDS>
          Optional. A field mask to restrict which fields on the person are returned. Multiple fields can be specified by separating them with commas. Defaults to empty if not set, which will skip the post mutate get. Valid values are: * addresses * ageRanges * biographies * birthdays * calendarUrls * clientData * coverPhotos * emailAddresses * events * externalIds * genders * imClients * interests * locales * locations * memberships * metadata * miscKeywords * names * nicknames * occupations * organizations * phoneNumbers * photos * relations * sipAddresses * skills * urls * userDefined

      --photo-bytes <PHOTO_BYTES>
          Required. Raw photo bytes

      --sources <SOURCES>...
          Optional. A mask of what source types to return. Defaults to READ_SOURCE_TYPE_CONTACT and READ_SOURCE_TYPE_PROFILE if not set.

Help:
  Use --help --verbose to see all parameters and full descriptions.
  Read parameters from --file PATH; explicit options override file values.
```

Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).
