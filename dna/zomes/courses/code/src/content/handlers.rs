use crate::course;
use crate::course::entry::Course;
use crate::module::Module;
use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_core_types::{entry::Entry, validation::EntryValidationData};
use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::LinkMatch;
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};
use holochain_wasm_utils::api_serialization::{
    get_entry::{GetEntryOptions, GetEntryResult},
    get_links::GetLinksOptions,
};
use std::convert::TryFrom;

use super::entry::Content;

pub fn create(
    name: String,
    module_address: Address,
    url: String,
    timestamp: u64,
    description: String,
) -> ZomeApiResult<Address> {
    let new_content = Content::new(name, module_address.clone(), url, timestamp, description);
    let new_content_entry = new_content.entry();
    let new_content_address = hdk::commit_entry(&new_content_entry)?;
    hdk::link_entries(
        &module_address,
        &new_content_address,
        "module->contents",
        "",
    )?;

    Ok(new_content_address)
}

pub fn get_contents(module_address: &Address) -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &module_address,
        LinkMatch::Exactly("module->contents"),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}
pub fn delete(content_address: Address) -> ZomeApiResult<Address> {
    let content: Content = hdk::utils::get_as_type(content_address.clone())?;

    hdk::remove_link(
        &content.module_address,
        &content_address,
        "module->contents",
        "",
    )?;

    hdk::remove_entry(&content_address)
}

pub fn update(
    content_address: Address,
    name: String,
    url: String,
    description: String,
) -> ZomeApiResult<Address> {
    let mut content: Content = hdk::utils::get_as_type(content_address.clone())?;
    content.description = description;
    content.name = name;
    content.url = url;
    hdk::update_entry(content.entry(), &content_address)
}
