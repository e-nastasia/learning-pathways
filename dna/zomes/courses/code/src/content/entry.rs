use hdk::prelude::*;

use super::validation::validate_author;

// NOTE: using self::DefaultJson to disambiguate usage of DefaultJson from this module (hdk::prelude imports it)
#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct Content {
    pub name: String,
    pub url: String,
    pub description: String,
    pub timestamp: u64,
    pub section_address: Address,
}

impl Content {
    pub fn new(
        name: String,
        section_address: Address,
        url: String,
        timestamp: u64,
        description: String,
    ) -> Self {
        Content {
            name,
            url,
            description,
            timestamp,
            section_address,
        }
    }

    pub fn entry(&self) -> Entry {
        Entry::App("content".into(), self.into())
    }
}

pub fn section_entry_def() -> ValidatingEntryType {
    entry!(
        name: "content",
        description: "this is the content for each section",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Content>| {
            match  validation_data {
                EntryValidationData::Create { entry, validation_data } => {
                    validate_author(&validation_data.sources(), &entry.section_address)?;
                    Ok(())
                },
                EntryValidationData::Modify { new_entry, old_entry, validation_data, .. } => {
                    if new_entry.section_address != old_entry.section_address {
                        return Err(String::from("Cannot change section to which this content belongs to"));
                    }
                    validate_author(&validation_data.sources(), &new_entry.section_address)?;
                    Ok(())
                },
                EntryValidationData::Delete { old_entry, validation_data, .. } => {
                    validate_author(&validation_data.sources(), &old_entry.section_address)?;

                    Ok(())
                }
            }
        }
    )
}
