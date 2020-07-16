use hdk::prelude::*;

use crate::course::entry::Course;
use super::validation::{validate_author, validate_module_title};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct Module {
    pub title: String,
    pub timestamp: u64,
    pub course_address: Address,
}

impl Module {
    pub fn new(title: String, course_address: Address, timestamp: u64) -> Self {
        Module {
            title: title,
            course_address: course_address,
            timestamp: timestamp,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("module".into(), self.into())
    }
}

pub fn entry_def() -> ValidatingEntryType {
    entry!(
        name: "module",
        description: "this is the definition of module",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Module>| {
            match  validation_data {
                EntryValidationData::Create { entry, validation_data } => {
                    validate_module_title(&entry.title)?;

                    validate_author(&validation_data.sources(), &entry)?;

                    Ok(())
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    validate_module_title(&new_entry.title)?;

                    if new_entry.course_address != old_entry.course_address {
                        return Err(String::from("Cannot modify the course of a module"));
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
                link_type: "module->contents",
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|_validation_data: hdk::LinkValidationData|{
                // TODO: Homework. Implement validation rules if required.
                    Ok(())
                }
            )
        ]
    )
}
