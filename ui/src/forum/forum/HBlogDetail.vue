<template>
  <div v-if="!loading">
    <div v-if="editing" style="display: flex; flex: 1;">
      <EditHBlog
        :original-h-blog-hash="hBlogHash"
        :current-record="record!"
        @h-blog-updated="editing = false; fetchHBlog();"
        @edit-canceled="editing = false"
      ></EditHBlog>
    </div>
    <div v-else-if="record" style="display: flex; flex-direction: column" class="blog-container">
      <div style="display: flex; flex-direction: row">
        <span style="flex: 1"></span>
      
        <mwc-icon-button style="margin-left: 8px" icon="edit" @click="editing = true"></mwc-icon-button>
        <mwc-icon-button style="margin-left: 8px" icon="delete" @click="deleteHBlog()"></mwc-icon-button>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Title: </strong></span>
 	<span style="white-space: pre-line">{{  hBlog?.title }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Content: </strong></span>
 	<span style="white-space: pre-line">{{  hBlog?.content }} </span>
      </div>

    </div>
    
    <span v-else>The requested blog was not found.</span>
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
import { HBlog } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditHBlog from './EditHBlog.vue';

export default defineComponent({
  components: {
    EditHBlog
  },
  props: {
    hBlogHash: {
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
    hBlog() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as HBlog;
    }
  },
  async mounted() {
    if (this.hBlogHash === undefined) {
      throw new Error(`The hBlogHash input is required for the HBlogDetail element`);
    }

    await this.fetchHBlog();
  },
  methods: {
    async fetchHBlog() {
      this.loading = true;
      this.record = undefined;

      this.record = await this.client.callZome({
        cap_secret: null,
        role_name: 'forum',
        zome_name: 'forum',
        fn_name: 'get_h_blog',
        payload: this.hBlogHash,
      });

      this.loading = false;
    },
    async deleteHBlog() {
      try {
        await this.client.callZome({
          cap_secret: null,
          role_name: 'forum',
          zome_name: 'forum',
          fn_name: 'delete_h_blog',
          payload: this.hBlogHash,
        });
        this.$emit('h-blog-deleted', this.hBlogHash);
        this.fetchHBlog();
      } catch (e: any) {
        const errorSnackbar = this.$refs['delete-error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the h blog: ${e.data.data}`;
        errorSnackbar.show();
      }
    }
  },
  emits: ['h-blog-deleted'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>
<style scoped>
.blog-container {
  border: 1px solid #ccc;
  border-radius: 4px;
  padding: 16px;
  margin-bottom: 16px;
}
</style>
