import * as util from "node:util";
import {ApiPromise} from "@polkadot/api";
import {decodeAbiParameters, publicActions, type PublicClient, type WalletClient, type Hash, keccak256} from "viem";
import {LIMIT, MANGATA_CONTRACT_ADDRESS, ROLLDOWN_METADATA, ROLLDOWN_ABI, L1_CHAIN} from "../common/constants.js";
import {ethAccount, getChain} from "../viem/client.js";
import {Cancel, L2Update, RequestResult, Withdrawal} from "../common/types.js";
import {estimateMaxPriorityFeePerGas} from "viem/actions";
import type { Option, u128, u32, BTreeMap  } from '@polkadot/types-codec';
import type { ITuple } from '@polkadot/types-codec/types';
import type { PalletRolldownL2Request } from '@polkadot/types/lookup';
import type { H256 } from '@polkadot/types/interfaces/runtime';
import type {
  SpRuntimeAccountAccountId20,
	PalletRolldownMessagesChain,
} from "@polkadot/types/lookup";


export async function getLatestRequestIdSubmittedToL1(publicClient: PublicClient) {
    return (await publicClient.readContract({
        address: MANGATA_CONTRACT_ADDRESS,
        abi: ROLLDOWN_ABI,
        functionName: "lastProcessedUpdate_origin_l2",
    })) as bigint;
}

async function getLastBatchId(api: ApiPromise, blockHash: Uint8Array) {
    const chain: PalletRolldownMessagesChain = api.createType('PalletRolldownMessagesChain', L1_CHAIN);
    let apiAt = await api.at(blockHash);
    let last_batch = await apiAt.query.rolldown.l2RequestsBatchLast();

    // NOTE: looks like === is not implemented for PalletRolldownMessagesChain
    // therefore its not possible to query valu from map using .get(chain) query ;<
    let found = Array.from(last_batch.keys()).findIndex( (key) => {
      return key.toString() === chain.toString();
    });

    if (found == -1){
      return null;
    } else {
      return Array.from(last_batch.values())[found][1].toBigInt();
    }
}

async function findBatchWithNewUpdates(api: ApiPromise, publicClient: PublicClient, blockHash: Uint8Array) {

    let batchId = await getLastBatchId(api, blockHash)
    if (batchId == null){
      return null;
    }

    const lastSubmittedId = await getLatestRequestIdSubmittedToL1(publicClient);
    const nextRequestId = lastSubmittedId + 1n;

    while (batchId > 0) {
      const chain: PalletRolldownMessagesChain = api.createType('PalletRolldownMessagesChain', L1_CHAIN);
      let batch: Option<ITuple<[u32, ITuple<[u128, u128]>, SpRuntimeAccountAccountId20]>> = await api.query.rolldown.l2RequestsBatch([chain, batchId]);

      batchId -= 1n;
      if (batch.isNone){
        continue;
      }

      let rangeStart = batch.value[1][0].toBigInt();
      let rangeStop = batch.value[1][1].toBigInt();


      if (rangeStart <= nextRequestId && rangeStop >= nextRequestId) {
        return [rangeStart, rangeStop];
      }

      if (rangeStart < nextRequestId && rangeStop < nextRequestId){
        return null;
      }
    }

    console.log(`couldnt find any batch with requestId: ${nextRequestId}`);
    return null;
}


async function estimateGasInWei(publicClient: PublicClient) {
    // https://www.blocknative.com/blog/eip-1559-fees
    // We do not want VIEM estimate we would like to make our own estimate
    // based on this equation: Max Fee = (2 * Base Fee) + Max Priority Fee

    // Max Fee = maxFeePerGas (viem)
    // Max Priority Fee = maxPriorityFeePerGas (viem)

    const baseFeeInWei = await publicClient.getGasPrice()

    const maxPriorityFeePerGasInWei =  await estimateMaxPriorityFeePerGas(publicClient)

    const maxFeeInWei = BigInt(2) * BigInt(baseFeeInWei) + BigInt(maxPriorityFeePerGasInWei)

    return {maxFeeInWei, maxPriorityFeePerGasInWei}
}

async function sendUpdateToL1(
    api: ApiPromise,
    walletClient: WalletClient,
    publicClient: PublicClient,
    blockHash: Uint8Array,
) {
    let requestsRange = await findBatchWithNewUpdates(api, publicClient, blockHash);

    if (requestsRange == null){
      return null;
    }
    const rangeStart = requestsRange[0];
    const rangeEnd = requestsRange[1];

    let root = await api.rpc.rolldown.get_merkle_root(L1_CHAIN, [rangeStart, rangeEnd]);
    if (root.toString() == "0x0000000000000000000000000000000000000000000000000000000000000000") {
      return null
    }

    const {maxFeeInWei, maxPriorityFeePerGasInWei} = await estimateGasInWei(publicClient)
    const {request} = await publicClient.simulateContract({
        account: ethAccount,
        chain: getChain(),
        abi: ROLLDOWN_ABI,
        address: MANGATA_CONTRACT_ADDRESS,
        functionName: "update_l1_from_l2",
        args: [root.toHex(), [rangeStart, rangeEnd]],
        maxFeePerGas: maxFeeInWei,
        maxPriorityFeePerGas: maxPriorityFeePerGasInWei
    })
    const txHash = await walletClient.writeContract(request);
    const result = await publicClient.waitForTransactionReceipt({ hash: txHash });
    console.log(`#${result.blockNumber} ${result.transactionHash} : ${result.status}`);
    return requestsRange
}

async function findMerkleRange(
    publicClient: PublicClient,
    requestId: bigint
) {
    return (await publicClient.readContract({
        address: MANGATA_CONTRACT_ADDRESS,
        abi: ROLLDOWN_ABI,
        functionName: "find_l2_batch",
        args: [requestId],
    }));
}

async function isL2RequestAlreadyExecuted(
    publicClient: PublicClient,
    requestHash: H256
) {
    let status = await publicClient.readContract({
        address: MANGATA_CONTRACT_ADDRESS,
        abi: ROLLDOWN_ABI,
        functionName: "processedL2Requests",
        args: [requestHash.toString()],
    })

    const closed = await publicClient.readContract({
      address: MANGATA_CONTRACT_ADDRESS,
      abi: ROLLDOWN_ABI,
      functionName: 'CLOSED',
    });

    return status === closed;
}

async function getRequest(
    api: ApiPromise,
    requestId: bigint
) {
  let request   = await api.query.rolldown.l2Requests(L1_CHAIN, {origin:'L2', id:requestId})!;
  return (request as Option<ITuple<[PalletRolldownL2Request, H256]>>);
}

export async function closeWithdrawals(
    api: ApiPromise,
    walletClient: WalletClient,
    publicClient: PublicClient,
    lastExecutedRequestId: bigint
)  {
    const lastSubmittedId = (await publicClient.readContract({
        address: MANGATA_CONTRACT_ADDRESS,
        abi: ROLLDOWN_ABI,
        functionName: "lastProcessedUpdate_origin_l2",
    })) as bigint;

    let indexes = [];
    for( let i = lastExecutedRequestId + 1n; i <= lastSubmittedId; i++){
      indexes.push(i);
    }

    if (indexes.length == 0){
      return lastExecutedRequestId;
    }

    for (let withdrawalRequestId of indexes){
      let req = await getRequest(api, withdrawalRequestId);
      if (req.isNone){
        console.log(`ignroing non existing request ${withdrawalRequestId}`);
        continue;
      }

      let [r, hash] = req.unwrap();
      if (!r.isWithdrawal){
        console.log(`ignroing non withdrawal request ${withdrawalRequestId}`);
        continue;
      }

      if (await isL2RequestAlreadyExecuted(publicClient, hash)){
        console.log(`withdrawal ${withdrawalRequestId} already executed - ignoring...`);
        continue;
      };

      let range = await findMerkleRange(publicClient, withdrawalRequestId)
      const rangeStart = (range as any).start;
      const rangeEnd = (range as any).end;
      let encodedWithdrawal = await api.rpc.rolldown.get_abi_encoded_l2_request(L1_CHAIN, withdrawalRequestId);
      const chain = api.createType('Chain', L1_CHAIN);
      console.log(`chain: ${chain} range: [${rangeStart}, ${rangeEnd}] withdrawalRequestId: ${withdrawalRequestId} encodedWithdrawal: ${encodedWithdrawal}`);
      let root = await api.rpc.rolldown.get_merkle_root(chain, [rangeStart, rangeEnd]);
      let proof = await api.rpc.rolldown.get_merkle_proof(chain, [rangeStart, rangeEnd], withdrawalRequestId);
      const withdrawal = decodeAbiParameters((ROLLDOWN_METADATA as any).output.abi.find((e: any) => e.name === "close_withdrawal")!.inputs[0].components, encodedWithdrawal.toHex())
      const {maxFeeInWei, maxPriorityFeePerGasInWei} = await estimateGasInWei(publicClient)
      const {request} = await publicClient.simulateContract({
          account: ethAccount,
          chain: getChain(),
          abi: ROLLDOWN_ABI,
          address: MANGATA_CONTRACT_ADDRESS,
          functionName: "close_withdrawal",
          // TODO: it should be possible to pass abi encoded withdrawal fetched from gasp directly (without deserialization)
          args: [withdrawal, root.toHuman(), proof.toHuman()],
          maxFeePerGas: maxFeeInWei,
          maxPriorityFeePerGas: maxPriorityFeePerGasInWei
      })
      const txHash = await walletClient.writeContract(request);
      const result = await publicClient.waitForTransactionReceipt({ hash: txHash });
      console.log(`closing withdrawal ${withdrawalRequestId}: tx:${result.transactionHash} - ${result.status}`);
      lastExecutedRequestId = withdrawalRequestId;
    }
    return lastExecutedRequestId;
}

function print(data: any) {
    console.log(util.inspect(data, { depth: null }));
}

export {
    print,
    sendUpdateToL1
}
