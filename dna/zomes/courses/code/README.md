# Course zome

This zome implements functionality for the learning Course that contains Sections that contain Content.

## TODO

 * receive timestamp from UI in all update functions
 

 ## Questions to clarify

 * why does `tag: ""` in `hdk::link_entries` compile ok in content::handlers but fails in section::handlers? what's different?
 * why does section::delete own it's parameter `section_anchor_address` and section::update accepts only reference? is there a particular logic behind it?
