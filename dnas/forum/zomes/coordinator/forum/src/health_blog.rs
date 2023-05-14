use hdk::prelude::*;
use forum_integrity::*;
#[hdk_extern]
pub fn create_health_blog(health_blog: HealthBlog) -> ExternResult<Record> {
    let health_blog_hash = create_entry(&EntryTypes::HealthBlog(health_blog.clone()))?;
    create_link(
        health_blog.creator.clone(),
        health_blog_hash.clone(),
        LinkTypes::CreatorToHealthBlogs,
        (),
    )?;
    create_link(
        health_blog.people_profile_hash.clone(),
        health_blog_hash.clone(),
        LinkTypes::PeopleProfileToHealthBlogs,
        (),
    )?;
    let record = get(health_blog_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created HealthBlog"))
            ),
        )?;
    let path = Path::from("all_health_blogs");
    create_link(
        path.path_entry_hash()?,
        health_blog_hash.clone(),
        LinkTypes::AllHealthBlogs,
        (),
    )?;
    let my_agent_pub_key = agent_info()?.agent_latest_pubkey;
    create_link(
        my_agent_pub_key,
        health_blog_hash.clone(),
        LinkTypes::MyHealthBlogs,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_health_blog(
    original_health_blog_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_health_blog_hash.clone(),
        LinkTypes::HealthBlogUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_health_blog_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_health_blog_hash.clone(),
    };
    get(latest_health_blog_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHealthBlogInput {
    pub original_health_blog_hash: ActionHash,
    pub previous_health_blog_hash: ActionHash,
    pub updated_health_blog: HealthBlog,
}
#[hdk_extern]
pub fn update_health_blog(input: UpdateHealthBlogInput) -> ExternResult<Record> {
    let updated_health_blog_hash = update_entry(
        input.previous_health_blog_hash.clone(),
        &input.updated_health_blog,
    )?;
    create_link(
        input.original_health_blog_hash.clone(),
        updated_health_blog_hash.clone(),
        LinkTypes::HealthBlogUpdates,
        (),
    )?;
    let record = get(updated_health_blog_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated HealthBlog"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_health_blog(
    original_health_blog_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_health_blog_hash)
}
#[hdk_extern]
pub fn get_health_blogs_for_creator(creator: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(creator, LinkTypes::CreatorToHealthBlogs, None)?;
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
pub fn get_health_blogs_for_people_profile(
    people_profile_hash: EntryHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(
        people_profile_hash,
        LinkTypes::PeopleProfileToHealthBlogs,
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
