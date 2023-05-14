
<template>
  <div v-if="loading" style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
  
  <div v-else style="display: flex; flex-direction: column">
    <span v-if="error">Error fetching the health blogs: {{error.data.data}}.</span>
    <div v-else-if="hashes && hashes.length > 0" style="margin-bottom: 8px">
      <HealthBlogDetail 
        v-for="hash in hashes" 
        :health-blog-hash="hash" 
      >
      </HealthBlogDetail>
    </div>
    <span v-else>No health blogs found for this people profile.</span>
  </div>

</template>

<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { decode } from '@msgpack/msgpack';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash } from '@holochain/client';
import '@material/mwc-circular-progress';
import HealthBlogDetail from './HealthBlogDetail.vue';

export default defineComponent({
  components: {
    HealthBlogDetail
  },
  props: {
    peopleProfileHashHash: {
      type: Object,
      required: true
    }
  },
  data(): { hashes: Array<ActionHash> | undefined; loading: boolean; error: any } {
    return {
      hashes: undefined,
      loading: true,
      error: undefined
    }
  },
  async mounted() {
    if (this.peopleProfileHashHash === undefined) {
      throw new Error(`The peopleProfileHashHash input is required for the HealthBlogsForPeopleProfile element`);
    }

    try {
      this.hashes = await this.client.callZome({
        cap_secret: null,
        role_name: '',
        zome_name: 'forum',
        fn_name: 'get_health_blogs_for_people_profile',
        payload: this.peopleProfileHashHash,
      });
    } catch (e) {
      this.error = e;
    }
    this.loading = false;
  },
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
