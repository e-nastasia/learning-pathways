use crate::course::entry::Course;
use crate::section::entry::Section;
use hdk::error::{ZomeApiError, ZomeApiResult};
use hdk::holochain_persistence_api::cas::content::Address;

/////////////////////////// Validations
pub fn validate_author(
    signing_addresses: &Vec<Address>,
    section_anchor_address: &Address,
) -> ZomeApiResult<()> {
    let section: Section = hdk::utils::get_as_type(section_anchor_address.clone())?;
    let course: Course = hdk::utils::get_as_type(section.course_address.clone())?;
    if !signing_addresses.contains(&course.teacher_address) {
        return Err(ZomeApiError::from(String::from(
            "Error: Only the teacher can create or modify a content for section",
        )));
    }
    Ok(())
}
