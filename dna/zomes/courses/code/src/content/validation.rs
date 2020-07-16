use crate::course;
use crate::course::entry::Course;
use crate::section::entry::Module;
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

/////////////////////////// Validations
pub fn validate_author(
    signing_addresses: &Vec<Address>,
    module_address: &Address,
) -> ZomeApiResult<()> {
    let module: Module = hdk::utils::get_as_type(module_address.clone())?;
    let course: Course = hdk::utils::get_as_type(module.course_address.clone())?;
    if !signing_addresses.contains(&course.teacher_address) {
        return Err(ZomeApiError::from(String::from(
            "Error: Only the teacher can create or modify a content for module",
        )));
    }
    Ok(())
}