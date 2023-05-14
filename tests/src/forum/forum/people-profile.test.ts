import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createPeopleProfile, samplePeopleProfile } from './common.js';

test('create PeopleProfile', async () => {
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

    // Alice creates a PeopleProfile
    const record: Record = await createPeopleProfile(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read PeopleProfile', async () => {
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

    const sample = await samplePeopleProfile(alice.cells[0]);

    // Alice creates a PeopleProfile
    const record: Record = await createPeopleProfile(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created PeopleProfile
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "forum",
      fn_name: "get_people_profile",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test('create and update PeopleProfile', async () => {
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

    // Alice creates a PeopleProfile
    const record: Record = await createPeopleProfile(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the PeopleProfile
    let contentUpdate: any = await samplePeopleProfile(alice.cells[0]);
    let updateInput = {
      original_people_profile_hash: originalActionHash,
      previous_people_profile_hash: originalActionHash,
      updated_people_profile: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "forum",
      fn_name: "update_people_profile",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated PeopleProfile
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "forum",
      fn_name: "get_people_profile",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the PeopleProfile again
    contentUpdate = await samplePeopleProfile(alice.cells[0]);
    updateInput = { 
      original_people_profile_hash: originalActionHash,
      previous_people_profile_hash: updatedRecord.signed_action.hashed.hash,
      updated_people_profile: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "forum",
      fn_name: "update_people_profile",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated PeopleProfile
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "forum",
      fn_name: "get_people_profile",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);
  });
});

test('create and delete PeopleProfile', async () => {
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

    // Alice creates a PeopleProfile
    const record: Record = await createPeopleProfile(alice.cells[0]);
    assert.ok(record);
        
    // Alice deletes the PeopleProfile
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "forum",
      fn_name: "delete_people_profile",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await pause(1200);
        
    // Bob tries to get the deleted PeopleProfile
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "forum",
      fn_name: "get_people_profile",
      payload: record.signed_action.hashed.hash,
    });
    assert.notOk(readDeletedOutput);
  });
});
