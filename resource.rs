multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::heap::String;

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct Resource {
    pub name: String,
    pub size: i32,
    pub level: String,
    pub url: String
}
