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

export type ForumSignal = {
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
 | ({ type: 'HBlog'; } & HBlog)
 | ({ type: 'HealthBlog'; } & HealthBlog)
 | ({ type: 'HealthForum'; } & HealthForum)
 | ({  type: 'PeopleProfile'; } & PeopleProfile);



export interface PeopleProfile { 
  person: AgentPubKey;

  first_name: string;

  last_name: string;

  user_name: string;

  location: string;

  bio: string;
}




export interface HealthForum { 
  creator: AgentPubKey;

  title: string;

  content: string;

  people_profile_hash: EntryHash;

  timestamp: number;
}




export interface HealthBlog { 
  creator: AgentPubKey;

  title: string;

  content: string;

  people_profile_hash: EntryHash;
}




export interface HBlog { 
  creator: AgentPubKey;

  title: string;

  content: string;
}

