use hdk::{
    entry_definition::ValidatingEntryType,
    holochain_core_types::{dna::entry_types::Sharing, validation::EntryValidationData},
    holochain_json_api::{error::JsonError, json::JsonString},
    holochain_persistence_api::cas::content::Address,
};
use holochain_entry_utils::HolochainEntry;

use super::validation::validate_course_title;

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Course {
    pub title: String,
    pub sections: Vec<Address>,
    pub teacher_address: Address,
    pub timestamp: u64,
    pub anchor_address: Address,
}

impl HolochainEntry for Course {
    fn entry_type() -> String {
        String::from("course")
    }
}

impl Course {
    pub fn new(
        title: String,
        sections: Vec<Address>,
        teacher_address: Address,
        timestamp: u64,
        anchor_address: Address,
    ) -> Self {
        Course {
            title: title,
            sections: sections,
            teacher_address: teacher_address,
            timestamp: timestamp,
            anchor_address: anchor_address,
        }
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
        // All links that course should have are defined for CoureAnchor and so this entry doesn't have ny
        links: []
    )
}
