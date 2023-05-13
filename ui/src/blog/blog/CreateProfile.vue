<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create Profile</span>
  
    <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Name" :value="name" @input="name = $event.target.value" required></mwc-textfield>
    </div>

    <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Location" :value="location" @input="location = $event.target.value" required></mwc-textfield>
    </div>

    <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Bio" :value="bio" @input="bio = $event.target.value" required></mwc-textarea>
    </div>

  
    <mwc-button 
      raised
      label="Create Profile"
      :disabled="!isProfileValid"
      @click="createProfile"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Profile } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
import '@material/mwc-textfield';
export default defineComponent({
  data(): {
    name: string;
    location: string;
    bio: string;
  } {
    return { 
      name: '',
      location: '',
      bio: '',
    }
  },
  props: {
    person: {
      type: null,
      required: true
    },
  },
  computed: {
    isProfileValid() {
    return true && this.name !== '' && this.location !== '' && this.bio !== '';
    },
  },
  mounted() {
    if (this.person === undefined) {
      throw new Error(`The person input is required for the CreateProfile element`);
    }
  },
  methods: {
    async createProfile() {
      const profile: Profile = { 
        //person: this.person!,

        person: this.client.myPubKey!,
        name: this.name!,

        location: this.location!,

        bio: this.bio!,
      };
      console.log("profile bio: "+profile.bio);
      console.log("person: "+profile.person);

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'blog',
          zome_name: 'blog',
          fn_name: 'create_profile',
          payload: profile,
        });
        this.$emit('profile-created', record.signed_action.hashed.hash);
        alert("Profile created successfully!!!");
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the profile: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['profile-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
