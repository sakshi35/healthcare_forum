use hdk::prelude::*;
use forum_integrity::*;
#[hdk_extern]
pub fn create_h_blog(h_blog: HBlog) -> ExternResult<Record> {
    let h_blog_hash = create_entry(&EntryTypes::HBlog(h_blog.clone()))?;
    create_link(
        h_blog.creator.clone(),
        h_blog_hash.clone(),
        LinkTypes::CreatorToHBlogs,
        (),
    )?;
    let record = get(h_blog_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created HBlog"))
            ),
        )?;
    let path = Path::from("all_h_blogs");
    create_link(path.path_entry_hash()?, h_blog_hash.clone(), LinkTypes::AllHBlogs, ())?;
    let my_agent_pub_key = agent_info()?.agent_latest_pubkey;
    create_link(my_agent_pub_key, h_blog_hash.clone(), LinkTypes::MyHBlogs, ())?;
    Ok(record)
}
#[hdk_extern]
pub fn get_h_blog(original_h_blog_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(original_h_blog_hash.clone(), LinkTypes::HBlogUpdates, None)?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_h_blog_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_h_blog_hash.clone(),
    };
    get(latest_h_blog_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateHBlogInput {
    pub original_h_blog_hash: ActionHash,
    pub previous_h_blog_hash: ActionHash,
    pub updated_h_blog: HBlog,
}
#[hdk_extern]
pub fn update_h_blog(input: UpdateHBlogInput) -> ExternResult<Record> {
    let updated_h_blog_hash = update_entry(
        input.previous_h_blog_hash.clone(),
        &input.updated_h_blog,
    )?;
    create_link(
        input.original_h_blog_hash.clone(),
        updated_h_blog_hash.clone(),
        LinkTypes::HBlogUpdates,
        (),
    )?;
    let record = get(updated_h_blog_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated HBlog"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_h_blog(original_h_blog_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_h_blog_hash)
}
#[hdk_extern]
pub fn get_h_blogs_for_creator(creator: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(creator, LinkTypes::CreatorToHBlogs, None)?;
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
