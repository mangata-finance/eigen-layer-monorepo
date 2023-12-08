export const eigenContractAbi = [
  {
    inputs: [
      {
        internalType: 'contract IBLSRegistryCoordinatorWithIndices',
        name: '_registryCoordinator',
        type: 'address'
      },
      {
        internalType: 'uint32',
        name: '_taskResponseWindowBlock',
        type: 'uint32'
      }
    ],
    stateMutability: 'nonpayable',
    type: 'constructor'
  },
  {
    anonymous: false,
    inputs: [
      { indexed: false, internalType: 'uint8', name: 'version', type: 'uint8' }
    ],
    name: 'Initialized',
    type: 'event'
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: 'uint32',
        name: 'taskIndex',
        type: 'uint32'
      },
      {
        components: [
          { internalType: 'uint256', name: 'blockNumber', type: 'uint256' },
          { internalType: 'uint32', name: 'taskCreatedBlock', type: 'uint32' },
          { internalType: 'bytes', name: 'quorumNumbers', type: 'bytes' },
          {
            internalType: 'uint32',
            name: 'quorumThresholdPercentage',
            type: 'uint32'
          }
        ],
        indexed: false,
        internalType: 'struct IMangataTaskManager.Task',
        name: 'task',
        type: 'tuple'
      }
    ],
    name: 'NewTaskCreated',
    type: 'event'
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: 'address',
        name: 'previousOwner',
        type: 'address'
      },
      {
        indexed: true,
        internalType: 'address',
        name: 'newOwner',
        type: 'address'
      }
    ],
    name: 'OwnershipTransferred',
    type: 'event'
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: 'address',
        name: 'account',
        type: 'address'
      },
      {
        indexed: false,
        internalType: 'uint256',
        name: 'newPausedStatus',
        type: 'uint256'
      }
    ],
    name: 'Paused',
    type: 'event'
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: false,
        internalType: 'contract IPauserRegistry',
        name: 'pauserRegistry',
        type: 'address'
      },
      {
        indexed: false,
        internalType: 'contract IPauserRegistry',
        name: 'newPauserRegistry',
        type: 'address'
      }
    ],
    name: 'PauserRegistrySet',
    type: 'event'
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: 'uint32',
        name: 'taskIndex',
        type: 'uint32'
      },
      {
        indexed: true,
        internalType: 'address',
        name: 'challenger',
        type: 'address'
      }
    ],
    name: 'TaskChallengedSuccessfully',
    type: 'event'
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: 'uint32',
        name: 'taskIndex',
        type: 'uint32'
      },
      {
        indexed: true,
        internalType: 'address',
        name: 'challenger',
        type: 'address'
      }
    ],
    name: 'TaskChallengedUnsuccessfully',
    type: 'event'
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: 'uint32',
        name: 'taskIndex',
        type: 'uint32'
      }
    ],
    name: 'TaskCompleted',
    type: 'event'
  },
  {
    anonymous: false,
    inputs: [
      {
        components: [
          {
            internalType: 'uint32',
            name: 'referenceTaskIndex',
            type: 'uint32'
          },
          { internalType: 'bytes32', name: 'blockHash', type: 'bytes32' }
        ],
        indexed: false,
        internalType: 'struct IMangataTaskManager.TaskResponse',
        name: 'taskResponse',
        type: 'tuple'
      },
      {
        components: [
          {
            internalType: 'uint32',
            name: 'taskResponsedBlock',
            type: 'uint32'
          },
          { internalType: 'bytes32', name: 'hashOfNonSigners', type: 'bytes32' }
        ],
        indexed: false,
        internalType: 'struct IMangataTaskManager.TaskResponseMetadata',
        name: 'taskResponseMetadata',
        type: 'tuple'
      }
    ],
    name: 'TaskResponded',
    type: 'event'
  },
  {
    anonymous: false,
    inputs: [
      {
        indexed: true,
        internalType: 'address',
        name: 'account',
        type: 'address'
      },
      {
        indexed: false,
        internalType: 'uint256',
        name: 'newPausedStatus',
        type: 'uint256'
      }
    ],
    name: 'Unpaused',
    type: 'event'
  },
  {
    inputs: [],
    name: 'TASK_CHALLENGE_WINDOW_BLOCK',
    outputs: [{ internalType: 'uint32', name: '', type: 'uint32' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [],
    name: 'TASK_RESPONSE_WINDOW_BLOCK',
    outputs: [{ internalType: 'uint32', name: '', type: 'uint32' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [],
    name: 'aggregator',
    outputs: [{ internalType: 'address', name: '', type: 'address' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [{ internalType: 'uint32', name: '', type: 'uint32' }],
    name: 'allTaskHashes',
    outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [{ internalType: 'uint32', name: '', type: 'uint32' }],
    name: 'allTaskResponses',
    outputs: [{ internalType: 'bytes32', name: '', type: 'bytes32' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [],
    name: 'blsPubkeyRegistry',
    outputs: [
      { internalType: 'contract IBLSPubkeyRegistry', name: '', type: 'address' }
    ],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      { internalType: 'bytes32', name: 'msgHash', type: 'bytes32' },
      { internalType: 'bytes', name: 'quorumNumbers', type: 'bytes' },
      { internalType: 'uint32', name: 'referenceBlockNumber', type: 'uint32' },
      {
        components: [
          {
            internalType: 'uint32[]',
            name: 'nonSignerQuorumBitmapIndices',
            type: 'uint32[]'
          },
          {
            components: [
              { internalType: 'uint256', name: 'X', type: 'uint256' },
              { internalType: 'uint256', name: 'Y', type: 'uint256' }
            ],
            internalType: 'struct BN254.G1Point[]',
            name: 'nonSignerPubkeys',
            type: 'tuple[]'
          },
          {
            components: [
              { internalType: 'uint256', name: 'X', type: 'uint256' },
              { internalType: 'uint256', name: 'Y', type: 'uint256' }
            ],
            internalType: 'struct BN254.G1Point[]',
            name: 'quorumApks',
            type: 'tuple[]'
          },
          {
            components: [
              { internalType: 'uint256[2]', name: 'X', type: 'uint256[2]' },
              { internalType: 'uint256[2]', name: 'Y', type: 'uint256[2]' }
            ],
            internalType: 'struct BN254.G2Point',
            name: 'apkG2',
            type: 'tuple'
          },
          {
            components: [
              { internalType: 'uint256', name: 'X', type: 'uint256' },
              { internalType: 'uint256', name: 'Y', type: 'uint256' }
            ],
            internalType: 'struct BN254.G1Point',
            name: 'sigma',
            type: 'tuple'
          },
          {
            internalType: 'uint32[]',
            name: 'quorumApkIndices',
            type: 'uint32[]'
          },
          {
            internalType: 'uint32[]',
            name: 'totalStakeIndices',
            type: 'uint32[]'
          },
          {
            internalType: 'uint32[][]',
            name: 'nonSignerStakeIndices',
            type: 'uint32[][]'
          }
        ],
        internalType: 'struct IBLSSignatureChecker.NonSignerStakesAndSignature',
        name: 'nonSignerStakesAndSignature',
        type: 'tuple'
      }
    ],
    name: 'checkSignatures',
    outputs: [
      {
        components: [
          {
            internalType: 'uint96[]',
            name: 'signedStakeForQuorum',
            type: 'uint96[]'
          },
          {
            internalType: 'uint96[]',
            name: 'totalStakeForQuorum',
            type: 'uint96[]'
          }
        ],
        internalType: 'struct IBLSSignatureChecker.QuorumStakeTotals',
        name: '',
        type: 'tuple'
      },
      { internalType: 'bytes32', name: '', type: 'bytes32' }
    ],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      { internalType: 'uint256', name: 'blockNumber', type: 'uint256' },
      {
        internalType: 'uint32',
        name: 'quorumThresholdPercentage',
        type: 'uint32'
      },
      { internalType: 'bytes', name: 'quorumNumbers', type: 'bytes' }
    ],
    name: 'createNewTask',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function'
  },
  {
    inputs: [],
    name: 'generator',
    outputs: [{ internalType: 'address', name: '', type: 'address' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      {
        internalType: 'contract IBLSRegistryCoordinatorWithIndices',
        name: 'registryCoordinator',
        type: 'address'
      },
      { internalType: 'uint32', name: 'referenceBlockNumber', type: 'uint32' },
      { internalType: 'bytes', name: 'quorumNumbers', type: 'bytes' },
      {
        internalType: 'bytes32[]',
        name: 'nonSignerOperatorIds',
        type: 'bytes32[]'
      }
    ],
    name: 'getCheckSignaturesIndices',
    outputs: [
      {
        components: [
          {
            internalType: 'uint32[]',
            name: 'nonSignerQuorumBitmapIndices',
            type: 'uint32[]'
          },
          {
            internalType: 'uint32[]',
            name: 'quorumApkIndices',
            type: 'uint32[]'
          },
          {
            internalType: 'uint32[]',
            name: 'totalStakeIndices',
            type: 'uint32[]'
          },
          {
            internalType: 'uint32[][]',
            name: 'nonSignerStakeIndices',
            type: 'uint32[][]'
          }
        ],
        internalType: 'struct BLSOperatorStateRetriever.CheckSignaturesIndices',
        name: '',
        type: 'tuple'
      }
    ],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      {
        internalType: 'contract IBLSRegistryCoordinatorWithIndices',
        name: 'registryCoordinator',
        type: 'address'
      },
      { internalType: 'bytes', name: 'quorumNumbers', type: 'bytes' },
      { internalType: 'uint32', name: 'blockNumber', type: 'uint32' }
    ],
    name: 'getOperatorState',
    outputs: [
      {
        components: [
          { internalType: 'bytes32', name: 'operatorId', type: 'bytes32' },
          { internalType: 'uint96', name: 'stake', type: 'uint96' }
        ],
        internalType: 'struct BLSOperatorStateRetriever.Operator[][]',
        name: '',
        type: 'tuple[][]'
      }
    ],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      {
        internalType: 'contract IBLSRegistryCoordinatorWithIndices',
        name: 'registryCoordinator',
        type: 'address'
      },
      { internalType: 'bytes32', name: 'operatorId', type: 'bytes32' },
      { internalType: 'uint32', name: 'blockNumber', type: 'uint32' }
    ],
    name: 'getOperatorState',
    outputs: [
      { internalType: 'uint256', name: '', type: 'uint256' },
      {
        components: [
          { internalType: 'bytes32', name: 'operatorId', type: 'bytes32' },
          { internalType: 'uint96', name: 'stake', type: 'uint96' }
        ],
        internalType: 'struct BLSOperatorStateRetriever.Operator[][]',
        name: '',
        type: 'tuple[][]'
      }
    ],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [],
    name: 'getTaskResponseWindowBlock',
    outputs: [{ internalType: 'uint32', name: '', type: 'uint32' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      {
        internalType: 'contract IPauserRegistry',
        name: '_pauserRegistry',
        type: 'address'
      },
      { internalType: 'address', name: 'initialOwner', type: 'address' },
      { internalType: 'address', name: '_aggregator', type: 'address' },
      { internalType: 'address', name: '_generator', type: 'address' }
    ],
    name: 'initialize',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function'
  },
  {
    inputs: [],
    name: 'latestTaskNum',
    outputs: [{ internalType: 'uint32', name: '', type: 'uint32' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [],
    name: 'owner',
    outputs: [{ internalType: 'address', name: '', type: 'address' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      { internalType: 'uint256', name: 'newPausedStatus', type: 'uint256' }
    ],
    name: 'pause',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function'
  },
  {
    inputs: [],
    name: 'pauseAll',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function'
  },
  {
    inputs: [{ internalType: 'uint8', name: 'index', type: 'uint8' }],
    name: 'paused',
    outputs: [{ internalType: 'bool', name: '', type: 'bool' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [],
    name: 'paused',
    outputs: [{ internalType: 'uint256', name: '', type: 'uint256' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [],
    name: 'pauserRegistry',
    outputs: [
      { internalType: 'contract IPauserRegistry', name: '', type: 'address' }
    ],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      {
        components: [
          { internalType: 'uint256', name: 'blockNumber', type: 'uint256' },
          { internalType: 'uint32', name: 'taskCreatedBlock', type: 'uint32' },
          { internalType: 'bytes', name: 'quorumNumbers', type: 'bytes' },
          {
            internalType: 'uint32',
            name: 'quorumThresholdPercentage',
            type: 'uint32'
          }
        ],
        internalType: 'struct IMangataTaskManager.Task',
        name: 'task',
        type: 'tuple'
      },
      {
        components: [
          {
            internalType: 'uint32',
            name: 'referenceTaskIndex',
            type: 'uint32'
          },
          { internalType: 'bytes32', name: 'blockHash', type: 'bytes32' }
        ],
        internalType: 'struct IMangataTaskManager.TaskResponse',
        name: 'taskResponse',
        type: 'tuple'
      },
      {
        components: [
          {
            internalType: 'uint32',
            name: 'taskResponsedBlock',
            type: 'uint32'
          },
          { internalType: 'bytes32', name: 'hashOfNonSigners', type: 'bytes32' }
        ],
        internalType: 'struct IMangataTaskManager.TaskResponseMetadata',
        name: 'taskResponseMetadata',
        type: 'tuple'
      },
      {
        components: [
          { internalType: 'uint256', name: 'X', type: 'uint256' },
          { internalType: 'uint256', name: 'Y', type: 'uint256' }
        ],
        internalType: 'struct BN254.G1Point[]',
        name: 'pubkeysOfNonSigningOperators',
        type: 'tuple[]'
      }
    ],
    name: 'raiseAndResolveChallenge',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function'
  },
  {
    inputs: [],
    name: 'registryCoordinator',
    outputs: [
      {
        internalType: 'contract IRegistryCoordinator',
        name: '',
        type: 'address'
      }
    ],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [],
    name: 'renounceOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function'
  },
  {
    inputs: [
      {
        components: [
          { internalType: 'uint256', name: 'blockNumber', type: 'uint256' },
          { internalType: 'uint32', name: 'taskCreatedBlock', type: 'uint32' },
          { internalType: 'bytes', name: 'quorumNumbers', type: 'bytes' },
          {
            internalType: 'uint32',
            name: 'quorumThresholdPercentage',
            type: 'uint32'
          }
        ],
        internalType: 'struct IMangataTaskManager.Task',
        name: 'task',
        type: 'tuple'
      },
      {
        components: [
          {
            internalType: 'uint32',
            name: 'referenceTaskIndex',
            type: 'uint32'
          },
          { internalType: 'bytes32', name: 'blockHash', type: 'bytes32' }
        ],
        internalType: 'struct IMangataTaskManager.TaskResponse',
        name: 'taskResponse',
        type: 'tuple'
      },
      {
        components: [
          {
            internalType: 'uint32[]',
            name: 'nonSignerQuorumBitmapIndices',
            type: 'uint32[]'
          },
          {
            components: [
              { internalType: 'uint256', name: 'X', type: 'uint256' },
              { internalType: 'uint256', name: 'Y', type: 'uint256' }
            ],
            internalType: 'struct BN254.G1Point[]',
            name: 'nonSignerPubkeys',
            type: 'tuple[]'
          },
          {
            components: [
              { internalType: 'uint256', name: 'X', type: 'uint256' },
              { internalType: 'uint256', name: 'Y', type: 'uint256' }
            ],
            internalType: 'struct BN254.G1Point[]',
            name: 'quorumApks',
            type: 'tuple[]'
          },
          {
            components: [
              { internalType: 'uint256[2]', name: 'X', type: 'uint256[2]' },
              { internalType: 'uint256[2]', name: 'Y', type: 'uint256[2]' }
            ],
            internalType: 'struct BN254.G2Point',
            name: 'apkG2',
            type: 'tuple'
          },
          {
            components: [
              { internalType: 'uint256', name: 'X', type: 'uint256' },
              { internalType: 'uint256', name: 'Y', type: 'uint256' }
            ],
            internalType: 'struct BN254.G1Point',
            name: 'sigma',
            type: 'tuple'
          },
          {
            internalType: 'uint32[]',
            name: 'quorumApkIndices',
            type: 'uint32[]'
          },
          {
            internalType: 'uint32[]',
            name: 'totalStakeIndices',
            type: 'uint32[]'
          },
          {
            internalType: 'uint32[][]',
            name: 'nonSignerStakeIndices',
            type: 'uint32[][]'
          }
        ],
        internalType: 'struct IBLSSignatureChecker.NonSignerStakesAndSignature',
        name: 'nonSignerStakesAndSignature',
        type: 'tuple'
      }
    ],
    name: 'respondToTask',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function'
  },
  {
    inputs: [
      {
        internalType: 'contract IPauserRegistry',
        name: 'newPauserRegistry',
        type: 'address'
      }
    ],
    name: 'setPauserRegistry',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function'
  },
  {
    inputs: [],
    name: 'stakeRegistry',
    outputs: [
      { internalType: 'contract IStakeRegistry', name: '', type: 'address' }
    ],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [],
    name: 'taskNumber',
    outputs: [{ internalType: 'uint32', name: '', type: 'uint32' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [{ internalType: 'uint32', name: '', type: 'uint32' }],
    name: 'taskSuccesfullyChallenged',
    outputs: [{ internalType: 'bool', name: '', type: 'bool' }],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [{ internalType: 'address', name: 'newOwner', type: 'address' }],
    name: 'transferOwnership',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function'
  },
  {
    inputs: [
      { internalType: 'bytes32', name: 'msgHash', type: 'bytes32' },
      {
        components: [
          { internalType: 'uint256', name: 'X', type: 'uint256' },
          { internalType: 'uint256', name: 'Y', type: 'uint256' }
        ],
        internalType: 'struct BN254.G1Point',
        name: 'apk',
        type: 'tuple'
      },
      {
        components: [
          { internalType: 'uint256[2]', name: 'X', type: 'uint256[2]' },
          { internalType: 'uint256[2]', name: 'Y', type: 'uint256[2]' }
        ],
        internalType: 'struct BN254.G2Point',
        name: 'apkG2',
        type: 'tuple'
      },
      {
        components: [
          { internalType: 'uint256', name: 'X', type: 'uint256' },
          { internalType: 'uint256', name: 'Y', type: 'uint256' }
        ],
        internalType: 'struct BN254.G1Point',
        name: 'sigma',
        type: 'tuple'
      }
    ],
    name: 'trySignatureAndApkVerification',
    outputs: [
      { internalType: 'bool', name: 'pairingSuccessful', type: 'bool' },
      { internalType: 'bool', name: 'siganatureIsValid', type: 'bool' }
    ],
    stateMutability: 'view',
    type: 'function'
  },
  {
    inputs: [
      { internalType: 'uint256', name: 'newPausedStatus', type: 'uint256' }
    ],
    name: 'unpause',
    outputs: [],
    stateMutability: 'nonpayable',
    type: 'function'
  }
] as const;
