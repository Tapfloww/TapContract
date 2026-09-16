use soroban_sdk::{symbol_short, Address, Env};

pub fn sponsored(env: &Env, from: &Address, to: &Address, amount: i128, fee: i128) {
    let _ = (env, from, to, amount, fee);
    // Keep event payload compact for host compatibility across SDK revisions.
    env.events()
        .publish((symbol_short!("sponsor"),), (amount, fee));
}

pub fn paused(env: &Env, value: bool) {
    env.events().publish((symbol_short!("paused"),), value);
}
