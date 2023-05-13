import { CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleName(cell: CallableCell, partialName = {}) {
    return {
        ...{
	  blog_name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  blog_description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialName
    };
}

export async function createName(cell: CallableCell, name = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "blog",
      fn_name: "create_name",
      payload: name || await sampleName(cell),
    });
}



export async function sampleListing(cell: CallableCell, partialListing = {}) {
    return {
        ...{
          creator: cell.cell_id[1],
	  listing_type: { type: 'Offer' },
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  is_active: true,
        },
        ...partialListing
    };
}

export async function createListing(cell: CallableCell, listing = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "blog",
      fn_name: "create_listing",
      payload: listing || await sampleListing(cell),
    });
}



export async function sampleListingFormat(cell: CallableCell, partialListingFormat = {}) {
    return {
        ...{
          listing_hash: ((await createListing(cell)).signed_action.hashed.content as NewEntryAction).entry_hash,
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  quantity: 10,
        },
        ...partialListingFormat
    };
}

export async function createListingFormat(cell: CallableCell, listingFormat = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "blog",
      fn_name: "create_listing_format",
      payload: listingFormat || await sampleListingFormat(cell),
    });
}



export async function sampleProfile(cell: CallableCell, partialProfile = {}) {
    return {
        ...{
          person: cell.cell_id[1],
	  name: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  location: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  bio: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialProfile
    };
}

export async function createProfile(cell: CallableCell, profile = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "blog",
      fn_name: "create_profile",
      payload: profile || await sampleProfile(cell),
    });
}

