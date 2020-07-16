use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{
        dna::entry_types::Sharing, entry::Entry, validation::EntryValidationData,
    },
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};

use super::validation::{validate_author, validate_section_title};
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Section {
    pub title: String,
    pub timestamp: u64,
    pub course_address: Address,
}

impl Section {
    pub fn new(title: String, course_address: Address, timestamp: u64) -> Self {
        Section {
            title: title,
            course_address: course_address,
            timestamp: timestamp,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("section".into(), self.into())
    }
}

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: "section",
        description: "this is the definition of section",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Section>| {
            match  validation_data {
                EntryValidationData::Create { entry, validation_data } => {
                    validate_section_title(&entry.title)?;

                    validate_author(&validation_data.sources(), &entry)?;

                    Ok(())
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    validate_section_title(&new_entry.title)?;

                    if new_entry.course_address != old_entry.course_address {
                        return Err(String::from("Cannot change course to which the section belongs"));
                    }
                    validate_author(&validation_data.sources(), &new_entry)?;
                    Ok(())
                },
                EntryValidationData::Delete { old_entry, validation_data, .. } => {
                    validate_author(&validation_data.sources(), &old_entry)?;

                    Ok(())
                }
            }
        },
        links:[
            to!(
                "content",
                link_type: "section->contents",
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                    Ok(())
                }
            )
        ]
    )
}
