# Course zome

This zome implements functionality for the learning Course that contains Sections that contain Content.

## TODO

* refactor all the ownership transfers: there shouldn't be any unneeded cloning
 * receive timestamp from UI in all update functions
 * refactor .to_string() to use https://doc.rust-lang.org/std/borrow/trait.ToOwned.html instead as it is more a more general way to do the same thing
 * refactor validation for Section
* validation for SectionAnchor
* validation for CourseAnchor
 * error handling with ZomeApiError

 ## Questions to clarify

 * why does `tag: ""` in `hdk::link_entries` compile ok in content::handlers but requires adding `to_string()` in section::handlers? what's different? (e-nastasia: this is driving me crazy! send help)
 * why does section::delete own it's parameter `section_anchor_address` and section::update accepts only reference? is there a particular logic behind it?
* could we assume that list of links from anchor to entry would only have a single entry? This is applicable to SectionAnchor -> Section and CourseAnchor -> Course link lists. Update method always removes the previous link and the only reason we'll have more than 1 link from anchor to entry is when update failed to remove a previous one. And even in this case we'll be able to get the latest entry because Holochain metadata would point us to the latest version
    * what if we run update, remove link to previous version and then fail creating link to new version? what would happen then? links list would be empty then!
    * UPD: no we can't assume that, it would be highly unreliable. We need to handle the case when there are no Live links (== no links that haven't been deleted). To do that we can introduce integer versioning for the links that would allow us to find the latest link in the list of all deleted links, and then we can take entry address from this link to use in update. This way, we'll be restoring the structure we have with anchor & entry.
