pub mod profile;
pub use profile::*;
pub mod listing_format;
pub use listing_format::*;
pub mod listing;
pub use listing::*;
pub mod name;
pub use name::*;
use hdi::prelude::*;
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Name(Name),
    Listing(Listing),
    ListingFormat(ListingFormat),
    Profile(Profile),
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    NameUpdates,
    AllPosts,
    CreatorToListings,
    ListingUpdates,
    AllListings,
    ListingsByCreator,
    ListingToListingFormats,
    ListingFormatUpdates,
    PersonToProfiles,
    ProfileUpdates,
    AllProfiles,
}
#[hdk_extern]
pub fn genesis_self_check(
    _data: GenesisSelfCheckData,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => {
            match store_entry {
                OpEntry::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::Name(name) => {
                            validate_create_name(
                                EntryCreationAction::Create(action),
                                name,
                            )
                        }
                        EntryTypes::Listing(listing) => {
                            validate_create_listing(
                                EntryCreationAction::Create(action),
                                listing,
                            )
                        }
                        EntryTypes::ListingFormat(listing_format) => {
                            validate_create_listing_format(
                                EntryCreationAction::Create(action),
                                listing_format,
                            )
                        }
                        EntryTypes::Profile(profile) => {
                            validate_create_profile(
                                EntryCreationAction::Create(action),
                                profile,
                            )
                        }
                    }
                }
                OpEntry::UpdateEntry { app_entry, action, .. } => {
                    match app_entry {
                        EntryTypes::Name(name) => {
                            validate_create_name(
                                EntryCreationAction::Update(action),
                                name,
                            )
                        }
                        EntryTypes::Listing(listing) => {
                            validate_create_listing(
                                EntryCreationAction::Update(action),
                                listing,
                            )
                        }
                        EntryTypes::ListingFormat(listing_format) => {
                            validate_create_listing_format(
                                EntryCreationAction::Update(action),
                                listing_format,
                            )
                        }
                        EntryTypes::Profile(profile) => {
                            validate_create_profile(
                                EntryCreationAction::Update(action),
                                profile,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterUpdate(update_entry) => {
            match update_entry {
                OpUpdate::Entry {
                    original_action,
                    original_app_entry,
                    app_entry,
                    action,
                } => {
                    match (app_entry, original_app_entry) {
                        (
                            EntryTypes::Profile(profile),
                            EntryTypes::Profile(original_profile),
                        ) => {
                            validate_update_profile(
                                action,
                                profile,
                                original_action,
                                original_profile,
                            )
                        }
                        (
                            EntryTypes::ListingFormat(listing_format),
                            EntryTypes::ListingFormat(original_listing_format),
                        ) => {
                            validate_update_listing_format(
                                action,
                                listing_format,
                                original_action,
                                original_listing_format,
                            )
                        }
                        (
                            EntryTypes::Listing(listing),
                            EntryTypes::Listing(original_listing),
                        ) => {
                            validate_update_listing(
                                action,
                                listing,
                                original_action,
                                original_listing,
                            )
                        }
                        (EntryTypes::Name(name), EntryTypes::Name(original_name)) => {
                            validate_update_name(
                                action,
                                name,
                                original_action,
                                original_name,
                            )
                        }
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original and updated entry types must be the same"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterDelete(delete_entry) => {
            match delete_entry {
                OpDelete::Entry { original_action, original_app_entry, action } => {
                    match original_app_entry {
                        EntryTypes::Name(name) => {
                            validate_delete_name(action, original_action, name)
                        }
                        EntryTypes::Listing(listing) => {
                            validate_delete_listing(action, original_action, listing)
                        }
                        EntryTypes::ListingFormat(listing_format) => {
                            validate_delete_listing_format(
                                action,
                                original_action,
                                listing_format,
                            )
                        }
                        EntryTypes::Profile(profile) => {
                            validate_delete_profile(action, original_action, profile)
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterCreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            action,
        } => {
            match link_type {
                LinkTypes::NameUpdates => {
                    validate_create_link_name_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllPosts => {
                    validate_create_link_all_posts(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CreatorToListings => {
                    validate_create_link_creator_to_listings(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ListingUpdates => {
                    validate_create_link_listing_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllListings => {
                    validate_create_link_all_listings(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ListingsByCreator => {
                    validate_create_link_listings_by_creator(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ListingToListingFormats => {
                    validate_create_link_listing_to_listing_formats(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ListingFormatUpdates => {
                    validate_create_link_listing_format_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::PersonToProfiles => {
                    validate_create_link_person_to_profiles(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ProfileUpdates => {
                    validate_create_link_profile_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllProfiles => {
                    validate_create_link_all_profiles(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        FlatOp::RegisterDeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        } => {
            match link_type {
                LinkTypes::NameUpdates => {
                    validate_delete_link_name_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllPosts => {
                    validate_delete_link_all_posts(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CreatorToListings => {
                    validate_delete_link_creator_to_listings(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ListingUpdates => {
                    validate_delete_link_listing_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllListings => {
                    validate_delete_link_all_listings(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ListingsByCreator => {
                    validate_delete_link_listings_by_creator(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ListingToListingFormats => {
                    validate_delete_link_listing_to_listing_formats(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ListingFormatUpdates => {
                    validate_delete_link_listing_format_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::PersonToProfiles => {
                    validate_delete_link_person_to_profiles(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ProfileUpdates => {
                    validate_delete_link_profile_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllProfiles => {
                    validate_delete_link_all_profiles(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        FlatOp::StoreRecord(store_record) => {
            match store_record {
                OpRecord::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::Name(name) => {
                            validate_create_name(
                                EntryCreationAction::Create(action),
                                name,
                            )
                        }
                        EntryTypes::Listing(listing) => {
                            validate_create_listing(
                                EntryCreationAction::Create(action),
                                listing,
                            )
                        }
                        EntryTypes::ListingFormat(listing_format) => {
                            validate_create_listing_format(
                                EntryCreationAction::Create(action),
                                listing_format,
                            )
                        }
                        EntryTypes::Profile(profile) => {
                            validate_create_profile(
                                EntryCreationAction::Create(action),
                                profile,
                            )
                        }
                    }
                }
                OpRecord::UpdateEntry {
                    original_action_hash,
                    app_entry,
                    action,
                    ..
                } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for an update must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match app_entry {
                        EntryTypes::Name(name) => {
                            let result = validate_create_name(
                                EntryCreationAction::Update(action.clone()),
                                name.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_name: Option<Name> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_name = match original_name {
                                    Some(name) => name,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_name(
                                    action,
                                    name,
                                    original_action,
                                    original_name,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::Listing(listing) => {
                            let result = validate_create_listing(
                                EntryCreationAction::Update(action.clone()),
                                listing.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_listing: Option<Listing> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_listing = match original_listing {
                                    Some(listing) => listing,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_listing(
                                    action,
                                    listing,
                                    original_action,
                                    original_listing,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::ListingFormat(listing_format) => {
                            let result = validate_create_listing_format(
                                EntryCreationAction::Update(action.clone()),
                                listing_format.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_listing_format: Option<ListingFormat> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_listing_format = match original_listing_format {
                                    Some(listing_format) => listing_format,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_listing_format(
                                    action,
                                    listing_format,
                                    original_action,
                                    original_listing_format,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::Profile(profile) => {
                            let result = validate_create_profile(
                                EntryCreationAction::Update(action.clone()),
                                profile.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_profile: Option<Profile> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_profile = match original_profile {
                                    Some(profile) => profile,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_profile(
                                    action,
                                    profile,
                                    original_action,
                                    original_profile,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                    }
                }
                OpRecord::DeleteEntry { original_action_hash, action, .. } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for a delete must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let app_entry_type = match original_action.entry_type() {
                        EntryType::App(app_entry_type) => app_entry_type,
                        _ => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    let entry = match original_record.entry().as_option() {
                        Some(entry) => entry,
                        None => {
                            if original_action.entry_type().visibility().is_public() {
                                return Ok(
                                    ValidateCallbackResult::Invalid(
                                        "Original record for a delete of a public entry must contain an entry"
                                            .to_string(),
                                    ),
                                );
                            } else {
                                return Ok(ValidateCallbackResult::Valid);
                            }
                        }
                    };
                    let original_app_entry = match EntryTypes::deserialize_from_type(
                        app_entry_type.zome_index.clone(),
                        app_entry_type.entry_index.clone(),
                        &entry,
                    )? {
                        Some(app_entry) => app_entry,
                        None => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match original_app_entry {
                        EntryTypes::Name(original_name) => {
                            validate_delete_name(action, original_action, original_name)
                        }
                        EntryTypes::Listing(original_listing) => {
                            validate_delete_listing(
                                action,
                                original_action,
                                original_listing,
                            )
                        }
                        EntryTypes::ListingFormat(original_listing_format) => {
                            validate_delete_listing_format(
                                action,
                                original_action,
                                original_listing_format,
                            )
                        }
                        EntryTypes::Profile(original_profile) => {
                            validate_delete_profile(
                                action,
                                original_action,
                                original_profile,
                            )
                        }
                    }
                }
                OpRecord::CreateLink {
                    base_address,
                    target_address,
                    tag,
                    link_type,
                    action,
                } => {
                    match link_type {
                        LinkTypes::NameUpdates => {
                            validate_create_link_name_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllPosts => {
                            validate_create_link_all_posts(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CreatorToListings => {
                            validate_create_link_creator_to_listings(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::ListingUpdates => {
                            validate_create_link_listing_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllListings => {
                            validate_create_link_all_listings(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::ListingsByCreator => {
                            validate_create_link_listings_by_creator(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::ListingToListingFormats => {
                            validate_create_link_listing_to_listing_formats(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::ListingFormatUpdates => {
                            validate_create_link_listing_format_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::PersonToProfiles => {
                            validate_create_link_person_to_profiles(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::ProfileUpdates => {
                            validate_create_link_profile_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllProfiles => {
                            validate_create_link_all_profiles(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                    }
                }
                OpRecord::DeleteLink { original_action_hash, base_address, action } => {
                    let record = must_get_valid_record(original_action_hash)?;
                    let create_link = match record.action() {
                        Action::CreateLink(create_link) => create_link.clone(),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "The action that a DeleteLink deletes must be a CreateLink"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let link_type = match LinkTypes::from_type(
                        create_link.zome_index.clone(),
                        create_link.link_type.clone(),
                    )? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    match link_type {
                        LinkTypes::NameUpdates => {
                            validate_delete_link_name_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllPosts => {
                            validate_delete_link_all_posts(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CreatorToListings => {
                            validate_delete_link_creator_to_listings(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ListingUpdates => {
                            validate_delete_link_listing_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllListings => {
                            validate_delete_link_all_listings(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ListingsByCreator => {
                            validate_delete_link_listings_by_creator(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ListingToListingFormats => {
                            validate_delete_link_listing_to_listing_formats(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ListingFormatUpdates => {
                            validate_delete_link_listing_format_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::PersonToProfiles => {
                            validate_delete_link_person_to_profiles(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ProfileUpdates => {
                            validate_delete_link_profile_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllProfiles => {
                            validate_delete_link_all_profiles(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                    }
                }
                OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterAgentActivity(agent_activity) => {
            match agent_activity {
                OpActivity::CreateAgent { agent, action } => {
                    let previous_action = must_get_action(action.prev_action)?;
                    match previous_action.action() {
                        Action::AgentValidationPkg(
                            AgentValidationPkg { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
    }
}
