PRIV=${PRIV:-0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80}
ETH_URL=${ETH_URL:-http://localhost:8545}
TOKEN_CONTRACT_ADDRESS=$(shell cat out/addresses.json | jq '.tokenAddress')
ROLLDOWN_CONTRACT_ADDRESS=$(shell cat out/addresses.json | jq '.rolldownContractAddress')

deploy:
	forge script script/rolldown.s.sol:MyScript --fork-url ${ETH_URL} --broadcast -vvvv --private-key ${PRIV}

deposit: deploy
	@echo TOKEN_CONTRACT_ADDRESS=${TOKEN_CONTRACT_ADDRESS}
	@echo ROLLDOWN_CONTRACT_ADDRESS=${ROLLDOWN_CONTRACT_ADDRESS}

	cast send ${TOKEN_CONTRACT_ADDRESS} "approve(address, uint256)" "${ROLLDOWN_CONTRACT_ADDRESS}" "4000" --private-key ${PRIV}
	cast send ${TOKEN_CONTRACT_ADDRESS} "mint(address, uint256)" "$(shell cast wallet address ${PRIV})" "4000" --private-key ${PRIV}
	cast send ${ROLLDOWN_CONTRACT_ADDRESS} "deposit(address, uint256)" "${TOKEN_CONTRACT_ADDRESS}" "4000" --private-key ${PRIV}

build:
	forge build

format:
	npx prettier -c .prettierrc.json --write --plugin=prettier-plugin-solidity './src/*.sol'
	npx prettier -c .prettierrc.json --write --plugin=prettier-plugin-solidity './test/*.sol'

update-abi: build
	cp out/rolldown.sol/RollDown.json ../rollup-sequencer/src
	cp out/rolldown.sol/RollDown.json ../rollup-updater/src

verify:
	docker run --network=host -it mangatasolutions/rolldown-contract:local verify-contract --verifier blockscout --verifier-url http://127.0.0.1/api 0x5f3f1dBD7B74C6B46e8c44f98792A1dAf8d69154 RollDown
