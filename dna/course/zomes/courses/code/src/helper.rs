use hdk::{
    error::ZomeApiResult, holochain_persistence_api::cas::content::Address, prelude::LinkMatch,
};
use holochain_entry_utils::HolochainEntry;

// validates title of some entity on not being longer than allowed_legth
pub fn validate_entity_title(
    title: &str,
    entity_name: &str,
    allowed_length: usize,
) -> Result<(), String> {
    if title.len() > allowed_length {
        Err(format!(
            "{} title is too long, has to be no longer than {}",
            entity_name, allowed_length
        ))
    } else {
        Ok(())
    }
}

// validates that agent with teacher_address is listed in the validation_data_sources
pub fn validate_only_teacher_can_do(
    teacher_address: &Address,
    validation_data_sources: Vec<Address>,
    action_name: &str,
) -> Result<(), String> {
    if !validation_data_sources.contains(teacher_address) {
        return Err(format!("Only the teacher can {}", action_name));
    }
    Ok(())
}

// gets latest data entry that is linked to anchor at entry_anchor_address
// This is a helper for anchor-first pattern entries
pub fn get_latest_data_entry<T: HolochainEntry>(
    entry_anchor_address: &Address,
    link_type: &str,
) -> ZomeApiResult<(T, Address)> {
    let entry_addresses = hdk::get_links(
        entry_anchor_address,
        LinkMatch::Exactly(link_type),
        // this parameter is for link tags. since we don't tag anchor->data entry link (see method create above)
        //  we need to ask for all tags
        LinkMatch::Any,
    )?
    .addresses();

    // NOTE: we're assuming that this vec would only have one item in it.
    // Question about it is added into zome README.md
    let latest_entry_address = entry_addresses[0].clone();
    let latest_entry: T = hdk::utils::get_as_type(latest_entry_address.clone())?;
    return Ok((latest_entry, latest_entry_address));
}
