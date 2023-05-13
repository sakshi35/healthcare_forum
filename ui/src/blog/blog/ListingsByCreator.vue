<template>
  <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <div v-else style="display: flex; flex-direction: column">
    <span v-if="error">Error fetching the listings: {{error.data.data}}.</span>
    <div v-else-if="hashes && hashes.length > 0" style="margin-bottom: 8px">
      <ListingDetail 
        v-for="hash in hashes" 
        :listing-hash="hash"
        @listing-deleted="fetchListing()"
      >
      </ListingDetail>
    </div>
    <span v-else>No listings found for this author.</span>
  </div>

</template>

<script lang="ts">
import { defineComponent, inject, toRaw, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, NewEntryAction, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import '@material/mwc-circular-progress';
import ListingDetail from './ListingDetail.vue';
import { BlogSignal } from './types';

export default defineComponent({
  components: {
    ListingDetail
  },
  props: {
    author: {
      type: Object,
      required: true
    }
  },
  data(): { hashes: Array<ActionHash> | undefined; loading: boolean; error: any } {
    return {
      hashes: undefined,
      loading: true,
      error: undefined
    }
  },
  async mounted() {
    if (this.author === undefined) {
      throw new Error(`The author property is required for the ListingsByCreator element`);
    }

    await this.fetchListing();
    toRaw(this.client).on('signal', signal => {
      if (signal.zome_name !== 'blog') return; 
      const payload = signal.payload as BlogSignal;
      if (payload.type !== 'EntryCreated') return;
      if (payload.app_entry.type !== 'Listing') return;
      if (this.author.toString() !== this.client.myPubKey.toString()) return;
      if (this.hashes) this.hashes.push(payload.action.hashed.hash);
    });
  },
  methods: {
    async fetchListing() {
      try {
        const records: Array<Record> = await this.client.callZome({
          cap_secret: null,
          role_name: 'blog',
          zome_name: 'blog',
          fn_name: 'get_listings_by_creator',
          payload: this.author,
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
