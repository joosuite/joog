import { execFileSync } from "node:child_process";
import { join, resolve } from "node:path";
import { parseArgs } from "node:util";
import { ROOT, isMain, main } from "./shared.ts";
import type { Schema } from "./shared.ts";

export function verifyTag(tag: string, version: string): void {
  if (tag !== `v${version}`) throw new Error("The tag must match the Cargo package version.");
}

export function packageVersion(root = ROOT): string {
  const metadata: Schema = JSON.parse(execFileSync(process.env.CARGO ?? "cargo", ["+stable", "metadata", "--locked", "--no-deps", "--format-version", "1"], { cwd: root, encoding: "utf8" }));
  const selected = metadata.packages.find((pkg: Schema) => resolve(pkg.manifest_path) === join(root, "Cargo.toml"));
  if (!selected) throw new Error("Cannot find the Joog package in Cargo metadata.");
  return selected.version;
}

if (isMain(import.meta.url)) main(() => {
  const { values } = parseArgs({ options: { tag: { type: "string" }, root: { type: "string" } } });
  verifyTag(values.tag ?? process.env.RELEASE_TAG ?? "", packageVersion(resolve(values.root ?? ROOT)));
});
