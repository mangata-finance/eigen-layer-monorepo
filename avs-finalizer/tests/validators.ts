import {
    DEFAULT_QUORUM,
    getEntryFromBlsApkRegistry, getEntryFromIndexRegistry,
    getEntryFromStakeRegistry,
    getOperatorId
} from "./operatorUtilities";
import {expect} from "@jest/globals";

export async function validateBLSApkRegistry(publicClient: any, operatorAddress: string, operatorId: string) {
    const response = await getEntryFromBlsApkRegistry(publicClient, "getRegisteredPubkey", [operatorAddress] );
    const g1Point = response[0];
    expect(response[1]).toBe(operatorId);
    const pubKey = await getEntryFromBlsApkRegistry(publicClient, "operatorToPubkey", [operatorAddress] );
    expect(pubKey[0].toString()).toBe(JSON.parse(JSON.stringify(g1Point)).X);
    expect(pubKey[1].toString()).toBe(JSON.parse(JSON.stringify(g1Point)).Y);
    const pubKeyHash = await getEntryFromBlsApkRegistry(publicClient, "operatorToPubkeyHash", [operatorAddress] );
    const operatorAddr = await getEntryFromBlsApkRegistry(publicClient, "getOperatorFromPubkeyHash", [pubKeyHash] );
    expect(operatorAddr).toBe(operatorAddress);

}

export async function validateOperatorOptInStakeRegistry(publicClient: any, operatorAddress: string) {
    const operatorId = await getOperatorId(publicClient, operatorAddress);
    const currStake = await getEntryFromStakeRegistry(publicClient, "getCurrentStake", [operatorId, DEFAULT_QUORUM] );
    expect(currStake).toBeGreaterThan(0);
}
export async function validateOperatorOptOutStakeRegistry(publicClient: any, operatorAddress: string) {
    const operatorId = await getOperatorId(publicClient, operatorAddress);
    const currStake = await getEntryFromStakeRegistry(publicClient, "getCurrentStake", [operatorId, DEFAULT_QUORUM] );
    // @ts-ignore
    expect(currStake).toBe(0n);
}

export async function getLatestQuorumUpdate(publicClient : any) {
    const latestQuorumUpdate = await getEntryFromIndexRegistry(publicClient, "getLatestQuorumUpdate", [DEFAULT_QUORUM] );
    return {
        fromBlockNumber: JSON.parse(JSON.stringify(latestQuorumUpdate)).fromBlockNumber,
        numOperators: JSON.parse(JSON.stringify(latestQuorumUpdate)).numOperators
    }
}
export async function validateOperatorOptInIndexRegistry(publicClient: any, operatorAddress: string) {
    const operatorId = await getOperatorId(publicClient, operatorAddress);
    const currOperatorIndex = await getEntryFromIndexRegistry(publicClient, "currentOperatorIndex", [DEFAULT_QUORUM, operatorId] );
    expect(currOperatorIndex).toBeGreaterThan(0);
    const latestUpdate = await getEntryFromIndexRegistry(publicClient, "getLatestOperatorUpdate", [DEFAULT_QUORUM, currOperatorIndex] );
    const latestQuorumUpdate = await getEntryFromIndexRegistry(publicClient, "getLatestQuorumUpdate", [DEFAULT_QUORUM] );
    expect(JSON.parse(JSON.stringify(latestUpdate)).fromBlockNumber).toBe(JSON.parse(JSON.stringify(latestQuorumUpdate)).fromBlockNumber);
    expect(parseInt(currOperatorIndex.toString()) +1 ).toBe(JSON.parse(JSON.stringify(latestQuorumUpdate)).numOperators);

}
export async function validateOperatorOptOutIndexRegistry(publicClient: any, operatorAddress: string, statusBeforeOptOut: any) {
    const operatorId = await getOperatorId(publicClient, operatorAddress);
    const numOperatorsBefore = statusBeforeOptOut.numOperators;

    const currOperatorIndex = await getEntryFromIndexRegistry(publicClient, "currentOperatorIndex", [DEFAULT_QUORUM, operatorId] );
    expect(numOperatorsBefore - parseInt(currOperatorIndex.toString())).toBe(1);

    const latestUpdate = await getEntryFromIndexRegistry(publicClient, "getLatestOperatorUpdate", [DEFAULT_QUORUM, currOperatorIndex] );
    const latestQuorumUpdate = await getEntryFromIndexRegistry(publicClient, "getLatestQuorumUpdate", [DEFAULT_QUORUM] );
    expect(JSON.parse(JSON.stringify(latestUpdate)).fromBlockNumber).toBe(JSON.parse(JSON.stringify(latestQuorumUpdate)).fromBlockNumber);
    expect(parseInt(numOperatorsBefore) - 1 ).toBe(JSON.parse(JSON.stringify(latestQuorumUpdate)).numOperators);

}