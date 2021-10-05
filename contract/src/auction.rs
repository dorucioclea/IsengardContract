elrond_wasm::derive_imports!();

use elrond_wasm::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedAddress,ManagedType},
};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Auction<M:ManagedTypeApi> {
    pub nft_owner: ManagedAddress<M>,
    pub starting_price: BigUint<M>,
    pub current_bid: BigUint<M>,
    pub current_winner: ManagedAddress<M>,
    pub final_price: BigUint<M>,
    pub deadline: u64,
}

impl<M: ManagedTypeApi> Auction<M> {
    pub fn new(
        nft_owner: &ManagedAddress<M>,
        starting_price: &BigUint<M>,
        final_price: &BigUint<M>,
        deadline: u64,
    ) -> Self {
        let type_manager = starting_price.type_manager();
        Auction {
            nft_owner : nft_owner.clone(),
            starting_price : starting_price.clone(),
            deadline,
            final_price : final_price.clone(),
            current_bid: BigUint::zero(type_manager.clone()),
            current_winner: ManagedAddress::zero_address(type_manager),
        }
    }
}