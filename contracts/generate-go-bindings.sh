#!/bin/bash

function create_binding {
    contract_dir=$1
    contract=$2
    binding_dir=$3
    echo "generating bindings for" $contract
    mkdir -p $binding_dir/${contract}
    mkdir -p data_bkp/${contract}
    contract_json="$contract_dir/out/${contract}.sol/${contract}.json"
    solc_abi=$(cat ${contract_json} | jq -r '.abi')
    solc_bin=$(cat ${contract_json} | jq -r '.bytecode.object')

    mkdir -p data
    echo ${solc_abi} >data/tmp.abi
    echo ${solc_bin} >data/tmp.bin

    rm -f $binding_dir/${contract}/binding.go
    abigen --bin=data/tmp.bin --abi=data/tmp.abi --pkg=contract${contract} --out=$binding_dir/${contract}/binding.go
    # mv data/tmp.abi data_bkp/${contract}/tmp.abi
    # mv data/tmp.bin data_bkp/${contract}/tmp.bin
    rm -rf ../data/tmp.abi ../data/tmp.bin
}

rm -rf bindings/*
forge clean
forge build

avs_service_contracts="FinalizerServiceManager FinalizerTaskManager"
for contract in $avs_service_contracts; do
    create_binding . $contract ../avs-aggregator/bindings
done

# create_binding . ERC20Mock ../avs-aggregator/bindings
