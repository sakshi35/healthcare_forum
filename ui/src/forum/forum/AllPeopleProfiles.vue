<template>
  <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <div v-else style="display: flex; flex-direction: column">
    <span v-if="error">Error fetching the people profiles: {{error.data.data}}.</span>
    <div v-else-if="hashes && hashes.length > 0" style="margin-bottom: 8px">
      <PeopleProfileDetail 
        v-for="hash in hashes" 
        :people-profile-hash="hash"
        @people-profile-deleted="fetchPeopleProfile()"
      >
      </PeopleProfileDetail>
    </div>
    <span v-else>No people profiles found.</span>
  </div>

</template>

<script lang="ts">
import { defineComponent, inject, toRaw, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, NewEntryAction, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import '@material/mwc-circular-progress';
import PeopleProfileDetail from './PeopleProfileDetail.vue';
import { ForumSignal } from './types';

export default defineComponent({
  components: {
    PeopleProfileDetail
  },
  data(): { hashes: Array<ActionHash> | undefined; loading: boolean; error: any } {
    return {
      hashes: undefined,
      loading: true,
      error: undefined
    }
  },
  async mounted() {
    await this.fetchPeopleProfile();
    toRaw(this.client).on('signal', signal => {
      if (signal.zome_name !== 'forum') return; 
      const payload = signal.payload as ForumSignal;
      if (payload.type !== 'EntryCreated') return;
      if (payload.app_entry.type !== 'PeopleProfile') return;
      if (this.hashes) this.hashes.push(payload.action.hashed.hash);
    });
  },
  methods: {
    async fetchPeopleProfile() {
      try {
        const records: Array<Record> = await this.client.callZome({
          cap_secret: null,
          role_name: 'forum',
          zome_name: 'forum',
          fn_name: 'get_all_people_profiles',
          payload: null,
        });
        this.hashes = records.map(r => r.signed_action.hashed.hash);
      } catch (e) {
        this.error = e;
      }
      this.loading = false;
    }
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
