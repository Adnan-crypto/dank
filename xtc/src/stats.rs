use crate::history::HistoryBuffer;
use ic_cdk::export::candid::{CandidType, Nat};
use ic_cdk::*;
use ic_cdk_macros::*;
use serde::Deserialize;

#[derive(Deserialize, CandidType, Clone)]
pub struct StatsDataV0 {
    supply: Nat,
    history_events: u64,
    balance: u64,
    // Usage statistics
    transfers_count: u64,
    mints_count: u64,
    burns_count: u64,
    proxy_calls_count: u64,
    canisters_created_count: u64,
}

#[derive(Deserialize, CandidType, Clone)]
pub struct StatsData {
    supply: Nat,
    fee: Nat,
    history_events: u64,
    balance: u64,
    // Usage statistics
    transfers_count: u64,
    mints_count: u64,
    burns_count: u64,
    proxy_calls_count: u64,
    canisters_created_count: u64,
}

impl Default for StatsData {
    fn default() -> Self {
        Self {
            supply: Nat::from(0),
            fee: Nat::from(0),
            history_events: 0,
            balance: 0,
            transfers_count: 0,
            mints_count: 0,
            burns_count: 0,
            proxy_calls_count: 0,
            canisters_created_count: 0,
        }
    }
}

impl From<StatsDataV0> for StatsData {
    fn from(v0: StatsDataV0) -> Self {
        Self {
            supply: v0.supply,
            fee: Nat::from(0),
            history_events: v0.history_events,
            balance: v0.balance,
            transfers_count: v0.transfers_count,
            mints_count: v0.mints_count,
            burns_count: v0.burns_count,
            proxy_calls_count: v0.proxy_calls_count,
            canisters_created_count: v0.canisters_created_count,
        }
    }
}

pub enum CountTarget {
    Transfer,
    Mint,
    Burn,
    ProxyCall,
    CanisterCreated,
}

impl StatsData {
    #[inline]
    pub fn load(data: StatsData) {
        let stats = storage::get_mut::<StatsData>();
        *stats = data;
    }

    #[inline]
    pub fn get() -> StatsData {
        let stats = storage::get_mut::<StatsData>();
        stats.history_events = storage::get::<HistoryBuffer>().len() as u64;
        stats.balance = api::canister_balance();
        stats.clone()
    }

    #[inline]
    pub fn increment(target: CountTarget) {
        let stats = storage::get_mut::<StatsData>();
        match target {
            CountTarget::Transfer => stats.transfers_count += 1,
            CountTarget::Mint => stats.mints_count += 1,
            CountTarget::Burn => stats.burns_count += 1,
            CountTarget::ProxyCall => stats.proxy_calls_count += 1,
            CountTarget::CanisterCreated => stats.canisters_created_count += 1,
        }
    }

    #[inline]
    pub fn deposit(amount: u64) {
        let stats = storage::get_mut::<StatsData>();
        stats.supply += amount;
    }

    #[inline]
    pub fn withdraw(amount: u64) {
        let stats = storage::get_mut::<StatsData>();
        stats.supply -= amount;
    }

    #[inline]
    pub fn fee(amount: u64) {
        let stats = storage::get_mut::<StatsData>();
        stats.fee += amount;
    }
}

#[query]
fn stats() -> StatsData {
    StatsData::get()
}
