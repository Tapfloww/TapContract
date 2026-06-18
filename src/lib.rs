#![no_std]
use soroban_sdk::{contract, contractimpl, Symbol, Env, Address, Map};

#[contract]
pub struct SponsorshipContract;

#[contractimpl]
impl SponsorshipContract {
    /// Initialize the sponsor vault
    pub fn init(env: Env, admin: Address) -> bool {
        let admin_sym = Symbol::short("admin");
        env.storage().persistent().set(&admin_sym, &admin);
        true
    }

    /// Get current admin
    pub fn get_admin(env: Env) -> Address {
        let admin_sym = Symbol::short("admin");
        env.storage().persistent().get(&admin_sym).unwrap()
    }

    /// Sponsor a transaction
    pub fn sponsor_payment(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
        fee: i128,
        asset: Symbol,
    ) -> bool {
        from.require_auth();

        // In production: transfer fee from sponsor account
        // For MVP: just log the sponsorship

        let tx_count_key = Symbol::short("tx_count");
        let current_count: i128 = env.storage()
            .persistent()
            .get(&tx_count_key)
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&tx_count_key, &(current_count + 1));

        true
    }

    /// Get total sponsored transactions
    pub fn get_tx_count(env: Env) -> i128 {
        let tx_count_key = Symbol::short("tx_count");
        env.storage()
            .persistent()
            .get(&tx_count_key)
            .unwrap_or(0)
    }

    /// Get total fees collected
    pub fn get_total_fees(env: Env) -> i128 {
        let fees_key = Symbol::short("total_fees");
        env.storage()
            .persistent()
            .get(&fees_key)
            .unwrap_or(0)
    }
}
