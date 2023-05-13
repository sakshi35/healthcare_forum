<template>
  <div v-if="!loading">
    <div v-if="editing" style="display: flex; flex: 1;">
      <EditListingFormat
        :original-listing-format-hash="listingFormatHash"
        :current-record="record!"
        @listing-format-updated="editing = false; fetchListingFormat();"
        @edit-canceled="editing = false"
      ></EditListingFormat>
    </div>
    <div v-else-if="record" style="display: flex; flex-direction: column">
      <div style="display: flex; flex-direction: row">
        <span style="flex: 1"></span>
      
        <mwc-icon-button style="margin-left: 8px" icon="edit" @click="editing = true"></mwc-icon-button>
        <mwc-icon-button style="margin-left: 8px" icon="delete" @click="deleteListingFormat()"></mwc-icon-button>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Description: </strong></span>
 	<span style="white-space: pre-line">{{  listingFormat?.description }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Quantity: </strong></span>
 	<span style="white-space: pre-line">{{  listingFormat?.quantity }} </span>
      </div>

    </div>
    
    <span v-else>The requested listing format was not found.</span>
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
import { ListingFormat } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditListingFormat from './EditListingFormat.vue';

export default defineComponent({
  components: {
    EditListingFormat
  },
  props: {
    listingFormatHash: {
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
    listingFormat() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as ListingFormat;
    }
  },
  async mounted() {
    if (this.listingFormatHash === undefined) {
      throw new Error(`The listingFormatHash input is required for the ListingFormatDetail element`);
    }

    await this.fetchListingFormat();
  },
  methods: {
    async fetchListingFormat() {
      this.loading = true;
      this.record = undefined;

      this.record = await this.client.callZome({
        cap_secret: null,
        role_name: 'blog',
        zome_name: 'blog',
        fn_name: 'get_listing_format',
        payload: this.listingFormatHash,
      });

      this.loading = false;
    },
    async deleteListingFormat() {
      try {
        await this.client.callZome({
          cap_secret: null,
          role_name: 'blog',
          zome_name: 'blog',
          fn_name: 'delete_listing_format',
          payload: this.listingFormatHash,
        });
        this.$emit('listing-format-deleted', this.listingFormatHash);
        this.fetchListingFormat();
      } catch (e: any) {
        const errorSnackbar = this.$refs['delete-error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the listing format: ${e.data.data}`;
        errorSnackbar.show();
      }
    }
  },
  emits: ['listing-format-deleted'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>
