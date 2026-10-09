// SPDX-License-Identifier: MIT

//! Converted AVR8js behavioral contract.
//!
//! Every scenario under [`suites`] is generated from the pinned `avr8js`
//! submodule by `tools/convert.cjs`. This crate carries the contract, the
//! converter and the upstream revision; the simulator that satisfies it lives
//! in the `avr8rs` crate.
//!
//! The re-exports below are named to match what the generated modules already
//! expect — `crate::Case`, `crate::run_native` and `crate::scenario::*` — so
//! regeneration stays a pure data operation and the generator needs to know
//! nothing about where the engine lives.

pub use avr8rs::scenario;
pub use avr8rs::{runtime, Backend, Case, Runtime, Value};

pub mod suites;

/// Run one converted scenario against the native backend.
pub fn run_native(case: Case) {
    Runtime::new(avr8rs::native_backend()).run(&case);
}
