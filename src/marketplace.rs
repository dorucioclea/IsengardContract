#![no_std]

elrond_wasm::imports!();

mod user_status;

use user_status::UserStatus;

/// Derived empirically.
const PONG_ALL_LOW_GAS_LIMIT: u64 = 3_000_000;

/// A contract that allows anyone to send a fixed sum, locks it for a while and then allows users to take it back.
/// Sending funds to the contract is called "ping".
/// Taking the same funds back is called "pong".
///
/// Restrictions:
/// - `ping` can be called only after the contract is activated. By default the contract is activated on deploy.
/// - Users can only `ping` once, ever.
/// - Only the set amount can be `ping`-ed, no more, no less.
/// - The contract can optionally have a maximum cap. No more users can `ping` after the cap has been reached.
/// - The `ping` endpoint optionally accepts
/// - `pong` can only be called after the contract expired (a certain duration has passed since activation).
/// - `pongAll` can be used to send to all users to `ping`-ed. If it runs low on gas, it will interrupt itself.
/// It can be continued anytime.
#[elrond_wasm::contract]
pub trait IsengardContract {
    #[init]
    fn init(&self) {}
}