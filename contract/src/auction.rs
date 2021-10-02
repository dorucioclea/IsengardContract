elrond_wasm::derive_imports!();

use elrond_wasm::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedAddress},
};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Auction<M:ManagedTypeApi> {
    pub nft_owner: ManagedAddress<M>,
    pub starting_price: BigUint<M>,
    pub current_bid: BigUint<M>,
    pub deadline: u64,
}