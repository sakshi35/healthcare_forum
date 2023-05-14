<template>
  <div v-if="!loading">
    <div v-if="editing" style="display: flex; flex: 1;">
      <EditHealthForum
        :original-health-forum-hash="healthForumHash"
        :current-record="record!"
        @health-forum-updated="editing = false; fetchHealthForum();"
        @edit-canceled="editing = false"
      ></EditHealthForum>
    </div>
    <div v-else-if="record" style="display: flex; flex-direction: column">
      <div style="display: flex; flex-direction: row">
        <span style="flex: 1"></span>
      
        <mwc-icon-button style="margin-left: 8px" icon="edit" @click="editing = true"></mwc-icon-button>
        <mwc-icon-button style="margin-left: 8px" icon="delete" @click="deleteHealthForum()"></mwc-icon-button>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Title: </strong></span>
 	<span style="white-space: pre-line">{{  healthForum?.title }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Content: </strong></span>
 	<span style="white-space: pre-line">{{  healthForum?.content }} </span>
      </div>

    </div>
    
    <span v-else>The requested health forum was not found.</span>
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
import { HealthForum } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditHealthForum from './EditHealthForum.vue';

export default defineComponent({
  components: {
    EditHealthForum
  },
  props: {
    healthForumHash: {
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
    healthForum() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as HealthForum;
    }
  },
  async mounted() {
    if (this.healthForumHash === undefined) {
      throw new Error(`The healthForumHash input is required for the HealthForumDetail element`);
    }

    await this.fetchHealthForum();
  },
  methods: {
    async fetchHealthForum() {
      this.loading = true;
      this.record = undefined;

      this.record = await this.client.callZome({
        cap_secret: null,
        role_name: 'forum',
        zome_name: 'forum',
        fn_name: 'get_health_forum',
        payload: this.healthForumHash,
      });

      this.loading = false;
    },
    async deleteHealthForum() {
      try {
        await this.client.callZome({
          cap_secret: null,
          role_name: 'forum',
          zome_name: 'forum',
          fn_name: 'delete_health_forum',
          payload: this.healthForumHash,
        });
        this.$emit('health-forum-deleted', this.healthForumHash);
        this.fetchHealthForum();
      } catch (e: any) {
        const errorSnackbar = this.$refs['delete-error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the health forum: ${e.data.data}`;
        errorSnackbar.show();
      }
    }
  },
  emits: ['health-forum-deleted'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>
