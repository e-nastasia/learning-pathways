use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS,
};

use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_core_types::{entry::Entry, validation::EntryValidationData};
use holochain_wasm_utils::api_serialization::{
    get_entry::{GetEntryOptions, GetEntryResult},
    get_links::GetLinksOptions,
};

use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::AddressableContent;
use hdk::prelude::LinkMatch;
use hdk::ValidationData;
use std::convert::TryFrom;

use super::entry::Course;
use super::anchor::{anchor_entry, anchor_address};

pub fn create(title: String, timestamp: u64) -> ZomeApiResult<Address> {
    let anchor_entry = anchor_entry();
    let anchor_address = hdk::commit_entry(&anchor_entry)?; // if Anchor exist, it returns the commited one.

    let new_course = Course::new(title, AGENT_ADDRESS.to_string().into(), timestamp);
    let new_course_entry = new_course.entry();
    let new_course_address = hdk::commit_entry(&new_course_entry)?;

    hdk::link_entries(&AGENT_ADDRESS, &new_course_address, "teacher->courses", "")?;

    hdk::link_entries(&anchor_address, &new_course_address, "course_list", "")?;

    Ok(new_course_address)
}

pub fn update(
    title: String,
    modules_addresses: Vec<Address>,
    course_address: &Address,
) -> ZomeApiResult<Address> {
    let course: Course = hdk::utils::get_as_type(course_address.clone())?;

    let new_version_course = Course::from(
        title,
        course.teacher_address,
        course.timestamp,
        modules_addresses,
    );
    let new_version_course_entry = new_version_course.entry();

    hdk::update_entry(new_version_course_entry, course_address)
}

pub fn delete(address: Address) -> ZomeApiResult<Address> {
    hdk::remove_link(&anchor_address()?, &address, "course_list", "")?;

    let students = get_students(address.clone())?;
    let course: Course = hdk::utils::get_as_type(address.clone())?;

    for student in students {
        hdk::remove_link(&student, &address, "student->course", "")?;
    }
    hdk::remove_link(&course.teacher_address, &address, "teacher->courses", "")?;

    hdk::remove_entry(&address)
}

pub fn list() -> ZomeApiResult<Vec<Address>> {
    let addresses = hdk::get_links(
        &anchor_address()?,
        LinkMatch::Exactly("course_list"),
        LinkMatch::Any,
    )?
    .addresses();

    Ok(addresses)
}

pub fn get_my_courses() -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &AGENT_ADDRESS,
        LinkMatch::Exactly("teacher->courses"),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

pub fn get_my_enrolled_courses() -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &AGENT_ADDRESS,
        LinkMatch::Exactly("student->courses"),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

pub fn add_module_to_course(
    course_address: &Address,
    module_address: &Address,
) -> ZomeApiResult<Address> {
    let current_course = hdk::get_entry(course_address).unwrap().unwrap();
    if let Entry::App(_, current_course) = current_course {
        let mut course_entry = Course::try_from(current_course.clone())
            .expect("Entry at this address is not Course. You sent a wrong address");
        course_entry.modules.push(module_address.clone());
        hdk::update_entry(
            Entry::App("course".into(), course_entry.into()),
            course_address,
        )
    } else {
        panic!("This address is not a valid address")
    }
}

// NOTE: fun fact for fellow English learners: there isn't a typo because both "enrol" and "enroll" are valid!
//  See: https://grammarist.com/spelling/enrol-enroll/ for more details
pub fn enrol_in_course(course_address: Address) -> ZomeApiResult<Address> {
    hdk::link_entries(&AGENT_ADDRESS, &course_address, "student->courses", "")?;
    hdk::link_entries(&course_address, &AGENT_ADDRESS, "course->students", "")
}

pub fn get_students(course_address: Address) -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &course_address,
        LinkMatch::Exactly("course->students"),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}