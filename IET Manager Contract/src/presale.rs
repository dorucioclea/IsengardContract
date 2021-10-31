elrond_wasm::derive_imports!();

use elrond_wasm::{
    api::ManagedTypeApi,
    types::{BigUint},
};


#[derive(NestedEncode,NestedDecode,TopEncode,TopDecode,TypeAbi)]
pub struct Presale<M:ManagedTypeApi>{
    pub max_tokens : BigUint<M>,
    pub sold_tokens : BigUint<M>,
    pub start_time: u64,
    pub end_time: u64,
    pub price: BigUint<M>
}

impl<M:ManagedTypeApi> Presale<M> {
    pub fn new(
        max_tokens : &BigUint<M>,
        sold_tokens : &BigUint<M>,
        start_time: u64,
        end_time: u64,
        price: BigUint<M>
    ) -> Self {
            Presale{
            max_tokens : max_tokens.clone(),
            sold_tokens : sold_tokens.clone(),
            start_time : start_time.clone(),
            end_time : end_time.clone(),
            price: price
            }
        
    }
}

#[derive(NestedEncode,NestedDecode,TopEncode,TopDecode,TypeAbi)]
pub struct UserTokens<M:ManagedTypeApi>{
    pub tokens: BigUint<M>,
    pub timestamp: u64,
}

impl<M:ManagedTypeApi> UserTokens<M>{
    pub fn new(
        tokens : &BigUint<M>,
        timestamp: u64,
    ) -> Self{
        UserTokens{
            tokens : tokens.clone(),
            timestamp: timestamp.clone()
        }
    }
}