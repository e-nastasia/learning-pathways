use crate::course::entry::Course;
use crate::section::entry::Module;
use hdk::error::{ZomeApiError, ZomeApiResult};
use hdk::holochain_persistence_api::cas::content::Address;

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
