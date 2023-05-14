<template>
  <mwc-snackbar ref="update-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Edit Health Forum</span>
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
        :disabled="!isHealthForumValid"
        @click="updateHealthForum"
        style="flex: 1;"
      ></mwc-button>
    </div>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { HealthForum } from './types';
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
    const currentHealthForum = decode((this.currentRecord.entry as any).Present.entry) as HealthForum;
    return { 
      title: currentHealthForum.title,
      content: currentHealthForum.content,
    }
  },
  props: {
    originalHealthForumHash: {
      type: null,
      required: true,
    },
    currentRecord: {
      type: Object,
      required: true
    }
  },
  computed: {
    currentHealthForum() {
      return decode((this.currentRecord.entry as any).Present.entry) as HealthForum;
    },
    isHealthForumValid() {
      return true && this.title !== '' && this.content !== '';
    },
  },
  mounted() {
    if (this.currentRecord === undefined) {
      throw new Error(`The currentRecord input is required for the EditHealthForum element`);
    }
    if (this.originalHealthForumHash === undefined) {
      throw new Error(`The originalHealthForumHash input is required for the EditHealthForum element`);
    }
  },
  methods: {
    async updateHealthForum() {

      const healthForum: HealthForum = { 
        creator: this.currentHealthForum.creator,
        title: this.title,
        content: this.content,
        people_profile_hash: this.currentHealthForum.people_profile_hash,
        timestamp: this.currentHealthForum.timestamp,
      };

      try {
        const updateRecord: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'forum',
          zome_name: 'forum',
          fn_name: 'update_health_forum',
          payload: {
            original_health_forum_hash: this.originalHealthForumHash,
            previous_health_forum_hash: this.currentRecord.signed_action.hashed.hash,
            updated_health_forum: healthForum
          }
        });
        this.$emit('health-forum-updated', updateRecord.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['update-error'] as Snackbar;
        errorSnackbar.labelText = `Error updating the health forum: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['health-forum-updated', 'edit-canceled'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
