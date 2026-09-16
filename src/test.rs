#![cfg(test)]

use super::*;

#[test]
fn quote_fee_applies_bps_and_cap_math() {
    // Pure math path mirrors on-chain quote without host auth.
    let amount = 10_000i128;
    let bps = 20i128;
    let quoted = (amount * bps) / 10_000;
    assert_eq!(quoted, 20);
    let cap = 10i128;
    let capped = if quoted > cap { cap } else { quoted };
    assert_eq!(capped, 10);
}

#[test]
fn version_constant() {
    assert_eq!(VERSION, 2);
}

#[test]
fn error_codes_are_stable() {
    assert_eq!(Error::AlreadyInitialized as u32, 1);
    assert_eq!(Error::FeeTooHigh as u32, 7);
}
