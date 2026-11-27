import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { cpSync, createReadStream, existsSync, mkdirSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { parseArgs } from "node:util";
import { ROOT, isMain, main, readJson } from "./shared.ts";
import { packageVersion, verifyTag } from "./check_release_version.ts";

export function archiveDirectory(stage: string, archive: string): void {
  if (existsSync(archive)) throw new Error(`Archive already exists: ${archive}`);
  const flags = archive.endsWith(".zip") ? ["-a", "-cf"] : ["-czf"];
  execFileSync("tar", [...flags, resolve(archive), "-C", dirname(stage), basename(stage)], { stdio: "inherit" });
}

export async function checksumFile(path: string): Promise<string> {
  const digest = createHash("sha256");
  for await (const chunk of createReadStream(path)) digest.update(chunk);
  return digest.digest("hex");
}

if (isMain(import.meta.url)) main(async () => {
  const { values } = parseArgs({ options: {
    tag: { type: "string" }, target: { type: "string" }, source: { type: "string" }, output: { type: "string" }, root: { type: "string" },
  } });
  const tag = values.tag ?? process.env.RELEASE_TAG ?? "";
  const target = values.target ?? process.env.TARGET ?? "";
  const root = resolve(values.root ?? ROOT);
  verifyTag(tag, packageVersion(root));
  if (!/^[a-z0-9_]+(?:-[a-z0-9_]+)+$/.test(target)) throw new Error("A valid Rust release target is required.");
  const suffix = process.platform === "win32" ? ".exe" : "";
  const source = resolve(values.source ?? join(root, "target", target, "release"));
  const output = resolve(values.output ?? join(root, "dist"));
  const stage = join(output, `joog-${tag}-${target}`);
  mkdirSync(output, { recursive: true });
  mkdirSync(stage);
  const wrappers = readJson(join(root, "schemas", "wrappers.schema.json"))["x-wrappers"];
  for (const name of ["joog", ...Object.keys(wrappers)]) {
    const executable = join(source, name + suffix);
    if (!statSync(executable).isFile()) throw new Error(`Missing native executable: ${name}`);
    const staged = join(stage, name + suffix);
    cpSync(executable, staged, { dereference: false, verbatimSymlinks: true, preserveTimestamps: true, force: false, errorOnExist: true });
    const result = execFileSync(staged, [name === "joog" ? "--version" : "--help"], { encoding: "utf8", maxBuffer: 16 * 1024 * 1024 });
    const expected = name === "joog" ? `joog ${tag.slice(1)}` : `Usage: ${name} ACCOUNT`;
    if (!result.includes(expected)) throw new Error(`Native executable verification failed: ${name}`);
  }
  for (const name of ["README.md", "LICENSE", "docs", "schemas", "examples/requests", "assets/brand/logo.png", "assets/brand/icon.png"]) {
    const destination = join(stage, name);
    mkdirSync(dirname(destination), { recursive: true });
    cpSync(join(root, name), destination, { recursive: true, dereference: false, verbatimSymlinks: true, preserveTimestamps: true, force: false, errorOnExist: true });
  }
  const commit = execFileSync("git", ["rev-parse", "HEAD"], { cwd: root, encoding: "utf8" }).trim();
  writeFileSync(join(stage, "BUILD-INFO.json"), JSON.stringify({ version: tag.slice(1), tag, target, commit }, null, 2) + "\n", "utf8");
  const archive = stage + (process.platform === "win32" ? ".zip" : ".tar.gz");
  archiveDirectory(stage, archive);
  writeFileSync(archive + ".sha256", `${await checksumFile(archive)}  ${basename(archive)}\n`, "utf8");
  console.log(`Packaged ${Object.keys(wrappers).length + 1} verified native executables: ${archive}`);
});
