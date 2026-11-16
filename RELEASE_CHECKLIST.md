# Release checklist

This checklist is meant to be used as a guide for the `crates.io` release process.

`cargo-release` bumps the versions, commits, tags and pushes; pushing the `v<X.Y.Z>` tag runs the
[publish workflow](./.github/workflows/publish.yml), which publishes every crate to `crates.io`
with trusted publishing and creates the GitHub release.

Releases are always made in lockstep, meaning that all crates in the repository
are released with the same version number, regardless of whether they have
changed or not.

## One-time setup

Do these in order. The workflow's own checks live in the tagged commit, so these settings are what
actually gate a release.

1. Before the workflow is on `main`, create the `release` environment with required reviewers and a
   deployment tag rule for `v*`, and add a tag ruleset that restricts creating, updating and
   deleting `v*` tags to maintainers. If the workflow runs before the environment exists, GitHub
   creates `release` with no protection rules.
2. On crates.io, add a GitHub trusted publisher (owner `alloy-rs`, repo `core`, workflow
   `publish.yml`, environment `release`) to every published crate. The environment is required: the
   `verify` job builds every dependency while it holds `id-token: write`, and only the environment
   keeps its tokens from publishing.
3. After the first release goes out through CI, enable "Require trusted publishing for all new
   versions" on every crate and revoke the personal publish tokens.

## Requirements

- [cargo-release](https://github.com/crate-ci/cargo-release): `cargo install cargo-release`
- [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks): `cargo install cargo-semver-checks`
- [git-cliff](https://github.com/orhun/git-cliff) (patched with [orhun/git-cliff#711](https://github.com/orhun/git-cliff/pull/711)): `cargo install --git https://github.com/DaniPopes/git-cliff.git --branch fix-include-paths git-cliff`

## Steps

- [ ] Make sure you're on the `main` branch.
- [ ] Make sure every crate is already on crates.io with the trusted publisher above. Trusted publishing can't create a crate, so the workflow stops if one is missing. For a new crate, a crates.io owner publishes a placeholder `0.0.0` by hand and adds the trusted publisher before you tag.
- [ ] (optional) Dry run `cargo-release`: `cargo release <version>`
- [ ] Check that every crate packages and builds: `cargo publish --workspace --dry-run --locked`. With `publish = false`, `cargo release` no longer does this, and CI only checks it after the tag is pushed, when the tag can no longer be deleted.
- [ ] Run `cargo-semver-checks` for a non-breaking release: `cargo +stable semver-checks`
  - [ ] Breaking changes are not a blocker even for non-breaking release, but you must review them carefully in case of an accidental breaking change.
- [ ] Run `cargo-release`: `cargo release --execute <version>`
  - [ ] If a failure happened before pushing, you should have an unpushed commit. After the issue is fixed, retry the release process with `--no-push` and squash the commits together, then push the commit and the tag together with `git push --atomic origin main v<version>`.
- [ ] Approve the `publish` job of the [publish workflow](./.github/workflows/publish.yml) run for the tag. Before approving, check that the run's commit is the release commit on `main` and that `verify` passed: a tag on another commit runs that commit's copy of the workflow.
  - [ ] If publishing stopped partway, re-run the failed job: crates already on `crates.io` are skipped.
- [ ] Check the GitHub release the workflow created.

## Backports

The workflow only publishes tags on `main`. To release from another branch (such as a past `v0.8.x`
release), a crates.io owner publishes by hand and creates the GitHub release. Once "Require trusted
publishing" is enabled, that needs it turned off for the duration.
