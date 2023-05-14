<template>
  <div v-if="!loading">
    <div v-if="editing" style="display: flex; flex: 1;">
      <EditHealthBlog
        :original-health-blog-hash="healthBlogHash"
        :current-record="record!"
        @health-blog-updated="editing = false; fetchHealthBlog();"
        @edit-canceled="editing = false"
      ></EditHealthBlog>
    </div>
    <div v-else-if="record" style="display: flex; flex-direction: column">
      <div style="display: flex; flex-direction: row">
        <span style="flex: 1"></span>
      
        <mwc-icon-button style="margin-left: 8px" icon="edit" @click="editing = true"></mwc-icon-button>
        <mwc-icon-button style="margin-left: 8px" icon="delete" @click="deleteHealthBlog()"></mwc-icon-button>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Title: </strong></span>
 	<span style="white-space: pre-line">{{  healthBlog?.title }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Content: </strong></span>
 	<span style="white-space: pre-line">{{  healthBlog?.content }} </span>
      </div>

    </div>
    
    <span v-else>The requested health blog was not found.</span>
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
import { HealthBlog } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditHealthBlog from './EditHealthBlog.vue';

export default defineComponent({
  components: {
    EditHealthBlog
  },
  props: {
    healthBlogHash: {
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
    healthBlog() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as HealthBlog;
    }
  },
  async mounted() {
    if (this.healthBlogHash === undefined) {
      throw new Error(`The healthBlogHash input is required for the HealthBlogDetail element`);
    }

    await this.fetchHealthBlog();
  },
  methods: {
    async fetchHealthBlog() {
      this.loading = true;
      this.record = undefined;

      this.record = await this.client.callZome({
        cap_secret: null,
        role_name: 'forum',
        zome_name: 'forum',
        fn_name: 'get_health_blog',
        payload: this.healthBlogHash,
      });

      this.loading = false;
    },
    async deleteHealthBlog() {
      try {
        await this.client.callZome({
          cap_secret: null,
          role_name: 'forum',
          zome_name: 'forum',
          fn_name: 'delete_health_blog',
          payload: this.healthBlogHash,
        });
        this.$emit('health-blog-deleted', this.healthBlogHash);
        this.fetchHealthBlog();
      } catch (e: any) {
        const errorSnackbar = this.$refs['delete-error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the health blog: ${e.data.data}`;
        errorSnackbar.show();
      }
    }
  },
  emits: ['health-blog-deleted'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>
