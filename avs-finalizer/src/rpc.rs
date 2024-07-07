use crate::{
    cli::CliArgs,
    crypto::bn254::{BlsKeypair, BlsSignature, PrivateKey},
};
use ark_ec::AffineRepr;
use ark_ff::PrimeField;
use bindings::{shared_types::{Task, TaskResponse,*},finalizer_task_manager::OperatorStateInfo};
use ethers::abi::AbiEncode;
use reqwest::Response;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{
    default_on_request_failure, default_on_request_success, policies::ExponentialBackoff,
    RetryTransientMiddleware, Retryable, RetryableStrategy,
};
use serde::{ser::SerializeStruct, Serialize};
use sp_runtime::traits::{Hash, Keccak256};
use tracing::instrument;
use sp_core::Bytes;

type Bytes32 = [u8; 32];

#[derive(Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SignedTaskResponse {
    task_response: Bytes,
    bls_signature: BlsSignatureWire,
    operator_id: Bytes32,
}

// #[derive(Serialize, Debug,
//     ::ethers::contract::EthAbiType,
//     ::ethers::contract::EthAbiCodec,)]
// #[serde(rename_all = "PascalCase")]
// struct TaskResponseWire {
//     pub reference_task_index: u32,
//     pub reference_task: Task,
//     // #[serde(with = "OperatorStateInfoWire")]
//     pub operators_state_info: OperatorStateInfo,
//     pub block_hash: Bytes32,
//     pub storage_proof_hash: Bytes32,
//     pub pending_state_hash: Bytes32,
// }


// #[derive(Serialize, Debug)]
// #[serde(rename_all = "PascalCase")]
// #[serde(remote = "OperatorStateInfo")]
// pub struct OperatorStateInfoWire {
//     pub operators_state_changed: bool,
//     pub operators_state_provided: bool,
//     pub quorums_removed: ::std::vec::Vec<u8>,
//     #[serde(with = "Vec<QuorumsAddedWire>")]
//     pub quorums_added: ::std::vec::Vec<QuorumsAdded>,
//     #[serde(with = "QuorumsStakeUpdateWire")]
//     pub quorums_stake_update: ::std::vec::Vec<QuorumsStakeUpdate>,
//     #[serde(with = "QuorumsApkUpdateWire")]
//     pub quorums_apk_update: ::std::vec::Vec<QuorumsApkUpdate>,
//     pub operators_removed: ::std::vec::Vec<[u8; 32]>,
//     #[serde(with = "OperatorsAddedWire")]
//     pub operators_added: ::std::vec::Vec<OperatorsAdded>,
//     #[serde(with = "OperatorsStakeUpdateWire")]
//     pub operators_stake_update: ::std::vec::Vec<OperatorsStakeUpdate>,
//     #[serde(with = "OperatorsQuorumCountUpdateWire")]
//     pub operators_quorum_count_update: ::std::vec::Vec<OperatorsQuorumCountUpdate>,
// }


// #[derive(Serialize, Debug)]
// #[serde(rename_all = "PascalCase")]
// #[serde(remote = "OperatorsAdded")]
// pub struct OperatorsAddedWire {
//     pub operator_id: [u8; 32],
//     pub quorum_for_stakes: ::std::vec::Vec<u8>,
//     pub quorum_stakes: ::std::vec::Vec<u128>,
//     pub quorum_count: u8,
// }
// #[derive(Serialize, Debug)]
// #[serde(rename_all = "PascalCase")]
// #[serde(remote = "OperatorsQuorumCountUpdate")]
// pub struct OperatorsQuorumCountUpdateWire {
//     pub operator_id: [u8; 32],
//     pub quorum_count: u8,
// }
// #[derive(Serialize, Debug)]
// #[serde(rename_all = "PascalCase")]
// #[serde(remote = "OperatorsStakeUpdate")]
// pub struct OperatorsStakeUpdateWire {
//     pub operator_id: [u8; 32],
//     pub quorum_for_stakes: ::std::vec::Vec<u8>,
//     pub quorum_stakes: ::std::vec::Vec<u128>,
// }
// #[derive(Serialize, Debug)]
// #[serde(rename_all = "PascalCase")]
// #[serde(remote = "QuorumsAdded")]
// pub struct QuorumsAddedWire {
//     pub quorum_number: u8,
//     pub quorum_stake: u128,
//     #[serde(with = "G1PointWire")]
//     pub quorum_apk: G1Point,
// }
// #[derive(Serialize, Debug)]
// #[serde(rename_all = "PascalCase")]
// #[serde(remote = "QuorumsApkUpdate")]
// pub struct QuorumsApkUpdateWire {
//     pub quorum_number: u8,
//     #[serde(with = "G1PointWire")]
//     pub quorum_apk: G1Point,
// }
// #[derive(Serialize, Debug)]
// #[serde(rename_all = "PascalCase")]
// #[serde(remote = "QuorumsStakeUpdate")]
// pub struct QuorumsStakeUpdateWire {
//     pub quorum_number: u8,
//     pub quorum_stake: u128,
// }


// impl From<TaskResponse> for TaskResponseWire {
//     fn from(value: TaskResponse) -> Self {
//         Self {
//             reference_task_index: value.reference_task_index,
//             reference_task: value.reference_task,
//             operators_state_info: value.operators_state_info,
//             block_hash: value.block_hash,
//             storage_proof_hash: value.storage_proof_hash,
//             pending_state_hash: value.pending_state_hash,
//         }
//     }
// }

#[derive(Serialize, Debug)]
struct BlsSignatureWire {
    g1_point: G1PointWire,
}

impl From<BlsSignature> for BlsSignatureWire {
    fn from(value: BlsSignature) -> Self {
        Self {
            g1_point: G1PointWire {
                x: value.x().unwrap().into_bigint(),
                y: value.y().unwrap().into_bigint(),
            },
        }
    }
}

#[derive(Debug)]
// #[derive(Serialize, Debug)]
// #[serde(rename_all = "PascalCase")]
// #[serde(remote = "G1Point")]
struct G1PointWire {
    x: <PrivateKey as PrimeField>::BigInt,
    y: <PrivateKey as PrimeField>::BigInt,
}

impl Serialize for G1PointWire {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("g1_point", 2)?;
        s.serialize_field("X", &self.x.to_string())?;
        s.serialize_field("Y", &self.y.to_string())?;
        s.end()
    }
}

#[derive(Debug)]
pub struct Rpc {
    client: ClientWithMiddleware,
    avs_url: String,
}

struct RetryFailed;
impl RetryableStrategy for RetryFailed {
    fn handle(&self, res: &reqwest_middleware::Result<reqwest::Response>) -> Option<Retryable> {
        match res {
            // retry if 404 task not initialized, in case block reexecution is faster the aggr task initialization, usually on local testnet
            Ok(success) if success.status() == 404 => Some(Retryable::Transient),
            Ok(success) => default_on_request_success(success),
            Err(error) => default_on_request_failure(error),
        }
    }
}

impl Rpc {
    pub fn build(cfg: &CliArgs) -> Self {
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
        let client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy_and_strategy(
                retry_policy,
                RetryFailed,
            ))
            .build();
        Self {
            client,
            avs_url: cfg.avs_rpc_url.to_owned(),
        }
    }

    #[instrument(skip(self, keypair))]
    pub async fn send_task_response(
        &self,
        task_response: TaskResponse,
        keypair: &BlsKeypair,
    ) -> eyre::Result<Response> {
        println!("{:?}", task_response.clone());
        let req = create_response(task_response, keypair)?;
        println!("{:?}", req);
        let json: String = serde_json::to_string(&req)?;
        println!("{:?}", json);

        Ok(self.client.post(&self.avs_url).body(json).send().await?)
    }
}

fn create_response(task: TaskResponse, keypair: &BlsKeypair) -> eyre::Result<SignedTaskResponse> {
    let encoded: Vec<u8> = vec![0u8;31].into_iter().chain(vec![32u8]).chain(
        task.clone().encode().into_iter()
    ).collect::<Vec<_>>().into();
    // let encoded: Vec<u8> = task.clone().encode();

    let hash = Keccak256::hash(encoded.as_ref());
    let sig = keypair.sign(hash.as_bytes())?;

    println!("{:?}", hash);
    println!("{:?}", sig);
    Ok(SignedTaskResponse {
        bls_signature: sig.into(),
        task_response: vec![0u8;31].into_iter().chain(vec![32u8]).chain(
            task.encode().into_iter()
        ).collect::<Vec<_>>().into(),
        // task_response: task.encode().into(),
        operator_id: keypair.operator_id().to_fixed_bytes(),
    })
}

// avs-aggregator/rpc_server.go:63	handler	{"response": {"TaskResponse":"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","BlsSignature":{"g1_point":{"X":"327143716551943380499027451660434120130853194850972325174736267589566235961","Y":"4499780053681215289133809248493470509520126308294493602809662272784687092145"}},"OperatorId":[228,221,146,1,219,55,27,130,158,236,61,74,92,4,181,120,218,104,236,204,76,62,6,85,178,21,174,173,169,18,49,16]}}
// avs-aggregator-1  | 2024-06-19T21:38:03.797Z	INFO	avs-aggregator/rpc_server.go:94	Received signed task response	{"response": {"TaskResponse":"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","BlsSignature":{"g1_point":{"X":"327143716551943380499027451660434120130853194850972325174736267589566235961","Y":"4499780053681215289133809248493470509520126308294493602809662272784687092145"}},"OperatorId":[228,221,146,1,219,55,27,130,158,236,61,74,92,4,181,120,218,104,236,204,76,62,6,85,178,21,174,173,169,18,49,16]}, "operatorId": "e4dd9201db371b829eec3d4a5c04b578da68eccc4c3e0655b215aeada9123110"}

// TaskResponse { reference_task_index: 2, reference_task_hash: [240, 106, 28, 134, 142, 51, 186, 114, 19, 46, 201, 169, 84, 93, 77, 241, 212, 108, 114, 64, 217, 13, 22, 100, 39, 254, 221, 15, 213, 163, 42, 12], hashes: [[], [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7], [76, 223, 243, 17, 101, 19, 229, 215, 217, 6, 120, 246, 47, 14, 0, 38, 127, 1, 112, 8, 145, 97, 138, 176, 104, 128, 43, 252, 69, 244, 126, 103], [105, 176, 120, 47, 250, 10, 215, 47, 100, 131, 144, 178, 212, 173, 138, 189, 194, 191, 199, 8, 158, 1, 141, 29, 20, 82, 236, 72, 143, 252, 71, 241], [12, 120, 215, 213, 7, 95, 138, 237, 130, 141, 60, 72, 235, 217, 108, 15, 228, 194, 193, 101, 15, 186, 187, 33, 91, 131, 126, 82, 143, 135, 78, 225], [31, 188, 19, 31, 78, 175, 205, 220, 101, 13, 225, 81, 155, 55, 247, 31, 107, 154, 134, 69, 35, 200, 63, 22, 57, 47, 71, 152, 204, 46, 185, 25], [12, 120, 215, 213, 7, 95, 138, 237, 130, 141, 60, 72, 235, 217, 108, 15, 228, 194, 193, 101, 15, 186, 187, 33, 91, 131, 126, 82, 143, 135, 78, 225]], [[76, 223, 243, 17, 101, 19, 229, 215, 217, 6, 120, 246, 47, 14, 0, 38, 127, 1, 112, 8, 145, 97, 138, 176, 104, 128, 43, 252, 69, 244, 126, 103], [105, 176, 120, 47, 250, 10, 215, 47, 100, 131, 144, 178, 212, 173, 138, 189, 194, 191, 199, 8, 158, 1, 141, 29, 20, 82, 236, 72, 143, 252, 71, 241], [12, 120, 215, 213, 7, 95, 138, 237, 130, 141, 60, 72, 235, 217, 108, 15, 228, 194, 193, 101, 15, 186, 187, 33, 91, 131, 126, 82, 143, 135, 78, 225]], [[76, 223, 243, 17, 101, 19, 229, 215, 217, 6, 120, 246, 47, 14, 0, 38, 127, 1, 112, 8, 145, 97, 138, 176, 104, 128, 43, 252, 69, 244, 126, 103]]], operators_state_info_hash: [76, 223, 243, 17, 101, 19, 229, 215, 217, 6, 120, 246, 47, 14, 0, 38, 127, 1, 112, 8, 145, 97, 138, 176, 104, 128, 43, 252, 69, 244, 126, 103], block_hash: [105, 176, 120, 47, 250, 10, 215, 47, 100, 131, 144, 178, 212, 173, 138, 189, 194, 191, 199, 8, 158, 1, 141, 29, 20, 82, 236, 72, 143, 252, 71, 241], storage_proof_hash: [12, 120, 215, 213, 7, 95, 138, 237, 130, 141, 60, 72, 235, 217, 108, 15, 228, 194, 193, 101, 15, 186, 187, 33, 91, 131, 126, 82, 143, 135, 78, 225], pending_state_hash: [31, 188, 19, 31, 78, 175, 205, 220, 101, 13, 225, 81, 155, 55, 247, 31, 107, 154, 134, 69, 35, 200, 63, 22, 57, 47, 71, 152, 204, 46, 185, 25] }