#!/usr/bin/env bash
set -euo pipefail

# Lists the workspace crates to publish to crates.io for a release tag and checks
# their manifest versions against it. Prints `packages=...` for `$GITHUB_OUTPUT`.
#
# - `v1.2.3` releases the workspace's publishable crates at 1.2.3. With
#   LOCKSTEP=true (the default) every publishable crate must be 1.2.3.
# - `<crate>-v1.2.3` releases only that crate, which must be 1.2.3.

TAG="${1:?usage: release-packages.sh <tag>}"
LOCKSTEP="${LOCKSTEP:-true}"

metadata="$(cargo metadata --format-version 1 --no-deps --locked)"
# `publish` is null when unrestricted, or a list of allowed registries.
publishable="$(jq -r '.packages[] | select(.publish == null or (.publish | index("crates-io"))) | .name' <<<"$metadata")"
if [[ -z "$publishable" ]]; then
    echo "error: no publishable packages in the workspace" >&2
    exit 1
fi

version_of() {
    jq -r --arg name "$1" '.packages[] | select(.name == $name) | .version' <<<"$metadata"
}

if [[ "$TAG" =~ ^v([0-9]+\.[0-9]+\.[0-9]+.*)$ ]]; then
    VERSION="${BASH_REMATCH[1]}"
    if [[ "$LOCKSTEP" == "true" ]]; then
        packages="$publishable"
    else
        packages=""
        for package in $publishable; do
            if [[ "$(version_of "$package")" == "$VERSION" ]]; then
                packages+="$package"$'\n'
            fi
        done
        packages="${packages%$'\n'}"
        if [[ -z "$packages" ]]; then
            echo "error: no publishable crate is at ${VERSION} for tag ${TAG}" >&2
            exit 1
        fi
    fi
elif [[ "$TAG" =~ ^(.+)-v([0-9]+\.[0-9]+\.[0-9]+.*)$ ]]; then
    packages="${BASH_REMATCH[1]}"
    VERSION="${BASH_REMATCH[2]}"
    if ! grep -qxF -- "$packages" <<<"$publishable"; then
        echo "error: tag ${TAG} names ${packages}, which is not a publishable crate in this workspace" >&2
        exit 1
    fi
else
    echo "error: '${TAG}' is not a release tag like v1.2.3 or <crate>-v1.2.3" >&2
    exit 1
fi

for package in $packages; do
    actual="$(version_of "$package")"
    if [[ "$actual" != "$VERSION" ]]; then
        echo "error: ${package} is ${actual}, but tag ${TAG} releases ${VERSION}" >&2
        exit 1
    fi
done

echo "packages=$(paste -sd ' ' - <<<"$packages")"
