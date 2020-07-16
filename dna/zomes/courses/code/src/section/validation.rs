use hdk::prelude::*;

use crate::course::entry::Course;
use super::entry::Module;

pub fn validate_module_title(title: &str) -> Result<(), String> {
    if title.len() > 200 {
        Err("Module title is too long".into())
    } else {
        Ok(())
    }
}

pub fn validate_author(signing_addresses: &Vec<Address>, module: &Module) -> ZomeApiResult<()> {
    let course: Course = hdk::utils::get_as_type(module.course_address.clone())?;
    hdk::debug(format!("{:?}", course))?;
    if !signing_addresses.contains(&course.teacher_address) {
        return Err(ZomeApiError::from(String::from(
            "Only the teacher can create or modify a module for it",
        )));
    }
    Ok(())
}
