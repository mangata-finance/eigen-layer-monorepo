use crate::{
    cli::CliArgs,
    crypto::bn254::{BlsKeypair, BlsSignature, PrivateKey},
};
use ark_ec::AffineRepr;
use ark_ff::PrimeField;
use bindings::shared_types::TaskResponse;
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
struct SignedTaskResponse {
    #[serde(rename = "TaskResponse")]
    task_response: Bytes,
    #[serde(rename = "TaskResponseWire")]
    task_response_wire: TaskResponseWire,
    #[serde(rename = "BlsSignature")]
    bls_signature: BlsSignatureWire,
    #[serde(rename = "OperatorId")]
    operator_id: Bytes32,
}

#[derive(Serialize, Debug)]
struct TaskResponseWire {
    #[serde(rename = "ReferenceTaskIndex")]
    pub reference_task_index: u32,
    #[serde(rename = "BlockHash")]
    pub block_hash: Bytes32,
    #[serde(rename = "StorageProofHash")]
    pub storage_proof_hash: Bytes32,
    #[serde(rename = "PendingStateHash")]
    pub pending_state_hash: Bytes32,
}

impl From<TaskResponse> for TaskResponseWire {
    fn from(value: TaskResponse) -> Self {
        Self {
            reference_task_index: value.reference_task_index,
            block_hash: value.block_hash,
            storage_proof_hash: value.storage_proof_hash,
            pending_state_hash: value.pending_state_hash,
        }
    }
}

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
        println!("task_response - {:?}", task_response);
        let req = create_response(task_response, keypair)?;
        println!("req - {:?}", req);
        let json: String = serde_json::to_string(&req)?;
        println!("json - {:?}", json);

        Ok(self.client.post(&self.avs_url).body(json).send().await?)
    }
}

fn create_response(task: TaskResponse, keypair: &BlsKeypair) -> eyre::Result<SignedTaskResponse> {
    let encoded = task.clone().encode();
    println!("encoded - {}", array_bytes::bytes2hex("0x", encoded.clone()));

    let hash = Keccak256::hash(encoded.as_ref());
    let sig = keypair.sign(hash.as_bytes())?;

    Ok(SignedTaskResponse {
        bls_signature: sig.into(),
        task_response: task.encode().into(),
        task_response_wire: task.into(),
        operator_id: keypair.operator_id().to_fixed_bytes(),
    })
}

// task_response - TaskResponse { reference_task_index: 6, block_hash: [244, 215, 139, 116, 178, 19, 164, 142, 52, 245, 160, 172, 88, 74, 164, 168, 88, 35, 124, 37, 252, 19, 177, 65, 123, 85, 26, 175, 227, 132, 242, 122], storage_proof_hash: [35, 107, 27, 141, 37, 99, 136, 166, 162, 35, 118, 43, 154, 139, 227, 199, 50, 210, 214, 159, 247, 240, 213, 143, 150, 167, 36, 190, 221, 64, 130, 228], pending_state_hash: [31, 188, 19, 31, 78, 175, 205, 220, 101, 13, 225, 81, 155, 55, 247, 31, 107, 154, 134, 69, 35, 200, 63, 22, 57, 47, 71, 152, 204, 46, 185, 25] }
// avs-finalizer-1  | encoded - 0x0000000000000000000000000000000000000000000000000000000000000006f4d78b74b213a48e34f5a0ac584aa4a858237c25fc13b1417b551aafe384f27a236b1b8d256388a6a223762b9a8be3c732d2d69ff7f0d58f96a724bedd4082e41fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919
// avs-finalizer-1  | req - SignedTaskResponse { task_response: Bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 244, 215, 139, 116, 178, 19, 164, 142, 52, 245, 160, 172, 88, 74, 164, 168, 88, 35, 124, 37, 252, 19, 177, 65, 123, 85, 26, 175, 227, 132, 242, 122, 35, 107, 27, 141, 37, 99, 136, 166, 162, 35, 118, 43, 154, 139, 227, 199, 50, 210, 214, 159, 247, 240, 213, 143, 150, 167, 36, 190, 221, 64, 130, 228, 31, 188, 19, 31, 78, 175, 205, 220, 101, 13, 225, 81, 155, 55, 247, 31, 107, 154, 134, 69, 35, 200, 63, 22, 57, 47, 71, 152, 204, 46, 185, 25]), bls_signature: BlsSignatureWire { g1_point: G1PointWire { x: BigInt([6414388623002460198, 3935234236590813146, 17272744584949058105, 1448781156749743865]), y: BigInt([318514756468758091, 18182375163070126606, 1342887054825783377, 2130240232483275987]) } }, operator_id: [9, 81, 196, 0, 159, 107, 77, 79, 159, 206, 241, 52, 130, 169, 21, 174, 17, 181, 110, 173, 14, 77, 52, 216, 113, 192, 146, 131, 57, 135, 245, 24] }
// avs-finalizer-1  | json - "{\"TaskResponse\":\"0x0000000000000000000000000000000000000000000000000000000000000006f4d78b74b213a48e34f5a0ac584aa4a858237c25fc13b1417b551aafe384f27a236b1b8d256388a6a223762b9a8be3c732d2d69ff7f0d58f96a724bedd4082e41fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919\",\"BlsSignature\":{\"g1_point\":{\"X\":\"9094146713229339985997605710714061505236762486261629008303477396915210838054\",\"Y\":\"13371734660111297977188861029117306156381884932095904435622397031304345048651\"}},\"OperatorId\":[9,81,196,0,159,107,77,79,159,206,241,52,130,169,21,174,17,181,110,173,14,77,52,216,113,192,146,131,57,135,245,24]}"

// [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 136 243 234 179 129 49 43 143 172 237 105 52 203 46 12 44 181 223 205 216 183 37 3 170 70 91 213 26 79 55 83 53 170 180 50 156 15 105 15 126 233 173 33 95 186 31 232 6 43 103 217 182 242 204 52 110 99 148 180 145 142 24 53 180 31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]

// [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,136,243,234,179,129,49,43,143,172,237,105,52,203,46,12,44,181,223,205,216,183,37,3,170,70,91,213,26,79,55,83,53,170,180,50,156,15,105,15,126,233,173,33,95,186,31,232,6,43,103,217,182,242,204,52,110,99,148,180,145,142,24,53,180,31,188,19,31,78,175,205,220,101,13,225,81,155,55,247,31,107,154,134,69,35,200,63,22,57,47,71,152,204,46,185,25]
// 0x000000000000000000000000000000000000000000000000000000000000000188f3eab381312b8faced6934cb2e0c2cb5dfcdd8b72503aa465bd51a4f375335aab4329c0f690f7ee9ad215fba1fe8062b67d9b6f2cc346e6394b4918e1835b41fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919