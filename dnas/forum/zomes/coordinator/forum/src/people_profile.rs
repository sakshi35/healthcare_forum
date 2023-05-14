use hdk::prelude::*;
use forum_integrity::*;
#[hdk_extern]
pub fn create_people_profile(people_profile: PeopleProfile) -> ExternResult<Record> {
    let people_profile_hash = create_entry(
        &EntryTypes::PeopleProfile(people_profile.clone()),
    )?;
    create_link(
        people_profile.person.clone(),
        people_profile_hash.clone(),
        LinkTypes::PersonToPeopleProfiles,
        (),
    )?;
    let record = get(people_profile_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created PeopleProfile"))
            ),
        )?;
    let path = Path::from("all_people_profiles");
    create_link(
        path.path_entry_hash()?,
        people_profile_hash.clone(),
        LinkTypes::AllPeopleProfiles,
        (),
    )?;
    let my_agent_pub_key = agent_info()?.agent_latest_pubkey;
    create_link(
        my_agent_pub_key,
        people_profile_hash.clone(),
        LinkTypes::MyProfiles,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_people_profile(
    original_people_profile_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_people_profile_hash.clone(),
        LinkTypes::PeopleProfileUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_people_profile_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_people_profile_hash.clone(),
    };
    get(latest_people_profile_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdatePeopleProfileInput {
    pub original_people_profile_hash: ActionHash,
    pub previous_people_profile_hash: ActionHash,
    pub updated_people_profile: PeopleProfile,
}
#[hdk_extern]
pub fn update_people_profile(input: UpdatePeopleProfileInput) -> ExternResult<Record> {
    let updated_people_profile_hash = update_entry(
        input.previous_people_profile_hash.clone(),
        &input.updated_people_profile,
    )?;
    create_link(
        input.original_people_profile_hash.clone(),
        updated_people_profile_hash.clone(),
        LinkTypes::PeopleProfileUpdates,
        (),
    )?;
    let record = get(updated_people_profile_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated PeopleProfile"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_people_profile(
    original_people_profile_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_people_profile_hash)
}
#[hdk_extern]
pub fn get_people_profiles_for_person(person: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(person, LinkTypes::PersonToPeopleProfiles, None)?;
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
