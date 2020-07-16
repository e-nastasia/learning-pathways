use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};

use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_core_types::{entry::Entry, validation::EntryValidationData};
use holochain_wasm_utils::api_serialization::{
    get_entry::{GetEntryOptions, GetEntryResult},
    get_links::GetLinksOptions,
};

use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::AddressableContent;
use hdk::prelude::LinkMatch;
use hdk::ValidationData;
use std::convert::TryFrom;
use super::validation::validate_course_title;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Course {
    pub title: String,
    pub modules: Vec<Address>,
    pub timestamp: u64,
    pub teacher_address: Address,
}

impl Course {
    // Constrcuctor
    pub fn new(title: String, owner: Address, timestamp: u64) -> Self {
        Course {
            title: title,
            teacher_address: owner,
            modules: Vec::default(),
            timestamp: timestamp,
        }
    }
    pub fn from(title: String, owner: Address, timestamp: u64, modules: Vec<Address>) -> Self {
        Course {
            title: title,
            teacher_address: owner,
            modules: modules,
            timestamp: timestamp,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("course".into(), self.into())
    }
}

////////////////////Course Entry Definition
pub fn course_entry_def() -> ValidatingEntryType {
    entry!(
        name: "course",
        description: "this is the definition of course",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Course>| {
            match validation_data{
                EntryValidationData::Create { entry, validation_data } => {
                    if !validation_data.sources().contains(&entry.teacher_address) {
                        return Err(String::from("Only the teacher can create their courses"));
                    }

                    validate_course_title(&entry.title)
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    if new_entry.teacher_address != old_entry.teacher_address {
                        return Err(String::from("Cannot change the teacher of the course"));
                    }

                    if !validation_data.sources().contains(&old_entry.teacher_address) {
                        return Err(String::from("Only the teacher can modify their courses"));
                    }

                    validate_course_title(&new_entry.title)?;

                    Ok(())
                },
                EntryValidationData::Delete {old_entry, validation_data, .. } => {
                    if !validation_data.sources().contains(&old_entry.teacher_address) {
                        return Err(String::from("Only the teacher can delete their courses"));
                    }

                    Ok(())
                }
            }
        },
        links: [
          from!( // to query all the courses of a user(all courses that a user is the teacher or owner of)
              "%agent_id",
              link_type: "teacher->courses",
              validation_package: || {
                  hdk::ValidationPackageDefinition::Entry
              }              ,
              validation: | _validation_data: hdk::LinkValidationData | {
                // TODO: Homework. Implement validation rules if required.
                Ok(())
              }
          ),
          from!( // to query all courses that one user enrolled
            "%agent_id",
            link_type: "student->courses",
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            }              ,
            validation: | _validation_data: hdk::LinkValidationData | {
                // TODO: Homework. Implement validation rules if required.
               Ok(())
            }
        ),
        to!( // to query all enrolled user for a course
            "%agent_id",
            link_type: "course->students",
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::LinkValidationData | {
                // TODO: Homework. Implement validation rules if required.
                Ok(())
            }
        )
      ]
    )
}