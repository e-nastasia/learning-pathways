use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

use crate::anchor_trait::AnchorTrait;
use crate::course::entry::Course;
use crate::course;
use super::entry::Section;
use super::anchor::SectionAnchor;

pub fn create(title: String, course_address: &Address, timestamp: u64) -> ZomeApiResult<Address> {
    // retrieve course at course_address. If this address isn't valid, we'll fail here, so it serves as input validation
    // we won't be using this course instance so we prefix it with _ symbol
    let  _course: Course = hdk::utils::get_as_type(course_address.clone())?;
    
    // initialize SectionAnchor instance
    let section_anchor = SectionAnchor::new(title.clone(), course_address.clone(), timestamp);
    // commit SectionAnchor to DHT
    let section_anchor_address = hdk::commit_entry(&section_anchor.entry())?;

    // initialize Section instance without commiting it to DHT: we'll need it to commit anchor
    let new_section = Section::new(title, course_address.clone(), timestamp, section_anchor_address.clone());
    // commit Section to DHT
    let new_section_address = hdk::commit_entry(&new_section.entry())?;

    hdk::link_entries(
        &section_anchor_address,
        &new_section_address,
        SectionAnchor::link_type(),
        "".to_string(),
    )?;

    course::handlers::add_section(&course_address, &section_anchor_address)?;
    // SectionAnchor serves as this section's ID so we return it
    Ok(section_anchor_address)
}

pub fn update(title: String, section_anchor_address: &Address) -> ZomeApiResult<Address> {
    let section_addresses = hdk::get_links(
        section_anchor_address,
        LinkMatch::Exactly(&SectionAnchor::link_type()),
        // this parameter is for link tags. since we don't tag section anchor link (see method create above)
        //  we can ask for all tags
        LinkMatch::Any, 
    )?.addresses();

    // Q: could we assume that this list would only have a single entry? 
    //  because there's only one agent in the entire DNA that could make that change: the one listed as course teacher.
    let previous_section_address = &section_addresses[0];
    let mut previous_section: Section = hdk::utils::get_as_type(previous_section_address.clone())?;

    // update the section
    previous_section.title = title;
    // commit this update to the DHT.
    let new_section_address = hdk::update_entry(previous_section.entry(), &previous_section_address)?;

    // remove link to previous version of section
    hdk::remove_link(
        section_anchor_address,
        &previous_section_address,
        SectionAnchor::link_type(),
        "".to_string(),
    )?;

    // create link to new version of section
    hdk::link_entries(
        section_anchor_address,
        &new_section_address,
        SectionAnchor::link_type(),
        "".to_string(),
    )?;

    Ok(new_section_address)
}

// TODO: Q: why does delete own section_anchor_address and update accepts only reference? is there a particular logic behind it?
pub fn delete(section_anchor_address: Address) -> ZomeApiResult<Address> {
    let section_anchor: SectionAnchor = hdk::utils::get_as_type(section_anchor_address.clone())?;

    // NOTE: we're using the fact that anchor contains course_address and that we don't allow
    //  to change course_address in a section entry.
    // By doing so, we avoid necessity to query links of the section_anchor to retrieve the latest section entry
    // which makes this method a little bit faster
    course::handlers::delete_section(&section_anchor.course_address, &section_anchor_address)?;

    // NOTE: let's try only deleting an anchor! (and don't touch links from anchor to section entry and section entry itself)
    // reasons:
    // 1) without it, we won't be able to reach the section because everywhere we link to section we only use anchor address
    // 2) we'll avoid polluting DHT by new deletion metadata
    let result = hdk::remove_entry(&section_anchor_address)?;
    Ok(result)
}
