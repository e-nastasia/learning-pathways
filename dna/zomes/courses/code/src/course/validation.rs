use hdk::{LinkValidationData, ValidationData};

pub fn validate_course_title(title: &str) -> Result<(), String> {
    if title.len() > 50 {
        Err("Course title is too long".into())
    } else {
        Ok(())
    }
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
