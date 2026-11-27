import { execFile } from "node:child_process";
import { join } from "node:path";
import { promisify } from "node:util";
import { ROOT, checkOption, generatedFile, main, readJson, sources } from "./shared.ts";

const execute = promisify(execFile);
const binary = join(ROOT, "target", "debug", process.platform === "win32" ? "joog.exe" : "joog");
const titles: Record<string, string> = {
  mail: "Gmail", drive: "Google Drive", calendar: "Google Calendar", sheets: "Google Sheets",
  docs: "Google Docs", slides: "Google Slides", forms: "Google Forms", tasks: "Google Tasks",
  contacts: "Google Contacts", youtube: "YouTube", photos: "Google Photos app-created media",
};

main(async () => {
  const check = checkOption();
  const catalogs = sources().map(source => ({ service: source.slice(0, -5), api: readJson(join(ROOT, "api", source)) }));
  const jobs = catalogs.flatMap(({ service, api }) => Object.keys(api.$defs).sort().map(name => ({ service, name })));
  const help = new Map<string, string>();
  let index = 0;
  await Promise.all(Array.from({ length: 4 }, async () => {
    while (index < jobs.length) {
      const { service, name } = jobs[index++];
      const { stdout } = await execute(binary, ["help", service, name, "--verbose"], { encoding: "utf8", maxBuffer: 16 * 1024 * 1024 });
      help.set(`${service}.${name}`, stdout.split("\nReference: ")[0].trimEnd());
    }
  }));
  for (const { service, api } of catalogs) {
    let text = `# ${titles[service]} methods\n\nSee the [command guide](../grammar.md) and [request-file format](../file-schemas.md).\n\n| Method | Description |\n| --- | --- |\n`;
    for (const name of Object.keys(api.$defs).sort()) {
      const definition = api.$defs[name];
      const method = api["x-google-methods"][definition["x-google-method-id"]];
      const description: string = "x-batch-operation" in definition ? definition.properties.body.description : method.description;
      const about = description.split("\n")[0].split(". ")[0].replaceAll("|", "\\|");
      text += `| \`${name}\` | ${about} |\n`;
    }
    text += "\n";
    for (const name of Object.keys(api.$defs).sort()) text += `## ${name}\n\n\`\`\`text\n${help.get(`${service}.${name}`)}\n\`\`\`\n\n`;
    text += "Method descriptions and request types come from [Google's Discovery documents](https://developers.google.com/discovery/v1/using).\n";
    generatedFile(join(ROOT, "docs", "services", `${service}.md`), text, check, "make reference");
    console.log(service);
  }
});
