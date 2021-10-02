elrond_wasm::derive_imports!();

use elrond_wasm::{
    api::ManagedTypeApi,
    types::{BigUint, ManagedAddress},
};

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Sale<M:ManagedTypeApi> {
    nft_owner: ManagedAddress<M>,
    price: BigUint<M>
}