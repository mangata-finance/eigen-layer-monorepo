import { signTx } from "gasp-sdk";
import "gasp-types";
import type { HeaderExtended } from "@polkadot/api-derive/type/types";
import "dotenv/config";
import { keccak256, BaseError } from "viem";
import { MANGATA_NODE_URL, MNEMONIC } from "./common/constants.js";
import {
	countRequests,
	filterUpdates,
	getApi,
	getCollator,
	getLastRequestId,
	getMaxRequestId,
	getSelectedSequencerWithRights,
	initReadContractWithRetry,
	isSuccess,
	print,
	processDataForL2Update,
	processPendingRequestsEvents,
} from "./utils/index.js";
import { getPublicClient } from "./viem/client.js";
import { webSocketTransport } from "./viem/transport.js";

async function main() {
	let inProgress = false;

	const publicClient = getPublicClient({ transport: webSocketTransport });

	const api = await getApi(MANGATA_NODE_URL);

	await initReadContractWithRetry(publicClient, api);

	let lastRequestId = await getLastRequestId(api);

  await api.derive.chain.subscribeNewHeads(async (header: HeaderExtended) => {
    const collator = getCollator("ethereum", MNEMONIC);
    print(`collator address: ${collator.address}`)
    print(`block #${header.number} was authored by ${header.author}`);
    const { isSequencerSelected, hasSequencerRights, selectedSequencer } =
      await getSelectedSequencerWithRights(api, collator.address, header.hash);
    print(`me ${collator.address}`);
    print(`selected : ${selectedSequencer}`);
    print(`is selected ${isSequencerSelected}`);
    print(`rights : ${hasSequencerRights}`);
    if (isSequencerSelected && hasSequencerRights) {
      try {
        if (inProgress) {
          return;
        }else{
					print("In progress, skipping...");
          inProgress = true;
        }
        const nativeL1Update = await processDataForL2Update(
          api,
          publicClient,
        );

				const filteredUpdates = filterUpdates(
					nativeL1Update.unwrap(),
					lastRequestId,
				);
				const requestsCount = countRequests(filteredUpdates);

				if (requestsCount > 0 && getMaxRequestId(filteredUpdates)! > lastRequestId) {
					const result = await signTx(
						api,
						api.tx.rolldown.updateL2FromL1(filteredUpdates),
						collator,
					);

					if (isSuccess(result)) {
						print("L1update was submitted successfully");
							lastRequestId = getMaxRequestId(filteredUpdates)!;
					} else {
						print("L1update was submitted unsuccessfully");
					}
				} else {
					print(`L1Update with max id == ${lastRequestId} was already submitted`);
				}
			} catch (e) {
        if (e instanceof BaseError) {
          print("Viem error occured - restarting service");
          print(e);
          throw e;
        }else{
          print("The contract function getUpdateForL2 returned no data");
          print(e);
          // Do nothing with error
          // Error only appear when we have block where there are no data for getUpdateForL2 at all.
          // This is only in the very beginning
          // ContractFunctionExecutionError: The contract function "getUpdateForL2" returned no data ("0x").
        }
			}
			inProgress = false;
		}

		await processPendingRequestsEvents(
			api,
			publicClient,
			header.hash,
			collator,
		);
	});
}

main()
  .then(() => {
    print("Success");
  })
  .catch((e) => {
    console.error("Something went wrong", e);
    process.exit( 1); 
  })
