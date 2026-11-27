import { readFileSync, readdirSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

// Discovery schemas contain arbitrary extension keys alongside JSON Schema keywords.
export type Schema = Record<string, any>;
export const ROOT = dirname(dirname(fileURLToPath(import.meta.url)));
export const isMain = (url: string): boolean => Boolean(process.argv[1]) && resolve(process.argv[1]) === fileURLToPath(url);
export const readJson = (path: string): Schema => JSON.parse(readFileSync(path, "utf8"));
export const entries = (value: Schema = {}): [string, Schema][] => Object.entries(value);
export const ordered = (value: Schema = {}): [string, Schema][] => entries(value).sort(([a], [b]) => a < b ? -1 : a > b ? 1 : 0);
export const nonempty = (value: Schema): boolean => Object.keys(value).length > 0;
export const capitalize = (value: string): string => value.slice(0, 1).toUpperCase() + value.slice(1).toLowerCase();
export const sources = (): string[] => readdirSync(join(ROOT, "api")).filter(name => name.endsWith(".json")).sort();

export function checkOption(args = process.argv.slice(2)): boolean {
  if (args.length === 1 && ["--help", "-h"].includes(args[0])) {
    console.log("Usage: node --experimental-strip-types SCRIPT.ts [--check]\n\n--check  Fail when a generated file is out of date.");
    process.exit(0);
  }
  if (args.some(arg => arg !== "--check") || args.length > 1) {
    throw new Error("Usage: node --experimental-strip-types SCRIPT.ts [--check]");
  }
  return args.includes("--check");
}

export function generatedFile(path: string, content: string, check: boolean, regenerate: string): void {
  if (check) {
    let actual: string | undefined;
    try { actual = readFileSync(path, "utf8"); }
    catch (error) { if ((error as NodeJS.ErrnoException).code !== "ENOENT") throw error; }
    if (actual !== content) throw new Error(`${path}: run ${regenerate}`);
  } else {
    writeFileSync(path, content, "utf8");
  }
}

export function main(run: () => void | Promise<void>): void {
  Promise.resolve().then(run).catch(error => {
    console.error(error instanceof Error ? error.message : String(error));
    process.exitCode = 1;
  });
}
