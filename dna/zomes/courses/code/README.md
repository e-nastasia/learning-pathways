# Course zome

This zome implements functionality for the learning Course that contains Sections that contain Content.

## TODO

 * receive timestamp from UI in all update functions
 * validation for SectionAnchor
 * validation for CourseAnchor
 * error handling with ZomeApiError

 ## Questions to clarify

 * why does `tag: ""` in `hdk::link_entries` compile ok in content::handlers but requires adding `to_string()` in section::handlers? what's different? (e-nastasia: this is driving me crazy! send help)
 * why does section::delete own it's parameter `section_anchor_address` and section::update accepts only reference? is there a particular logic behind it?
* could we assume that list of links from anchor to entry would only have a single entry? This is applicable to SectionAnchor -> Section and CourseAnchor -> Course link lists. Update method always removes the previous link and the only reason we'll have more than 1 link from anchor to entry is when update failed to remove a previous one. And even in this case we'll be able to get the latest entry because Holochain metadata would point us to the latest version