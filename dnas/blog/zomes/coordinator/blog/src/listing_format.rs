use hdk::prelude::*;
use blog_integrity::*;
#[hdk_extern]
pub fn create_listing_format(listing_format: ListingFormat) -> ExternResult<Record> {
    let listing_format_hash = create_entry(
        &EntryTypes::ListingFormat(listing_format.clone()),
    )?;
    create_link(
        listing_format.listing_hash.clone(),
        listing_format_hash.clone(),
        LinkTypes::ListingToListingFormats,
        (),
    )?;
    let record = get(listing_format_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created ListingFormat"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_listing_format(
    original_listing_format_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_listing_format_hash.clone(),
        LinkTypes::ListingFormatUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_listing_format_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_listing_format_hash.clone(),
    };
    get(latest_listing_format_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateListingFormatInput {
    pub original_listing_format_hash: ActionHash,
    pub previous_listing_format_hash: ActionHash,
    pub updated_listing_format: ListingFormat,
}
#[hdk_extern]
pub fn update_listing_format(input: UpdateListingFormatInput) -> ExternResult<Record> {
    let updated_listing_format_hash = update_entry(
        input.previous_listing_format_hash.clone(),
        &input.updated_listing_format,
    )?;
    create_link(
        input.original_listing_format_hash.clone(),
        updated_listing_format_hash.clone(),
        LinkTypes::ListingFormatUpdates,
        (),
    )?;
    let record = get(updated_listing_format_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated ListingFormat"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_listing_format(
    original_listing_format_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_listing_format_hash)
}
#[hdk_extern]
pub fn get_listing_formats_for_listing(
    listing_hash: EntryHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(listing_hash, LinkTypes::ListingToListingFormats, None)?;
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
