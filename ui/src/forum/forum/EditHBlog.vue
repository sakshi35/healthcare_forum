<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit H Blog</span>
      <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Title" :value="title" @input="title = $event.target.value" required></mwc-textfield>
      </div>

      <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Content" :value="content" @input="content = $event.target.value" required></mwc-textarea>
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
        :disabled="!isHBlogValid"
        @click="updateHBlog"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { HBlog } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';

import '@material/mwc-textfield';
export default defineComponent({
  data(): {
    title: string;
    content: string;
  } {
    const currentHBlog = decode((this.currentRecord.entry as any).Present.entry) as HBlog;
    return { 
      title: currentHBlog.title,
      content: currentHBlog.content,
    }
  },
  props: {
    originalHBlogHash: {
      type: null,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentHBlog() {
      return decode((this.currentRecord.entry as any).Present.entry) as HBlog;
    },
    isHBlogValid() {
      return true && this.title !== '' && this.content !== '';
    },
  },
  mounted() {
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord input is required for the EditHBlog element`);
    }
    if (this.originalHBlogHash === undefined) {
      throw new Error(`The originalHBlogHash input is required for the EditHBlog element`);
    }
  },
  methods: {
    async updateHBlog() {

      const hBlog: HBlog = { 
        creator: this.currentHBlog.creator,
        title: this.title,
        content: this.content,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'forum',
          zome_name: 'forum',
          fn_name: 'update_h_blog',
          payload: {
            original_h_blog_hash: this.originalHBlogHash,
            previous_h_blog_hash: this.currentRecord.signed_action.hashed.hash,
            updated_h_blog: hBlog
          }
        });
        this.$emit('h-blog-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the h blog: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['h-blog-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
