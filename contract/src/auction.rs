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

impl<M: ManagedTypeApi> Auction<M> {
    pub fn new(
        nft_owner: &ManagedAddress<M>,
        starting_price: &BigUint<M>,
        current_bid: &BigUint<M>,
        deadline: u64,
    ) -> Self {
        Auction {
            nft_owner : nft_owner.clone(),
            starting_price : starting_price.clone(),
            current_bid : current_bid.clone(),
            deadline
        }
    }
}