use soroban_sdk::{contracttype, Address};

pub const VERSION: u32 = 2;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    Version,
    Paused,
    TxCount,
    TotalFees,
    MaxFee,
    PolicyBps,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub struct SponsorshipRecord {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
    pub fee: i128,
    pub timestamp: u64,
}
