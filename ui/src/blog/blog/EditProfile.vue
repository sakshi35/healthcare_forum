<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit Profile</span>
      <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Name" :value="name" @input="name = $event.target.value" required></mwc-textfield>
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
        :disabled="!isProfileValid"
        @click="updateProfile"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Profile } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';
import '@material/mwc-textfield';

export default defineComponent({
  data(): {
    name: string;
    location: string;
    bio: string;
  } {
    const currentProfile = decode((this.currentRecord.entry as any).Present.entry) as Profile;
    return { 
      name: currentProfile.name,
      location: currentProfile.location,
      bio: currentProfile.bio,
    }
  },
  props: {
    originalProfileHash: {
      type: null,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentProfile() {
      return decode((this.currentRecord.entry as any).Present.entry) as Profile;
    },
    isProfileValid() {
      return true && this.name !== '' && this.location !== '' && this.bio !== '';
    },
  },
  mounted() {
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord input is required for the EditProfile element`);
    }
    if (this.originalProfileHash === undefined) {
      throw new Error(`The originalProfileHash input is required for the EditProfile element`);
    }
  },
  methods: {
    async updateProfile() {

      const profile: Profile = { 
        person: this.currentProfile.person,
        name: this.name,
        location: this.location,
        bio: this.bio,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'blog',
          zome_name: 'blog',
          fn_name: 'update_profile',
          payload: {
            original_profile_hash: this.originalProfileHash,
            previous_profile_hash: this.currentRecord.signed_action.hashed.hash,
            updated_profile: profile
          }
        });
        this.$emit('profile-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the profile: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['profile-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
