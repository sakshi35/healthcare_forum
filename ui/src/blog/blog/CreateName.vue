<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create Name</span>
  
    <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Blog Name" :value="blogName" @input="blogName = $event.target.value" required></mwc-textfield>
    </div>

    <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Blog Description" :value="blogDescription" @input="blogDescription = $event.target.value" required></mwc-textarea>
    </div>

  
    <mwc-button 
      raised
      label="Create Name"
      :disabled="!isNameValid"
      @click="createName"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { Name } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';

import '@material/mwc-textfield';
export default defineComponent({
  data(): {
    blogName: string;
    blogDescription: string;
  } {
    return { 
      blogName: '',
      blogDescription: '',
    }
  },
  computed: {
    isNameValid() {
    return true && this.blogName !== '' && this.blogDescription !== '';
    },
  },
  mounted() {
  },
  methods: {
    async createName() {
      const name: Name = { 
        blog_name: this.blogName!,

        blog_description: this.blogDescription!,
      };

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'blog',
          zome_name: 'blog',
          fn_name: 'create_name',
          payload: name,
        });
        this.$emit('name-created', record.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the name: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['name-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
