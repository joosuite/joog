#!/usr/bin/env python3
"""Generate the wrapper reference from the shared parameter schemas."""
import argparse
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCHEMA = json.loads((ROOT / 'schemas/wrappers.schema.json').read_text())


def expand(value):
    if isinstance(value, list):
        return [expand(item) for item in value]
    if not isinstance(value, dict):
        return value
    result = expand(SCHEMA['$defs'][value['$ref'].split('/')[-1]]) if '$ref' in value else {}
    result.update({key: expand(child) for key, child in value.items() if key != '$ref'})
    for rule in result.get('allOf', []):
        if 'description' not in rule and 'if' not in rule:
            result.setdefault('properties', {}).update(rule.get('properties', {}))
    return result


def option(name, value):
    import re
    return value.get('x-key', re.sub(r'([a-z0-9])([A-Z])', r'\1-\2', name).lower())


def render():
    text = '''# Short command wrappers

Run `make build` to create the wrappers in `bin/`. `make install` installs them
beside `joog`. Add that directory to your `PATH`.

`joog` and its wrappers are native executables. Installing and running them
requires no Python. Building from source requires Rust and Cargo; `make build`
and `make install` also create the wrapper entry points with a native installer.

```sh
jmail ACCOUNT to:lara@example.com body:"Hello\\nHow are you?"
jcal ACCOUNT add "Lunch" at:"tomorrow 12:30" for:1h with:lara@example.com
jdocs ACCOUNT append DOCUMENT text-file:notes.txt
```

The short commands and `key:value` parameters belong to the wrappers. Existing
`joog` commands keep their syntax. For example, `joog mail ACCOUNT messages.send
--to lara@example.com --text Hello` remains available.

Quote values containing spaces. Repeat parameters to supply several recipients,
attachments or rows. Inline text accepts `\\n`, `\\t`, and `\\\\`.
Use `--` before positional text that looks like a named parameter.

Use `file:PATH` to read parameters from a JSON object. File property names use
camelCase, such as `bodyFile` and `matchCase`. Explicit parameters replace file
values, including an entire repeated list. Explicit inline text replaces its
corresponding file input. Required values are checked after combining the inputs.
Select the command definition under `$defs` in
[wrappers.schema.json](../schemas/wrappers.schema.json) to validate complete input.

Lists default to 20 results unless the command describes a date window. Use
`limit:all` to fetch all results. Downloads and exports require `out:PATH` and
never overwrite existing files. CSV imports write literal values, including
text beginning with `=`. Task due dates use the computer's local date. Calendar
dates use the selected calendar's timezone. Ambiguous times at clock changes
require a timestamp with an explicit offset.

Use `WRAPPER --help` to list its commands, `WRAPPER ACCOUNT COMMAND --help` for
common parameters, and `--help --verbose` for advanced parameters. `json:true`
changes only the output format. Both formats contain the same records and fields,
including message IDs and thread IDs in mail results. Advanced `home:PATH` and
`timeout:SECONDS` select the credential
directory and request timeout.

The wrappers use your existing Joog sign-in. Document, spreadsheet, presentation,
and form listing use Drive. Document sharing and native file exports also use
Drive. Authorize it with the service when needed, for example
`joog auth ACCOUNT login docs,drive`. Photos commands access content created by
this application.

Compound commands can perform several requests. If a later request fails, the
error identifies an already-created resource where available. Check that
resource before retrying a creation command.

'''
    for wrapper, config in SCHEMA['x-wrappers'].items():
        commands = [(key.split('.', 1)[1], expand(value)) for key, value in SCHEMA['$defs'].items() if key.startswith(wrapper + '.')]
        text += f'## {wrapper}\n\n{config["description"]} The default command is `{config["default"]}`.\n\n'
        text += '| Command | Description |\n| --- | --- |\n'
        for command, value in commands:
            text += f'| `{command}` | {value["description"]} |\n'
        text += '\n'
        for command, value in commands:
            positions = sorted(((field['x-position'], name, field) for name, field in value['properties'].items() if 'x-position' in field))
            usage = f'{wrapper} ACCOUNT {command}'
            for _, name, field in positions:
                label = field.get('x-label', name.upper())
                usage += ' ' + (label if name in value.get('required', []) else '[' + label + ']')
            text += f'### {command}\n\n{value["description"]}\n\n```text\n{usage} [key:value ...]\n```\n\n| Parameter | Required | Description |\n| --- | --- | --- |\n'
            for name, field in value['properties'].items():
                description = field.get('description', field.get('x-label', name) + '.')
                if field.get('x-repeat'):
                    description += ' Repeat for multiple values.'
                if 'enum' in field:
                    description += ' Choices: ' + ', '.join(f'`{choice}`' for choice in field['enum']) + '.'
                if 'default' in field:
                    description += f' Default: `{json.dumps(field["default"], ensure_ascii=False)}`.'
                if 'maxLength' in field:
                    description += f' Maximum {field["maxLength"]} characters.'
                if 'maxItems' in field:
                    description += f' Maximum {field["maxItems"]} values.'
                if 'minimum' in field and 'maximum' in field:
                    description += f' Allowed range: {field["minimum"]} to {field["maximum"]}.'
                if field.get('x-advanced'):
                    description += ' Advanced parameter.'
                required = 'yes' if name in value.get('required', []) else 'no'
                description = description.replace('|', '\\|')
                text += f'| `{option(name, field)}:` | {required} | {description} |\n'
            text += '\n'
            explanations = set()
            for rule in value.get('allOf', []):
                if 'description' in rule and rule['description'] not in explanations:
                    explanations.add(rule['description'])
                    text += rule['description'] + '\n\n'
    return text


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true')
    args = parser.parse_args()
    destination = ROOT / 'docs/wrappers.md'
    rendered = render()
    if args.check:
        if not destination.exists() or destination.read_text() != rendered:
            parser.exit(1, 'run python3 scripts/generate_wrapper_reference.py\n')
    else:
        destination.write_text(rendered, encoding='utf-8', newline='\n')
