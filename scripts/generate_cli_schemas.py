#!/usr/bin/env python3
"""Generate CLI input schemas from the pinned API contracts in api/."""
import copy
import argparse
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
RESERVED = set('file params body fields max max-pages ttl upload content-type output home timeout json help'.split())

# These names describe the CLI input. Their targets remain ordinary API fields.
ALIASES = {
    'calendar': {('start', 'dateTime'): 'start-time', ('end', 'dateTime'): 'end-time'},
    'sheets': {('properties', 'title'): 'title'},
    'docs': {('location', 'index'): 'index', ('containsText', 'text'): 'find', ('containsText', 'matchCase'): 'match-case'},
    'slides': {('slideLayoutReference', 'predefinedLayout'): 'layout', ('containsText', 'text'): 'find', ('containsText', 'matchCase'): 'match-case'},
    'forms': {('info', 'title'): 'title'},
    'youtube': {('snippet', 'title'): 'title', ('snippet', 'description'): 'description', ('snippet', 'tags'): 'tags', ('status', 'privacyStatus'): 'privacy', ('snippet', 'resourceId', 'videoId'): 'video-id', ('snippet', 'resourceId', 'channelId'): 'channel-id'},
    'photos': {('album', 'title'): 'title', ('filters', 'mediaTypeFilter', 'mediaTypes'): 'media-types', ('filters', 'includeArchivedMedia'): 'include-archived'},
}


def kebab(name):
    return re.sub(r'([a-z0-9])([A-Z])', r'\1-\2', re.sub(r'([A-Z])([A-Z][a-z])', r'\1-\2', name)).replace('_', '-').replace('.', '-').lower()


def camel(name):
    head, *tail = name.split('-')
    return head + ''.join(word.title() for word in tail)


def shape(api, value):
    seen = set()
    while value:
        if 'anyOf' in value:
            value = next((part for part in value['anyOf'] if part.get('type') != 'null'), {})
        elif '$ref' in value:
            name = value['$ref'].split('/')[-1]
            if name in seen:
                return {}
            seen.add(name)
            value = api['x-body-types'][name]
        else:
            return value
    return {}


def binding(schema, long, source, prop, **extra):
    result = copy.deepcopy(schema)
    result['x-cli-option'] = long
    result['x-cli-binding'] = dict(source=source, property=prop, **extra)
    return result


def presence(path, **checks):
    result = {'not': {'type': 'null'}, **checks}
    for key in reversed(path):
        result = {'required': [key], 'properties': {key: result}}
    return result


def required_for(value, method):
    return method in value.get('x-required-in-methods', []) or any(required_for(part, method) for part in value.get('anyOf', []))


def user_description(text):
    if 'b/157517979' in text:
        return 'Deprecated. This field is no longer populated.'
    if 'b/33408548' in text:
        return 'MENA content rating, using the MPAA rating categories.'
    return text


def command_rules(api, old, method, command):
    """Express merged CLI requirements as ordinary JSON Schema assertions.

    The x-cli annotations only select help text and argv conflict groups; the
    assertions themselves are also usable by a Draft 2020-12 validator.
    """
    props = command['properties']
    rules = []
    targets = {}
    for key, value in props.items():
        mapping = value.get('x-cli-binding', {})
        source = mapping.get('source')
        if source == 'raw':
            targets[key] = []
        elif source == 'body':
            targets[key] = [mapping['property']]
        elif source == 'nested':
            targets[key] = mapping['path']

    def either(conditions):
        return {'anyOf': conditions}

    def at(path):
        conditions = []
        for key, target in targets.items():
            if path[:len(target)] == target:
                conditions.append(presence([key] + path[len(target):]))
            elif target[:len(path)] == path:
                conditions.append({'required': [key]})
        return either(conditions)

    def keys_at(path):
        return [key for key, target in targets.items() if target[:len(path)] == path and target]

    def names(keys):
        return ' or '.join(props[key]['x-cli-value-name'] if props[key].get('x-cli-binding', {}).get('positional') else '--' + props[key].get('x-cli-option', key) for key in keys)

    def rule(assertion, description, keys=(), kind='choice', groups=None):
        result = dict(assertion, description=description, **{'x-cli-kind': kind, 'x-cli-parameters': list(keys)})
        if groups:
            result['x-cli-groups'] = groups
        rules.append(result)

    def conflict(conditions, groups, description):
        for i, left in enumerate(conditions):
            for j in range(i + 1, len(conditions)):
                rule({'not': {'allOf': [left, conditions[j]]}}, description,
                     groups[i] + groups[j], 'conflict', [groups[i], groups[j]])

    composition = [key for key, value in props.items() if value.get('x-cli-binding', {}).get('source') == 'compose']
    for key in method.get('x-required-cli-inputs', []):
        rule(presence([key]), f'Provide {names([key])}.', [key], 'required')
    compose = either([{'required': [key]} for key in composition] + ([presence(['compose'])] if composition else []))
    compose_path = old.get('x-cli-compose-path', [])
    defaults = {'gmail': {'userId': 'me'}, 'calendar': {'calendarId': 'primary'}, 'tasks': {'tasklist': '@default'}, 'youtube': {'part': 'snippet'}}
    for key, value in props.items():
        mapping = value.get('x-cli-binding', {})
        prop = mapping.get('property')
        parameter = method.get('parameters', {}).get(prop, {}) if mapping.get('source') == 'query' else {}
        default = defaults.get(api.get('x-google-api'), {}).get(prop)
        if api.get('x-google-api') == 'youtube' and prop == 'part':
            def has_snippet(value):
                resolved = shape(api, value)
                return 'snippet' in resolved.get('properties', {}) or 'snippet' in shape(api, resolved.get('properties', {}).get('items', {}).get('items', {})).get('properties', {})
            if not parameter.get('required') or ('snippet' not in parameter.get('description', '') and not has_snippet(method.get('request', {})) and not has_snippet(method.get('response', {}))):
                default = None
        if default is not None:
            value['default'] = default
        if (parameter.get('required') and default is None) or value.pop('x-required-after-merge', False):
            rule(presence([key]), f'Provide {names([key])}.', [key], 'required')

    request = old.get('properties', {}).get('body')
    def visit(schema, path, ancestors, mandatory):
        reference = schema.get('$ref')
        if reference and reference in ancestors or len(path) > 8:
            return
        resolved = shape(api, schema)
        ancestors = ancestors | ({reference} if reference else set())
        choices = resolved.get('x-required-one-of', {}).get(method.get('id'))
        choice_description = None
        for group in resolved.get('x-exclusive-fields', []):
            paths = [path + [key] for key in group]
            groups = [keys_at(target) for target in paths]
            if group == ['date', 'dateTime']:
                date, time = [names(keys_at(target)) for target in paths]
                description = f'Use {date} for an all-day event or {time} for a timed event; do not supply both.'
                if choices:
                    prefix = f'{".".join(path).capitalize()} is required: ' if mandatory else f'When setting {".".join(path)}, '
                    description = prefix + f'use {date} for an all-day event or {time} for a timed event.'
                choice_description = description
            elif group == ['albumId', 'filters']:
                description = 'Search an album with --album-id, or search with filters. Album searches do not accept filters.'
            else:
                description = f'Choose one of {names([key for branch in groups for key in branch])}.'
            conflict([at(target) for target in paths], groups, description)
        if choices:
            assertion = either([at(path + [key]) for key in choices])
            if path:
                assertion = {'if': at(path), 'then': assertion}
            keys = [key for field in choices for key in keys_at(path + [field])]
            rule(assertion, choice_description or f'For {".".join(path)}, provide {names(keys)}.', keys)
        for key, value in resolved.get('properties', {}).items():
            if shape(api, value).get('readOnly'):
                continue
            target = path + [key]
            required = key in resolved.get('required', []) + resolved.get('x-required-fields', []) or required_for(value, method.get('id'))
            if required:
                assertion = at(target)
                if composition and compose_path[:len(target)] == target:
                    assertion = either([assertion, compose] + ([presence(['upload'])] if 'upload' in props else []))
                if path:
                    assertion = {'if': at(path), 'then': assertion}
                keys = keys_at(target)
                alternatives = [k for k in keys if targets[k] != target]
                is_required = mandatory and not alternatives and not (composition and compose_path[:len(target)] == target)
                description = f'Provide {names(keys) or ".".join(target)}.'
                required_children = []
                if alternatives:
                    child = shape(api, value)
                    child_choices = child.get('x-required-one-of', {}).get(method.get('id'), [])
                    required_children = [field for field, prop in child.get('properties', {}).items() if field in child_choices + child.get('required', []) + child.get('x-required-fields', []) or required_for(prop, method.get('id'))]
                    preferred = [k for field in required_children for k in keys_at(target + [field])] or alternatives
                    exact = [k for k in keys if targets[k] == target]
                    description = f'Provide {names(preferred)} for {".".join(target)}.' if required_children else f'Provide {names(exact)} as a JSON object, or set its named fields.'
                rule(assertion, description, keys, 'required' if is_required else 'choice')
                if not mandatory or alternatives and required_children or composition and compose_path[:len(target)] == target:
                    rules[-1]['x-cli-help'] = False
            if shape(api, value).get('type') == 'object':
                visit(value, target, ancestors, mandatory and required)
    if request:
        visit(request, [], set(), True)
        body_keys = [key for key, target in targets.items() if target]
        if shape(api, request).get('properties') and 'x-batch-operation' not in old:
            choices = body_keys + composition + (['upload'] if 'upload' in props else []) + ['body']
            if composition:
                choices.append('compose')
            rule(either([{'required': [key]} if key in body_keys + composition else presence([key]) for key in choices]), 'Provide the content to write using named parameters, a JSON file, or --body.', choices, 'content')
    if composition:
        for inline, file in api['x-cli-types']['MailComposition']['x-exclusive-fields']:
            conflict([either([presence([key]), presence(['compose', key])]) for key in [inline, file]], [[inline], [file]],
                     f'Use --{props[inline]["x-cli-option"]} for inline content or --{props[file]["x-cli-option"]} to read a file.')
        raw_paths = [compose_path, compose_path[:-1] + ['payload']]
        conditions = [at(path) for path in raw_paths] + ([presence(['upload'])] if 'upload' in props else [])
        raw_keys = [key for path in raw_paths for key in keys_at(path)] + (['upload'] if 'upload' in props else [])
        rule({'not': {'allOf': [compose, either(conditions)]}}, 'Compose a message with recipients and content, or supply raw MIME. Composition cannot be combined with raw message content or --upload.',
             composition + raw_keys, 'conflict', [composition, raw_keys])
        if method.get('id', '').endswith('.send'):
            recipients = [presence(path, type='array', minItems=1) for key in ['to', 'cc', 'bcc'] for path in [[key], ['compose', key]]]
            rule({'if': compose, 'then': either(recipients)}, 'When composing a message to send, provide at least one recipient with --to, --cc, or --bcc.', ['to', 'cc', 'bcc'])
    if 'output' in props:
        for key in ['max', 'maxPages', 'upload']:
            if key in props:
                conflict([presence(['output']), presence([key])], [['output'], [key]], f'Use --output for a single download; omit --{props[key]["x-cli-option"]} when downloading.')
    if 'contentType' in props:
        rule({'if': presence(['contentType']), 'then': presence(['upload'])}, '--content-type sets the MIME type of a file supplied with --upload.', ['contentType', 'upload'], 'dependency')
    for selected in method.get('x-unpaginated-when', []):
        if selected in props:
            for page in ['maxResults', 'pageSize', 'pageToken', 'maxPages']:
                if page in props:
                    conflict([presence([selected]), presence([page])], [[selected], [page]],
                             f'When selecting records with {names([selected])}, omit {names([page])}. Use --max to limit the returned records.')

    # Array records stay structured JSON inputs. Preserve requirements inside
    # those records as standard assertions, including when supplied via --body.
    def array_rules(schema, ancestors, inside_array=False, path=''):
        resolved = shape(api, schema)
        marker = id(resolved)
        if marker in ancestors:
            return {}, []
        ancestors = ancestors | {marker}
        if resolved.get('type') == 'array':
            assertion, labels = array_rules(resolved.get('items', {}), ancestors, True, path + '[]')
            return ({'items': assertion} if assertion else {}), labels
        properties, required, labels, assertions = {}, [], [], []
        for key, value in resolved.get('properties', {}).items():
            if shape(api, value).get('readOnly'):
                continue
            label = (path + '.' if path else '') + key
            child, child_labels = array_rules(value, ancestors, inside_array, label)
            labels += child_labels
            if inside_array and (key in resolved.get('required', []) + resolved.get('x-required-fields', []) or required_for(value, method.get('id'))):
                required.append(key)
                labels.append(label)
                child = dict(child, **{'not': {'type': 'null'}})
            if child:
                properties[key] = child
        if inside_array:
            for group in resolved.get('x-exclusive-fields', []):
                for i, key in enumerate(group):
                    assertions += [{'not': {'allOf': [presence([key]), presence([other])]}} for other in group[i + 1:]]
            choices = resolved.get('x-required-one-of', {}).get(method.get('id'))
            if choices:
                assertions.append(either([presence([key]) for key in choices]))
        result = {}
        if properties:
            result['properties'] = properties
        if required:
            result['required'] = required
        if assertions:
            result['allOf'] = assertions
        return result, labels
    for key, value in props.items():
        if value.get('x-cli-binding', {}).get('source') not in {'raw', 'body', 'nested', 'envelope'}:
            continue
        assertion, labels = array_rules(value, set())
        if assertion:
            label = '--' + value['x-cli-option']
            description = f'Records in {label} require {", ".join(sorted(set(labels)))}.' if labels else f'Choose compatible record fields in {label}.'
            rule({'properties': {key: assertion}}, description, [key], 'structured')
    if rules:
        command['allOf'] = rules


def make_service(service, api):
    commands = {}
    for name, old in api['$defs'].items():
        method = api['x-google-methods'][old['x-google-method-id']]
        props = {}
        used = set(RESERVED)
        for prop, parameter in sorted(method.get('parameters', {}).items()):
            if prop in {'fields', 'access_token', 'oauth_token', 'key'}:
                continue
            long = kebab(prop)
            if long in used:
                long = 'query-' + long
            used.add(long)
            schema = copy.deepcopy(old['properties'][prop])
            schema['description'] = parameter.get('description', '')
            schema = binding(schema, long, 'query', prop)
            schema['x-cli-repeated'] = parameter.get('repeated', False)
            props[camel(long)] = schema
        props['fields'] = binding({'type': 'string', 'description': 'Select response fields.'}, 'fields', 'control', 'fields')
        request = old.get('properties', {}).get('body')
        if request:
            props['body'] = binding(request, 'body', 'raw', 'body')
            for prop, schema in sorted(shape(api, request).get('properties', {}).items()):
                resolved = shape(api, schema)
                if resolved.get('readOnly'):
                    continue
                long = kebab(prop)
                if long in used:
                    long = 'body-' + long
                used.add(long)
                item = binding(schema, long, 'body', prop)
                item['x-cli-repeated'] = resolved.get('type') == 'array' and shape(api, resolved.get('items', {})).get('type') in {'string', 'integer', 'number', 'boolean'}
                props[camel(long)] = item
            # Object children become named inputs too. Arrays of records retain a
            # structured JSON input, so fields cannot accidentally get mis-zipped.
            def children(schema, path, ancestors):
                reference = schema.get('$ref')
                if reference and reference in ancestors:
                    return
                resolved = shape(api, schema)
                if resolved.get('readOnly') or len(path) > 4:
                    return
                if resolved.get('type') == 'object':
                    for child, value in sorted(resolved.get('properties', {}).items()):
                        children(value, path + [child], ancestors | ({reference} if reference else set()))
                elif len(path) > 1:
                    long = ALIASES.get(service, {}).get(tuple(path), '-'.join(kebab(part) for part in path))
                    if long in used:
                        return
                    used.add(long)
                    item = binding(schema, long, 'nested', camel(long), path=path)
                    item['x-cli-repeated'] = resolved.get('type') == 'array' and shape(api, resolved.get('items', {})).get('type') in {'string', 'integer', 'number', 'boolean'}
                    props[camel(long)] = item
            for prop, schema in sorted(shape(api, request).get('properties', {}).items()):
                children(schema, [prop], set())
        if 'x-batch-operation' in old:
            for prop, schema in old['properties'].items():
                if prop in {'body', 'fields'} or prop in method.get('parameters', {}):
                    continue
                long = kebab(prop)
                if long in used:
                    long = 'batch-' + long
                used.add(long)
                props[camel(long)] = binding(schema, long, 'envelope', prop)
        # Preserve the existing title option while making it a real file input.
        if name == 'create' and service in {'sheets', 'forms'}:
            path = ['properties' if service == 'sheets' else 'info', 'title']
            props['title'] = binding({'type': 'string', 'description': 'Title.'}, 'title', 'nested', 'title', path=path)
        if 'x-cli-compose-path' in old:
            for prop, schema in api['x-cli-types']['MailComposition']['properties'].items():
                long = schema.get('x-cli-option', kebab(prop))
                props[camel(long)] = binding(schema, long, 'compose', prop)
            # Kept as a compatibility input; ordinary composition is flat.
            props['compose'] = {'$ref': '#/x-body-types/MailComposition', 'description': 'Composition object. The same parameters can be supplied at the top level.'}
        read = method['httpMethod'] in {'GET', 'HEAD'} or method.get('x-read-only', False)
        direct = method.get('supportsMediaDownload') and ('response' not in method or method['id'].endswith('.media.download'))
        controls = {}
        if read and not method.get('x-fresh-response') and not direct:
            controls['ttl'] = {'type': 'integer', 'minimum': 0, 'maximum': 4294967295, 'description': 'Cache freshness in minutes. Zero fetches fresh data.'}
        paginated = 'pageToken' in method.get('parameters', {}) or 'pageToken' in method.get('x-body-parameters', {})
        listed = paginated or any(k in method.get('parameters', {}) or k in method.get('x-body-parameters', {}) for k in ['pageSize', 'maxResults']) or method['id'].endswith(('.list', '.search'))
        if read and listed:
            controls['max'] = {'type': 'integer', 'minimum': 1, 'maximum': 4294967295, 'description': 'Maximum total results.'}
        if read and paginated:
            controls['max-pages'] = {'type': 'integer', 'minimum': 1, 'maximum': 10000, 'description': 'Maximum pages to fetch.'}
        if 'mediaUpload' in method:
            media = method['mediaUpload']
            raw_limit = str(media.get('maxSize', ''))
            limit = min(int(raw_limit) if raw_limit.isdigit() else 1024**3, 1024**3)
            size = next((f'{limit // unit} {name}' for unit, name in [(1024**3, 'GiB'), (1024**2, 'MiB'), (1024, 'KiB')] if limit and limit % unit == 0), f'{limit} bytes')
            controls['upload'] = {'type': 'string', 'minLength': 1, 'description': f'Upload a local file of at most {size}.', 'x-file-max-bytes': limit, 'x-file-mime-types': media.get('accept', ['*/*'])}
            controls['content-type'] = {'type': 'string', 'minLength': 1, 'description': 'Upload MIME type.'}
        if method.get('supportsMediaDownload') or method['id'] == 'gmail.users.messages.attachments.get':
            controls['output'] = {'type': 'string', 'minLength': 1, 'description': 'Write downloaded content to this new file.'}
            if direct:
                controls['output']['x-required-after-merge'] = True
        for long, schema in controls.items():
            props[camel(long)] = binding(schema, long, 'control', long)
        commands[name] = {'type': 'object', 'title': f'joog {service} ACCOUNT {name}', 'description': method['description'] if 'x-batch-operation' not in old else request.get('description', name), 'additionalProperties': False, 'properties': props}
        command_rules(api, old, method, commands[name])
    add_native(service, commands)
    for name, command in commands.items():
        if name not in api['$defs']:
            command_rules(api, {}, {}, command)
    result = {'$schema': 'https://json-schema.org/draft/2020-12/schema', '$id': f'urn:joog:file:{service}', 'title': f'Joog {service} command parameters', 'description': 'Parameters accepted by Joog from --file. Each command definition describes CLI inputs. Explicit command-line values override file values. Select a command under $defs.', 'type': 'object', 'anyOf': [{'$ref': '#/$defs/' + name} for name in commands], '$defs': commands}
    # Include only types actually referenced by accepted CLI JSON inputs.
    types = dict(api['x-body-types'])
    types.update(api.get('x-cli-types', {}))
    needed = {}
    def visit(value):
        if isinstance(value, dict):
            if isinstance(value.get('description'), str):
                value['description'] = user_description(value['description'])
            if '$ref' in value:
                type_name = value['$ref'].split('/')[-1]
                value['$ref'] = '#/x-input-types/' + type_name
                if type_name not in needed:
                    needed[type_name] = copy.deepcopy(types[type_name])
                    visit(needed[type_name])
            for key, child in value.items():
                if key != '$ref':
                    visit(child)
        elif isinstance(value, list):
            for child in value:
                visit(child)
    visit(commands)
    if needed:
        result['x-input-types'] = dict(sorted(needed.items()))
    return result


def add_native(service, commands):
    specifications = {
        'mail': {name: [('id', 'MESSAGE_ID', True)] for name in ['read', 'unread', 'star', 'unstar']},
        'drive': {'search': [('query', 'TEXT', True), ('parent', 'ID', False), ('type', 'MIME', False)], 'restore': [('id', 'FILE_ID', True)]},
        'calendar': {'events': [('calendar', 'ID', False), ('from', 'TIMESTAMP', False), ('until', 'TIMESTAMP', False), ('query', 'TEXT', False)]},
        'forms': {'responses': [('id', 'FORM_ID', True), ('since', 'TIMESTAMP', False)], 'response': [('id', 'FORM_ID', True), ('response-id', 'RESPONSE_ID', True)]},
        'youtube': {'search': [('query', 'QUERY', True), ('type', 'TYPE', False), ('order', 'ORDER', False)]},
    }
    for name, arguments in specifications.get(service, {}).items():
        props = {}
        for long, label, positional in arguments:
            field = binding({'type': 'string', 'minLength': 1, 'description': label.replace('_', ' ').capitalize() + '.'}, long, 'native', long, positional=positional)
            field['x-cli-value-name'] = label
            if positional:
                field['x-required-after-merge'] = True
            if service == 'youtube' and name == 'search':
                if long == 'type':
                    field['enum'] = ['video', 'channel', 'playlist']
                if long == 'order':
                    field['enum'] = ['date', 'rating', 'relevance', 'title', 'videoCount', 'viewCount']
            props[camel(long)] = field
        controls = ['fields']
        if (service, name) in {('drive', 'search'), ('calendar', 'events'), ('forms', 'responses'), ('forms', 'response'), ('youtube', 'search')}:
            controls.append('ttl')
        if (service, name) in {('drive', 'search'), ('calendar', 'events'), ('forms', 'responses'), ('youtube', 'search')}:
            controls += ['max', 'max-pages']
        for long in controls:
            schema = {'type': 'string'} if long == 'fields' else {'type': 'integer', 'minimum': 0 if long == 'ttl' else 1, 'maximum': 10000 if long == 'max-pages' else 4294967295}
            props[camel(long)] = binding(schema, long, 'control', long)
        commands[name] = {'type': 'object', 'title': f'joog {service} ACCOUNT {name}', 'additionalProperties': False, 'properties': props}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true', help='Fail when a generated schema is out of date.')
    arguments = parser.parse_args()
    for source in sorted((ROOT / 'api').glob('*.json')):
        schema = make_service(source.stem, json.loads(source.read_text(encoding='utf-8')))
        destination = ROOT / 'schemas' / (source.stem + '.schema.json')
        rendered = json.dumps(schema, ensure_ascii=False, indent=2) + '\n'
        if arguments.check:
            if not destination.exists() or destination.read_text(encoding='utf-8') != rendered:
                parser.exit(1, f'{destination}: run make schemas\n')
        else:
            destination.write_text(rendered, encoding='utf-8', newline='\n')
        print(f'{source.stem}: {len(schema["$defs"])} commands')
