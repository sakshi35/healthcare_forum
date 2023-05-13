<template>
  <div v-if="!loading">
    <div v-if="editing" style="display: flex; flex: 1;">
      <EditProfile
        :original-profile-hash="profileHash"
        :current-record="record!"
        @profile-updated="editing = false; fetchProfile();"
        @edit-canceled="editing = false"
      ></EditProfile>
    </div>
    <div v-else-if="record" style="display: flex; flex-direction: column">
      <div style="display: flex; flex-direction: row">
        <span style="flex: 1"></span>
      
        <mwc-icon-button style="margin-left: 8px" icon="edit" @click="editing = true"></mwc-icon-button>
        <mwc-icon-button style="margin-left: 8px" icon="delete" @click="deleteProfile()"></mwc-icon-button>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Name: </strong></span>
 	<span style="white-space: pre-line">{{  profile?.name }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Location: </strong></span>
 	<span style="white-space: pre-line">{{  profile?.location }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Bio: </strong></span>
 	<span style="white-space: pre-line">{{  profile?.bio }} </span>
      </div>

    </div>
    
    <span v-else>The requested profile was not found.</span>
  </div>

  <div v-else style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <mwc-snackbar ref="delete-error" leading>
  </mwc-snackbar>
</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Profile } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditProfile from './EditProfile.vue';

export default defineComponent({
  components: {
    EditProfile
  },
  props: {
    profileHash: {
      type: Object,
      required: true
    }
  },
  data(): { record: Record | undefined; loading: boolean; editing: boolean; } {
    return {
      record: undefined,
      loading: true,
      editing: false,
    }
  },
  computed: {
    profile() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as Profile;
    }
  },
  async mounted() {
    if (this.profileHash === undefined) {
      throw new Error(`The profileHash input is required for the ProfileDetail element`);
    }

    await this.fetchProfile();
  },
  methods: {
    async fetchProfile() {
      this.loading = true;
      this.record = undefined;

      this.record = await this.client.callZome({
        cap_secret: null,
        role_name: 'blog',
        zome_name: 'blog',
        fn_name: 'get_profile',
        payload: this.profileHash,
      });

      this.loading = false;
    },
    async deleteProfile() {
      try {
        await this.client.callZome({
          cap_secret: null,
          role_name: 'blog',
          zome_name: 'blog',
          fn_name: 'delete_profile',
          payload: this.profileHash,
        });
        this.$emit('profile-deleted', this.profileHash);
        this.fetchProfile();
      } catch (e: any) {
        const errorSnackbar = this.$refs['delete-error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the profile: ${e.data.data}`;
        errorSnackbar.show();
      }
    }
  },
  emits: ['profile-deleted'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>
