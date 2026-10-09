// SPDX-License-Identifier: MIT

//! Structural coverage of the converted contract itself, independent of whether
//! the engine currently satisfies it: every upstream case is present, uniquely
//! named, and its recorded assertion count matches the steps actually lowered.

use avr8js_parity::scenario::assertion_count;
use avr8js_parity::suites;
use std::collections::BTreeSet;

#[test]
fn all_upstream_cases_and_assertions_are_present() {
    let cases = suites::all_cases();
    assert_eq!(cases.len(), 347);
    assert_eq!(cases.iter().map(|c| c.assertions).sum::<usize>(), 853);
    let mut names = BTreeSet::new();
    let mut sources = BTreeSet::new();
    for case in cases {
        assert!(
            names.insert((case.source, case.name)),
            "duplicate case {}",
            case.name
        );
        sources.insert(case.source);
        assert!(case.line > 0);
        assert!(!case.body.is_empty());
        assert_eq!(
            assertion_count(&case.body),
            case.assertions,
            "{}",
            case.name
        );
        assert_eq!(
            assertion_count(&case.setup),
            0,
            "unexpected assertion in setup"
        );
    }
    assert_eq!(sources.len(), 14);
}
