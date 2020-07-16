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

use super::validation::validate_author;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Content {
    pub name: String,
    pub url: String,
    pub description: String,
    pub timestamp: u64,
    pub module_address: Address,
}

impl Content {
    pub fn new(
        name: String,
        module_address: Address,
        url: String,
        timestamp: u64,
        description: String,
    ) -> Self {
        Content {
            name,
            url,
            description,
            timestamp,
            module_address,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("content".into(), self.into())
    }
}

pub fn module_entry_def() -> ValidatingEntryType {
    entry!(
        name: "content",
        description: "this is the content for each module",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Content>| {
            match  validation_data {
                EntryValidationData::Create { entry, validation_data } => {
                    validate_author(&validation_data.sources(), &entry.module_address)?;
                    Ok(())
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    if new_entry.module_address != old_entry.module_address {
                        return Err(String::from("Cannot modify the module of a content"));
                    }
                    validate_author(&validation_data.sources(), &new_entry.module_address)?;
                    Ok(())
                },
                EntryValidationData::Delete { old_entry, validation_data, .. } => {
                    validate_author(&validation_data.sources(), &old_entry.module_address)?;

                    Ok(())
                }
            }
        }
    )
}