use super::entry::Course;
use crate::helper;
use hdk::holochain_core_types::chain_header::ChainHeader;
use hdk::{LinkValidationData, ValidationData};
use holochain_entry_utils::HolochainEntry;

pub fn create(entry: Course, validation_data: ValidationData) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "create their courses",
    )?;
    helper::validate_entity_title(&entry.title, &Course::entry_type(), 50)
}

pub fn modify(
    new_entry: Course,
    old_entry: Course,
    _old_entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &new_entry.teacher_address,
        validation_data.sources(),
        "create their courses",
    )?;
    helper::validate_entity_title(&new_entry.title, "course", 50)?;
    validate_no_teacher_change(old_entry, new_entry)
}

// this fn is only needed in the current module so it's private
fn validate_no_teacher_change(old_entry: Course, new_entry: Course) -> Result<(), String> {
    if new_entry.teacher_address != old_entry.teacher_address {
        return Err(String::from("Cannot change the teacher of the course"));
    }
    Ok(())
}

pub fn delete(
    entry: Course,
    _entry_header: ChainHeader,
    validation_data: ValidationData,
) -> Result<(), String> {
    helper::validate_only_teacher_can_do(
        &entry.teacher_address,
        validation_data.sources(),
        "delete their courses",
    )
}

pub fn anchor_create(_validation_data: ValidationData) -> Result<(), String> {
    Ok(())
}

pub fn anchor_modify(_validation_data: ValidationData) -> Result<(), String> {
    Ok(())
}

pub fn anchor_delete(_validation_data: ValidationData) -> Result<(), String> {
    Ok(())
}

pub fn anchor_link(validation_data: LinkValidationData) -> Result<(), String> {
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
