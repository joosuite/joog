# Publish a Rust CLI release

Use this procedure for the suite's Rust CLIs. Publish both tagged source archives
and ready-to-run binaries through the GitHub CLI. Work on the repository's release
branch, create no temporary branches, and add no co-author attribution.

Run the blocks from the selected CLI's repository root in the same Bash or Zsh
session. You need Git, authenticated `gh`, Rust, Cargo, `jq`, `tar`, `unzip`, and
`shasum`. Install other development tools only when that project's checks need
them. Release users need neither Rust nor the packaging tools to run the binaries.

## 1. Set the project settings

Inspect `Cargo.toml`, the CI workflow, and the artifact workflow. Replace the
package and binary placeholders below. Adjust the workflow names, archive prefix,
and target list to match the project.

```sh
set -euo pipefail
REMOTE=origin
REPO=$(gh repo view --json nameWithOwner --jq .nameWithOwner)
RELEASE_BRANCH=$(gh repo view "$REPO" --json defaultBranchRef --jq .defaultBranchRef.name)

PACKAGE=CARGO_PACKAGE_NAME
BINARY=CLI_BINARY_NAME
ARCHIVE_PREFIX="$BINARY"
CI_WORKFLOW=rust.yml
BUILD_WORKFLOW=release.yml
ARTIFACT_PREFIX=release-
PRERELEASE=false
MAKE_LATEST=true

TARGETS=(
	x86_64-unknown-linux-gnu
	aarch64-unknown-linux-gnu
	x86_64-apple-darwin
	aarch64-apple-darwin
	x86_64-pc-windows-msvc
)

git remote -v
gh auth status
printf 'Repository: %s\nRelease branch: %s\n' "$REPO" "$RELEASE_BRANCH"
```

Confirm that `REPO` matches the intended Git remote. Use a different
`RELEASE_BRANCH` only when the project releases from another branch.
For a workspace, select the package that owns the CLI binary rather than assuming
the first package returned by Cargo is the release package.

The commands below use the shared artifact layout:

- One Actions artifact named `release-TARGET` per supported target.
- One `PREFIX-vVERSION-TARGET.tar.gz` archive per Unix target, or `.zip` on Windows.
- A matching `ARCHIVE.sha256` file beside each archive.
- `BUILD-INFO.json` inside the archive's top-level package directory, containing
  `version`, `tag`, `target`, and `commit`.

Confirm that the artifact workflow follows this layout and accepts a `tag` input
for manual dispatch. It must build the requested tag and check its package version.
Set `TARGETS` to its full supported matrix before starting the release.
For a preview, set `PRERELEASE=true` and `MAKE_LATEST=false`. For a stable
maintenance release that must not replace the latest version, set `MAKE_LATEST=false`.

## 2. Prepare and check the source

Start with a clean checkout on the release branch. Resolve local changes before
continuing. After a repository move, update repository links in package metadata,
documentation, and issue templates.

```sh
git status --short --branch
test "$(git branch --show-current)" = "$RELEASE_BRANCH"
test -z "$(git status --porcelain)"
git fetch "$REMOTE" "$RELEASE_BRANCH" --tags
git merge --ff-only "$REMOTE/$RELEASE_BRANCH"
```

For a new version, update the selected package's version and `Cargo.lock`.
If the package inherits a workspace version, update the workspace declaration.
Commit the release changes on the release branch before creating the tag.

Run the project's required checks. A standard Rust baseline is:

```sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
git diff --check
test -z "$(git status --porcelain)"
```

Match the feature combinations required by the project's CI. Run its schema,
generated-reference, credential-store, and dependency-audit checks where defined.
Install their development dependencies when needed. Keep live service tests
separate from this procedure; they require explicit account authorization.

## 3. Create and push the version tag

Derive the version from the selected Cargo package and record the exact commit.

```sh
VERSION=$(cargo metadata --locked --no-deps --format-version 1 |
	jq -er --arg package "$PACKAGE" \
	'.packages[] | select(.name == $package) | .version')
TAG="v$VERSION"
COMMIT=$(git rev-parse HEAD)

gh release list --repo "$REPO"
git tag -l "$TAG"
```

If the tag exists, follow **Rebuild an existing tag** instead. Otherwise, push the
release branch and annotated tag together.

```sh
git tag -a "$TAG" -m "$PACKAGE $TAG"
git push --atomic "$REMOTE" "$RELEASE_BRANCH" "$TAG"
```

Confirm that the branch push starts CI and the tag push starts artifact builds.
The shared artifact workflow builds packages without publishing a GitHub release.
If a project's workflow also publishes, use that automated publishing path rather
than creating a duplicate release in step 6.

### Rebuild an existing tag

Keep the tag at its original commit. Select the version to rebuild and dispatch
the artifact workflow.

```sh
TAG=v0.1.0
VERSION=${TAG#v}
COMMIT=$(git rev-list -n 1 "$TAG")
gh workflow run "$BUILD_WORKFLOW" --repo "$REPO" \
	--ref "$RELEASE_BRANCH" -f tag="$TAG"
```

Use successful CI for that exact source commit. After an infrastructure failure,
repeat a failed run with `gh run rerun RUN_ID --repo "$REPO" --failed`.
If a source fix is necessary, commit it and release a new version. Preserve
published tags, binaries, and checksums.

## 4. Wait for CI and every package

Allow time for newly pushed runs to appear, then select the matching runs.

```sh
gh run list --repo "$REPO" --workflow "$CI_WORKFLOW" --commit "$COMMIT" --limit 20
gh run list --repo "$REPO" --workflow "$BUILD_WORKFLOW" --limit 20
```

Replace the run-ID placeholders below. For a new tag, select its artifact run.
For a rebuild, select the manual dispatch. A manual run lists the branch that
supplies its workflow definition; the package source must still come from `TAG`.

```sh
CI_RUN=CI_RUN_ID
BUILD_RUN=ARTIFACT_RUN_ID

gh run watch "$CI_RUN" --repo "$REPO" --interval 30 --exit-status
gh run watch "$BUILD_RUN" --repo "$REPO" --interval 30 --exit-status
```

Require every expected CI check and every configured target to succeed.
Inspect failures with `gh run view RUN_ID --repo "$REPO" --log-failed`.
Fix the cause before publishing. Do not skip a failed test or remove a supported
target to make the release pass.

## 5. Download and verify the packages

Use a fresh output directory for each artifact run. Download before the retention
period configured in the artifact workflow expires.

```sh
OUT="$PWD/target/releases/$TAG-$BUILD_RUN"
mkdir -p "$OUT"
gh run download "$BUILD_RUN" --repo "$REPO" \
	--pattern "$ARTIFACT_PREFIX*" --dir "$OUT"
```

Wait for the download command to finish. Build the expected file list from the
configured targets, then verify each checksum and build identity.

```sh
ARCHIVES=()
: > "$OUT/SHA256SUMS"
for target in "${TARGETS[@]}"; do
	case "$target" in
		*-windows-*) extension=zip ;;
		*) extension=tar.gz ;;
	esac
	package_dir="$ARCHIVE_PREFIX-$TAG-$target"
	archive_name="$package_dir.$extension"
	artifact_dir="$OUT/$ARTIFACT_PREFIX$target"
	archive="$artifact_dir/$archive_name"
	test -f "$archive"
	(
		cd "$artifact_dir"
		tr -d '\r' < "$archive_name.sha256" | shasum -a 256 -c -
	)
	tr -d '\r' < "$archive.sha256" >> "$OUT/SHA256SUMS"
	if [ "$extension" = zip ]; then
		build_info=$(unzip -p "$archive" "$package_dir/BUILD-INFO.json")
	else
		build_info=$(tar -xOf "$archive" "$package_dir/BUILD-INFO.json")
	fi
	printf '%s\n' "$build_info" |
		jq -e --arg tag "$TAG" --arg commit "$COMMIT" \
		--arg target "$target" --arg version "$VERSION" \
		'.tag == $tag and .commit == $commit and .target == $target and .version == $version'
	ARCHIVES+=("$archive")
done
test "$(wc -l < "$OUT/SHA256SUMS")" -eq "${#TARGETS[@]}"
```

Windows checksum files can use CRLF line endings. The commands normalize them
before verification.

Confirm that each package includes its CLI executable, license, required runtime
files, and documentation. Include wrappers and schemas when the project supplies
them. Require the workflow to smoke-test the binary's `--version` and packaged
entry points on their native runners. Check Unix executable permissions and
relative wrapper symlinks. Check Windows executable names and required DLLs.

## 6. Prepare the draft and upload the assets

Write `$OUT/RELEASE-NOTES.md` with the changes, platform download links, checksum
link, and setup instructions. Use the selected repository and tag in every link.
List actual runtime prerequisites, including any external libraries or OS services.

Inspect any existing release for this tag before changing it. If none exists,
create a draft, then upload the verified archives and combined checksums.

```sh
gh release create "$TAG" --repo "$REPO" --verify-tag --draft \
	--title "$BINARY $TAG" --notes-file "$OUT/RELEASE-NOTES.md"

gh release upload "$TAG" --repo "$REPO" \
	"${ARCHIVES[@]}" "$OUT/SHA256SUMS"
```

Compare GitHub's uploaded asset digests with the local files. The expected set
contains one archive per target plus `SHA256SUMS`.

```sh
{
	cat "$OUT/SHA256SUMS"
	shasum -a 256 "$OUT/SHA256SUMS" | awk '{print $1 "  SHA256SUMS"}'
} | sort > "$OUT/expected-assets.sha256"

gh release view "$TAG" --repo "$REPO" --json assets \
	--jq '.assets[] | select(.state == "uploaded") | "\(.digest | ltrimstr("sha256:"))  \(.name)"' |
	sort > "$OUT/uploaded-assets.sha256"

diff -u "$OUT/expected-assets.sha256" "$OUT/uploaded-assets.sha256"
```

After an upload failure, inspect the release before retrying. Upload only missing
files. Use `--clobber` only to replace a verified file in a draft.

## 7. Publish the draft

Confirm that the release is a draft and both workflows succeeded.

```sh
test "$(gh release view "$TAG" --repo "$REPO" --json isDraft --jq .isDraft)" = true
test "$(gh run view "$CI_RUN" --repo "$REPO" --json conclusion --jq .conclusion)" = success
test "$(gh run view "$BUILD_RUN" --repo "$REPO" --json conclusion --jq .conclusion)" = success

RELEASE_ID=$(gh release view "$TAG" --repo "$REPO" --json databaseId --jq .databaseId)
gh api --method PATCH "repos/$REPO/releases/$RELEASE_ID" \
	-H 'X-GitHub-Api-Version: 2022-11-28' \
	-F draft=false -F prerelease="$PRERELEASE" -f make_latest="$MAKE_LATEST" \
	-f tag_name="$TAG" -f target_commitish="$COMMIT" \
	--jq '{draft, tag_name, published_at, html_url}'
```

After an API error, read the release state before another attempt. Keep the
existing draft and assets while resolving the failure.

## 8. Verify the published release

```sh
gh release view "$TAG" --repo "$REPO" \
	--json isDraft,isPrerelease,publishedAt,url,tagName,assets,zipballUrl,tarballUrl

gh release download "$TAG" --repo "$REPO" --pattern SHA256SUMS \
	--dir "$OUT/published-verification"
cmp "$OUT/SHA256SUMS" "$OUT/published-verification/SHA256SUMS"
git status --short --branch
```

Confirm that `isDraft` is `false`, `publishedAt` is set, and every expected asset
is present. For a new current stable release, confirm that
`gh api "repos/$REPO/releases/latest" --jq .tag_name` returns the intended tag.

GitHub adds **Source code (zip)** and **Source code (tar.gz)** automatically.
Those downloads contain source files. The platform archives contain the compiled
executables. Share the published release URL after verifying both kinds of download.
