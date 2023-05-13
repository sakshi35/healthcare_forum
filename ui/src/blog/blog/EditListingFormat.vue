<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit Listing Format</span>
      <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Description" :value="description" @input="description = $event.target.value" required></mwc-textfield>
      </div>

      <div style="margin-bottom: 16px">
      <div style="display: flex; flex-direction: row">
        <span style="margin-right: 4px">Quantity</span>
      
        <mwc-slider :value="quantity" @input="quantity = $event.detail.value" discrete></mwc-slider>
      </div>
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
        :disabled="!isListingFormatValid"
        @click="updateListingFormat"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { ListingFormat } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-slider';
import '@material/mwc-textfield';
export default defineComponent({
  data(): {
    description: string;
    quantity: number;
  } {
    const currentListingFormat = decode((this.currentRecord.entry as any).Present.entry) as ListingFormat;
    return { 
      description: currentListingFormat.description,
      quantity: currentListingFormat.quantity,
    }
  },
  props: {
    originalListingFormatHash: {
      type: null,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentListingFormat() {
      return decode((this.currentRecord.entry as any).Present.entry) as ListingFormat;
    },
    isListingFormatValid() {
      return true && this.description !== '' && true;
    },
  },
  mounted() {
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord input is required for the EditListingFormat element`);
    }
    if (this.originalListingFormatHash === undefined) {
      throw new Error(`The originalListingFormatHash input is required for the EditListingFormat element`);
    }
  },
  methods: {
    async updateListingFormat() {

      const listingFormat: ListingFormat = { 
        listing_hash: this.currentListingFormat.listing_hash,
        description: this.description,
        quantity: this.quantity,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'blog',
          zome_name: 'blog',
          fn_name: 'update_listing_format',
          payload: {
            original_listing_format_hash: this.originalListingFormatHash,
            previous_listing_format_hash: this.currentRecord.signed_action.hashed.hash,
            updated_listing_format: listingFormat
          }
        });
        this.$emit('listing-format-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the listing format: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['listing-format-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
