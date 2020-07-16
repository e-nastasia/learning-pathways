use hdk::error::ZomeApiResult;
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::LinkMatch;

use super::entry::Content;

pub fn create(
    name: String,
    section_address: Address,
    url: String,
    timestamp: u64,
    description: String,
) -> ZomeApiResult<Address> {
    let new_content = Content::new(name, section_address.clone(), url, timestamp, description);
    let new_content_entry = new_content.entry();
    let new_content_address = hdk::commit_entry(&new_content_entry)?;
    hdk::link_entries(
        &section_address,
        &new_content_address,
        "section->contents",
        "",
    )?;

    Ok(new_content_address)
}

pub fn get_contents(section_address: &Address) -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &section_address,
        LinkMatch::Exactly("section->contents"),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}
pub fn delete(content_address: Address) -> ZomeApiResult<Address> {
    let content: Content = hdk::utils::get_as_type(content_address.clone())?;

    hdk::remove_link(
        &content.section_address,
        &content_address,
        "section->contents",
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
