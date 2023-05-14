import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createHealthForum, sampleHealthForum } from './common.js';

test('create HealthForum', async () => {
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

    // Alice creates a HealthForum
    const record: Record = await createHealthForum(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read HealthForum', async () => {
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

    const sample = await sampleHealthForum(alice.cells[0]);

    // Alice creates a HealthForum
    const record: Record = await createHealthForum(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created HealthForum
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "forum",
      fn_name: "get_health_forum",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test('create and update HealthForum', async () => {
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

    // Alice creates a HealthForum
    const record: Record = await createHealthForum(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the HealthForum
    let contentUpdate: any = await sampleHealthForum(alice.cells[0]);
    let updateInput = {
      original_health_forum_hash: originalActionHash,
      previous_health_forum_hash: originalActionHash,
      updated_health_forum: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "forum",
      fn_name: "update_health_forum",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated HealthForum
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "forum",
      fn_name: "get_health_forum",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the HealthForum again
    contentUpdate = await sampleHealthForum(alice.cells[0]);
    updateInput = { 
      original_health_forum_hash: originalActionHash,
      previous_health_forum_hash: updatedRecord.signed_action.hashed.hash,
      updated_health_forum: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "forum",
      fn_name: "update_health_forum",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated HealthForum
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "forum",
      fn_name: "get_health_forum",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);
  });
});

test('create and delete HealthForum', async () => {
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

    // Alice creates a HealthForum
    const record: Record = await createHealthForum(alice.cells[0]);
    assert.ok(record);
        
    // Alice deletes the HealthForum
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "forum",
      fn_name: "delete_health_forum",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await pause(1200);
        
    // Bob tries to get the deleted HealthForum
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "forum",
      fn_name: "get_health_forum",
      payload: record.signed_action.hashed.hash,
    });
    assert.notOk(readDeletedOutput);
  });
});
