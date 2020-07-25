use hdk::holochain_persistence_api::cas::content::Address;

// validates title of some entity on not being longer than allowed_legth
pub fn validate_entity_title(
    title: &str,
    entity_name: &str,
    allowed_length: usize,
) -> Result<(), String> {
    if title.len() > allowed_length {
        Err(format!(
            "{} title is too long, has to be no longer than {}",
            entity_name, allowed_length
        ))
    } else {
        Ok(())
    }
}

// validates that agent with teacher_address is listed in the validation_data_sources
pub fn validate_only_teacher_can_do(
    teacher_address: &Address,
    validation_data_sources: Vec<Address>,
    action_name: &str,
) -> Result<(), String> {
    if !validation_data_sources.contains(teacher_address) {
        return Err(format!("Only the teacher can {}", action_name));
    }
    Ok(())
}
