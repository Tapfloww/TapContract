#![no_std]

mod errors;
mod events;
mod storage;

use errors::Error;
use storage::{DataKey, VERSION};
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol};

pub use errors::Error as SponsorshipError;
pub use storage::VERSION as CONTRACT_VERSION;

fn require_init(env: &Env) -> Result<(), Error> {
    if env.storage().instance().has(&DataKey::Admin) {
        Ok(())
    } else {
        Err(Error::NotInitialized)
    }
}

fn require_admin(env: &Env) -> Result<Address, Error> {
    require_init(env)?;
    let admin: Address = env
        .storage()
        .instance()
        .get(&DataKey::Admin)
        .ok_or(Error::NotInitialized)?;
    admin.require_auth();
    Ok(admin)
}

fn require_unpaused(env: &Env) -> Result<(), Error> {
    let paused: bool = env
        .storage()
        .instance()
        .get(&DataKey::Paused)
        .unwrap_or(false);
    if paused {
        Err(Error::Paused)
    } else {
        Ok(())
    }
}

#[contract]
pub struct SponsorshipContract;

#[contractimpl]
impl SponsorshipContract {
    /// Initialize the sponsor vault with an admin and fee policy.
    pub fn initialize(env: Env, admin: Address, max_fee: i128, policy_bps: i128) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        if max_fee <= 0 || policy_bps < 0 || policy_bps > 10_000 {
            return Err(Error::InvalidPolicy);
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Version, &VERSION);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.storage().instance().set(&DataKey::MaxFee, &max_fee);
        env.storage().instance().set(&DataKey::PolicyBps, &policy_bps);
        env.storage().instance().set(&DataKey::TxCount, &0i128);
        env.storage().instance().set(&DataKey::TotalFees, &0i128);
        env.storage().instance().extend_ttl(100_000, 100_000);
        Ok(())
    }

    pub fn version(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::Version)
            .unwrap_or(0)
    }

    pub fn get_admin(env: Env) -> Result<Address, Error> {
        require_init(&env)?;
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)
    }

    pub fn is_paused(env: Env) -> bool {
        env.storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false)
    }

    pub fn pause(env: Env) -> Result<(), Error> {
        require_admin(&env)?;
        env.storage().instance().set(&DataKey::Paused, &true);
        events::paused(&env, true);
        Ok(())
    }

    pub fn unpause(env: Env) -> Result<(), Error> {
        require_admin(&env)?;
        env.storage().instance().set(&DataKey::Paused, &false);
        events::paused(&env, false);
        Ok(())
    }

    pub fn set_policy(env: Env, max_fee: i128, policy_bps: i128) -> Result<(), Error> {
        require_admin(&env)?;
        if max_fee <= 0 || policy_bps < 0 || policy_bps > 10_000 {
            return Err(Error::InvalidPolicy);
        }
        env.storage().instance().set(&DataKey::MaxFee, &max_fee);
        env.storage().instance().set(&DataKey::PolicyBps, &policy_bps);
        Ok(())
    }

    pub fn max_fee(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::MaxFee)
            .unwrap_or(0)
    }

    pub fn policy_bps(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::PolicyBps)
            .unwrap_or(0)
    }

    /// Suggested fee for an amount under current policy.
    pub fn quote_fee(env: Env, amount: i128) -> Result<i128, Error> {
        require_init(&env)?;
        if amount <= 0 {
            return Err(Error::ZeroAmount);
        }
        let bps = Self::policy_bps(env.clone());
        let quoted = (amount * bps) / 10_000;
        let cap = Self::max_fee(env);
        Ok(if quoted > cap { cap } else { quoted })
    }

    /// Record a sponsored payment. Caller (`from`) must auth.
    /// Fee must be > 0, <= max_fee, and amount must be positive.
    pub fn sponsor_payment(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
        fee: i128,
        _asset: Symbol,
    ) -> Result<(), Error> {
        require_init(&env)?;
        require_unpaused(&env)?;
        from.require_auth();
        if amount <= 0 {
            return Err(Error::ZeroAmount);
        }
        if fee <= 0 {
            return Err(Error::ZeroFee);
        }
        let cap = Self::max_fee(env.clone());
        if fee > cap {
            return Err(Error::FeeTooHigh);
        }

        let tx_count: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TxCount)
            .unwrap_or(0);
        let total_fees: i128 = env
            .storage()
            .instance()
            .get(&DataKey::TotalFees)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::TxCount, &(tx_count + 1));
        env.storage()
            .instance()
            .set(&DataKey::TotalFees, &(total_fees + fee));

        events::sponsored(&env, &from, &to, amount, fee);
        Ok(())
    }

    pub fn get_tx_count(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TxCount)
            .unwrap_or(0)
    }

    pub fn get_total_fees(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&DataKey::TotalFees)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
