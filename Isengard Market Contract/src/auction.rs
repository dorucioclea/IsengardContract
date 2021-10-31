elrond_wasm::derive_imports!();

use elrond_wasm::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedAddress,ManagedType},
};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub enum NftState {
    Default,
    Sale,
    Auction,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct NftStates<M:ManagedTypeApi>{  // Maybe remove this and use this in Wrapper!
    pub nft_owner: ManagedAddress<M>,
    pub state : NftState
}

impl<M: ManagedTypeApi> NftStates<M> {
    pub fn new(
        nft_owner: &ManagedAddress<M>,
        state : NftState
    ) -> Self {
        NftStates {
            nft_owner : nft_owner.clone(),
            state : state
        }
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Auction<M:ManagedTypeApi> {
    pub nft_owner: ManagedAddress<M>,
    pub starting_price: BigUint<M>,
    pub current_bid: BigUint<M>,
    pub current_winner: ManagedAddress<M>,
    pub final_price: BigUint<M>,
    pub deadline: u64,
    pub start_time: u64,
}

impl<M: ManagedTypeApi> Auction<M> {
    pub fn new(
        nft_owner: &ManagedAddress<M>,
        starting_price: &BigUint<M>,
        final_price: &BigUint<M>,
        deadline: u64,
        start_time : u64
    ) -> Self {
        let type_manager = starting_price.type_manager();
        Auction {
            nft_owner : nft_owner.clone(),
            starting_price : starting_price.clone(),
            deadline,
            start_time,
            final_price : final_price.clone(),
            current_bid: BigUint::zero(type_manager.clone()),
            current_winner: ManagedAddress::zero(type_manager),
        }
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Sale<M:ManagedTypeApi> {
    pub nft_owner: ManagedAddress<M>,
    pub price: BigUint<M>
}

impl<M: ManagedTypeApi> Sale<M> {
    pub fn new(
        nft_owner: &ManagedAddress<M>,
        price: &BigUint<M>
    ) -> Self {
        Sale {
            nft_owner : nft_owner.clone(),
            price : price.clone(),
        }
    }
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct SaleWrapper<M:ManagedTypeApi> {
    pub sale : Option<Sale<M>>,
    pub auction : Option<Auction<M>>,
    pub state : NftState
}

impl <M:ManagedTypeApi>SaleWrapper<M> {
    pub fn new_sale(
        sale: Sale<M>,
        state : NftState
    ) -> Self {
        SaleWrapper  {
          sale : Some(sale),
          auction: None,
          state : state
        }
    }
    pub fn new_auction(
        auction: Auction<M>,
        state : NftState
    ) -> Self {
        SaleWrapper  {
          sale: None,
          auction : Some(auction),
          state : state
        }
    }
}