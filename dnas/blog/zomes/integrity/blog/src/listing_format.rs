use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct ListingFormat {
    pub listing_hash: EntryHash,
    pub description: String,
    pub quantity: i32,
}
pub fn validate_create_listing_format(
    _action: EntryCreationAction,
    listing_format: ListingFormat,
) -> ExternResult<ValidateCallbackResult> {
    let entry = must_get_entry(listing_format.listing_hash.clone())?;
    let _listing = crate::Listing::try_from(entry)?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_listing_format(
    _action: Update,
    _listing_format: ListingFormat,
    _original_action: EntryCreationAction,
    _original_listing_format: ListingFormat,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_listing_format(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_listing_format: ListingFormat,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_listing_to_listing_formats(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let entry_hash = EntryHash::from(base_address);
    let entry = must_get_entry(entry_hash)?.content;
    let _listing = crate::Listing::try_from(entry)?;
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _listing_format: crate::ListingFormat = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_listing_to_listing_formats(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("ListingToListingFormats links cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_listing_format_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _listing_format: crate::ListingFormat = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _listing_format: crate::ListingFormat = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_listing_format_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("ListingFormatUpdates links cannot be deleted"),
        ),
    )
}
