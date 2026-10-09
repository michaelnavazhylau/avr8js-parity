# avr8js-parity

The converted **AVR8js behavioral contract** for
[`avr8rs`](https://github.com/michaelnavazhylau/arduino-sim-rs): all **14
upstream AVR8js test suites**, **347 test cases** and **853 assertion sites**,
lowered from TypeScript into typed Rust scenarios — together with the converter
that produces them and the pinned upstream revision they are asserted against.

The simulator that satisfies this contract is deliberately **not** here. It is
the `avr8rs` crate, which lives in `rust_port/` of
[`arduino-sim-rs`](https://github.com/michaelnavazhylau/arduino-sim-rs). This
repository exists so 27k lines of machine-written scenarios, bound to one
upstream commit, stop sharing a history and a lockfile with the hand-written
engine.

## Layout

| Path | What it is |
| --- | --- |
| `src/suites/` | Generated Rust scenarios, one module per upstream suite |
| `src/lib.rs` | Re-exports the engine API under the names the generated code expects |
| `tools/convert.cjs` | The TypeScript-AST converter, and the only writer of `src/suites/` |
| `conversion-manifest.json` | Per-source SHA-256 plus the case and assertion inventory |
| `avr8js/` | Upstream reference, submodule pinned at `bee6f0a94e0e27786f6bc21aee3c775849fb50fd` |
| `tests/contract.rs` | Structural coverage: every case present, uniquely named, counted |

## Running the contract

`avr8rs` is consumed as a **sibling path dependency** on purpose — see
[Release rule](#release-rule) below — so check the engine out next to this
repository:

```sh
git clone https://github.com/michaelnavazhylau/arduino-sim-rs
git clone --recurse-submodules https://github.com/michaelnavazhylau/avr8js-parity
cd avr8js-parity
bash tools/verify-parity.sh
```

`verify-parity.sh` is fail-closed. It refuses to run while any generated suite
is `#[ignore]`d — a green run may never imply coverage that does not exist —
then runs `cargo fmt --check`, `clippy --all-targets -D warnings` and the whole
contract in debug and release. Node, TypeScript and the upstream submodule are
not needed for it.

## Release rule

[![avr8rs on crates.io](https://img.shields.io/crates/v/avr8rs.svg)](https://crates.io/crates/avr8rs)

The engine is published as [`avr8rs`](https://crates.io/crates/avr8rs), and
this contract is what justifies a release. Publish a new `avr8rs` version
**only from a commit where this repository's gate is green against that commit**,
so "released" means "contract-verified" rather than "presumed compatible".

That is also why the dependency stays a path dependency. A registry-only pin
could only ever test *released* engine versions, which would turn this gate into
a lagging indicator: a break would surface after a release instead of on the
engine's next commit. Today the two are equivalent — each published version was
built from a commit this gate passed against.

| `avr8rs` release | Published from | Gate green at that commit |
| --- | --- | --- |
| 0.1.0 | `42e1436` | yes |

`avr-sim` was an earlier placeholder name for the same crate. `avr-sim 0.1.0`
is yanked, so new dependents resolve `avr8rs` only.

## Regenerating

```sh
npm --prefix tools ci --ignore-scripts
npm --prefix tools run check      # verify without writing
npm --prefix tools run generate   # rewrite src/suites/ and the manifest
```

`check` re-derives every scenario from the pinned submodule, formats it with
`rustfmt`, and compares byte-for-byte. It also fails on a generated module that
the upstream no longer produces, so a green check means `src/suites/` is exactly
what that upstream revision yields — not merely close to it. Any change to
`conversion-manifest.json` is therefore either a converter change or an upstream
revision bump, and both are reviewable as data.

The converter is AST-only: it lowers Vitest `describe`/`it`/`expect` constructs
explicitly and throws on anything it does not model, rather than passing unknown
syntax through as an unresolved backend call.

## Boundaries

- **This repository owns** the converted contract, the converter, the pinned
  upstream revision, and the deterministic-conversion check.
- **`arduino-sim-rs` owns** the `avr8rs` engine, the scenario IR and scenario
  runner (`scenario.rs`, `runtime.rs`), the AVR hardware reference bundle
  (`specs/`, vendor datasheets by provenance only) and the implementation plan.
- The scenario IR and its runner stayed with the engine rather than moving here.
  The runner's expression evaluator is shared between the `Case`-driven path and
  the callback path that `breadboard` and `blink-gui` drive, so extracting it
  would have cut through the engine's `Backend`/`Runtime` contract for no
  functional gain. Both files are dependency-free, so the engine remains
  publishable. Splitting them is possible only after that evaluator is factored
  apart, and is not attempted here.

## Provenance

Derived from [AVR8js](https://github.com/wokwi/avr8js) by Uri Shaked and
contributors, pinned at the revision above and licensed MIT. Generated files
carry an SPDX header identifying the upstream copyright; see
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) and [LICENSE](LICENSE).

This is an independent reimplementation, not a fork: the upstream TypeScript is
read only as a specification, and `avr8js/` is referenced but not redistributed
in any produced artifact.
