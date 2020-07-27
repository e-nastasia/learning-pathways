use hdk::prelude::*;
use hdk::{LinkValidationData, ValidationData};

use super::entry::Section;
use crate::course::handlers::get_latest_course;
use crate::helper;

pub fn validate_section_title(title: &str) -> Result<(), String> {
    if title.len() > 200 {
        Err("Section title is too long".into())
    } else {
        Ok(())
    }
}

pub fn validate_author(validation_data: ValidationData, section: &Section) -> ZomeApiResult<()> {
    hdk::debug(format!("{:?}", section))?;
    let latest_course_result = get_latest_course(&section.course_address)?;
    // we are using if let here since at validation time, we are sure that course entry exist
    if let Some((current_course, _current_course_address)) = latest_course_result {
        helper::validate_only_teacher_can_do(
            &current_course.teacher_address,
            validation_data.sources(),
            "create a section in this course",
        )?;
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
