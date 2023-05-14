<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create People Profile</span>
  
    <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="First Name" :value="firstName" @input="firstName = $event.target.value" required></mwc-textfield>
    </div>

    <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Last Name" :value="lastName" @input="lastName = $event.target.value" required></mwc-textfield>
    </div>

    <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="User Name" :value="userName" @input="userName = $event.target.value" required></mwc-textfield>
    </div>

    <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Location" :value="location" @input="location = $event.target.value" required></mwc-textfield>
    </div>

    <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Bio" :value="bio" @input="bio = $event.target.value" required></mwc-textarea>
    </div>

  
    <mwc-button 
      raised
      label="Create People Profile"
      :disabled="!isPeopleProfileValid"
      @click="createPeopleProfile"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { PeopleProfile } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
import '@material/mwc-textfield';
export default defineComponent({
  data(): {
    firstName: string;
    lastName: string;
    userName: string;
    location: string;
    bio: string;
  } {
    return { 
      firstName: '',
      lastName: '',
      userName: '',
      location: '',
      bio: '',
    }
  },

  props: {    person: {
      type: null,
      required: true
    },

  },  computed: {
    isPeopleProfileValid() {
    return true && this.firstName !== '' && this.lastName !== '' && this.userName !== '' && this.location !== '' && this.bio !== '';
    },
  },
  mounted() {
    if (this.person === undefined) {
      throw new Error(`The person input is required for the CreatePeopleProfile element`);
    }
  },
  methods: {
    async createPeopleProfile() {
      const peopleProfile: PeopleProfile = { 
        person: this.client.myPubKey!,

        first_name: this.firstName!,

        last_name: this.lastName!,

        user_name: this.userName!,

        location: this.location!,

        bio: this.bio!,
      };

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'forum',
          zome_name: 'forum',
          fn_name: 'create_people_profile',
          payload: peopleProfile,
        });
        this.$emit('people-profile-created', record.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the people profile: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['people-profile-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
