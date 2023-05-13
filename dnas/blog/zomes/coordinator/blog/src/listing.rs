use hdk::prelude::*;
use blog_integrity::*;
#[hdk_extern]
pub fn create_listing(listing: Listing) -> ExternResult<Record> {
    let listing_hash = create_entry(&EntryTypes::Listing(listing.clone()))?;
    create_link(
        listing.creator.clone(),
        listing_hash.clone(),
        LinkTypes::CreatorToListings,
        (),
    )?;
    let record = get(listing_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Listing"))
            ),
        )?;
    let path = Path::from("all_listings");
    create_link(
        path.path_entry_hash()?,
        listing_hash.clone(),
        LinkTypes::AllListings,
        (),
    )?;
    let my_agent_pub_key = agent_info()?.agent_latest_pubkey;
    create_link(
        my_agent_pub_key,
        listing_hash.clone(),
        LinkTypes::ListingsByCreator,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_listing(original_listing_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_listing_hash.clone(),
        LinkTypes::ListingUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_listing_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_listing_hash.clone(),
    };
    get(latest_listing_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateListingInput {
    pub original_listing_hash: ActionHash,
    pub previous_listing_hash: ActionHash,
    pub updated_listing: Listing,
}
#[hdk_extern]
pub fn update_listing(input: UpdateListingInput) -> ExternResult<Record> {
    let updated_listing_hash = update_entry(
        input.previous_listing_hash.clone(),
        &input.updated_listing,
    )?;
    create_link(
        input.original_listing_hash.clone(),
        updated_listing_hash.clone(),
        LinkTypes::ListingUpdates,
        (),
    )?;
    let record = get(updated_listing_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Listing"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_listing(original_listing_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_listing_hash)
}
#[hdk_extern]
pub fn get_listings_for_creator(creator: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(creator, LinkTypes::CreatorToListings, None)?;
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
