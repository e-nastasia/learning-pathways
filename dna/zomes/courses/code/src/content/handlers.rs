use hdk::error::ZomeApiResult;
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::LinkMatch;
use holochain_entry_utils::HolochainEntry;

use super::entry::Content;
use crate::section::anchor::SECTION_TO_CONTENT_LINK;


pub fn create(
    name: String,
    section_anchor_address: Address,
    url: String,
    timestamp: u64,
    description: String,
) -> ZomeApiResult<Address> {
    let new_content = Content::new(name, section_anchor_address.clone(), url, timestamp, description);
    let new_content_entry = new_content.entry();
    let new_content_address = hdk::commit_entry(&new_content_entry)?;
    hdk::link_entries(
        &section_anchor_address,
        &new_content_address,
        SECTION_TO_CONTENT_LINK,
        "",
    )?;

    Ok(new_content_address)
}

pub fn get_contents(section_anchor_address: &Address) -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &section_anchor_address,
        LinkMatch::Exactly(SECTION_TO_CONTENT_LINK),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

pub fn delete(content_address: Address) -> ZomeApiResult<Address> {
    let content: Content = hdk::utils::get_as_type(content_address.clone())?;

    hdk::remove_link(
        &content.section_anchor_address,
        &content_address,
        SECTION_TO_CONTENT_LINK,
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
