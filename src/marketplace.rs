#![no_std]

elrond_wasm::imports!();

mod user_status;

use user_status::UserStatus;

/// Derived empirically.
const PONG_ALL_LOW_GAS_LIMIT: u64 = 3_000_000;

#[elrond_wasm::contract]
pub trait IsengardContract {
    #[init]
    fn init(&self) {}

    // Endpoints
    #[endpoint(getSum)]
    fn do_smth(&self) -> Self::BigInt;

    // Views
    #[view(getSum)]
    #[storage_get("sum")] /// Get the number of transactions that were executed by the SC.
    fn get_sum(&self) -> Self::BigInt;

    // Private

    // Storage
    #[endpoint]

    //#[storage_get("key")] /// Try not to use this one. Instead use SingleValueMapper, VecMapper,SetMapper, LinkedListMapper, MapMapper 

    //#[event("transfer")]

    #[storage_set("sum")] /// Add a new transaction
    fn set_sum(&self, sum: &Self::BigInt);

    

}