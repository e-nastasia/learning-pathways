use hdk::prelude::*;

use super::entry::Section;
use crate::course::entry::Course;

pub fn create(title: String, course_address: &Address, timestamp: u64) -> ZomeApiResult<Address> {
    let mut course: Course = hdk::utils::get_as_type(course_address.clone())?;

    let new_section = Section::new(title, course_address.clone(), timestamp);
    let new_section_address = hdk::commit_entry(&new_section.entry())?;

    course.sections.push(new_section_address.clone());
    course.timestamp += 1;
    hdk::update_entry(course.entry(), &course_address)?;

    Ok(new_section_address)
}

pub fn update(title: String, section_address: &Address) -> ZomeApiResult<Address> {
    let mut section: Section = hdk::utils::get_as_type(section_address.clone())?;

    section.title = title;

    hdk::update_entry(section.entry(), section_address)
}

pub fn delete(section_address: Address) -> ZomeApiResult<Address> {
    let section: Section = hdk::utils::get_as_type(section_address.clone())?;

    let mut course: Course = hdk::utils::get_as_type(section.course_address.clone())?;

    let result = hdk::remove_entry(&section_address)?;

    course.sections.remove_item(&section_address);
    course.timestamp += 1; // we need to prevent duplication by changing the array.
    hdk::update_entry(course.entry(), &section.course_address)?;

    Ok(result)
}
