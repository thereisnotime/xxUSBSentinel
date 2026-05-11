set shell := ["bash", "-cu"]

version := `cargo metadata --no-deps --format-version 1 | python3 -c "import sys,json; print(json.load(sys.stdin)['packages'][0]['version'])"`
bin     := "xxusbsentinel"
dist    := "dist"

# List available recipes
default:
    @just --list

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
    cp README.md {{dist}}/"$pkg"/ 2>/dev/null || true
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
