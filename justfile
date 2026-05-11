set shell := ["bash", "-cu"]

version := `cargo metadata --no-deps --format-version 1 | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])"`
bin     := "xxusbsentinel"
dist    := "dist"

# List available recipes
default:
    @just --list

# ── Setup ─────────────────────────────────────────────────────────────────────

# Install all tool versions via asdf (.tool-versions) and configure git hooks
setup:
    #!/usr/bin/env bash
    set -euo pipefail
    asdf plugin add rust 2>/dev/null || true
    asdf plugin add just 2>/dev/null || true
    asdf plugin add actionlint https://github.com/crazy-matt/asdf-actionlint.git 2>/dev/null || true
    asdf install
    just hooks
    echo "Done. Restart your shell or run: asdf reshim"

# Configure git to use the repo's hook scripts
hooks:
    git config core.hooksPath .githooks
    chmod +x .githooks/*
    @echo "Git hooks installed (.githooks/)"

# ── Build ─────────────────────────────────────────────────────────────────────

# Debug build
build:
    cargo build

# Optimised release build
release:
    cargo build --release
    @echo "Binary: target/release/{{bin}}"

# Run debug build
run *args:
    cargo run -- {{args}}

# Run release build
run-release *args:
    cargo run --release -- {{args}}

# ── Quality ───────────────────────────────────────────────────────────────────

# Type-check without producing a binary
check:
    cargo check

# Lint with Clippy
clippy:
    cargo clippy -- -D warnings

# Auto-format source
fmt:
    cargo fmt

# Check formatting without modifying files
fmt-check:
    cargo fmt -- --check

# Run tests
test:
    cargo test

# Full CI gate: fmt-check + clippy + test
ci: fmt-check clippy test

# Lint GitHub Actions workflow files
actionlint:
    actionlint .github/workflows/*.yml

# ── Maintenance ───────────────────────────────────────────────────────────────

# Remove build artefacts
clean:
    cargo clean
    rm -rf {{dist}}

# Update all dependencies to latest compatible versions
update:
    cargo update

# Security audit of dependencies (requires cargo-audit)
audit:
    cargo audit

# Show dependency tree
tree:
    cargo tree

# ── Versioning ────────────────────────────────────────────────────────────────

# Print current version
version:
    @echo "{{version}}"

# Bump patch version (x.y.Z)
bump-patch:
    #!/usr/bin/env bash
    set -euo pipefail
    old=$(cargo metadata --no-deps --format-version 1 | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])")
    IFS='.' read -r major minor patch <<< "$old"
    new="$major.$minor.$((patch+1))"
    sed -i "s/^version = \"$old\"/version = \"$new\"/" Cargo.toml
    echo "Bumped $old → $new"

# Bump minor version (x.Y.0)
bump-minor:
    #!/usr/bin/env bash
    set -euo pipefail
    old=$(cargo metadata --no-deps --format-version 1 | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])")
    IFS='.' read -r major minor patch <<< "$old"
    new="$major.$((minor+1)).0"
    sed -i "s/^version = \"$old\"/version = \"$new\"/" Cargo.toml
    echo "Bumped $old → $new"

# Bump major version (X.0.0)
bump-major:
    #!/usr/bin/env bash
    set -euo pipefail
    old=$(cargo metadata --no-deps --format-version 1 | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])")
    IFS='.' read -r major minor patch <<< "$old"
    new="$((major+1)).0.0"
    sed -i "s/^version = \"$old\"/version = \"$new\"/" Cargo.toml
    echo "Bumped $old → $new"

# ── Distribution ─────────────────────────────────────────────────────────────

# Build release and package as a .tar.gz (Linux)
dist-linux: release
    #!/usr/bin/env bash
    set -euo pipefail
    ver=$(cargo metadata --no-deps --format-version 1 | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])")
    arch=$(uname -m)
    pkg="{{bin}}-v${ver}-linux-${arch}"
    mkdir -p {{dist}}/"$pkg"
    cp target/release/{{bin}} {{dist}}/"$pkg"/
    cp README.md LICENSE {{dist}}/"$pkg"/ 2>/dev/null || true
    tar -czf {{dist}}/"$pkg".tar.gz -C {{dist}} "$pkg"
    rm -rf {{dist}}/"$pkg"
    echo "Package: {{dist}}/${pkg}.tar.gz"

# Install the release binary to ~/.local/bin
install: release
    install -Dm755 target/release/{{bin}} ~/.local/bin/{{bin}}
    @echo "Installed to ~/.local/bin/{{bin}}"

# Uninstall from ~/.local/bin
uninstall:
    rm -f ~/.local/bin/{{bin}}
    @echo "Removed ~/.local/bin/{{bin}}"

# ── Release ───────────────────────────────────────────────────────────────────

# Tag the current commit with the version from Cargo.toml and push — triggers CI release
tag:
    #!/usr/bin/env bash
    set -euo pipefail
    ver=$(cargo metadata --no-deps --format-version 1 | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])")
    tag="v${ver}"
    if git rev-parse "$tag" >/dev/null 2>&1; then
        echo "Tag $tag already exists — bump the version first (just bump-patch / bump-minor / bump-major)"
        exit 1
    fi
    git tag "$tag"
    git push origin "$tag"
    echo "Pushed tag $tag — CI release workflow started"

# Bump patch version, commit, tag, and push — full release in one step
release-patch: bump-patch
    #!/usr/bin/env bash
    set -euo pipefail
    ver=$(cargo metadata --no-deps --format-version 1 | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])")
    git add Cargo.toml Cargo.lock
    git commit -m "chore: bump version to ${ver}"
    git push origin HEAD
    just tag

# Bump minor version, commit, tag, and push
release-minor: bump-minor
    #!/usr/bin/env bash
    set -euo pipefail
    ver=$(cargo metadata --no-deps --format-version 1 | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])")
    git add Cargo.toml Cargo.lock
    git commit -m "chore: bump version to ${ver}"
    git push origin HEAD
    just tag

# Bump major version, commit, tag, and push
release-major: bump-major
    #!/usr/bin/env bash
    set -euo pipefail
    ver=$(cargo metadata --no-deps --format-version 1 | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])")
    git add Cargo.toml Cargo.lock
    git commit -m "chore: bump version to ${ver}"
    git push origin HEAD
    just tag
