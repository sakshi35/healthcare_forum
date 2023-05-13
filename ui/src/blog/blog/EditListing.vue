<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit Listing</span>
      <div style="margin-bottom: 16px">
      <mwc-select outlined helper="Listing Type" required>
  <mwc-list-item :selected="listingType.type === 'Offer' " @request-selected="listingType = { type: 'Offer'}">Offer</mwc-list-item>
  <mwc-list-item :selected="listingType.type === 'Request' " @request-selected="listingType = { type: 'Request'}">Request</mwc-list-item>
</mwc-select>
      </div>

      <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Title" :value="title" @input="title = $event.target.value" required></mwc-textfield>
      </div>

      <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Description" :value="description" @input="description = $event.target.value" required></mwc-textarea>
      </div>

      <div style="margin-bottom: 16px">
      <mwc-formfield label="Is Active">
        <mwc-checkbox :checked="isActive" @change="isActive = $event.target.checked"></mwc-checkbox>
      </mwc-formfield>
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
        :disabled="!isListingValid"
        @click="updateListing"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Listing, ListingType } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';
import '@material/mwc-textarea';

import '@material/mwc-formfield';
import '@material/mwc-select';
import '@material/mwc-checkbox';
export default defineComponent({
  data(): {
    listingType: ListingType;
    title: string;
    description: string;
    isActive: boolean;
  } {
    const currentListing = decode((this.currentRecord.entry as any).Present.entry) as Listing;
    return { 
      listingType: currentListing.listing_type,
      title: currentListing.title,
      description: currentListing.description,
      isActive: currentListing.is_active,
    }
  },
  props: {
    originalListingHash: {
      type: null,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentListing() {
      return decode((this.currentRecord.entry as any).Present.entry) as Listing;
    },
    isListingValid() {
      return true && true && this.title !== '' && this.description !== '' && true;
    },
  },
  mounted() {
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord input is required for the EditListing element`);
    }
    if (this.originalListingHash === undefined) {
      throw new Error(`The originalListingHash input is required for the EditListing element`);
    }
  },
  methods: {
    async updateListing() {

      const listing: Listing = { 
        creator: this.currentListing.creator,
        listing_type: this.listingType,
        title: this.title,
        description: this.description,
        is_active: this.isActive,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'blog',
          zome_name: 'blog',
          fn_name: 'update_listing',
          payload: {
            original_listing_hash: this.originalListingHash,
            previous_listing_hash: this.currentRecord.signed_action.hashed.hash,
            updated_listing: listing
          }
        });
        this.$emit('listing-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the listing: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['listing-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
