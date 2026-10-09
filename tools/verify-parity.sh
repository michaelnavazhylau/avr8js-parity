#!/usr/bin/env bash
# Fail-closed gate for the converted AVR8js contract. Requires a sibling
# `avr8rs` checkout for the `avr8rs` path dependency, but not Node and
# not the upstream submodule: the deterministic conversion check is a separate
# step, because it hashes upstream TypeScript rather than running the contract.
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

# The contract must never silently shrink: a generated suite that is `#[ignore]`d
# would let a green run imply coverage that does not exist.
test -d src/suites
if grep -Rn '^#\[ignore' src/suites; then
  echo 'The parity contract requires every converted scenario to be enabled.' >&2
  exit 1
else
  status=$?
  if [ "$status" -ne 1 ]; then
    echo 'Could not inspect converted scenario gates.' >&2
    exit "$status"
  fi
fi

cargo fmt --check
cargo clippy --all-targets --locked --offline -- -D warnings
cargo test --all-targets --locked --offline -- --include-ignored
cargo test --all-targets --release --locked --offline -- --include-ignored
