<template>
  <mwc-snackbar ref="create-error"></mwc-snackbar>

  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create Health Forum</span>
  
    <div style="margin-bottom: 16px">
      <mwc-textfield outlined label="Title" :value="title" @input="title = $event.target.value" required></mwc-textfield>
    </div>

    <div style="margin-bottom: 16px">
      <mwc-textarea outlined label="Content" :value="content" @input="content = $event.target.value" required></mwc-textarea>
    </div>

  
    <mwc-button 
      raised
      label="Create Health Forum"
      :disabled="!isHealthForumValid"
      @click="createHealthForum"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { AppAgentClient, Record, AgentPubKey, EntryHash, ActionHash, DnaHash } from '@holochain/client';
import { HealthForum } from './types';
import '@material/mwc-button';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';

import '@material/mwc-textarea';
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
    peopleProfileHash: {
      type: null,
      required: true
    },
    timestamp: {
      type: null,
      required: true
    },
  },
  computed: {
    isHealthForumValid() {
    return true && this.title !== '' && this.content !== '';
    },
  },
  mounted() {
    if (this.creator === undefined) {
      throw new Error(`The creator input is required for the CreateHealthForum element`);
    }
    if (this.peopleProfileHash === undefined) {
      throw new Error(`The peopleProfileHash input is required for the CreateHealthForum element`);
    }
    if (this.timestamp === undefined) {
      throw new Error(`The timestamp input is required for the CreateHealthForum element`);
    }
  },
  methods: {
    async createHealthForum() {
      const healthForum: HealthForum = { 
        creator: this.client.myPubKey!,

        title: this.title!,

        content: this.content!,

        people_profile_hash: this.peopleProfileHash!,

        timestamp: this.timestamp!,
      };

      try {
        const record: Record = await this.client.callZome({
          cap_secret: null,
          role_name: 'forum',
          zome_name: 'forum',
          fn_name: 'create_health_forum',
          payload: healthForum,
        });
        this.$emit('health-forum-created', record.signed_action.hashed.hash);
      } catch (e: any) {
        const errorSnackbar = this.$refs['create-error'] as Snackbar;
        errorSnackbar.labelText = `Error creating the health forum: ${e.data.data}`;
        errorSnackbar.show();
      }
    },
  },
  emits: ['health-forum-created'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client,
    };
  },
})
</script>
