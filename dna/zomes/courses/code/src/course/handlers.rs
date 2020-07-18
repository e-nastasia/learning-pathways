use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::LinkMatch;
use hdk::{error::ZomeApiResult, AGENT_ADDRESS};
use holochain_entry_utils::HolochainEntry;

use super::anchor::{
    CourseAnchor, 
    COURSE_ANCHOR_TO_STUDENT_LINK, 
    STUDENT_TO_COURSE_ANCHOR_LINK,
    TEACHER_TO_COURSE_ANCHOR_LINK,
};
use crate::anchor_trait::AnchorTrait;
use super::catalog_anchor::CourseCatalogAnchor;
use super::entry::Course;

pub fn create(title: String, timestamp: u64) -> ZomeApiResult<Address> {
    // if catalog anchor already exists, this function would just return it's address without actually writing anything
    // new to the DHT.
    let catalog_anchor_address = hdk::commit_entry(&CourseCatalogAnchor::new().entry())?;

    // just a helper variable because we'll need this value a few times
    let teacher_address = AGENT_ADDRESS.clone();

    // initialize CourseAnchor instance to represent this particular course
    let course_anchor = CourseAnchor::new(title.clone(), teacher_address.clone(), timestamp);
    // commit CourseAnchor to DHT
    let course_anchor_address = hdk::commit_entry(&course_anchor.entry())?;

    // create new Course entry
    let new_course = Course::new(
        title,
        Vec::default(), // section vector is empty by default
        teacher_address.to_string().into(),
        timestamp,
        course_anchor_address.clone(),
    );
    // commit this entry to DHT and save it's address
    let new_course_address = hdk::commit_entry(&new_course.entry())?;

    // link CourseAnchor to Course entry
    hdk::link_entries(
        &course_anchor_address,
        &new_course_address,
        CourseAnchor::link_to(),
        "".to_string(),
    )?;

    // link CourseCatalogAnchor to CourseAnchor entry for this course to be findable
    hdk::link_entries(
        &catalog_anchor_address,
        &course_anchor_address,
        CourseAnchor::link_to(),
        "".to_string(),
    )?;

    // link address of the agent who called course::create to CourseAnchor
    // for this course to be findable in the list of courses that agent teaches
    hdk::link_entries(
        &AGENT_ADDRESS,
        &course_anchor_address,
        TEACHER_TO_COURSE_ANCHOR_LINK,
        "",
    )?;

    Ok(new_course_address)
}

// NOTE: this function isn't public because it's only needed in the current module
fn get_latest_course(course_anchor_address: &Address) -> ZomeApiResult<(Course, Address)> {
    let course_addresses = hdk::get_links(
        course_anchor_address,
        LinkMatch::Exactly(&CourseAnchor::link_type()),
        // this parameter is for link tags. since we don't tag course anchor link (see method create above)
        //  we need to ask for all tags
        LinkMatch::Any,
    )?
    .addresses();

    // NOTE: we're assuming that this vec would only have one item in it.
    // Question about it is added into zome README.md
    let latest_course_address = course_addresses[0].clone();
    let latest_course: Course = hdk::utils::get_as_type(latest_course_address.clone())?;
    return Ok((latest_course, latest_course_address));
}

// NOTE: this function isn't public because it's only needed in the current module
fn commit_update(course: Course, previous_course_address: &Address, course_anchor_address: &Address) -> ZomeApiResult<Address> {
    // commit updated course to DHT and get it's new address
    let new_course_address = hdk::update_entry(course.entry(), previous_course_address)?;

    // remove link to previous version of course
    hdk::remove_link(
        course_anchor_address,
        &previous_course_address,
        CourseAnchor::link_type(),
        "".to_string(),
    )?;

    // create link to new version of course
    hdk::link_entries(
        course_anchor_address,
        &new_course_address,
        CourseAnchor::link_type(),
        "".to_string(),
    )?;

    Ok(new_course_address)
}

pub fn update(
    title: String,
    // NOTE(e-nastasia): since we have separate methods for section management 
    // (add_section and delete_section) we might not need to have sections_addresses 
    // here because it leaves us with inconsistent API. This needs further discussion.
    sections_addresses: Vec<Address>,
    course_anchor_address: &Address,
) -> ZomeApiResult<Address> {
    let (mut previous_course, previous_course_address) = get_latest_course(course_anchor_address)?;

    // update this course
    previous_course.title = title;
    previous_course.sections = sections_addresses;
    
    commit_update(previous_course, &previous_course_address, course_anchor_address)
}

pub fn delete(course_anchor_address: Address) -> ZomeApiResult<Address> {
    // retrieve course_anchor entry. If it doesn't exist, we'll fail with error here so we're also validating input
    let course_anchor: CourseAnchor = hdk::utils::get_as_type(course_anchor_address.clone())?;

    // remove link from CourseCatalogAnchor to CourseAnchor
    hdk::remove_link(
        &CourseCatalogAnchor::new().address()?,
        &course_anchor_address,
        CourseCatalogAnchor::link_type(),
        "".to_string(),
    )?;

    // retrieve list of students that have enrolled in this course
    let students = get_students(course_anchor_address.clone())?;
    // go through all students and remove their links to this course
    for student in students {
        hdk::remove_link(
            &student,
            &course_anchor_address,
            STUDENT_TO_COURSE_ANCHOR_LINK,
            "",
        )?;
    }

    // NOTE: using the fact that course_anchor stores teacher_address and that we don't allow to change teacher's address ever
    // so we don't have to retrieve the latest Course entry to get the teacher address and it makes this method a little bit faster
    hdk::remove_link(
        &course_anchor.teacher_address,
        &course_anchor_address,
        TEACHER_TO_COURSE_ANCHOR_LINK,
        "",
    )?;

    // NOTE: let's try only deleting an anchor! (and don't touch links from anchor to Course entry and Course entry itself)
    // reasons:
    // 1) without it, we won't be able to reach the Course because everywhere we link to course we only use anchor address
    // 2) we'll avoid polluting DHT by new deletion metadata
    hdk::remove_entry(&course_anchor_address)
}

pub fn list_all_courses() -> ZomeApiResult<Vec<Address>> {
    let addresses = hdk::get_links(
        &CourseCatalogAnchor::new().address()?,
        LinkMatch::Exactly(&CourseCatalogAnchor::link_type()),
        LinkMatch::Any,
    )?
    .addresses();

    Ok(addresses)
}

pub fn get_my_courses() -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &AGENT_ADDRESS,
        LinkMatch::Exactly(TEACHER_TO_COURSE_ANCHOR_LINK),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

pub fn get_my_enrolled_courses() -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &AGENT_ADDRESS,
        LinkMatch::Exactly(STUDENT_TO_COURSE_ANCHOR_LINK),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

pub fn add_section(
    course_anchor_address: &Address,
    section_anchor_address: &Address,
) -> ZomeApiResult<Address> {
    let (mut previous_course, previous_course_address) = get_latest_course(course_anchor_address)?;

    previous_course.sections.push(section_anchor_address.clone());
    // we won't use this new address but we need to save method's result somewhere
    // so this variable is prefixed with _
    let _new_course_address = commit_update(previous_course, &previous_course_address, course_anchor_address)?;

    Ok(course_anchor_address.clone())
}

pub fn delete_section(
    course_anchor_address: &Address,
    section_anchor_address: &Address,
) -> ZomeApiResult<Address> {
    let (mut previous_course, previous_course_address) = get_latest_course(course_anchor_address)?;

    previous_course.sections.remove_item(section_anchor_address);
    // we won't use this new address but we need to save method's result somewhere
    // so this variable is prefixed with _
    let _new_course_address = commit_update(previous_course, &previous_course_address, course_anchor_address)?;

    Ok(course_anchor_address.clone())
}

// NOTE: fun fact for fellow English learners: there isn't a typo because both "enrol" and "enroll" are valid!
//  See: https://grammarist.com/spelling/enrol-enroll/ for more details
pub fn enrol_in_course(course_anchor_address: Address) -> ZomeApiResult<Address> {
    hdk::link_entries(&AGENT_ADDRESS, &course_anchor_address, STUDENT_TO_COURSE_ANCHOR_LINK, "")?;
    hdk::link_entries(&course_anchor_address, &AGENT_ADDRESS, COURSE_ANCHOR_TO_STUDENT_LINK, "")
}

pub fn get_students(course_anchor_address: Address) -> ZomeApiResult<Vec<Address>> {
    let links = hdk::get_links(
        &course_anchor_address,
        LinkMatch::Exactly(COURSE_ANCHOR_TO_STUDENT_LINK),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}
