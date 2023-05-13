<template>
  <div>
    <div>
      <!-- Navigation bar -->
      <nav>
        <ul>
          <li @click="selectedTab = 'allBlogs'" :class="{ active: selectedTab === 'allBlogs' }">All Blogs</li>
          <li @click="selectedTab = 'createBlog'" :class="{ active: selectedTab === 'createBlog' }">Create Blog</li>
          <li @click="selectedTab = 'allProfiles'" :class="{ active: selectedTab === 'allProfiles' }">All Profiles</li>
          <li @click="selectedTab = 'createProfile'" :class="{ active: selectedTab === 'createProfile' }">Create Profile</li>
        </ul>
      </nav>
    </div>
    <div v-if="loading">
      <mwc-circular-progress indeterminate></mwc-circular-progress>
    </div>
    <div v-else>
      <div id="content" style="display: flex; flex-direction: column; flex: 1;">
        <AllListings v-if="selectedTab === 'allBlogs'"></AllListings>
        <CreateListing v-else-if="selectedTab === 'createBlog'" creator={}></CreateListing>
        <CreateProfile v-else-if="selectedTab === 'createProfile'" person={}></CreateProfile>
        <AllProfiles v-else-if="selectedTab === 'allProfiles'" person={}></AllProfiles>
        <CreateListingFormat listing-hash={}></CreateListingFormat>
        <ListingsByCreator :author="authorData"></ListingsByCreator>
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


export default defineComponent({
  components: {
    // Add your subcomponents here
    AllListings,
    CreateListing,
    CreateProfile,
    AllProfiles,
    CreateListingFormat,
    ListingsByCreator,
  },
  data(): {
    client: AppAgentClient | undefined;
    loading: boolean;
    selectedTab: 'allBlogs' | 'createBlog' | 'allProfiles' | 'createProfile';
    authorData: Object;
  } {
    return {
      client: undefined,
      loading: true,
      selectedTab: 'allBlogs',
      authorData: {},
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