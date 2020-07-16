use hdk::prelude::*;

use crate::course::entry::Course;
use super::entry::Module;
use std::convert::TryFrom;


pub fn create(title: String, course_address: &Address, timestamp: u64) -> ZomeApiResult<Address> {
    let mut course: Course = hdk::utils::get_as_type(course_address.clone())?;

    let new_module = Module::new(title, course_address.clone(), timestamp);
    let new_module_address = hdk::commit_entry(&new_module.entry())?;

    course.modules.push(new_module_address.clone());
    course.timestamp += 1;
    hdk::update_entry(course.entry(), &course_address)?;

    Ok(new_module_address)
}

pub fn update(title: String, module_address: &Address) -> ZomeApiResult<Address> {
    let mut module: Module = hdk::utils::get_as_type(module_address.clone())?;

    module.title = title;

    hdk::update_entry(module.entry(), module_address)
}

pub fn delete(module_address: Address) -> ZomeApiResult<Address> {
    let module: Module = hdk::utils::get_as_type(module_address.clone())?;

    let mut course: Course = hdk::utils::get_as_type(module.course_address.clone())?;

    let result = hdk::remove_entry(&module_address)?;

    course.modules.remove_item(&module_address);
    course.timestamp += 1; // we need to prevent duplication by changing the array.
    hdk::update_entry(course.entry(), &module.course_address)?;

    Ok(result)
}
