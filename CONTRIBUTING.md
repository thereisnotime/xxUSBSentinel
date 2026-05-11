# Contributing

## Development setup

### System dependencies (Linux)

```sh
sudo apt install libusb-1.0-0-dev libxdo-dev pkg-config \
  libgtk-3-dev libxkbcommon-dev libgles2-mesa-dev \
  libwayland-dev libxrandr-dev libxi-dev libxcursor-dev
```

### Toolchain

Install tools with [asdf](https://asdf-vm.com) (versions in `.tool-versions`):

```sh
just setup   # installs all tools via asdf
```

Or manually:
- [rustup](https://rustup.rs) — stable Rust toolchain
- [just](https://github.com/casey/just) — task runner

## All just targets

```
just setup          # install all tools via asdf (.tool-versions)

# Build
just build          # debug build
just release        # optimised release build
just run [args]     # run debug build
just run-release    # run release build

# Quality
just check          # type-check without producing a binary
just clippy         # lint (all warnings as errors)
just fmt            # auto-format source
just fmt-check      # check formatting without modifying files
just test           # run tests
just ci             # full gate: fmt-check + clippy + test

# Maintenance
just clean          # remove build artefacts and dist/
just update         # update dependencies to latest compatible versions
just audit          # security audit (requires cargo-audit)
just tree           # show dependency tree

# Versioning
just version        # print current version from Cargo.toml
just bump-patch     # bump x.y.Z
just bump-minor     # bump x.Y.0
just bump-major     # bump X.0.0

# Distribution
just dist-linux     # package release binary as .tar.gz
just install        # install binary to ~/.local/bin
just uninstall      # remove binary from ~/.local/bin

# Release
just tag            # tag current commit with Cargo.toml version and push
just release-patch  # bump-patch + commit + tag + push
just release-minor  # bump-minor + commit + tag + push
just release-major  # bump-major + commit + tag + push
```

## Pull requests

- Keep PRs focused — one change per PR.
- Run `just ci` locally before pushing.
- Match the commit message style in the existing log.
- Do not add `Co-Authored-By` trailers.

## Versioning and releases

Pushing a `v*` tag triggers the release workflow, which builds Linux (binary + AppImage) and Windows (binary + Chocolatey package) and publishes a GitHub release automatically.

## Verifying release artifacts

Each release artifact is signed with [cosign](https://github.com/sigstore/cosign) (keyless, via GitHub OIDC) and has a SLSA provenance attestation. A `checksums.sha256` file is included in every release.

Verify a checksum:
```sh
sha256sum --check checksums.sha256
```

Verify the cosign signature:
```sh
cosign verify-blob \
  --bundle <artifact>.bundle \
  <artifact>
```

Verify the GitHub provenance attestation:
```sh
gh attestation verify <artifact> --repo thereisnotime/xxUSBSentinel
```

## Contributors

See [CONTRIBUTORS](CONTRIBUTORS).
