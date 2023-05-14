<template>
    <div>
      <div v-if="loading">
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>
      <div v-else>
        <div id="content" style="display: flex; flex-direction: column; flex: 1;">
          <div v-if="isProfileCreated">
            <div>
            <nav>
              <ul>
                <li @click="selectedTab = 'allBlogs'" :class="{ active: selectedTab === 'allBlogs' }">All Blogs</li>
                <li @click="selectedTab = 'createBlog'" :class="{ active: selectedTab === 'createBlog' }">Create Blog</li>
                <li @click="selectedTab = 'myblogs'" :class="{ active: selectedTab === 'myblogs' }">My Blogs</li>
                <li @click="selectedTab = 'myProfiles'" :class="{ active: selectedTab === 'myProfiles' }">My Profile</li>
              </ul>
            </nav>
          </div>
            <CreateHBlog v-if="selectedTab === 'createBlog'" :creator="authorData" ></CreateHBlog>
            <AllHBlogs v-else-if="selectedTab === 'allBlogs'" ></AllHBlogs>
            <MyProfiles v-else-if="selectedTab === 'myProfiles'" :author="authorData"></MyProfiles>
            <MyHBlogs v-else-if="selectedTab === 'myblogs'" :author="authorData"></MyHBlogs>
          </div>
          <div v-else>
            <CreatePeopleProfile @people-profile-created="handleProfileCreated" :person="authorData"></CreatePeopleProfile>
          </div>
        </div>
      </div>
    </div>
  </template>
  <script lang="ts">
  import { defineComponent, computed } from 'vue';
  import { AppAgentClient, AppAgentWebsocket } from '@holochain/client';
  import '@material/mwc-circular-progress';
  import CreateHBlog from './forum/forum/CreateHBlog.vue';
  import AllHBlogs from './forum/forum/AllHBlogs.vue';
  import MyHBlogs from './forum/forum/MyHBlogs.vue';
  import CreatePeopleProfile from './forum/forum/CreatePeopleProfile.vue';
  import MyProfiles from './forum/forum/MyProfiles.vue';
  import {EntryHash} from '@holochain/client';
  
  
  export default defineComponent({
    components: {
      // Add your subcomponents here
      CreateHBlog,
      AllHBlogs,
      CreatePeopleProfile,
      MyProfiles,
      MyHBlogs,
    },
    data(): {
      client: AppAgentClient | undefined;
      loading: boolean;
      selectedTab: 'allBlogs' | 'createBlog' | 'myProfiles' | 'createProfile' | 'myblogs';
      authorData: Object;
      isProfileCreated: boolean;
      //profileHash: EntryHash;
      profileName: string;
    } {
      return {
        client: undefined,
        loading: true,
        selectedTab: 'allBlogs',
        authorData: {},
        isProfileCreated: false,
        //profileHash: new Uint8Array(0) as EntryHash,
        profileName: "",
      };
    },
    async mounted() {
      // We pass '' as url because it will dynamically be replaced in launcher environments
      this.client = await AppAgentWebsocket.connect('', 'healthcare_forum');
      this.authorData = this.client.myPubKey;
      this.loading = false;
    },
    provide() {
      return {
        client: computed(() => this.client),
        //myPubKey: computed(() => this.client?.myPubKey),
      };
    },
    methods: {
      async handleProfileCreated(hash: EntryHash) {
        // ... your logic to handle the profile creation
        //console.log("profileName: "+profileName);
        //this.profileName = profileName;
        console.log("hash: "+hash);
        //this.profileHash = hash;
        this.isProfileCreated = true;
      },
    },
  });
  </script>
  <style>
  nav ul {
    display: flex;
    list-style: none;
    padding: 0;
  }
  
  nav li {
    margin-right: 10px;
    cursor: pointer;
    border: 1px solid #ccc;
    padding: 5px 10px;
  }
  
  nav li.active {
    background-color: #ccc;
    font-weight: bold;
  }
  
  </style>