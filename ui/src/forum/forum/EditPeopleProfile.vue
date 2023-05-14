<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit People Profile</span>
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



    <div style="display: flex; flex-direction: row">
      <mwc-button
        outlined
        label="Cancel"
        @click="$emit('edit-canceled')"
        style="flex: 1; margin-right: 16px;"
      ></mwc-button>
      <mwc-button 
        raised
        label="Save"
        :disabled="!isPeopleProfileValid"
        @click="updatePeopleProfile"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { PeopleProfile } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
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
    const currentPeopleProfile = decode((this.currentRecord.entry as any).Present.entry) as PeopleProfile;
    return { 
      firstName: currentPeopleProfile.first_name,
      lastName: currentPeopleProfile.last_name,
      userName: currentPeopleProfile.user_name,
      location: currentPeopleProfile.location,
      bio: currentPeopleProfile.bio,
    }
  },
  props: {
    originalPeopleProfileHash: {
      type: null,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentPeopleProfile() {
      return decode((this.currentRecord.entry as any).Present.entry) as PeopleProfile;
    },
    isPeopleProfileValid() {
      return true && this.firstName !== '' && this.lastName !== '' && this.userName !== '' && this.location !== '' && this.bio !== '';
    },
  },
  mounted() {
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord input is required for the EditPeopleProfile element`);
    }
    if (this.originalPeopleProfileHash === undefined) {
      throw new Error(`The originalPeopleProfileHash input is required for the EditPeopleProfile element`);
    }
  },
  methods: {
    async updatePeopleProfile() {

      const peopleProfile: PeopleProfile = { 
        person: this.currentPeopleProfile.person,
        first_name: this.firstName,
        last_name: this.lastName,
        user_name: this.userName,
        location: this.location,
        bio: this.bio,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'forum',
          zome_name: 'forum',
          fn_name: 'update_people_profile',
          payload: {
            original_people_profile_hash: this.originalPeopleProfileHash,
            previous_people_profile_hash: this.currentRecord.signed_action.hashed.hash,
            updated_people_profile: peopleProfile
          }
        });
        this.$emit('people-profile-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the people profile: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['people-profile-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
