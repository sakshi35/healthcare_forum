<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create Listing Format</span>
  
    <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Description" :value="description" @input="description = $event.target.value" required></mwc-textfield>
    </div>

    <div style="margin-bottom: 16px">
      <div style="display: flex; flex-direction: row">
        <span style="margin-right: 4px">Quantity</span>
      
        <mwc-slider :value="quantity" @input="quantity = $event.detail.value" discrete></mwc-slider>
      </div>
    </div>

  
    <mwc-button 
      raised
      label="Create Listing Format"
      :disabled="!isListingFormatValid"
      @click="createListingFormat"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { ListingFormat } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textfield';
import '@material/mwc-slider';
export default defineComponent({
  data(): {
    description: string;
    quantity: number;
  } {
    return { 
      description: '',
      quantity: 0,
    }
  },

  props: {    listingHash: {
      type: null,
      required: true
    },

  },  computed: {
    isListingFormatValid() {
    return true && this.description !== '' && true;
    },
  },
  mounted() {
    if (this.listingHash === undefined) {
      throw new Error(`The listingHash input is required for the CreateListingFormat element`);
    }
  },
  methods: {
    async createListingFormat() {
      const listingFormat: ListingFormat = { 
        listing_hash: this.listingHash!,

        description: this.description!,

        quantity: this.quantity!,
      };

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'blog',
          zome_name: 'blog',
          fn_name: 'create_listing_format',
          payload: listingFormat,
        });
        this.$emit('listing-format-created', record.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the listing format: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['listing-format-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
