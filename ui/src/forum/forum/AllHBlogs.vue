<template>
  <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>

  <div v-else style="display: flex; flex-direction: column">
    <span v-if="error">Error fetching the blogs: {{error.data.data}}.</span>
    <div v-else-if="hashes && hashes.length > 0" style="margin-bottom: 8px">
      <HBlogDetail 
        v-for="hash in hashes" 
        :h-blog-hash="hash"
        @h-blog-deleted="fetchHBlog()"
      >
      </HBlogDetail>
    </div>
    <span v-else>No blogs found.</span>
  </div>

</template>

<script lang="ts">
import { defineComponent, inject, toRaw, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, NewEntryAction, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import '@material/mwc-circular-progress';
import HBlogDetail from './HBlogDetail.vue';
import { ForumSignal } from './types';

export default defineComponent({
  components: {
    HBlogDetail
  },
  data(): { hashes: Array<ActionHash> | undefined; loading: boolean; error: any } {
    return {
      hashes: undefined,
      loading: true,
      error: undefined
    }
  },
  async mounted() {
    await this.fetchHBlog();
    toRaw(this.client).on('signal', signal => {
      if (signal.zome_name !== 'forum') return; 
      const payload = signal.payload as ForumSignal;
      if (payload.type !== 'EntryCreated') return;
      if (payload.app_entry.type !== 'HBlog') return;
      if (this.hashes) this.hashes.push(payload.action.hashed.hash);
    });
  },
  methods: {
    async fetchHBlog() {
      try {
        const records: Array<Record> = await this.client.callZome({
          cap_secret: null,
          role_name: 'forum',
          zome_name: 'forum',
          fn_name: 'get_all_h_blogs',
          payload: null,
        });
        this.hashes = records.map(r => r.signed_action.hashed.hash);
      } catch (e) {
        this.error = e;
      }
      this.loading = false;
    }
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
