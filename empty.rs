#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

mod resource;
use resource::Resource;
use crate::heap::String;

#[multiversx_sc::contract]
pub trait ResourceContract:
{
    #[init]
    fn init(&self) {
        self.resources_count().set(0)
    }

    #[endpoint]
    fn add_file(&self, name: String, size: i32, level: String, url: String) {

        require!(name.eq(""), "Name is empty");
        require!(size.eq(&0), "Size is 0");
        require!(level.eq(""), "Level is empty");
        require!(url.eq(""), "URL is empty");

        self.resources_count().update(|resources_count| *resources_count += 1);
        self.resources().insert(Resource {
            name,
            size,
            level,
            url
        });
    }

    #[view]
    #[storage_mapper("resources_count")]
    fn resources_count(&self) -> SingleValueMapper<i32>;

    #[view(getResources)]
    #[storage_mapper("resources")]
    fn resources(&self) -> UnorderedSetMapper<Resource>;
}
