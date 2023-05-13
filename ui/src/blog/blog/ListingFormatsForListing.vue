
<template>
  <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
  
  <div v-else style="display: flex; flex-direction: column">
    <span v-if="error">Error fetching the listing formats: {{error.data.data}}.</span>
    <div v-else-if="hashes && hashes.length > 0" style="margin-bottom: 8px">
      <ListingFormatDetail 
        v-for="hash in hashes" 
        :listing-format-hash="hash" 
      >
      </ListingFormatDetail>
    </div>
    <span v-else>No listing formats found for this listing.</span>
  </div>

</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import '@material/mwc-circular-progress';
import ListingFormatDetail from './ListingFormatDetail.vue';

export default defineComponent({
  components: {
    ListingFormatDetail
  },
  props: {
    listingHashHash: {
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
    if (this.listingHashHash === undefined) {
      throw new Error(`The listingHashHash input is required for the ListingFormatsForListing element`);
    }

    try {
      this.hashes = await this.client.callZome({
        cap_secret: null,
        role_name: '',
        zome_name: 'blog',
        fn_name: 'get_listing_formats_for_listing',
        payload: this.listingHashHash,
      });
    } catch (e) {
      this.error = e;
    }
    this.loading = false;
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
