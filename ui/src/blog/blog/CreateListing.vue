<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create Listing</span>
  
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

  
    <mwc-button 
      raised
      label="Create Listing"
      :disabled="!isListingValid"
      @click="createListing"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Listing, ListingType } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-formfield';
import '@material/mwc-textfield';
import '@material/mwc-select';
import '@material/mwc-checkbox';
import '@material/mwc-textarea';
export default defineComponent({
  data(): {
    listingType: ListingType;
    title: string;
    description: string;
    isActive: boolean;
  } {
    return { 
      listingType: { type: 'Offer' },
      title: '',
      description: '',
      isActive: true,
    } 
  },
  props: {
    creator: {
      type: null,
      required: true
    },

  },  computed: {
    isListingValid() {
    return true && true && this.title !== '' && this.description !== '' && true;
    },
  },
  mounted() {
    if (this.creator === undefined) {
      throw new Error(`The creator input is required for the CreateListing element`);
    }
  },
  methods: {
    async createListing() {
      const listing: Listing = { 
        creator: this.client.myPubKey!,

        listing_type: this.listingType!,

        title: this.title!,

        description: this.description!,

        is_active: this.isActive!,
      };

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'blog',
          zome_name: 'blog',
          fn_name: 'create_listing',
          payload: listing,
        });
        this.$emit('listing-created', record.signed_action.hashed.hash);
        alert("Post created successfully!!!");
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the listing: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['listing-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
