use crate::course::handlers::get_latest_course;
use crate::helper;
use crate::section::handlers::get_latest_section;
use hdk::error::ZomeApiResult;
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::ValidationData;

/////////////////////////// Validations
pub fn validate_author(
    validation_data: ValidationData,
    section_anchor_address: &Address,
) -> ZomeApiResult<()> {
    let latest_section_result = get_latest_section(&section_anchor_address)?;
    if let Some((current_section, _current_section_address)) = latest_section_result {
        let latest_course_result = get_latest_course(&current_section.course_address)?;
        if let Some((current_course, _current_course_address)) = latest_course_result {
            helper::validate_only_teacher_can_do(
                &current_course.teacher_address,
                validation_data.sources(),
                "create a content in this section",
            )?;
        }
    }
    Ok(())
}
