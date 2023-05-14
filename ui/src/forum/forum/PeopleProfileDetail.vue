<template>
  <div v-if="!loading">
    <div v-if="editing" style="display: flex; flex: 1;">
      <EditPeopleProfile
        :original-people-profile-hash="peopleProfileHash"
        :current-record="record!"
        @people-profile-updated="editing = false; fetchPeopleProfile();"
        @edit-canceled="editing = false"
      ></EditPeopleProfile>
    </div>
    <div v-else-if="record" style="display: flex; flex-direction: column" class="blog-container">
      <div style="display: flex; flex-direction: row">
        <span style="flex: 1"></span>
      
        <mwc-icon-button style="margin-left: 8px" icon="edit" @click="editing = true"></mwc-icon-button>
        <mwc-icon-button style="margin-left: 8px" icon="delete" @click="deletePeopleProfile()"></mwc-icon-button>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>First Name: </strong></span>
 	<span style="white-space: pre-line">{{  peopleProfile?.first_name }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Last Name: </strong></span>
 	<span style="white-space: pre-line">{{  peopleProfile?.last_name }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>User Name: </strong></span>
 	<span style="white-space: pre-line">{{  peopleProfile?.user_name }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Location: </strong></span>
 	<span style="white-space: pre-line">{{  peopleProfile?.location }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Bio: </strong></span>
 	<span style="white-space: pre-line">{{  peopleProfile?.bio }} </span>
      </div>

    </div>
    
    <span v-else>The requested people profile was not found.</span>
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
import { PeopleProfile } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditPeopleProfile from './EditPeopleProfile.vue';

export default defineComponent({
  components: {
    EditPeopleProfile
  },
  props: {
    peopleProfileHash: {
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
    peopleProfile() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as PeopleProfile;
    }
  },
  async mounted() {
    if (this.peopleProfileHash === undefined) {
      throw new Error(`The peopleProfileHash input is required for the PeopleProfileDetail element`);
    }

    await this.fetchPeopleProfile();
  },
  methods: {
    async fetchPeopleProfile() {
      this.loading = true;
      this.record = undefined;

      this.record = await this.client.callZome({
        cap_secret: null,
        role_name: 'forum',
        zome_name: 'forum',
        fn_name: 'get_people_profile',
        payload: this.peopleProfileHash,
      });

      this.loading = false;
    },
    async deletePeopleProfile() {
      try {
        await this.client.callZome({
          cap_secret: null,
          role_name: 'forum',
          zome_name: 'forum',
          fn_name: 'delete_people_profile',
          payload: this.peopleProfileHash,
        });
        this.$emit('people-profile-deleted', this.peopleProfileHash);
        this.fetchPeopleProfile();
      } catch (e: any) {
        const errorSnackbar = this.$refs['delete-error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the people profile: ${e.data.data}`;
        errorSnackbar.show();
      }
    }
  },
  emits: ['people-profile-deleted'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>
<style scoped>
.blog-container {
  border: 1px solid #ccc;
  border-radius: 4px;
  padding: 16px;
  margin-bottom: 16px;
}
</style>
