<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create Blog</span>
  
    <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Title" :value="title" @input="title = $event.target.value" required></mwc-textfield>
    </div>

    <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Content" :value="content" @input="content = $event.target.value" required></mwc-textarea>
    </div>

  
    <mwc-button 
      raised
      label="Create Blog"
      :disabled="!isHBlogValid"
      @click="createHBlog"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { HBlog } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';

import '@material/mwc-textfield';
export default defineComponent({
  data(): {
    title: string;
    content: string;
  } {
    return { 
      title: '',
      content: '',
    }
  },
  props: {
    creator: {
      type: null,
      required: true
    },
  },
  computed: {
    isHBlogValid() {
    return true && this.title !== '' && this.content !== '';
    },
  },
  mounted() {
    if (this.creator === undefined) {
      throw new Error(`The creator input is required for the CreateHBlog element`);
    }
  },
  methods: {
    async createHBlog() {
      const hBlog: HBlog = { 
        creator: this.creator!,

        title: this.title!,

        content: this.content!,
      };

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'forum',
          zome_name: 'forum',
          fn_name: 'create_h_blog',
          payload: hBlog,
        });
        this.$emit('h-blog-created', record.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the h blog: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['h-blog-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
