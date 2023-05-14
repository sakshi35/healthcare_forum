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
                <li @click="selectedTab = 'createBlog'" :class="{ active: selectedTab === 'createBlog' }" @listing-created="handleListingCreated">Create Blog</li>
                <li @click="selectedTab = 'myProfiles'" :class="{ active: selectedTab === 'myProfiles' }">My Profile</li>
              </ul>
            </nav>
          </div>
            <AllListings v-if="selectedTab === 'allBlogs'"></AllListings>
            <!-- <CreateListing v-else-if="selectedTab === 'createBlog'" :creator="authorData" @listing-created="handleListingCreated"></CreateListing> -->
            <div v-else-if="selectedTab === 'createBlog'">
              <CreateListing :creator="authorData" @listing-created="handleListingCreated"></CreateListing>
              <CreateListingFormat :listing-hash="listingHash"></CreateListingFormat>
              <ListingsByCreator :author="authorData"></ListingsByCreator>
            </div>
            <MyProfiles v-else-if="selectedTab === 'myProfiles'" :author="authorData"></MyProfiles>
          </div>
          <div v-else>
            <CreateProfile @profile-created="handleProfileCreated" :person="authorData"></CreateProfile>
          </div>
        </div>
      </div>
    </div>
  </template>
  <script lang="ts">
  import { defineComponent, computed } from 'vue';
  import { AppAgentClient, AppAgentWebsocket } from '@holochain/client';
  import '@material/mwc-circular-progress';
  import AllListings from './blog/blog/AllListings.vue';
  import CreateListing from './blog/blog/CreateListing.vue';
  import CreateProfile from './blog/blog/CreateProfile.vue';
  import AllProfiles from './blog/blog/AllProfiles.vue';
  import CreateListingFormat from './blog/blog/CreateListingFormat.vue';
  import ListingsByCreator from './blog/blog/ListingsByCreator.vue';
  import MyProfiles from './blog/blog/MyProfiles.vue';
  
  
  export default defineComponent({
    components: {
      // Add your subcomponents here
      AllListings,
      CreateListing,
      CreateProfile,
      AllProfiles,
      CreateListingFormat,
      ListingsByCreator,
      MyProfiles,
    },
    data(): {
      client: AppAgentClient | undefined;
      loading: boolean;
      selectedTab: 'allBlogs' | 'createBlog' | 'myProfiles' | 'createProfile';
      authorData: Object;
      isProfileCreated: boolean;
      listingHash: String;
      profileName: String;
    } {
      return {
        client: undefined,
        loading: true,
        selectedTab: 'allBlogs',
        authorData: {},
        isProfileCreated: false,
        listingHash: "",
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
      handleListingCreated(hash: string) {
        // Handle the created listing hash
        this.listingHash = hash;
        console.log('Listing created:', hash);
        // You can store the hash in the parent component's data or use it as needed
      },
      async handleProfileCreated(profileName: String) {
        // ... your logic to handle the profile creation
        this.profileName = profileName;
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