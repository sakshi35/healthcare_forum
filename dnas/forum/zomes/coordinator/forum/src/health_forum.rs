use hdk::prelude::*;
use forum_integrity::*;
#[hdk_extern]
pub fn create_health_forum(health_forum: HealthForum) -> ExternResult<Record> {
    let health_forum_hash = create_entry(
        &EntryTypes::HealthForum(health_forum.clone()),
    )?;
    create_link(
        health_forum.creator.clone(),
        health_forum_hash.clone(),
        LinkTypes::CreatorToHealthForums,
        (),
    )?;
    create_link(
        health_forum.people_profile_hash.clone(),
        health_forum_hash.clone(),
        LinkTypes::PeopleProfileToHealthForums,
        (),
    )?;
    let record = get(health_forum_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created HealthForum"))
            ),
        )?;
    let path = Path::from("all_health_forum");
    create_link(
        path.path_entry_hash()?,
        health_forum_hash.clone(),
        LinkTypes::AllHealthForum,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_health_forum(
    original_health_forum_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_health_forum_hash.clone(),
        LinkTypes::HealthForumUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_health_forum_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_health_forum_hash.clone(),
    };
    get(latest_health_forum_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHealthForumInput {
    pub original_health_forum_hash: ActionHash,
    pub previous_health_forum_hash: ActionHash,
    pub updated_health_forum: HealthForum,
}
#[hdk_extern]
pub fn update_health_forum(input: UpdateHealthForumInput) -> ExternResult<Record> {
    let updated_health_forum_hash = update_entry(
        input.previous_health_forum_hash.clone(),
        &input.updated_health_forum,
    )?;
    create_link(
        input.original_health_forum_hash.clone(),
        updated_health_forum_hash.clone(),
        LinkTypes::HealthForumUpdates,
        (),
    )?;
    let record = get(updated_health_forum_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated HealthForum"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_health_forum(
    original_health_forum_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_health_forum_hash)
}
#[hdk_extern]
pub fn get_health_forums_for_creator(creator: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(creator, LinkTypes::CreatorToHealthForums, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
#[hdk_extern]
pub fn get_health_forums_for_people_profile(
    people_profile_hash: EntryHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(
        people_profile_hash,
        LinkTypes::PeopleProfileToHealthForums,
        None,
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
            GetOptions::default(),
        ))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
