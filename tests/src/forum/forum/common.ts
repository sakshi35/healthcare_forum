import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function samplePeopleProfile(cell: CallableCell, partialPeopleProfile = {}) {
    return {
        ...{
          person: cell.cell_id[1],
	  first_name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  last_name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  user_name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  location: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  bio: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialPeopleProfile
    };
}

export async function createPeopleProfile(cell: CallableCell, peopleProfile = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "forum",
      fn_name: "create_people_profile",
      payload: peopleProfile || await samplePeopleProfile(cell),
    });
}



export async function sampleHealthForum(cell: CallableCell, partialHealthForum = {}) {
    return {
        ...{
          creator: cell.cell_id[1],
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
          people_profile_hash: ((await createPeopleProfile(cell)).signed_action.hashed.content as NewEntryAction).entry_hash,
	  timestamp: 1674053334548000,
        },
        ...partialHealthForum
    };
}

export async function createHealthForum(cell: CallableCell, healthForum = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "forum",
      fn_name: "create_health_forum",
      payload: healthForum || await sampleHealthForum(cell),
    });
}



export async function sampleHealthBlog(cell: CallableCell, partialHealthBlog = {}) {
    return {
        ...{
          creator: cell.cell_id[1],
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
          people_profile_hash: ((await createPeopleProfile(cell)).signed_action.hashed.content as NewEntryAction).entry_hash,
        },
        ...partialHealthBlog
    };
}

export async function createHealthBlog(cell: CallableCell, healthBlog = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "forum",
      fn_name: "create_health_blog",
      payload: healthBlog || await sampleHealthBlog(cell),
    });
}



export async function sampleHBlog(cell: CallableCell, partialHBlog = {}) {
    return {
        ...{
          creator: cell.cell_id[1],
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialHBlog
    };
}

export async function createHBlog(cell: CallableCell, hBlog = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "forum",
      fn_name: "create_h_blog",
      payload: hBlog || await sampleHBlog(cell),
    });
}

