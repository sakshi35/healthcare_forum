<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit Name</span>
      <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Blog Name" :value="blogName" @input="blogName = $event.target.value" required></mwc-textfield>
      </div>

      <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Blog Description" :value="blogDescription" @input="blogDescription = $event.target.value" required></mwc-textarea>
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
        :disabled="!isNameValid"
        @click="updateName"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Name } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textfield';
import '@material/mwc-textarea';
export default defineComponent({
  data(): {
    blogName: string;
    blogDescription: string;
  } {
    const currentName = decode((this.currentRecord.entry as any).Present.entry) as Name;
    return { 
      blogName: currentName.blog_name,
      blogDescription: currentName.blog_description,
    }
  },
  props: {
    originalNameHash: {
      type: null,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentName() {
      return decode((this.currentRecord.entry as any).Present.entry) as Name;
    },
    isNameValid() {
      return true && this.blogName !== '' && this.blogDescription !== '';
    },
  },
  mounted() {
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord input is required for the EditName element`);
    }
    if (this.originalNameHash === undefined) {
      throw new Error(`The originalNameHash input is required for the EditName element`);
    }
  },
  methods: {
    async updateName() {

      const name: Name = { 
        blog_name: this.blogName,
        blog_description: this.blogDescription,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'blog',
          zome_name: 'blog',
          fn_name: 'update_name',
          payload: {
            original_name_hash: this.originalNameHash,
            previous_name_hash: this.currentRecord.signed_action.hashed.hash,
            updated_name: name
          }
        });
        this.$emit('name-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the name: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['name-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
