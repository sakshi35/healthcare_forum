use hdk::prelude::*;
use blog_integrity::*;
#[hdk_extern]
pub fn create_name(name: Name) -> ExternResult<Record> {
    let name_hash = create_entry(&EntryTypes::Name(name.clone()))?;
    let record = get(name_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Name"))
            ),
        )?;
    let path = Path::from("all_posts");
    create_link(path.path_entry_hash()?, name_hash.clone(), LinkTypes::AllPosts, ())?;
    Ok(record)
}
#[hdk_extern]
pub fn get_name(original_name_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(original_name_hash.clone(), LinkTypes::NameUpdates, None)?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_name_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_name_hash.clone(),
    };
    get(latest_name_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNameInput {
    pub original_name_hash: ActionHash,
    pub previous_name_hash: ActionHash,
    pub updated_name: Name,
}
#[hdk_extern]
pub fn update_name(input: UpdateNameInput) -> ExternResult<Record> {
    let updated_name_hash = update_entry(
        input.previous_name_hash.clone(),
        &input.updated_name,
    )?;
    create_link(
        input.original_name_hash.clone(),
        updated_name_hash.clone(),
        LinkTypes::NameUpdates,
        (),
    )?;
    let record = get(updated_name_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Name"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_name(original_name_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_name_hash)
}
