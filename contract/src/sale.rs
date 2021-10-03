elrond_wasm::derive_imports!();

use elrond_wasm::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedAddress},
};

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
