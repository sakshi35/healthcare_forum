<template>
  <div v-if="!loading">
    <div v-if="editing" style="display: flex; flex: 1;">
      <EditName
        :original-name-hash="nameHash"
        :current-record="record!"
        @name-updated="editing = false; fetchName();"
        @edit-canceled="editing = false"
      ></EditName>
    </div>
    <div v-else-if="record" style="display: flex; flex-direction: column">
      <div style="display: flex; flex-direction: row">
        <span style="flex: 1"></span>
      
        <mwc-icon-button style="margin-left: 8px" icon="edit" @click="editing = true"></mwc-icon-button>
        <mwc-icon-button style="margin-left: 8px" icon="delete" @click="deleteName()"></mwc-icon-button>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Blog Name: </strong></span>
 	<span style="white-space: pre-line">{{  name?.blog_name }} </span>
      </div>

      <div style="display: flex; flex-direction: row; margin-bottom: 16px;">
	<span style="margin-right: 4px"><strong>Blog Description: </strong></span>
 	<span style="white-space: pre-line">{{  name?.blog_description }} </span>
      </div>

    </div>
    
    <span v-else>The requested name was not found.</span>
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
import { Name } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-icon-button';
import '@material/mwc-snackbar';
import { Snackbar } from '@material/mwc-snackbar';
import EditName from './EditName.vue';

export default defineComponent({
  components: {
    EditName
  },
  props: {
    nameHash: {
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
    name() {
      if (!this.record) return undefined;
      return decode((this.record.entry as any).Present.entry) as Name;
    }
  },
  async mounted() {
    if (this.nameHash === undefined) {
      throw new Error(`The nameHash input is required for the NameDetail element`);
    }

    await this.fetchName();
  },
  methods: {
    async fetchName() {
      this.loading = true;
      this.record = undefined;

      this.record = await this.client.callZome({
        cap_secret: null,
        role_name: 'blog',
        zome_name: 'blog',
        fn_name: 'get_name',
        payload: this.nameHash,
      });

      this.loading = false;
    },
    async deleteName() {
      try {
        await this.client.callZome({
          cap_secret: null,
          role_name: 'blog',
          zome_name: 'blog',
          fn_name: 'delete_name',
          payload: this.nameHash,
        });
        this.$emit('name-deleted', this.nameHash);
        this.fetchName();
      } catch (e: any) {
        const errorSnackbar = this.$refs['delete-error'] as Snackbar;
        errorSnackbar.labelText = `Error deleting the name: ${e.data.data}`;
        errorSnackbar.show();
      }
    }
  },
  emits: ['name-deleted'],
  setup() {
    const client = (inject('client') as ComputedRef<AppAgentClient>).value;
    return {
      client
    };
  },
})
</script>
