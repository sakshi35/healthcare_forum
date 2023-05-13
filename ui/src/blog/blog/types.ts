import { 
  Record, 
  ActionHash, 
  SignedActionHashed,
  DnaHash,
  EntryHash, 
  AgentPubKey,
  Create,
  Update,
  Delete,
  CreateLink,
  DeleteLink
} from '@holochain/client';

export type BlogSignal = {
  type: 'EntryCreated';
  action: SignedActionHashed<Create>;
  app_entry: EntryTypes;
} | {
  type: 'EntryUpdated';
  action: SignedActionHashed<Update>;
  app_entry: EntryTypes;
  original_app_entry: EntryTypes;
} | {
  type: 'EntryDeleted';
  action: SignedActionHashed<Delete>;
  original_app_entry: EntryTypes;
} | {
  type: 'LinkCreated';
  action: SignedActionHashed<CreateLink>;
  link_type: string;
} | {
  type: 'LinkDeleted';
  action: SignedActionHashed<DeleteLink>;
  link_type: string;
};

export type EntryTypes =
 | ({ type: 'Profile'; } & Profile)
 | ({ type: 'ListingFormat'; } & ListingFormat)
 | ({ type: 'Listing'; } & Listing)
 | ({  type: 'Name'; } & Name);



export interface Name { 
  blog_name: string;

  blog_description: string;
}



export interface ListingType {
  type:  
    | 'Offer'
        | 'Request'
    ;
}

export interface Listing { 
  creator: AgentPubKey;

  listing_type: ListingType;

  title: string;

  description: string;

  is_active: boolean;
}




export interface ListingFormat { 
  listing_hash: EntryHash;

  description: string;

  quantity: number;
}




export interface Profile { 
  person: AgentPubKey;

  name: string;

  location: string;

  bio: string;
}

