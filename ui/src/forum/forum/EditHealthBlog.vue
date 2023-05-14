<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit Health Blog</span>
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
        :disabled="!isHealthBlogValid"
        @click="updateHealthBlog"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { HealthBlog } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import { decode } from '@msgpack/msgpack';
import { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textfield';
import '@material/mwc-textarea';
export default defineComponent({
  data(): {
    title: string;
    content: string;
  } {
    const currentHealthBlog = decode((this.currentRecord.entry as any).Present.entry) as HealthBlog;
    return { 
      title: currentHealthBlog.title,
      content: currentHealthBlog.content,
    }
  },
  props: {
    originalHealthBlogHash: {
      type: null,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentHealthBlog() {
      return decode((this.currentRecord.entry as any).Present.entry) as HealthBlog;
    },
    isHealthBlogValid() {
      return true && this.title !== '' && this.content !== '';
    },
  },
  mounted() {
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord input is required for the EditHealthBlog element`);
    }
    if (this.originalHealthBlogHash === undefined) {
      throw new Error(`The originalHealthBlogHash input is required for the EditHealthBlog element`);
    }
  },
  methods: {
    async updateHealthBlog() {

      const healthBlog: HealthBlog = { 
        creator: this.currentHealthBlog.creator,
        title: this.title,
        content: this.content,
        people_profile_hash: this.currentHealthBlog.people_profile_hash,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'forum',
          zome_name: 'forum',
          fn_name: 'update_health_blog',
          payload: {
            original_health_blog_hash: this.originalHealthBlogHash,
            previous_health_blog_hash: this.currentRecord.signed_action.hashed.hash,
            updated_health_blog: healthBlog
          }
        });
        this.$emit('health-blog-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the health blog: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['health-blog-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
