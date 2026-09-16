# TapFlow Contract

Soroban **fee sponsorship** vault for TapFlow on Stellar.

Apps sponsor end-user transaction fees so users can interact without holding the fee asset. This crate is the on-chain policy + accounting layer.

## v2

- `initialize(admin, max_fee, policy_bps)` with typed errors
- Admin `pause` / `unpause` and `set_policy`
- `quote_fee(amount)` for BPS fee with max cap
- `sponsor_payment` requires auth, positive amount/fee, rejects `FeeTooHigh`
- Tracks `tx_count` and `total_fees`; emits sponsorship events
- Unit tests + GitHub Actions (`cargo test`, wasm build)

```bash
cargo test
cargo build --target wasm32-unknown-unknown --release
```

Never commit private keys or deploy secrets.
