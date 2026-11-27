import { join } from "node:path";
import { ROOT, checkOption, entries, generatedFile, main, readJson } from "./shared.ts";
import type { Schema } from "./shared.ts";

const schema = readJson(join(ROOT, "schemas/wrappers.schema.json"));

function expand(value: any): any {
  if (Array.isArray(value)) return value.map(expand);
  if (value === null || typeof value !== "object") return value;
  const result = value.$ref ? expand(schema.$defs[value.$ref.split("/").at(-1)]) : {};
  for (const [key, child] of Object.entries(value)) if (key !== "$ref") result[key] = expand(child);
  for (const rule of result.allOf ?? []) {
    if (!("description" in rule) && !("if" in rule)) result.properties = { ...result.properties, ...rule.properties };
  }
  return result;
}

function option(name: string, value: Schema): string {
  return value["x-key"] ?? name.replace(/([a-z0-9])([A-Z])/g, "$1-$2").toLowerCase();
}

function render(): string {
  let text = `# Short command wrappers

Run \`make build\` to create the wrappers in \`bin/\`. \`make install\` installs them
beside \`joog\`. Add that directory to your \`PATH\`.

\`joog\` and its wrappers are native executables. Building from source requires
Rust and Cargo; \`make build\` and \`make install\` also create the wrapper entry
points with a native installer.

\`\`\`sh
jmail ACCOUNT to:lara@example.com body:"Hello\\nHow are you?"
jcal ACCOUNT add "Lunch" at:"tomorrow 12:30" for:1h with:lara@example.com
jdocs ACCOUNT append DOCUMENT text-file:notes.txt
\`\`\`

The short commands and \`key:value\` parameters belong to the wrappers. Existing
\`joog\` commands keep their syntax. For example, \`joog mail ACCOUNT messages.send
--to lara@example.com --text Hello\` remains available.

Quote values containing spaces. Repeat parameters to supply several recipients,
attachments or rows. Inline text accepts \`\\n\`, \`\\t\`, and \`\\\\\`.
Use \`--\` before positional text that looks like a named parameter.

Use \`file:PATH\` to read parameters from a JSON object. File property names use
camelCase, such as \`bodyFile\` and \`matchCase\`. Explicit parameters replace file
values, including an entire repeated list. Explicit inline text replaces its
corresponding file input. Required values are checked after combining the inputs.
Select the command definition under \`$defs\` in
[wrappers.schema.json](../schemas/wrappers.schema.json) to validate complete input.

Lists default to 20 results unless the command describes a date window. Use
\`limit:all\` to fetch all results. Downloads and exports require \`out:PATH\` and
never overwrite existing files. CSV imports write literal values, including
text beginning with \`=\`. Task due dates use the computer's local date. Calendar
dates use the selected calendar's timezone. Ambiguous times at clock changes
require a timestamp with an explicit offset.

Use \`WRAPPER --help\` to list its commands, \`WRAPPER ACCOUNT COMMAND --help\` for
common parameters, and \`--help --verbose\` for advanced parameters. \`json:true\`
changes only the output format. Both formats contain the same records and fields,
including message IDs and thread IDs in mail results. Advanced \`home:PATH\` and
\`timeout:SECONDS\` select the credential
directory and request timeout.

The wrappers use your existing Joog sign-in. Document, spreadsheet, presentation,
and form listing use Drive. Document sharing and native file exports also use
Drive. Authorize it with the service when needed, for example
\`joog auth ACCOUNT login docs,drive\`. Photos commands access content created by
this application.

Compound commands can perform several requests. If a later request fails, the
error identifies an already-created resource where available. Check that
resource before retrying a creation command.

`;
  for (const [wrapper, config] of entries(schema["x-wrappers"])) {
    const commands: [string, Schema][] = entries(schema.$defs).filter(([key]) => key.startsWith(wrapper + ".")).map(([key, value]) => [key.slice(wrapper.length + 1), expand(value)]);
    text += `## ${wrapper}\n\n${config.description} The default command is \`${config.default}\`.\n\n`;
    text += "| Command | Description |\n| --- | --- |\n";
    for (const [command, value] of commands) text += `| \`${command}\` | ${value.description} |\n`;
    text += "\n";
    for (const [command, value] of commands) {
      const positions = entries(value.properties).filter(([, field]) => "x-position" in field).sort(([a, left], [b, right]) => left["x-position"] - right["x-position"] || (a < b ? -1 : a > b ? 1 : 0));
      let usage = `${wrapper} ACCOUNT ${command}`;
      for (const [name, field] of positions) {
        const label = field["x-label"] ?? name.toUpperCase();
        usage += " " + ((value.required ?? []).includes(name) ? label : `[${label}]`);
      }
      text += `### ${command}\n\n${value.description}\n\n\`\`\`text\n${usage} [key:value ...]\n\`\`\`\n\n| Parameter | Required | Description |\n| --- | --- | --- |\n`;
      for (const [name, field] of entries(value.properties)) {
        let description = field.description ?? (field["x-label"] ?? name) + ".";
        if (field["x-repeat"]) description += " Repeat for multiple values.";
        if ("enum" in field) description += " Choices: " + field.enum.map((choice: string) => `\`${choice}\``).join(", ") + ".";
        if ("default" in field) description += ` Default: \`${JSON.stringify(field.default)}\`.`;
        if ("maxLength" in field) description += ` Maximum ${field.maxLength} characters.`;
        if ("maxItems" in field) description += ` Maximum ${field.maxItems} values.`;
        if ("minimum" in field && "maximum" in field) description += ` Allowed range: ${field.minimum} to ${field.maximum}.`;
        if (field["x-advanced"]) description += " Advanced parameter.";
        const required = (value.required ?? []).includes(name) ? "yes" : "no";
        text += `| \`${option(name, field)}:\` | ${required} | ${description.replaceAll("|", "\\|")} |\n`;
      }
      text += "\n";
      const explanations = new Set<string>();
      for (const rule of value.allOf ?? []) {
        if ("description" in rule && !explanations.has(rule.description)) {
          explanations.add(rule.description);
          text += rule.description + "\n\n";
        }
      }
    }
  }
  return text;
}

main(() => generatedFile(join(ROOT, "docs/wrappers.md"), render(), checkOption(), "node --experimental-strip-types scripts/generate_wrapper_reference.ts"));
