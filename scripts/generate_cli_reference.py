#!/usr/bin/env python3
"""Regenerate service references from the built Joog command help."""
import concurrent.futures
import json
import os
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BIN = ROOT / 'target' / 'debug' / ('joog.exe' if os.name == 'nt' else 'joog')
TITLES = dict(mail='Gmail', drive='Google Drive', calendar='Google Calendar', sheets='Google Sheets', docs='Google Docs', slides='Google Slides', forms='Google Forms', tasks='Google Tasks', contacts='Google Contacts', youtube='YouTube', photos='Google Photos app-created media')


def help_text(pair):
    service, name = pair
    output = subprocess.run([str(BIN), 'help', service, name, '--verbose'], check=True, capture_output=True, text=True, encoding='utf-8').stdout
    return pair, output.split('\nReference: ')[0].rstrip()


if __name__ == '__main__':
    sources = {p.stem: json.loads(p.read_text(encoding='utf-8')) for p in (ROOT / 'api').glob('*.json')}
    pairs = [(service, name) for service, source in sources.items() for name in sorted(source['$defs'])]
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as executor:
        results = dict(executor.map(help_text, pairs))
    for service, source in sources.items():
        text = f'# {TITLES[service]} methods\n\nSee the [command guide](../grammar.md) and [request-file format](../file-schemas.md).\n\n| Method | Description |\n| --- | --- |\n'
        for name in sorted(source['$defs']):
            definition = source['$defs'][name]
            method = source['x-google-methods'][definition['x-google-method-id']]
            description = definition['properties']['body']['description'] if 'x-batch-operation' in definition else method['description']
            about = description.split('\n')[0].split('. ')[0].replace('|', '\\|')
            text += f'| `{name}` | {about} |\n'
        text += '\n'
        for name in sorted(source['$defs']):
            text += f'## {name}\n\n```text\n{results[service, name]}\n```\n\n'
        text += "Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).\n"
        (ROOT / 'docs' / 'services' / f'{service}.md').write_text(text, encoding='utf-8', newline='\n')
        print(service)
