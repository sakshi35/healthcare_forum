use hdk::prelude::*;
use forum_integrity::*;
#[hdk_extern]
pub fn get_my_health_blogs(author: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(author, LinkTypes::MyHealthBlogs, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}
