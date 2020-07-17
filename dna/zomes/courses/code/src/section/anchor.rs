use super::{entry::Section, validation};
use crate::content::entry::Content;
use crate::anchor_trait::AnchorTrait;

use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

pub const SECTION_TO_CONTENT_LINK: &str = "section_anchor->content";

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct SectionAnchor {
    // NOTE: these fields are here to ensure the uniqueness of every particular anchor
    //  and wouldn't be used to display data about section to a user
    pub title: String,
    pub course_address: Address,
    pub timestamp: u64,
}

impl AnchorTrait for SectionAnchor {
    fn entry_type() -> String {
        String::from("section_anchor")
    }
    fn link_to() -> String {
        Section::entry_type()
    }
    fn link_type() -> String {
        "section_anchor->section".to_string()
    }
}

impl SectionAnchor {
    pub fn new(title: String, course_address: Address, timestamp: u64) -> Self {
        SectionAnchor {
            title: title,
            course_address: course_address,
            timestamp: timestamp,
        }
    }
}

pub fn section_anchor_def() -> ValidatingEntryType {
    entry!(
        name: SectionAnchor::entry_type(),
        description: "Anchor to the valid course section",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<SectionAnchor>| {
            match validation_data{
                EntryValidationData::Create { validation_data, .. } => {
                    validation::validate_anchor_create(validation_data)?;
                     Ok(())
                 },
                 EntryValidationData::Modify { validation_data, .. } => {
                    validation::validate_anchor_modify(validation_data)?;
                    Ok(())
                 },
                 EntryValidationData::Delete { validation_data, .. } => {
                    validation::validate_anchor_delete(validation_data)?;
                    Ok(())
                 }
            }
        },
        links:[
            to!(
                SectionAnchor::link_to(),
                link_type: SectionAnchor::link_type(),
                validation_package:||{
                    hdk::ValidationPackageDefinition::Entry
                },
                validation:|validation_data: hdk::LinkValidationData|{
                   validation::validate_anchor_link(validation_data)
                }
            ),
            to!(
                Content::entry_type(),
                link_type: SECTION_TO_CONTENT_LINK,
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
