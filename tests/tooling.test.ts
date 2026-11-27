import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { chmodSync, lstatSync, mkdtempSync, mkdirSync, readFileSync, readlinkSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import test from "node:test";
import { verifyTag } from "../scripts/check_release_version.ts";
import { archiveDirectory, checksumFile } from "../scripts/package_release.ts";

test("release tags must match the selected Rust package version", () => {
  verifyTag("v1.2.3", "1.2.3");
  verifyTag("v1.2.3-beta.1", "1.2.3-beta.1");
  for (const tag of ["", "1.2.3", "v1.2.4", "../v1.2.3"]) assert.throws(() => verifyTag(tag, "1.2.3"), /tag must match/);
});

// The Windows and macOS runners provide BSD tar's ZIP writer; Linux packages use gzip.
const formats = process.platform === "linux" ? ["tar.gz"] : ["tar.gz", "zip"];
for (const extension of formats) {
  test(`${extension} archives preserve package files and have verifiable checksums`, async () => {
    const directory = mkdtempSync(join(tmpdir(), "joog-package-"));
    try {
      const stage = join(directory, "joog-v1.2.3-target");
      const archive = stage + "." + extension;
      mkdirSync(stage);
      const executable = join(stage, "joog");
      writeFileSync(executable, "synthetic executable\n");
      chmodSync(executable, 0o755);
      writeFileSync(join(stage, "notes with spaces.txt"), "Unicode café\n");
      if (process.platform !== "win32") symlinkSync("joog", join(stage, "jmail"));
      archiveDirectory(stage, archive);
      assert.deepEqual(readFileSync(archive).subarray(0, 2), extension === "zip" ? Buffer.from("PK") : Buffer.from([0x1f, 0x8b]));
      assert.throws(() => archiveDirectory(stage, archive), /already exists/);
      const hash = await checksumFile(archive);
      assert.equal(hash, createHash("sha256").update(readFileSync(archive)).digest("hex"));
      const extracted = join(directory, "extracted");
      mkdirSync(extracted);
      execFileSync("tar", ["-xf", archive, "-C", extracted]);
      const contents = join(extracted, "joog-v1.2.3-target");
      assert.equal(readFileSync(join(contents, "joog"), "utf8"), "synthetic executable\n");
      assert.equal(readFileSync(join(contents, "notes with spaces.txt"), "utf8"), "Unicode café\n");
      if (process.platform !== "win32") {
        assert.equal(lstatSync(join(contents, "joog")).mode & 0o777, 0o755);
        assert.equal(readlinkSync(join(contents, "jmail")), "joog");
      }
    } finally {
      rmSync(directory, { recursive: true, force: true });
    }
  });
}
