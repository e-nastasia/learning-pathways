use hdk::prelude::*;
use hdk::{LinkValidationData, ValidationData};

use super::entry::Section;
use crate::course::entry::Course;

pub fn validate_section_title(title: &str) -> Result<(), String> {
    if title.len() > 200 {
        Err("Section title is too long".into())
    } else {
        Ok(())
    }
}

pub fn validate_author(signing_addresses: &Vec<Address>, section: &Section) -> ZomeApiResult<()> {
    let course: Course = hdk::utils::get_as_type(section.course_address.clone())?;
    hdk::debug(format!("{:?}", course))?;
    if !signing_addresses.contains(&course.teacher_address) {
        return Err(ZomeApiError::from(String::from(
            "Only the teacher can create or modify a section for it",
        )));
    }
    Ok(())
}

pub fn validate_anchor_create(_validation_data: ValidationData) -> Result<(), String> {
    Ok(())
}

pub fn validate_anchor_modify(_validation_data: ValidationData) -> Result<(), String> {
    Ok(())
}

pub fn validate_anchor_delete(_validation_data: ValidationData) -> Result<(), String> {
    Ok(())
}

pub fn validate_anchor_link(validation_data: LinkValidationData) -> Result<(), String> {
    match validation_data {
        hdk::LinkValidationData::LinkAdd {
            link: _,
            validation_data: _,
        } => Ok(()),
        hdk::LinkValidationData::LinkRemove {
            link: _,
            validation_data: _,
        } => Ok(()),
    }
}
