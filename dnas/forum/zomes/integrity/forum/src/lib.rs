pub mod h_blog;
pub use h_blog::*;
pub mod health_blog;
pub use health_blog::*;
pub mod health_forum;
pub use health_forum::*;
pub mod people_profile;
pub use people_profile::*;
use hdi::prelude::*;
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    PeopleProfile(PeopleProfile),
    HealthForum(HealthForum),
    HealthBlog(HealthBlog),
    HBlog(HBlog),
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    PersonToPeopleProfiles,
    PeopleProfileUpdates,
    AllPeopleProfiles,
    CreatorToHealthForums,
    PeopleProfileToHealthForums,
    HealthForumUpdates,
    AllHealthForum,
    MyProfiles,
    CreatorToHealthBlogs,
    PeopleProfileToHealthBlogs,
    HealthBlogUpdates,
    AllHealthBlogs,
    MyHealthBlogs,
    CreatorToHBlogs,
    HBlogUpdates,
    AllHBlogs,
    MyHBlogs,
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
                        EntryTypes::PeopleProfile(people_profile) => {
                            validate_create_people_profile(
                                EntryCreationAction::Create(action),
                                people_profile,
                            )
                        }
                        EntryTypes::HealthForum(health_forum) => {
                            validate_create_health_forum(
                                EntryCreationAction::Create(action),
                                health_forum,
                            )
                        }
                        EntryTypes::HealthBlog(health_blog) => {
                            validate_create_health_blog(
                                EntryCreationAction::Create(action),
                                health_blog,
                            )
                        }
                        EntryTypes::HBlog(h_blog) => {
                            validate_create_h_blog(
                                EntryCreationAction::Create(action),
                                h_blog,
                            )
                        }
                    }
                }
                OpEntry::UpdateEntry { app_entry, action, .. } => {
                    match app_entry {
                        EntryTypes::PeopleProfile(people_profile) => {
                            validate_create_people_profile(
                                EntryCreationAction::Update(action),
                                people_profile,
                            )
                        }
                        EntryTypes::HealthForum(health_forum) => {
                            validate_create_health_forum(
                                EntryCreationAction::Update(action),
                                health_forum,
                            )
                        }
                        EntryTypes::HealthBlog(health_blog) => {
                            validate_create_health_blog(
                                EntryCreationAction::Update(action),
                                health_blog,
                            )
                        }
                        EntryTypes::HBlog(h_blog) => {
                            validate_create_h_blog(
                                EntryCreationAction::Update(action),
                                h_blog,
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
                            EntryTypes::HBlog(h_blog),
                            EntryTypes::HBlog(original_h_blog),
                        ) => {
                            validate_update_h_blog(
                                action,
                                h_blog,
                                original_action,
                                original_h_blog,
                            )
                        }
                        (
                            EntryTypes::HealthBlog(health_blog),
                            EntryTypes::HealthBlog(original_health_blog),
                        ) => {
                            validate_update_health_blog(
                                action,
                                health_blog,
                                original_action,
                                original_health_blog,
                            )
                        }
                        (
                            EntryTypes::HealthForum(health_forum),
                            EntryTypes::HealthForum(original_health_forum),
                        ) => {
                            validate_update_health_forum(
                                action,
                                health_forum,
                                original_action,
                                original_health_forum,
                            )
                        }
                        (
                            EntryTypes::PeopleProfile(people_profile),
                            EntryTypes::PeopleProfile(original_people_profile),
                        ) => {
                            validate_update_people_profile(
                                action,
                                people_profile,
                                original_action,
                                original_people_profile,
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
                        EntryTypes::PeopleProfile(people_profile) => {
                            validate_delete_people_profile(
                                action,
                                original_action,
                                people_profile,
                            )
                        }
                        EntryTypes::HealthForum(health_forum) => {
                            validate_delete_health_forum(
                                action,
                                original_action,
                                health_forum,
                            )
                        }
                        EntryTypes::HealthBlog(health_blog) => {
                            validate_delete_health_blog(
                                action,
                                original_action,
                                health_blog,
                            )
                        }
                        EntryTypes::HBlog(h_blog) => {
                            validate_delete_h_blog(action, original_action, h_blog)
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
                LinkTypes::PersonToPeopleProfiles => {
                    validate_create_link_person_to_people_profiles(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::PeopleProfileUpdates => {
                    validate_create_link_people_profile_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllPeopleProfiles => {
                    validate_create_link_all_people_profiles(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CreatorToHealthForums => {
                    validate_create_link_creator_to_health_forums(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::PeopleProfileToHealthForums => {
                    validate_create_link_people_profile_to_health_forums(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::HealthForumUpdates => {
                    validate_create_link_health_forum_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllHealthForum => {
                    validate_create_link_all_health_forum(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::MyProfiles => {
                    validate_create_link_my_profiles(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CreatorToHealthBlogs => {
                    validate_create_link_creator_to_health_blogs(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::PeopleProfileToHealthBlogs => {
                    validate_create_link_people_profile_to_health_blogs(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::HealthBlogUpdates => {
                    validate_create_link_health_blog_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllHealthBlogs => {
                    validate_create_link_all_health_blogs(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::MyHealthBlogs => {
                    validate_create_link_my_health_blogs(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CreatorToHBlogs => {
                    validate_create_link_creator_to_h_blogs(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::HBlogUpdates => {
                    validate_create_link_h_blog_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllHBlogs => {
                    validate_create_link_all_h_blogs(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::MyHBlogs => {
                    validate_create_link_my_h_blogs(
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
                LinkTypes::PersonToPeopleProfiles => {
                    validate_delete_link_person_to_people_profiles(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::PeopleProfileUpdates => {
                    validate_delete_link_people_profile_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllPeopleProfiles => {
                    validate_delete_link_all_people_profiles(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CreatorToHealthForums => {
                    validate_delete_link_creator_to_health_forums(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::PeopleProfileToHealthForums => {
                    validate_delete_link_people_profile_to_health_forums(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::HealthForumUpdates => {
                    validate_delete_link_health_forum_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllHealthForum => {
                    validate_delete_link_all_health_forum(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::MyProfiles => {
                    validate_delete_link_my_profiles(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CreatorToHealthBlogs => {
                    validate_delete_link_creator_to_health_blogs(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::PeopleProfileToHealthBlogs => {
                    validate_delete_link_people_profile_to_health_blogs(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::HealthBlogUpdates => {
                    validate_delete_link_health_blog_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllHealthBlogs => {
                    validate_delete_link_all_health_blogs(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::MyHealthBlogs => {
                    validate_delete_link_my_health_blogs(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CreatorToHBlogs => {
                    validate_delete_link_creator_to_h_blogs(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::HBlogUpdates => {
                    validate_delete_link_h_blog_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllHBlogs => {
                    validate_delete_link_all_h_blogs(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::MyHBlogs => {
                    validate_delete_link_my_h_blogs(
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
                        EntryTypes::PeopleProfile(people_profile) => {
                            validate_create_people_profile(
                                EntryCreationAction::Create(action),
                                people_profile,
                            )
                        }
                        EntryTypes::HealthForum(health_forum) => {
                            validate_create_health_forum(
                                EntryCreationAction::Create(action),
                                health_forum,
                            )
                        }
                        EntryTypes::HealthBlog(health_blog) => {
                            validate_create_health_blog(
                                EntryCreationAction::Create(action),
                                health_blog,
                            )
                        }
                        EntryTypes::HBlog(h_blog) => {
                            validate_create_h_blog(
                                EntryCreationAction::Create(action),
                                h_blog,
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
                        EntryTypes::PeopleProfile(people_profile) => {
                            let result = validate_create_people_profile(
                                EntryCreationAction::Update(action.clone()),
                                people_profile.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_people_profile: Option<PeopleProfile> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_people_profile = match original_people_profile {
                                    Some(people_profile) => people_profile,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_people_profile(
                                    action,
                                    people_profile,
                                    original_action,
                                    original_people_profile,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::HealthForum(health_forum) => {
                            let result = validate_create_health_forum(
                                EntryCreationAction::Update(action.clone()),
                                health_forum.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_health_forum: Option<HealthForum> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_health_forum = match original_health_forum {
                                    Some(health_forum) => health_forum,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_health_forum(
                                    action,
                                    health_forum,
                                    original_action,
                                    original_health_forum,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::HealthBlog(health_blog) => {
                            let result = validate_create_health_blog(
                                EntryCreationAction::Update(action.clone()),
                                health_blog.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_health_blog: Option<HealthBlog> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_health_blog = match original_health_blog {
                                    Some(health_blog) => health_blog,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_health_blog(
                                    action,
                                    health_blog,
                                    original_action,
                                    original_health_blog,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::HBlog(h_blog) => {
                            let result = validate_create_h_blog(
                                EntryCreationAction::Update(action.clone()),
                                h_blog.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_h_blog: Option<HBlog> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_h_blog = match original_h_blog {
                                    Some(h_blog) => h_blog,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_h_blog(
                                    action,
                                    h_blog,
                                    original_action,
                                    original_h_blog,
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
                        EntryTypes::PeopleProfile(original_people_profile) => {
                            validate_delete_people_profile(
                                action,
                                original_action,
                                original_people_profile,
                            )
                        }
                        EntryTypes::HealthForum(original_health_forum) => {
                            validate_delete_health_forum(
                                action,
                                original_action,
                                original_health_forum,
                            )
                        }
                        EntryTypes::HealthBlog(original_health_blog) => {
                            validate_delete_health_blog(
                                action,
                                original_action,
                                original_health_blog,
                            )
                        }
                        EntryTypes::HBlog(original_h_blog) => {
                            validate_delete_h_blog(
                                action,
                                original_action,
                                original_h_blog,
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
                        LinkTypes::PersonToPeopleProfiles => {
                            validate_create_link_person_to_people_profiles(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::PeopleProfileUpdates => {
                            validate_create_link_people_profile_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllPeopleProfiles => {
                            validate_create_link_all_people_profiles(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CreatorToHealthForums => {
                            validate_create_link_creator_to_health_forums(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::PeopleProfileToHealthForums => {
                            validate_create_link_people_profile_to_health_forums(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::HealthForumUpdates => {
                            validate_create_link_health_forum_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllHealthForum => {
                            validate_create_link_all_health_forum(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::MyProfiles => {
                            validate_create_link_my_profiles(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CreatorToHealthBlogs => {
                            validate_create_link_creator_to_health_blogs(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::PeopleProfileToHealthBlogs => {
                            validate_create_link_people_profile_to_health_blogs(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::HealthBlogUpdates => {
                            validate_create_link_health_blog_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllHealthBlogs => {
                            validate_create_link_all_health_blogs(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::MyHealthBlogs => {
                            validate_create_link_my_health_blogs(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CreatorToHBlogs => {
                            validate_create_link_creator_to_h_blogs(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::HBlogUpdates => {
                            validate_create_link_h_blog_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllHBlogs => {
                            validate_create_link_all_h_blogs(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::MyHBlogs => {
                            validate_create_link_my_h_blogs(
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
                        LinkTypes::PersonToPeopleProfiles => {
                            validate_delete_link_person_to_people_profiles(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::PeopleProfileUpdates => {
                            validate_delete_link_people_profile_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllPeopleProfiles => {
                            validate_delete_link_all_people_profiles(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CreatorToHealthForums => {
                            validate_delete_link_creator_to_health_forums(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::PeopleProfileToHealthForums => {
                            validate_delete_link_people_profile_to_health_forums(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::HealthForumUpdates => {
                            validate_delete_link_health_forum_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllHealthForum => {
                            validate_delete_link_all_health_forum(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::MyProfiles => {
                            validate_delete_link_my_profiles(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CreatorToHealthBlogs => {
                            validate_delete_link_creator_to_health_blogs(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::PeopleProfileToHealthBlogs => {
                            validate_delete_link_people_profile_to_health_blogs(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::HealthBlogUpdates => {
                            validate_delete_link_health_blog_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllHealthBlogs => {
                            validate_delete_link_all_health_blogs(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::MyHealthBlogs => {
                            validate_delete_link_my_health_blogs(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CreatorToHBlogs => {
                            validate_delete_link_creator_to_h_blogs(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::HBlogUpdates => {
                            validate_delete_link_h_blog_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllHBlogs => {
                            validate_delete_link_all_h_blogs(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::MyHBlogs => {
                            validate_delete_link_my_h_blogs(
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
