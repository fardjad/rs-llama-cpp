#!/usr/bin/env bash
set -eu

# Mostly based on https://github.com/EmbarkStudios/cargo-about/blob/main/.github/scripts/package.sh

# When run in a container, the ownership will be messed up, so mark the
# checkout dir as safe regardless of our env
git config --global --add safe.directory "$GITHUB_WORKSPACE"

tag="$TAG"
release_name="$NAME-$tag-$TARGET"
release_tar="${release_name}.tar.gz"
mkdir "$release_name"

if [[ "$TARGET" =~ windows ]]; then
    # TODO: remove this once I figure out the name of the file
    ls target/$TARGET/release
    rlib="lib$NAME.rlib"
else
    rlib="lib$NAME.rlib"
fi

cp "target/$TARGET/release/$rlib" "$release_name/"
cp README.md LICENSE "$release_name/"
tar czf "$release_tar" "$release_name"

rm -r "$release_name"

# Windows environments in github actions don't have the gnu coreutils installed,
# which includes the shasum exe, so we just use powershell instead
if [[ "$TARGET" =~ windows ]]; then
    echo "(Get-FileHash \"${release_tar}\" -Algorithm SHA256).Hash | Out-File -Encoding ASCII -NoNewline \"${release_tar}.sha256\"" | pwsh -c -
else
    echo -n "$(shasum -ba 256 "${release_tar}" | cut -d " " -f 1)" > "${release_tar}.sha256"
fi