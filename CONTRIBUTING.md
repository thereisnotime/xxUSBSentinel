# Contributing

## Development setup

**Requirements (Linux):**
```sh
sudo apt install libusb-1.0-0-dev libxdo-dev pkg-config \
  libgtk-3-dev libxkbcommon-dev libgles2-mesa-dev \
  libwayland-dev libxrandr-dev libxi-dev libxcursor-dev
```

Install [just](https://github.com/casey/just) and a stable Rust toolchain via [rustup](https://rustup.rs).

## Common tasks

```sh
just build        # debug build
just run          # run debug build
just check        # type-check only
just clippy       # lint (all warnings as errors)
just fmt          # auto-format
just test         # run tests
just ci           # full gate: fmt-check + clippy + test
just clean        # remove build artefacts
```

## Versioning and releases

```sh
just bump-patch   # bump x.y.Z
just bump-minor   # bump x.Y.0
just bump-major   # bump X.0.0
just dist-linux   # package release binary as .tar.gz
```

Pushing a `v*` tag triggers the CI release workflow, which builds Linux and Windows binaries and publishes a GitHub release automatically.

## Pull requests

- Keep PRs focused — one change per PR.
- Run `just ci` locally before pushing.
- Match the commit message style in the existing log.
- Do not add `Co-Authored-By` trailers.

## Contributors

See [CONTRIBUTORS](CONTRIBUTORS).
