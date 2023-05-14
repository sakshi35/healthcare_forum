import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource,  fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createHBlog } from './common.js';

test('create a HBlog and get my h blogs', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/healthcare_forum.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Bob gets my h blogs
    let collectionOutput: Record[] = await bob.cells[0].callZome({
      zome_name: "forum",
      fn_name: "get_my_h_blogs",
      payload: alice.agentPubKey
    });
    assert.equal(collectionOutput.length, 0);

    // Alice creates a HBlog
    const createdRecord: Record = await createHBlog(alice.cells[0]);
    assert.ok(createdRecord);
    
    await pause(1200);
    
    // Bob gets my h blogs again
    collectionOutput = await bob.cells[0].callZome({
      zome_name: "forum",
      fn_name: "get_my_h_blogs",
      payload: alice.agentPubKey
    });
    assert.equal(collectionOutput.length, 1);
    assert.deepEqual(createdRecord, collectionOutput[0]);    
  });
});

