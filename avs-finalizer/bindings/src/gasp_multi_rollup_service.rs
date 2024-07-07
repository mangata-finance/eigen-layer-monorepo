pub use gasp_multi_rollup_service::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod gasp_multi_rollup_service {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("checkSignatures"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("checkSignatures"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("params"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                        2usize,
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                        2usize,
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSSignatureChecker.QuorumStakeTotals",
                                        ),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("initialize"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("initialize"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_pauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("initialOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_updater"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rolldown"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IRolldown"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("lastUpdateBlockTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastUpdateBlockTimestamp",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestCompletedTaskCreatedBlock"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestCompletedTaskCreatedBlock",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestCompletedTaskNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestCompletedTaskNumber",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("latestPendingStateHash"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestPendingStateHash",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorAndQuorumToStakes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "operatorAndQuorumToStakes",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("operatorIdQuorumCount"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "operatorIdQuorumCount",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("owner"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("owner"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pause"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pauseAll"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauseAll"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("index"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("paused"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("process_eigen_reinit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "process_eigen_reinit",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("task"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.Task",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("taskResponse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.TaskResponse",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorStateInfo"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                ),
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                                ),
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                ),
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                                ),
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IGaspMultiRollupServicePrimitives.OperatorStateInfo",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("process_eigen_update"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "process_eigen_update",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("task"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.Task",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("taskResponse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.TaskResponse",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "nonSignerStakesAndSignatureForOldState",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                        2usize,
                                                    ),
                                                    ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                        ),
                                                        2usize,
                                                    ),
                                                ],
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("operatorStateInfo"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Bool,
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                                ::std::vec![
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                                ],
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                ),
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                                ),
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                                ),
                                                            ),
                                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                                ::std::boxed::Box::new(
                                                                    ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                                                ),
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                        ::std::vec![
                                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                                        ],
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IGaspMultiRollupServicePrimitives.OperatorStateInfo",
                                        ),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("qourumApk"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("qourumApk"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("X"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("Y"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumNumbers"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bytes,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quorumThresholdPercentage"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "quorumThresholdPercentage",
                            ),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("quorumToStakes"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("quorumToStakes"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint8"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(96usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint96"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("renounceOwnership"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("rolldown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("rolldown"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IRolldown"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setPauserRegistry"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IPauserRegistry"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("set_rolldown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("set_rolldown"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rolldown"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IRolldown"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("set_updater"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("set_updater"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_updater"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("stalled"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("stalled"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("transferOwnership"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("transferOwnership"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("trySignatureAndApkVerification"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "trySignatureAndApkVerification",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("msgHash"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::FixedBytes(
                                        32usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("apk"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("apkG2"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                2usize,
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::FixedArray(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ),
                                                2usize,
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G2Point"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("sigma"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("struct BN254.G1Point"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("pairingSuccessful"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("siganatureIsValid"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("unpause"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("unpause"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint256"),
                                    ),
                                },
                            ],
                            outputs: ::std::vec![],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::NonPayable,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("updater"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("updater"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("address"),
                                    ),
                                },
                            ],
                            constant: ::core::option::Option::None,
                            state_mutability: ::ethers::core::abi::ethabi::StateMutability::View,
                        },
                    ],
                ),
            ]),
            events: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("EigenReinitProcessed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EigenReinitProcessed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskCreatedBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("EigenUpdateProcessed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EigenUpdateProcessed",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskNumber"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("taskCreatedBlock"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Initialized"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Initialized"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("version"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("OwnershipTransferred"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "OwnershipTransferred",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("previousOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newOwner"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Paused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Paused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("PauserRegistrySet"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("PauserRegistrySet"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("pauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPauserRegistry"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
                (
                    ::std::borrow::ToOwned::to_owned("Unpaused"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned("Unpaused"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("account"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    indexed: true,
                                },
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("newPausedStatus"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(
                                        256usize,
                                    ),
                                    indexed: false,
                                },
                            ],
                            anonymous: false,
                        },
                    ],
                ),
            ]),
            errors: ::std::collections::BTreeMap::new(),
            receive: false,
            fallback: false,
        }
    }
    ///The parsed JSON ABI of the contract.
    pub static GASPMULTIROLLUPSERVICE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaJj\x80a\0 `\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xDAW`\x005`\xE0\x1C\x80c\\\x97Z\xBB\x11a\x01\x04W\x80c\xDF\x03L\xD0\x11a\0\xA2W\x80c\xF2\xFD\xE3\x8B\x11a\0qW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xBBW\x80c\xF8\xC8v^\x14a\x04\xCEW\x80c\xFA\xBC\x1C\xBC\x14a\x04\xE1W\x80c\xFCv]\xD5\x14a\x04\xF4W`\0\x80\xFD[\x80c\xDF\x03L\xD0\x14a\x04tW\x80c\xE6\x1D\xB1u\x14a\x04\x87W\x80c\xEDZ\x04\xFE\x14a\x04\x90W\x80c\xF1\xE9\x8A\\\x14a\x04\xA8W`\0\x80\xFD[\x80c\x81@\xDF\xD3\x11a\0\xDEW\x80c\x81@\xDF\xD3\x14a\x04\x1DW\x80c\x88o\x11\x95\x14a\x040W\x80c\x8D\xA5\xCB[\x14a\x04CW\x80c\xC4\xE1\x91L\x14a\x04TW`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x03\xE4W\x80cqP\x18\xA6\x14a\x03\xECW\x80cz\xD7Ua\x14a\x03\xF4W`\0\x80\xFD[\x80c=\x9F\xB0\x0C\x11a\x01|W\x80cM\xEA\xBC!\x11a\x01KW\x80cM\xEA\xBC!\x14a\x03pW\x80cRn>d\x14a\x03\x95W\x80cY\\jg\x14a\x03\xB9W\x80cZ\xC8j\xB7\x14a\x03\xC1W`\0\x80\xFD[\x80c=\x9F\xB0\x0C\x14a\x02\xADW\x80cC\r;9\x14a\x02\xD8W\x80cI\x9Do\xB6\x14a\x03\rW\x80cJ\xE6\xB2\x03\x14a\x03YW`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x01\xB8W\x80c\x13d9\xDD\x14a\x02HW\x80c\x17\x1F\x1D[\x14a\x02[W\x80c\x1E-K\xF7\x14a\x02\x85W\x80c*\x84\x14\xFD\x14a\x02\x98W`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x01\xDFW\x80c\x10\xD6z/\x14a\x02 W\x80c\x12FH\xC9\x14a\x025W[`\0\x80\xFD[a\x02\x06a\x01\xED6`\x04a7\xAFV[`\xA0` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x023a\x02.6`\x04a7\xE6V[a\x05\x04V[\0[a\x023a\x02C6`\x04a7\xE6V[a\x05\xC0V[a\x023a\x02V6`\x04a8\x03V[a\x05\xEAV[a\x02na\x02i6`\x04a9\x80V[a\x07)V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02\x17V[a\x023a\x02\x936`\x04a7\xE6V[a\x08\xB3V[a\x02\xA0a\x08\xDDV[`@Qa\x02\x17\x91\x90a9\xD1V[`\x98Ta\x02\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x02\xFBa\x02\xE66`\x04a8\x03V[`\xA1` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03Aa\x03\x1B6`\x04a:&V[`\x9F` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03b`\x9AT\x81V[`@Q\x90\x81R` \x01a\x02\x17V[`\x9DTa\x03\x80\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[`\x98Ta\x03\xA9\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x17V[a\x023a\tkV[a\x03\xA9a\x03\xCF6`\x04a7\xAFV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03bV[a\x023a\n2V[a\x03Aa\x04\x026`\x04a7\xAFV[`\x9E` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x023a\x04+6`\x04a:\x90V[a\nFV[`eTa\x02\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xC0V[a\x04ga\x04b6`\x04a;\xF7V[a\x15\nV[`@Qa\x02\x17\x91\x90a<\x81V[`\x97Ta\x02\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03b`\x99T\x81V[`\x9BTa\x03\x80\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x023a\x04\xB66`\x04a<\xC3V[a\x1A\xF7V[a\x023a\x04\xC96`\x04a7\xE6V[a)3V[a\x023a\x04\xDC6`\x04a=oV[a)\xA9V[a\x023a\x04\xEF6`\x04a8\x03V[a*\xFAV[`\x9BTa\x03\x80\x90c\xFF\xFF\xFF\xFF\x16\x81V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05{\x91\x90a=\xCBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a=\xE8V[`@Q\x80\x91\x03\x90\xFD[a\x05\xBD\x81a,VV[PV[a\x05\xC8a-MV[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x062W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06V\x91\x90a>@V[a\x06rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a>]V[`fT\x81\x81\x16\x14a\x06\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07qWa\x07qa>\xA5V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x07\x96Wa\x07\x96a>\xA5V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x07\xB2Wa\x07\xB2a>\xA5V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08\x0F\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x082\x91\x90a>\xBBV[\x90Pa\x08\xA5a\x08Ka\x08D\x88\x84a-\xA7V[\x86\x90a.>V[a\x08Sa.\xD2V[a\x08\x9Ba\x08\x8C\x85a\x08\x86`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a-\xA7V[a\x08\x95\x8Ca/\x92V[\x90a.>V[\x88b\x01\xD4\xC0a0\"V[\x90\x98\x90\x97P\x95PPPPPPV[a\x08\xBBa-MV[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9C\x80Ta\x08\xEA\x90a>\xDDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\x16\x90a>\xDDV[\x80\x15a\tcW\x80`\x1F\x10a\t8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\tcV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xD7\x91\x90a>@V[a\t\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a>]V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\n:a-MV[a\nD`\0a2FV[V[a\nNa-MV[`\0[a\n^` \x83\x01\x83a?\x12V[\x90P\x81\x10\x15a\x0B#W`\x9E`\0a\nx` \x85\x01\x85a?\x12V[\x84\x81\x81\x10a\n\x88Wa\n\x88a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\n\x9D\x91\x90a7\xAFV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\xA0\x91a\n\xD0\x90\x85\x01\x85a?\x12V[\x84\x81\x81\x10a\n\xE0Wa\n\xE0a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\n\xF5\x91\x90a7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\x0B\x1B\x81a?xV[\x91PPa\nQV[P`\0[a\x0B4`@\x83\x01\x83a?\x93V[\x90P\x81\x10\x15a\x0CoWa\x0BJ`@\x83\x01\x83a?\x93V[\x82\x81\x81\x10a\x0BZWa\x0BZa>\xA5V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x0Br\x91\x90a?\xF3V[`\x9E`\0a\x0B\x83`@\x86\x01\x86a?\x93V[\x85\x81\x81\x10a\x0B\x93Wa\x0B\x93a>\xA5V[a\x0B\xA9\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x0B\xE9\x90\x83\x01\x83a?\x93V[\x82\x81\x81\x10a\x0B\xF9Wa\x0B\xF9a>\xA5V[\x90P`\x80\x02\x01`@\x01`\xA0`\0\x84\x80`@\x01\x90a\x0C\x16\x91\x90a?\x93V[\x85\x81\x81\x10a\x0C&Wa\x0C&a>\xA5V[a\x0C<\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x0Cg\x81a?xV[\x91PPa\x0B'V[P`\0[a\x0C\x80``\x83\x01\x83a@\x0EV[\x90P\x81\x10\x15a\r8Wa\x0C\x96``\x83\x01\x83a@\x0EV[\x82\x81\x81\x10a\x0C\xA6Wa\x0C\xA6a>\xA5V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x0C\xBE\x91\x90a?\xF3V[`\x9E`\0a\x0C\xCF``\x86\x01\x86a@\x0EV[\x85\x81\x81\x10a\x0C\xDFWa\x0C\xDFa>\xA5V[a\x0C\xF5\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\r0\x81a?xV[\x91PPa\x0CsV[P`\0[a\rI`\x80\x83\x01\x83a@WV[\x90P\x81\x10\x15a\r\xE5Wa\r_`\x80\x83\x01\x83a@WV[\x82\x81\x81\x10a\roWa\roa>\xA5V[\x90P``\x02\x01` \x01`\xA0`\0\x84\x80`\x80\x01\x90a\r\x8C\x91\x90a@WV[\x85\x81\x81\x10a\r\x9CWa\r\x9Ca>\xA5V[a\r\xB2\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\r\xDD\x81a?xV[\x91PPa\r<V[P`\0[a\r\xF6`\xA0\x83\x01\x83a?\x12V[\x90P\x81\x10\x15a\x0F\x1DW`\0[`\x9C\x80Ta\x0E\x0F\x90a>\xDDV[\x90P\x81\x10\x15a\x0E\xC8W`\x9F`\0a\x0E)`\xA0\x86\x01\x86a?\x12V[\x85\x81\x81\x10a\x0E9Wa\x0E9a>\xA5V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9C\x83\x81Ta\x0E^\x90a>\xDDV[\x81\x10a\x0ElWa\x0Ela>\xA5V[\x81T`\x01\x16\x15a\x0E\x8BW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x0E\xC0\x81a?xV[\x92PPa\x0E\x02V[P`\xA1`\0a\x0E\xDA`\xA0\x85\x01\x85a?\x12V[\x84\x81\x81\x10a\x0E\xEAWa\x0E\xEAa>\xA5V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x0F\x15\x81a?xV[\x91PPa\r\xE9V[P`\0[a\x0F.`\xC0\x83\x01\x83a?\x12V[\x90P\x81\x10\x15a\x11~Wa\x0FD`\xC0\x83\x01\x83a?\x12V[\x82\x81\x81\x10a\x0FTWa\x0FTa>\xA5V[\x90P` \x02\x81\x01\x90a\x0Ff\x91\x90a@\x9FV[a\x0Fw\x90`\x80\x81\x01\x90``\x01a7\xAFV[`\xA1`\0a\x0F\x88`\xC0\x86\x01\x86a?\x12V[\x85\x81\x81\x10a\x0F\x98Wa\x0F\x98a>\xA5V[\x90P` \x02\x81\x01\x90a\x0F\xAA\x91\x90a@\x9FV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x0F\xE4`\xC0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a\x0F\xF4Wa\x0F\xF4a>\xA5V[\x90P` \x02\x81\x01\x90a\x10\x06\x91\x90a@\x9FV[a\x10\x14\x90` \x81\x01\x90a?\x12V[\x90P\x81\x10\x15a\x11kWa\x10*`\xC0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a\x10:Wa\x10:a>\xA5V[\x90P` \x02\x81\x01\x90a\x10L\x91\x90a@\x9FV[a\x10Z\x90`@\x81\x01\x90a?\x12V[\x82\x81\x81\x10a\x10jWa\x10ja>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\x10\x7F\x91\x90a?\xF3V[`\x9F`\0a\x10\x90`\xC0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a\x10\xA0Wa\x10\xA0a>\xA5V[\x90P` \x02\x81\x01\x90a\x10\xB2\x91\x90a@\x9FV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x10\xD3`\xC0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a\x10\xE3Wa\x10\xE3a>\xA5V[\x90P` \x02\x81\x01\x90a\x10\xF5\x91\x90a@\x9FV[a\x11\x03\x90` \x81\x01\x90a?\x12V[\x85\x81\x81\x10a\x11\x13Wa\x11\x13a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\x11(\x91\x90a7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x11c\x81a?xV[\x91PPa\x0F\xD7V[P\x80a\x11v\x81a?xV[\x91PPa\x0F!V[P`\0[a\x11\x8F`\xE0\x83\x01\x83a?\x12V[\x90P\x81\x10\x15a\x13BW`\0[a\x11\xA8`\xE0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a\x11\xB8Wa\x11\xB8a>\xA5V[\x90P` \x02\x81\x01\x90a\x11\xCA\x91\x90a@\xBFV[a\x11\xD8\x90` \x81\x01\x90a?\x12V[\x90P\x81\x10\x15a\x13/Wa\x11\xEE`\xE0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a\x11\xFEWa\x11\xFEa>\xA5V[\x90P` \x02\x81\x01\x90a\x12\x10\x91\x90a@\xBFV[a\x12\x1E\x90`@\x81\x01\x90a?\x12V[\x82\x81\x81\x10a\x12.Wa\x12.a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\x12C\x91\x90a?\xF3V[`\x9F`\0a\x12T`\xE0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a\x12dWa\x12da>\xA5V[\x90P` \x02\x81\x01\x90a\x12v\x91\x90a@\xBFV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x12\x97`\xE0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a\x12\xA7Wa\x12\xA7a>\xA5V[\x90P` \x02\x81\x01\x90a\x12\xB9\x91\x90a@\xBFV[a\x12\xC7\x90` \x81\x01\x90a?\x12V[\x85\x81\x81\x10a\x12\xD7Wa\x12\xD7a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\x12\xEC\x91\x90a7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x13'\x81a?xV[\x91PPa\x11\x9BV[P\x80a\x13:\x81a?xV[\x91PPa\x11\x82V[P`\0[a\x13Ta\x01\0\x83\x01\x83a@\x0EV[\x90P\x81\x10\x15a\x13\xF8Wa\x13ka\x01\0\x83\x01\x83a@\x0EV[\x82\x81\x81\x10a\x13{Wa\x13{a>\xA5V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x13\x93\x91\x90a7\xAFV[`\xA1`\0a\x13\xA5a\x01\0\x86\x01\x86a@\x0EV[\x85\x81\x81\x10a\x13\xB5Wa\x13\xB5a>\xA5V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x13\xF0\x90a?xV[\x91PPa\x13FV[P`\xC0\x82\x015`\x9AUa\x14\x0E` \x84\x01\x84a@\xE9V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x148``\x84\x01`@\x85\x01a@\xE9V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\x14p`\x80\x84\x01\x84aA\x04V[a\x14|\x91`\x9C\x91a6+V[Pa\x14\x8D`\xC0\x84\x01`\xA0\x85\x01a@\xE9V[`\x9D\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa\x14\xD5` \x85\x01\x85a@\xE9V[a\x14\xE5``\x86\x01`@\x87\x01a@\xE9V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9C\x80Ta\x15A\x90a>\xDDV[\x90P\x90Pa\x15b`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15zWa\x15za8\x1CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xA3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xC1Wa\x15\xC1a8\x1CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xEAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R\x84QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\nWa\x16\na8\x1CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x163W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87QQ\x81\x10\x15a\x17\xEEWa\x16|\x88`\0\x01Q\x82\x81Q\x81\x10a\x16]Wa\x16]a>\xA5V[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x16\x8EWa\x16\x8Ea>\xA5V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x17lW\x82a\x16\xAB`\x01\x83aAJV[\x81Q\x81\x10a\x16\xBBWa\x16\xBBa>\xA5V[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x16\xD8Wa\x16\xD8a>\xA5V[` \x02` \x01\x01Q`\0\x1C\x11a\x17lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerG1PubkeysForOldSta`d\x82\x01Rl\x1D\x19H\x1B\x9B\xDD\x08\x1C\xDB\xDC\x9D\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[a\x17\xDAa\x17\xD3`\xA1`\0\x86\x85\x81Q\x81\x10a\x17\x88Wa\x17\x88a>\xA5V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T\x8AQ\x80Q`\xFF\x90\x92\x16\x91\x85\x90\x81\x10a\x17\xBDWa\x17\xBDa>\xA5V[` \x02` \x01\x01Qa2\x98\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a.>V[\x95P\x80a\x17\xE6\x81a?xV[\x91PPa\x16:V[Pa\x17\xF8\x85a3|V[\x94P`\0[\x84\x81\x10\x15a\x19\xD9W`\x9C\x81\x81Ta\x18\x13\x90a>\xDDV[\x81\x10a\x18!Wa\x18!a>\xA5V[\x81T`\x01\x16\x15a\x18@W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\xA0` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x18\x81\x90\x87\x90a.>V[`\xFF\x83\x16`\0\x90\x81R`\x9E` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x18\xBCWa\x18\xBCa>\xA5V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x18\xE8Wa\x18\xE8a>\xA5V[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x19\x06Wa\x19\x06a>\xA5V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88QQ\x81\x10\x15a\x19\xC6W`\x9F`\0\x85\x83\x81Q\x81\x10a\x19IWa\x19Ia>\xA5V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x19\x94Wa\x19\x94a>\xA5V[` \x02` \x01\x01\x81\x81Qa\x19\xA8\x91\x90aAaV[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x19\xBE\x81a?xV[\x91PPa\x19)V[P\x80a\x19\xD1\x81a?xV[\x91PPa\x17\xFDV[P`\0\x80a\x19\xF1\x8A\x88\x8B` \x01Q\x8C`@\x01Qa\x07)V[\x91P\x91P\x81a\x1AtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[\x80a\x1A\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[P\x92\x95PPPPPP[\x92\x91PPV[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BQW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[a\x1Ba`\x80\x85\x01``\x86\x01a@\xE9V[`\x9BTd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x1B\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x05\xABV[\x83`@Q` \x01a\x1B\xD4\x91\x90aA\xF7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83` \x015\x14a\x1C<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[\x80`@Q` \x01a\x1CM\x91\x90aGLV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83``\x015\x14a\x1C\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[`\x9BTd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a\x1EuWa\x1C\xDB``\x85\x01`@\x86\x01a@\xE9V[`\x9BTc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a\x1C\xFC\x91d\x01\0\0\0\0\x90\x04\x16a8@aH\xB3V[c\xFF\xFF\xFF\xFF\x16\x11a\x1D?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05\xABV[B`\x99Tb\x03\xF4\x80a\x1DQ\x91\x90aH\xDBV[\x11a\x1D\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05\xABV[`\0a\x1D\xC4\x84`@Q` \x01a\x1D\xA4\x91\x90aH\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84a\x04b\x90aI\xBAV[`\x9DT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9C\x80Ta\x1D\xE0\x90a>\xDDV[\x90P\x81\x10\x15a\x1EqW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x1E\x03Wa\x1E\x03a>\xA5V[` \x02` \x01\x01Qa\x1E\x15\x91\x90aI\xC6V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x1E6Wa\x1E6a>\xA5V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x1EQ\x91\x90aI\xF5V[\x10\x15a\x1E_WPPPa)-V[\x80a\x1Ei\x81a?xV[\x91PPa\x1D\xD3V[PPP[`\0[a\x1E\x85` \x83\x01\x83a?\x12V[\x90P\x81\x10\x15a\x1FJW`\x9E`\0a\x1E\x9F` \x85\x01\x85a?\x12V[\x84\x81\x81\x10a\x1E\xAFWa\x1E\xAFa>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\x1E\xC4\x91\x90a7\xAFV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\xA0\x91a\x1E\xF7\x90\x85\x01\x85a?\x12V[\x84\x81\x81\x10a\x1F\x07Wa\x1F\x07a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\x1F\x1C\x91\x90a7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\x1FB\x81a?xV[\x91PPa\x1ExV[P`\0[a\x1F[`@\x83\x01\x83a?\x93V[\x90P\x81\x10\x15a \x96Wa\x1Fq`@\x83\x01\x83a?\x93V[\x82\x81\x81\x10a\x1F\x81Wa\x1F\x81a>\xA5V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x1F\x99\x91\x90a?\xF3V[`\x9E`\0a\x1F\xAA`@\x86\x01\x86a?\x93V[\x85\x81\x81\x10a\x1F\xBAWa\x1F\xBAa>\xA5V[a\x1F\xD0\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua \x10\x90\x83\x01\x83a?\x93V[\x82\x81\x81\x10a  Wa  a>\xA5V[\x90P`\x80\x02\x01`@\x01`\xA0`\0\x84\x80`@\x01\x90a =\x91\x90a?\x93V[\x85\x81\x81\x10a MWa Ma>\xA5V[a c\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a \x8E\x81a?xV[\x91PPa\x1FNV[P`\0[a \xA7``\x83\x01\x83a@\x0EV[\x90P\x81\x10\x15a!_Wa \xBD``\x83\x01\x83a@\x0EV[\x82\x81\x81\x10a \xCDWa \xCDa>\xA5V[\x90P`@\x02\x01` \x01` \x81\x01\x90a \xE5\x91\x90a?\xF3V[`\x9E`\0a \xF6``\x86\x01\x86a@\x0EV[\x85\x81\x81\x10a!\x06Wa!\x06a>\xA5V[a!\x1C\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a!W\x81a?xV[\x91PPa \x9AV[P`\0[a!p`\x80\x83\x01\x83a@WV[\x90P\x81\x10\x15a\"\x0CWa!\x86`\x80\x83\x01\x83a@WV[\x82\x81\x81\x10a!\x96Wa!\x96a>\xA5V[\x90P``\x02\x01` \x01`\xA0`\0\x84\x80`\x80\x01\x90a!\xB3\x91\x90a@WV[\x85\x81\x81\x10a!\xC3Wa!\xC3a>\xA5V[a!\xD9\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\"\x04\x81a?xV[\x91PPa!cV[P`\0[a\"\x1D`\xA0\x83\x01\x83a?\x12V[\x90P\x81\x10\x15a#DW`\0[`\x9C\x80Ta\"6\x90a>\xDDV[\x90P\x81\x10\x15a\"\xEFW`\x9F`\0a\"P`\xA0\x86\x01\x86a?\x12V[\x85\x81\x81\x10a\"`Wa\"`a>\xA5V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9C\x83\x81Ta\"\x85\x90a>\xDDV[\x81\x10a\"\x93Wa\"\x93a>\xA5V[\x81T`\x01\x16\x15a\"\xB2W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\"\xE7\x81a?xV[\x92PPa\")V[P`\xA1`\0a#\x01`\xA0\x85\x01\x85a?\x12V[\x84\x81\x81\x10a#\x11Wa#\x11a>\xA5V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a#<\x81a?xV[\x91PPa\"\x10V[P`\0[a#U`\xC0\x83\x01\x83a?\x12V[\x90P\x81\x10\x15a%\xA5Wa#k`\xC0\x83\x01\x83a?\x12V[\x82\x81\x81\x10a#{Wa#{a>\xA5V[\x90P` \x02\x81\x01\x90a#\x8D\x91\x90a@\x9FV[a#\x9E\x90`\x80\x81\x01\x90``\x01a7\xAFV[`\xA1`\0a#\xAF`\xC0\x86\x01\x86a?\x12V[\x85\x81\x81\x10a#\xBFWa#\xBFa>\xA5V[\x90P` \x02\x81\x01\x90a#\xD1\x91\x90a@\x9FV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a$\x0B`\xC0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a$\x1BWa$\x1Ba>\xA5V[\x90P` \x02\x81\x01\x90a$-\x91\x90a@\x9FV[a$;\x90` \x81\x01\x90a?\x12V[\x90P\x81\x10\x15a%\x92Wa$Q`\xC0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a$aWa$aa>\xA5V[\x90P` \x02\x81\x01\x90a$s\x91\x90a@\x9FV[a$\x81\x90`@\x81\x01\x90a?\x12V[\x82\x81\x81\x10a$\x91Wa$\x91a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a$\xA6\x91\x90a?\xF3V[`\x9F`\0a$\xB7`\xC0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a$\xC7Wa$\xC7a>\xA5V[\x90P` \x02\x81\x01\x90a$\xD9\x91\x90a@\x9FV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a$\xFA`\xC0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a%\nWa%\na>\xA5V[\x90P` \x02\x81\x01\x90a%\x1C\x91\x90a@\x9FV[a%*\x90` \x81\x01\x90a?\x12V[\x85\x81\x81\x10a%:Wa%:a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a%O\x91\x90a7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a%\x8A\x81a?xV[\x91PPa#\xFEV[P\x80a%\x9D\x81a?xV[\x91PPa#HV[P`\0[a%\xB6`\xE0\x83\x01\x83a?\x12V[\x90P\x81\x10\x15a'iW`\0[a%\xCF`\xE0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a%\xDFWa%\xDFa>\xA5V[\x90P` \x02\x81\x01\x90a%\xF1\x91\x90a@\xBFV[a%\xFF\x90` \x81\x01\x90a?\x12V[\x90P\x81\x10\x15a'VWa&\x15`\xE0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a&%Wa&%a>\xA5V[\x90P` \x02\x81\x01\x90a&7\x91\x90a@\xBFV[a&E\x90`@\x81\x01\x90a?\x12V[\x82\x81\x81\x10a&UWa&Ua>\xA5V[\x90P` \x02\x01` \x81\x01\x90a&j\x91\x90a?\xF3V[`\x9F`\0a&{`\xE0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a&\x8BWa&\x8Ba>\xA5V[\x90P` \x02\x81\x01\x90a&\x9D\x91\x90a@\xBFV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a&\xBE`\xE0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a&\xCEWa&\xCEa>\xA5V[\x90P` \x02\x81\x01\x90a&\xE0\x91\x90a@\xBFV[a&\xEE\x90` \x81\x01\x90a?\x12V[\x85\x81\x81\x10a&\xFEWa&\xFEa>\xA5V[\x90P` \x02\x01` \x81\x01\x90a'\x13\x91\x90a7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a'N\x81a?xV[\x91PPa%\xC2V[P\x80a'a\x81a?xV[\x91PPa%\xA9V[P`\0[a'{a\x01\0\x83\x01\x83a@\x0EV[\x90P\x81\x10\x15a(\x1FWa'\x92a\x01\0\x83\x01\x83a@\x0EV[\x82\x81\x81\x10a'\xA2Wa'\xA2a>\xA5V[\x90P`@\x02\x01` \x01` \x81\x01\x90a'\xBA\x91\x90a7\xAFV[`\xA1`\0a'\xCCa\x01\0\x86\x01\x86a@\x0EV[\x85\x81\x81\x10a'\xDCWa'\xDCa>\xA5V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a(\x17\x90a?xV[\x91PPa'mV[P`\xC0\x83\x015`\x9AUa(5` \x85\x01\x85a@\xE9V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua(_``\x85\x01`@\x86\x01a@\xE9V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua(\x97`\x80\x85\x01\x85aA\x04V[a(\xA3\x91`\x9C\x91a6+V[Pa(\xB4`\xC0\x85\x01`\xA0\x86\x01a@\xE9V[`\x9D\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F\x9A\x12\x8F\xE74\x7F\x1E\x11\xCA\"\xAA\x9D\xEBc.\xC9\xAB\xB0\x96\x08\xC1:\x99L`\xF7zV/F\x01qa(\xFC` \x86\x01\x86a@\xE9V[a)\x0C``\x87\x01`@\x88\x01a@\xE9V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1[PPPPV[a);a-MV[`\x01`\x01`\xA0\x1B\x03\x81\x16a)\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xABV[a\x05\xBD\x81a2FV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a)\xC9WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a)\xE3WP0;\x15\x80\x15a)\xE3WP`\0T`\xFF\x16`\x01\x14[a*FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05\xABV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a*iW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a*t\x85`\0a4\x17V[a*}\x84a2FV[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x98\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x80\x15a*\xF3W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+MW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+q\x91\x90a=\xCBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a+\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a=\xE8V[`fT\x19\x81\x19`fT\x19\x16\x14a,\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\x1EV[`\x01`\x01`\xA0\x1B\x03\x81\x16a,\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\nDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\xABV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra-\xC3a6\xAFV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a-\xF6Wa-\xF8V[\xFE[P\x80a.6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xABV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra.Za6\xCDV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a-\xF6WP\x80a.6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xABV[a.\xDAa6\xEBV[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a/\xC2`\0\x80Q` aJ\x15\x839\x81Q\x91R\x86a>\xBBV[\x90P[a/\xCE\x81a5\x01V[\x90\x93P\x91P`\0\x80Q` aJ\x15\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a0\x08W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aJ\x15\x839\x81Q\x91R`\x01\x82\x08\x90Pa/\xC5V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a0Ta7\x10V[`\0[`\x02\x81\x10\x15a2\x19W`\0a0m\x82`\x06aI\xF5V[\x90P\x84\x82`\x02\x81\x10a0\x81Wa0\x81a>\xA5V[` \x02\x01QQ\x83a0\x93\x83`\0aH\xDBV[`\x0C\x81\x10a0\xA3Wa0\xA3a>\xA5V[` \x02\x01R\x84\x82`\x02\x81\x10a0\xBAWa0\xBAa>\xA5V[` \x02\x01Q` \x01Q\x83\x82`\x01a0\xD1\x91\x90aH\xDBV[`\x0C\x81\x10a0\xE1Wa0\xE1a>\xA5V[` \x02\x01R\x83\x82`\x02\x81\x10a0\xF8Wa0\xF8a>\xA5V[` \x02\x01QQQ\x83a1\x0B\x83`\x02aH\xDBV[`\x0C\x81\x10a1\x1BWa1\x1Ba>\xA5V[` \x02\x01R\x83\x82`\x02\x81\x10a12Wa12a>\xA5V[` \x02\x01QQ`\x01` \x02\x01Q\x83a1K\x83`\x03aH\xDBV[`\x0C\x81\x10a1[Wa1[a>\xA5V[` \x02\x01R\x83\x82`\x02\x81\x10a1rWa1ra>\xA5V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a1\x8DWa1\x8Da>\xA5V[` \x02\x01Q\x83a1\x9E\x83`\x04aH\xDBV[`\x0C\x81\x10a1\xAEWa1\xAEa>\xA5V[` \x02\x01R\x83\x82`\x02\x81\x10a1\xC5Wa1\xC5a>\xA5V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a1\xE0Wa1\xE0a>\xA5V[` \x02\x01Q\x83a1\xF1\x83`\x05aH\xDBV[`\x0C\x81\x10a2\x01Wa2\x01a>\xA5V[` \x02\x01RP\x80a2\x11\x81a?xV[\x91PPa0WV[Pa2\"a7/V[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a2\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x05\xABV[\x81a\xFF\xFF\x16`\x01\x14\x15a3\x08WP\x81a\x1A\xF1V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a3qW`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a3TWa3Q\x84\x84a.>V[\x93P[a3^\x83\x84a.>V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a3$V[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a3\xA1WP` \x82\x01Q\x15[\x15a3\xBFWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aJ\x15\x839\x81Q\x91R\x84` \x01Qa3\xF2\x91\x90a>\xBBV[a4\n\x90`\0\x80Q` aJ\x15\x839\x81Q\x91RaAJV[\x90R\x92\x91PPV[\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a48WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a4\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a4\xFD\x82a,VV[PPV[`\0\x80\x80`\0\x80Q` aJ\x15\x839\x81Q\x91R`\x03`\0\x80Q` aJ\x15\x839\x81Q\x91R\x86`\0\x80Q` aJ\x15\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a5w\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aJ\x15\x839\x81Q\x91Ra5\x83V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a5\x8Ea7/V[a5\x96a7MV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a-\xF6WP\x82a6 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[PQ\x95\x94PPPPPV[\x82\x80Ta67\x90a>\xDDV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a6YW`\0\x85Ua6\x9FV[\x82`\x1F\x10a6rW\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua6\x9FV[\x82\x80\x01`\x01\x01\x85U\x82\x15a6\x9FW\x91\x82\x01[\x82\x81\x11\x15a6\x9FW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a6\x84V[Pa6\xAB\x92\x91Pa7kV[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a6\xFEa7\x80V[\x81R` \x01a7\x0Ba7\x80V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a6\xABW`\0\x81U`\x01\x01a7lV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a4\x12W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7\xC1W`\0\x80\xFD[a7\xCA\x82a7\x9EV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xBDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7\xF8W`\0\x80\xFD[\x815a7\xCA\x81a7\xD1V[`\0` \x82\x84\x03\x12\x15a8\x15W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a8TWa8Ta8\x1CV[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a8TWa8Ta8\x1CV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a8\xA4Wa8\xA4a8\x1CV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a8\xBEW`\0\x80\xFD[a8\xC6a82V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a8\xEDW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a9\x0FWa9\x0Fa8\x1CV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a9&W`\0\x80\xFD[\x84[\x81\x81\x10\x15a3qW\x805\x83R` \x92\x83\x01\x92\x01a9(V[`\0`\x80\x82\x84\x03\x12\x15a9RW`\0\x80\xFD[a9Za82V[\x90Pa9f\x83\x83a8\xDCV[\x81Ra9u\x83`@\x84\x01a8\xDCV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a9\x97W`\0\x80\xFD[\x845\x93Pa9\xA8\x86` \x87\x01a8\xACV[\x92Pa9\xB7\x86``\x87\x01a9@V[\x91Pa9\xC6\x86`\xE0\x87\x01a8\xACV[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a9\xFEW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a9\xE2V[\x81\x81\x11\x15a:\x10W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a:9W`\0\x80\xFD[\x825\x91Pa:I` \x84\x01a7\x9EV[\x90P\x92P\x92\x90PV[`\0a\x01\0\x82\x84\x03\x12\x15a:eW`\0\x80\xFD[P\x91\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a:eW`\0\x80\xFD[`\0a\x01 \x82\x84\x03\x12\x15a:eW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a:\xA5W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:\xBCW`\0\x80\xFD[a:\xC8\x87\x83\x88\x01a:RV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a:\xDEW`\0\x80\xFD[a:\xEA\x87\x83\x88\x01a:kV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a;\0W`\0\x80\xFD[Pa;\r\x86\x82\x87\x01a:}V[\x91PP\x92P\x92P\x92V[`\0`\xE0\x82\x84\x03\x12\x15a;)W`\0\x80\xFD[a;1a8ZV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;JW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a;^W`\0\x80\xFD[\x815` \x82\x82\x11\x15a;rWa;ra8\x1CV[a;\x80\x81\x83`\x05\x1B\x01a8|V[\x82\x81R\x81\x81\x01\x93P`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x87\x83\x11\x15a;\xA0W`\0\x80\xFD[\x93\x81\x01\x93[\x82\x85\x10\x15a;\xC9Wa;\xB7\x88\x86a8\xACV[\x84R\x81\x84\x01\x93P`@\x85\x01\x94Pa;\xA5V[\x85Ra;\xD7\x87\x87\x83\x01a9@V[\x81\x86\x01RPPPPa;\xEC\x83`\xA0\x84\x01a8\xACV[`@\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a<\nW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<'W`\0\x80\xFD[a<3\x85\x82\x86\x01a;\x17V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a<vW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a<QV[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra<\x9D``\x84\x01\x82a<=V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra<\xBA\x82\x82a<=V[\x95\x94PPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a<\xD9W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<\xF0W`\0\x80\xFD[a<\xFC\x88\x83\x89\x01a:RV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a=\x12W`\0\x80\xFD[a=\x1E\x88\x83\x89\x01a:kV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a=4W`\0\x80\xFD[a=@\x88\x83\x89\x01a:kV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a=VW`\0\x80\xFD[Pa=c\x87\x82\x88\x01a:}V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a=\x85W`\0\x80\xFD[\x845a=\x90\x81a7\xD1V[\x93P` \x85\x015a=\xA0\x81a7\xD1V[\x92P`@\x85\x015a=\xB0\x81a7\xD1V[\x91P``\x85\x015a=\xC0\x81a7\xD1V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a=\xDDW`\0\x80\xFD[\x81Qa7\xCA\x81a7\xD1V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[\x80\x15\x15\x81\x14a\x05\xBDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a>RW`\0\x80\xFD[\x81Qa7\xCA\x81a>2V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82a>\xD8WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a>\xF1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a:eWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a?)W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a?CW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a?[W`\0\x80\xFD[\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a?\x8CWa?\x8Ca?bV[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a?\xAAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a?\xC4W`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a?[W`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a4\x12W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a@\x05W`\0\x80\xFD[a7\xCA\x82a?\xDCV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a@%W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a@?W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a?[W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a@nW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a@\x88W`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a?[W`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12a@\xB5W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12a@\xB5W`\0\x80\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a4\x12W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a@\xFBW`\0\x80\xFD[a7\xCA\x82a@\xD5V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aA\x1BW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aA5W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a?[W`\0\x80\xFD[`\0\x82\x82\x10\x15aA\\WaA\\a?bV[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aA\x81WaA\x81a?bV[\x03\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aA\xA0W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xBFW`\0\x80\xFD[\x806\x03\x83\x13\x15a?[W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81Rc\xFF\xFF\xFF\xFFaB\t\x83a@\xD5V[\x16` \x82\x01R` \x82\x015`@\x82\x01R`\0aB'`@\x84\x01a@\xD5V[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaB@``\x84\x01a@\xD5V[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPaBZ`\x80\x84\x01\x84aA\x89V[a\x01\0\x80`\xA0\x86\x01RaBra\x01 \x86\x01\x83\x85aA\xCEV[\x92PaB\x80`\xA0\x87\x01a@\xD5V[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x87\x01R\x91PaB\x9B`\xC0\x87\x01\x87aA\x89V[\x86\x85\x03`\x1F\x19\x01`\xE0\x88\x01R\x92PaB\xB4\x84\x84\x83aA\xCEV[\x93PPaB\xC3`\xE0\x87\x01a@\xD5V[c\xFF\xFF\xFF\xFF\x16\x94\x01\x93\x90\x93R\x93\x92PPPV[\x805a4\x12\x81a>2V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aB\xF8W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aC\x17W`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15a?[W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<vW`\xFFaCL\x83a7\x9EV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aC9V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aCvW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aC\x95W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15a?[W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<vW`\xFFaC\xCA\x83a7\x9EV[\x16\x87R`\x01`\x01``\x1B\x03aC\xE0\x84\x84\x01a?\xDCV[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aC\xB7V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aD\"W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aDAW`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15a?[W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<vW`\xFFaDv\x83a7\x9EV[\x16\x87R`\x01`\x01``\x1B\x03aD\x8C\x84\x84\x01a?\xDCV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aDcV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aD\xBBW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xDAW`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15a?[W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<vW`\xFFaE\x0F\x83a7\x9EV[\x16\x87RaE*\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aD\xFCV[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aEVW`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<vW`\x01`\x01``\x1B\x03aE\x9C\x83a?\xDCV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aE\x83V[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aF`W\x83\x85\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aE\xEAW\x82\x83\xFD[\x88\x01\x805\x86R`\x80aE\xFE\x88\x83\x01\x83aB\xE1V[\x82\x8A\x8A\x01RaF\x10\x83\x8A\x01\x82\x84aC)V[\x92PPP`@aF\"\x81\x84\x01\x84aB\xE1V[\x89\x84\x03\x83\x8B\x01RaF4\x84\x82\x84aEsV[\x93PPPP```\xFFaFH\x82\x85\x01a7\x9EV[\x16\x97\x01\x96\x90\x96RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aE\xCAV[P\x92\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aF`W\x83\x85\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12aF\xA9W\x82\x83\xFD[\x88\x01\x805\x86R``aF\xBD\x88\x83\x01\x83aB\xE1V[\x82\x8A\x8A\x01RaF\xCF\x83\x8A\x01\x82\x84aC)V[\x92PPP`@aF\xE1\x81\x84\x01\x84aB\xE1V[\x93P\x88\x83\x03\x82\x8A\x01RaF\xF5\x83\x85\x83aEsV[\x9D\x8A\x01\x9D\x98PPP\x93\x87\x01\x93PP`\x01\x01aF\x89V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<vW\x815\x87R`\xFFaG4\x84\x84\x01a7\x9EV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aG\x1BV[` \x81RaGf` \x82\x01aG`\x84aB\xD6V[\x15\x15\x90RV[`\0aGu` \x84\x01\x84aB\xE1V[a\x01 \x80`@\x86\x01RaG\x8Da\x01@\x86\x01\x83\x85aC)V[\x92PaG\x9C`@\x87\x01\x87aC_V[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaG\xB6\x85\x85\x84aC\xA7V[\x94PaG\xC5``\x89\x01\x89aD\x0BV[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaG\xDE\x85\x85\x84aDSV[\x94PaG\xED`\x80\x89\x01\x89aD\xA4V[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaH\x06\x85\x85\x84aD\xECV[\x94PaH\x15`\xA0\x89\x01\x89aB\xE1V[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaH.\x85\x85\x84aE=V[\x94PaH=`\xC0\x89\x01\x89aB\xE1V[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaHV\x85\x85\x84aE\xAFV[\x94PaHe`\xE0\x89\x01\x89aB\xE1V[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaH\x80\x86\x86\x85aFnV[\x95PaH\x8E\x81\x8A\x01\x8AaD\x0BV[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaH\xA8\x84\x84\x83aG\x0BV[\x97\x96PPPPPPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aH\xD2WaH\xD2a?bV[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15aH\xEEWaH\xEEa?bV[P\x01\x90V[`\0` \x80\x83Ra\x01\0\x83\x01c\xFF\xFF\xFF\xFFaI\r\x86a@\xD5V[\x16\x82\x85\x01R\x81\x85\x015`@\x85\x01RaI(`@\x86\x01\x86aB\xE1V[`\xE0``\x87\x01R\x91\x82\x90Ra\x01 `\x05\x83\x90\x1B\x86\x01\x81\x01\x92\x90\x86\x01\x82`\0[\x83\x81\x10\x15aI\x82W\x88\x86\x03a\x01\x1F\x19\x01\x83RaIc\x82\x86aB\xE1V[aIn\x88\x82\x84aE=V[\x97PPP\x91\x86\x01\x91\x90\x86\x01\x90`\x01\x01aIGV[PPPPP``\x85\x015`\x80\x85\x01R`\x80\x85\x015`\xA0\x85\x01R`\xA0\x85\x015`\xC0\x85\x01R`\xC0\x85\x015`\xE0\x85\x01R\x80\x92PPP\x92\x91PPV[`\0a\x1A\xF16\x83a;\x17V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aI\xECWaI\xECa?bV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aJ\x0FWaJ\x0Fa?bV[P\x02\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \x18\xFEc@\x0F\x0F~\xC4F#?\x9C\xAC\x83\t\xA7\x93G\x17\\.U\x90\x83\x96\xAF\xDE\x8B%`]\xC3dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICE_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x01\xDAW`\x005`\xE0\x1C\x80c\\\x97Z\xBB\x11a\x01\x04W\x80c\xDF\x03L\xD0\x11a\0\xA2W\x80c\xF2\xFD\xE3\x8B\x11a\0qW\x80c\xF2\xFD\xE3\x8B\x14a\x04\xBBW\x80c\xF8\xC8v^\x14a\x04\xCEW\x80c\xFA\xBC\x1C\xBC\x14a\x04\xE1W\x80c\xFCv]\xD5\x14a\x04\xF4W`\0\x80\xFD[\x80c\xDF\x03L\xD0\x14a\x04tW\x80c\xE6\x1D\xB1u\x14a\x04\x87W\x80c\xEDZ\x04\xFE\x14a\x04\x90W\x80c\xF1\xE9\x8A\\\x14a\x04\xA8W`\0\x80\xFD[\x80c\x81@\xDF\xD3\x11a\0\xDEW\x80c\x81@\xDF\xD3\x14a\x04\x1DW\x80c\x88o\x11\x95\x14a\x040W\x80c\x8D\xA5\xCB[\x14a\x04CW\x80c\xC4\xE1\x91L\x14a\x04TW`\0\x80\xFD[\x80c\\\x97Z\xBB\x14a\x03\xE4W\x80cqP\x18\xA6\x14a\x03\xECW\x80cz\xD7Ua\x14a\x03\xF4W`\0\x80\xFD[\x80c=\x9F\xB0\x0C\x11a\x01|W\x80cM\xEA\xBC!\x11a\x01KW\x80cM\xEA\xBC!\x14a\x03pW\x80cRn>d\x14a\x03\x95W\x80cY\\jg\x14a\x03\xB9W\x80cZ\xC8j\xB7\x14a\x03\xC1W`\0\x80\xFD[\x80c=\x9F\xB0\x0C\x14a\x02\xADW\x80cC\r;9\x14a\x02\xD8W\x80cI\x9Do\xB6\x14a\x03\rW\x80cJ\xE6\xB2\x03\x14a\x03YW`\0\x80\xFD[\x80c\x13d9\xDD\x11a\x01\xB8W\x80c\x13d9\xDD\x14a\x02HW\x80c\x17\x1F\x1D[\x14a\x02[W\x80c\x1E-K\xF7\x14a\x02\x85W\x80c*\x84\x14\xFD\x14a\x02\x98W`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x01\xDFW\x80c\x10\xD6z/\x14a\x02 W\x80c\x12FH\xC9\x14a\x025W[`\0\x80\xFD[a\x02\x06a\x01\xED6`\x04a7\xAFV[`\xA0` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[a\x023a\x02.6`\x04a7\xE6V[a\x05\x04V[\0[a\x023a\x02C6`\x04a7\xE6V[a\x05\xC0V[a\x023a\x02V6`\x04a8\x03V[a\x05\xEAV[a\x02na\x02i6`\x04a9\x80V[a\x07)V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02\x17V[a\x023a\x02\x936`\x04a7\xE6V[a\x08\xB3V[a\x02\xA0a\x08\xDDV[`@Qa\x02\x17\x91\x90a9\xD1V[`\x98Ta\x02\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x02\xFBa\x02\xE66`\x04a8\x03V[`\xA1` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03Aa\x03\x1B6`\x04a:&V[`\x9F` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02\x17V[a\x03b`\x9AT\x81V[`@Q\x90\x81R` \x01a\x02\x17V[`\x9DTa\x03\x80\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02\x17V[`\x98Ta\x03\xA9\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02\x17V[a\x023a\tkV[a\x03\xA9a\x03\xCF6`\x04a7\xAFV[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fTa\x03bV[a\x023a\n2V[a\x03Aa\x04\x026`\x04a7\xAFV[`\x9E` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x023a\x04+6`\x04a:\x90V[a\nFV[`eTa\x02\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x02\xC0V[a\x04ga\x04b6`\x04a;\xF7V[a\x15\nV[`@Qa\x02\x17\x91\x90a<\x81V[`\x97Ta\x02\xC0\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[a\x03b`\x99T\x81V[`\x9BTa\x03\x80\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x023a\x04\xB66`\x04a<\xC3V[a\x1A\xF7V[a\x023a\x04\xC96`\x04a7\xE6V[a)3V[a\x023a\x04\xDC6`\x04a=oV[a)\xA9V[a\x023a\x04\xEF6`\x04a8\x03V[a*\xFAV[`\x9BTa\x03\x80\x90c\xFF\xFF\xFF\xFF\x16\x81V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05WW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x05{\x91\x90a=\xCBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x05\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a=\xE8V[`@Q\x80\x91\x03\x90\xFD[a\x05\xBD\x81a,VV[PV[a\x05\xC8a-MV[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x062W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06V\x91\x90a>@V[a\x06rW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a>]V[`fT\x81\x81\x16\x14a\x06\xEBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07qWa\x07qa>\xA5V[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x07\x96Wa\x07\x96a>\xA5V[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x07\xB2Wa\x07\xB2a>\xA5V[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08\x0F\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x082\x91\x90a>\xBBV[\x90Pa\x08\xA5a\x08Ka\x08D\x88\x84a-\xA7V[\x86\x90a.>V[a\x08Sa.\xD2V[a\x08\x9Ba\x08\x8C\x85a\x08\x86`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a-\xA7V[a\x08\x95\x8Ca/\x92V[\x90a.>V[\x88b\x01\xD4\xC0a0\"V[\x90\x98\x90\x97P\x95PPPPPPV[a\x08\xBBa-MV[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x9C\x80Ta\x08\xEA\x90a>\xDDV[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\t\x16\x90a>\xDDV[\x80\x15a\tcW\x80`\x1F\x10a\t8Wa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\tcV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\tFW\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\t\xB3W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\t\xD7\x91\x90a>@V[a\t\xF3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a>]V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a\n:a-MV[a\nD`\0a2FV[V[a\nNa-MV[`\0[a\n^` \x83\x01\x83a?\x12V[\x90P\x81\x10\x15a\x0B#W`\x9E`\0a\nx` \x85\x01\x85a?\x12V[\x84\x81\x81\x10a\n\x88Wa\n\x88a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\n\x9D\x91\x90a7\xAFV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\xA0\x91a\n\xD0\x90\x85\x01\x85a?\x12V[\x84\x81\x81\x10a\n\xE0Wa\n\xE0a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\n\xF5\x91\x90a7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\x0B\x1B\x81a?xV[\x91PPa\nQV[P`\0[a\x0B4`@\x83\x01\x83a?\x93V[\x90P\x81\x10\x15a\x0CoWa\x0BJ`@\x83\x01\x83a?\x93V[\x82\x81\x81\x10a\x0BZWa\x0BZa>\xA5V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x0Br\x91\x90a?\xF3V[`\x9E`\0a\x0B\x83`@\x86\x01\x86a?\x93V[\x85\x81\x81\x10a\x0B\x93Wa\x0B\x93a>\xA5V[a\x0B\xA9\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x0B\xE9\x90\x83\x01\x83a?\x93V[\x82\x81\x81\x10a\x0B\xF9Wa\x0B\xF9a>\xA5V[\x90P`\x80\x02\x01`@\x01`\xA0`\0\x84\x80`@\x01\x90a\x0C\x16\x91\x90a?\x93V[\x85\x81\x81\x10a\x0C&Wa\x0C&a>\xA5V[a\x0C<\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x0Cg\x81a?xV[\x91PPa\x0B'V[P`\0[a\x0C\x80``\x83\x01\x83a@\x0EV[\x90P\x81\x10\x15a\r8Wa\x0C\x96``\x83\x01\x83a@\x0EV[\x82\x81\x81\x10a\x0C\xA6Wa\x0C\xA6a>\xA5V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x0C\xBE\x91\x90a?\xF3V[`\x9E`\0a\x0C\xCF``\x86\x01\x86a@\x0EV[\x85\x81\x81\x10a\x0C\xDFWa\x0C\xDFa>\xA5V[a\x0C\xF5\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\r0\x81a?xV[\x91PPa\x0CsV[P`\0[a\rI`\x80\x83\x01\x83a@WV[\x90P\x81\x10\x15a\r\xE5Wa\r_`\x80\x83\x01\x83a@WV[\x82\x81\x81\x10a\roWa\roa>\xA5V[\x90P``\x02\x01` \x01`\xA0`\0\x84\x80`\x80\x01\x90a\r\x8C\x91\x90a@WV[\x85\x81\x81\x10a\r\x9CWa\r\x9Ca>\xA5V[a\r\xB2\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\r\xDD\x81a?xV[\x91PPa\r<V[P`\0[a\r\xF6`\xA0\x83\x01\x83a?\x12V[\x90P\x81\x10\x15a\x0F\x1DW`\0[`\x9C\x80Ta\x0E\x0F\x90a>\xDDV[\x90P\x81\x10\x15a\x0E\xC8W`\x9F`\0a\x0E)`\xA0\x86\x01\x86a?\x12V[\x85\x81\x81\x10a\x0E9Wa\x0E9a>\xA5V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9C\x83\x81Ta\x0E^\x90a>\xDDV[\x81\x10a\x0ElWa\x0Ela>\xA5V[\x81T`\x01\x16\x15a\x0E\x8BW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x0E\xC0\x81a?xV[\x92PPa\x0E\x02V[P`\xA1`\0a\x0E\xDA`\xA0\x85\x01\x85a?\x12V[\x84\x81\x81\x10a\x0E\xEAWa\x0E\xEAa>\xA5V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x0F\x15\x81a?xV[\x91PPa\r\xE9V[P`\0[a\x0F.`\xC0\x83\x01\x83a?\x12V[\x90P\x81\x10\x15a\x11~Wa\x0FD`\xC0\x83\x01\x83a?\x12V[\x82\x81\x81\x10a\x0FTWa\x0FTa>\xA5V[\x90P` \x02\x81\x01\x90a\x0Ff\x91\x90a@\x9FV[a\x0Fw\x90`\x80\x81\x01\x90``\x01a7\xAFV[`\xA1`\0a\x0F\x88`\xC0\x86\x01\x86a?\x12V[\x85\x81\x81\x10a\x0F\x98Wa\x0F\x98a>\xA5V[\x90P` \x02\x81\x01\x90a\x0F\xAA\x91\x90a@\x9FV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x0F\xE4`\xC0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a\x0F\xF4Wa\x0F\xF4a>\xA5V[\x90P` \x02\x81\x01\x90a\x10\x06\x91\x90a@\x9FV[a\x10\x14\x90` \x81\x01\x90a?\x12V[\x90P\x81\x10\x15a\x11kWa\x10*`\xC0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a\x10:Wa\x10:a>\xA5V[\x90P` \x02\x81\x01\x90a\x10L\x91\x90a@\x9FV[a\x10Z\x90`@\x81\x01\x90a?\x12V[\x82\x81\x81\x10a\x10jWa\x10ja>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\x10\x7F\x91\x90a?\xF3V[`\x9F`\0a\x10\x90`\xC0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a\x10\xA0Wa\x10\xA0a>\xA5V[\x90P` \x02\x81\x01\x90a\x10\xB2\x91\x90a@\x9FV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x10\xD3`\xC0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a\x10\xE3Wa\x10\xE3a>\xA5V[\x90P` \x02\x81\x01\x90a\x10\xF5\x91\x90a@\x9FV[a\x11\x03\x90` \x81\x01\x90a?\x12V[\x85\x81\x81\x10a\x11\x13Wa\x11\x13a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\x11(\x91\x90a7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x11c\x81a?xV[\x91PPa\x0F\xD7V[P\x80a\x11v\x81a?xV[\x91PPa\x0F!V[P`\0[a\x11\x8F`\xE0\x83\x01\x83a?\x12V[\x90P\x81\x10\x15a\x13BW`\0[a\x11\xA8`\xE0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a\x11\xB8Wa\x11\xB8a>\xA5V[\x90P` \x02\x81\x01\x90a\x11\xCA\x91\x90a@\xBFV[a\x11\xD8\x90` \x81\x01\x90a?\x12V[\x90P\x81\x10\x15a\x13/Wa\x11\xEE`\xE0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a\x11\xFEWa\x11\xFEa>\xA5V[\x90P` \x02\x81\x01\x90a\x12\x10\x91\x90a@\xBFV[a\x12\x1E\x90`@\x81\x01\x90a?\x12V[\x82\x81\x81\x10a\x12.Wa\x12.a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\x12C\x91\x90a?\xF3V[`\x9F`\0a\x12T`\xE0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a\x12dWa\x12da>\xA5V[\x90P` \x02\x81\x01\x90a\x12v\x91\x90a@\xBFV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x12\x97`\xE0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a\x12\xA7Wa\x12\xA7a>\xA5V[\x90P` \x02\x81\x01\x90a\x12\xB9\x91\x90a@\xBFV[a\x12\xC7\x90` \x81\x01\x90a?\x12V[\x85\x81\x81\x10a\x12\xD7Wa\x12\xD7a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\x12\xEC\x91\x90a7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x13'\x81a?xV[\x91PPa\x11\x9BV[P\x80a\x13:\x81a?xV[\x91PPa\x11\x82V[P`\0[a\x13Ta\x01\0\x83\x01\x83a@\x0EV[\x90P\x81\x10\x15a\x13\xF8Wa\x13ka\x01\0\x83\x01\x83a@\x0EV[\x82\x81\x81\x10a\x13{Wa\x13{a>\xA5V[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x13\x93\x91\x90a7\xAFV[`\xA1`\0a\x13\xA5a\x01\0\x86\x01\x86a@\x0EV[\x85\x81\x81\x10a\x13\xB5Wa\x13\xB5a>\xA5V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x13\xF0\x90a?xV[\x91PPa\x13FV[P`\xC0\x82\x015`\x9AUa\x14\x0E` \x84\x01\x84a@\xE9V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua\x148``\x84\x01`@\x85\x01a@\xE9V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\x14p`\x80\x84\x01\x84aA\x04V[a\x14|\x91`\x9C\x91a6+V[Pa\x14\x8D`\xC0\x84\x01`\xA0\x85\x01a@\xE9V[`\x9D\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa\x14\xD5` \x85\x01\x85a@\xE9V[a\x14\xE5``\x86\x01`@\x87\x01a@\xE9V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPV[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9C\x80Ta\x15A\x90a>\xDDV[\x90P\x90Pa\x15b`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15zWa\x15za8\x1CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xA3W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a\x15\xC1Wa\x15\xC1a8\x1CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xEAW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R\x84QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a\x16\nWa\x16\na8\x1CV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x163W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87QQ\x81\x10\x15a\x17\xEEWa\x16|\x88`\0\x01Q\x82\x81Q\x81\x10a\x16]Wa\x16]a>\xA5V[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a\x16\x8EWa\x16\x8Ea>\xA5V[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a\x17lW\x82a\x16\xAB`\x01\x83aAJV[\x81Q\x81\x10a\x16\xBBWa\x16\xBBa>\xA5V[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a\x16\xD8Wa\x16\xD8a>\xA5V[` \x02` \x01\x01Q`\0\x1C\x11a\x17lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`M`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerG1PubkeysForOldSta`d\x82\x01Rl\x1D\x19H\x1B\x9B\xDD\x08\x1C\xDB\xDC\x9D\x19Y`\x9A\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[a\x17\xDAa\x17\xD3`\xA1`\0\x86\x85\x81Q\x81\x10a\x17\x88Wa\x17\x88a>\xA5V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x01\x91\x90\x91R`@\x01`\0 T\x8AQ\x80Q`\xFF\x90\x92\x16\x91\x85\x90\x81\x10a\x17\xBDWa\x17\xBDa>\xA5V[` \x02` \x01\x01Qa2\x98\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a.>V[\x95P\x80a\x17\xE6\x81a?xV[\x91PPa\x16:V[Pa\x17\xF8\x85a3|V[\x94P`\0[\x84\x81\x10\x15a\x19\xD9W`\x9C\x81\x81Ta\x18\x13\x90a>\xDDV[\x81\x10a\x18!Wa\x18!a>\xA5V[\x81T`\x01\x16\x15a\x18@W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\xA0` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa\x18\x81\x90\x87\x90a.>V[`\xFF\x83\x16`\0\x90\x81R`\x9E` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a\x18\xBCWa\x18\xBCa>\xA5V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a\x18\xE8Wa\x18\xE8a>\xA5V[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a\x19\x06Wa\x19\x06a>\xA5V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88QQ\x81\x10\x15a\x19\xC6W`\x9F`\0\x85\x83\x81Q\x81\x10a\x19IWa\x19Ia>\xA5V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a\x19\x94Wa\x19\x94a>\xA5V[` \x02` \x01\x01\x81\x81Qa\x19\xA8\x91\x90aAaV[`\x01`\x01``\x1B\x03\x16\x90RP\x80a\x19\xBE\x81a?xV[\x91PPa\x19)V[P\x80a\x19\xD1\x81a?xV[\x91PPa\x17\xFDV[P`\0\x80a\x19\xF1\x8A\x88\x8B` \x01Q\x8C`@\x01Qa\x07)V[\x91P\x91P\x81a\x1AtW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[\x80a\x1A\xE7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[P\x92\x95PPPPPP[\x92\x91PPV[`\x97T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x1BQW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[a\x1Ba`\x80\x85\x01``\x86\x01a@\xE9V[`\x9BTd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x1B\xC3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x05\xABV[\x83`@Q` \x01a\x1B\xD4\x91\x90aA\xF7V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83` \x015\x14a\x1C<W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[\x80`@Q` \x01a\x1CM\x91\x90aGLV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83``\x015\x14a\x1C\xB5W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x05\xABV[`\x9BTd\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x15a\x1EuWa\x1C\xDB``\x85\x01`@\x86\x01a@\xE9V[`\x9BTc\xFF\xFF\xFF\xFF\x91\x82\x16\x91a\x1C\xFC\x91d\x01\0\0\0\0\x90\x04\x16a8@aH\xB3V[c\xFF\xFF\xFF\xFF\x16\x11a\x1D?W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x077F\x16\xC6R\x077F\x17FR\x03`\x9C\x1B`D\x82\x01R`d\x01a\x05\xABV[B`\x99Tb\x03\xF4\x80a\x1DQ\x91\x90aH\xDBV[\x11a\x1D\x8EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rlstale state 1`\x98\x1B`D\x82\x01R`d\x01a\x05\xABV[`\0a\x1D\xC4\x84`@Q` \x01a\x1D\xA4\x91\x90aH\xF3V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84a\x04b\x90aI\xBAV[`\x9DT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9C\x80Ta\x1D\xE0\x90a>\xDDV[\x90P\x81\x10\x15a\x1EqW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x1E\x03Wa\x1E\x03a>\xA5V[` \x02` \x01\x01Qa\x1E\x15\x91\x90aI\xC6V[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x1E6Wa\x1E6a>\xA5V[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x1EQ\x91\x90aI\xF5V[\x10\x15a\x1E_WPPPa)-V[\x80a\x1Ei\x81a?xV[\x91PPa\x1D\xD3V[PPP[`\0[a\x1E\x85` \x83\x01\x83a?\x12V[\x90P\x81\x10\x15a\x1FJW`\x9E`\0a\x1E\x9F` \x85\x01\x85a?\x12V[\x84\x81\x81\x10a\x1E\xAFWa\x1E\xAFa>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\x1E\xC4\x91\x90a7\xAFV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\xA0\x91a\x1E\xF7\x90\x85\x01\x85a?\x12V[\x84\x81\x81\x10a\x1F\x07Wa\x1F\x07a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a\x1F\x1C\x91\x90a7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\x1FB\x81a?xV[\x91PPa\x1ExV[P`\0[a\x1F[`@\x83\x01\x83a?\x93V[\x90P\x81\x10\x15a \x96Wa\x1Fq`@\x83\x01\x83a?\x93V[\x82\x81\x81\x10a\x1F\x81Wa\x1F\x81a>\xA5V[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x1F\x99\x91\x90a?\xF3V[`\x9E`\0a\x1F\xAA`@\x86\x01\x86a?\x93V[\x85\x81\x81\x10a\x1F\xBAWa\x1F\xBAa>\xA5V[a\x1F\xD0\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua \x10\x90\x83\x01\x83a?\x93V[\x82\x81\x81\x10a  Wa  a>\xA5V[\x90P`\x80\x02\x01`@\x01`\xA0`\0\x84\x80`@\x01\x90a =\x91\x90a?\x93V[\x85\x81\x81\x10a MWa Ma>\xA5V[a c\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a \x8E\x81a?xV[\x91PPa\x1FNV[P`\0[a \xA7``\x83\x01\x83a@\x0EV[\x90P\x81\x10\x15a!_Wa \xBD``\x83\x01\x83a@\x0EV[\x82\x81\x81\x10a \xCDWa \xCDa>\xA5V[\x90P`@\x02\x01` \x01` \x81\x01\x90a \xE5\x91\x90a?\xF3V[`\x9E`\0a \xF6``\x86\x01\x86a@\x0EV[\x85\x81\x81\x10a!\x06Wa!\x06a>\xA5V[a!\x1C\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a!W\x81a?xV[\x91PPa \x9AV[P`\0[a!p`\x80\x83\x01\x83a@WV[\x90P\x81\x10\x15a\"\x0CWa!\x86`\x80\x83\x01\x83a@WV[\x82\x81\x81\x10a!\x96Wa!\x96a>\xA5V[\x90P``\x02\x01` \x01`\xA0`\0\x84\x80`\x80\x01\x90a!\xB3\x91\x90a@WV[\x85\x81\x81\x10a!\xC3Wa!\xC3a>\xA5V[a!\xD9\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa7\xAFV[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\"\x04\x81a?xV[\x91PPa!cV[P`\0[a\"\x1D`\xA0\x83\x01\x83a?\x12V[\x90P\x81\x10\x15a#DW`\0[`\x9C\x80Ta\"6\x90a>\xDDV[\x90P\x81\x10\x15a\"\xEFW`\x9F`\0a\"P`\xA0\x86\x01\x86a?\x12V[\x85\x81\x81\x10a\"`Wa\"`a>\xA5V[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9C\x83\x81Ta\"\x85\x90a>\xDDV[\x81\x10a\"\x93Wa\"\x93a>\xA5V[\x81T`\x01\x16\x15a\"\xB2W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\"\xE7\x81a?xV[\x92PPa\")V[P`\xA1`\0a#\x01`\xA0\x85\x01\x85a?\x12V[\x84\x81\x81\x10a#\x11Wa#\x11a>\xA5V[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a#<\x81a?xV[\x91PPa\"\x10V[P`\0[a#U`\xC0\x83\x01\x83a?\x12V[\x90P\x81\x10\x15a%\xA5Wa#k`\xC0\x83\x01\x83a?\x12V[\x82\x81\x81\x10a#{Wa#{a>\xA5V[\x90P` \x02\x81\x01\x90a#\x8D\x91\x90a@\x9FV[a#\x9E\x90`\x80\x81\x01\x90``\x01a7\xAFV[`\xA1`\0a#\xAF`\xC0\x86\x01\x86a?\x12V[\x85\x81\x81\x10a#\xBFWa#\xBFa>\xA5V[\x90P` \x02\x81\x01\x90a#\xD1\x91\x90a@\x9FV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a$\x0B`\xC0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a$\x1BWa$\x1Ba>\xA5V[\x90P` \x02\x81\x01\x90a$-\x91\x90a@\x9FV[a$;\x90` \x81\x01\x90a?\x12V[\x90P\x81\x10\x15a%\x92Wa$Q`\xC0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a$aWa$aa>\xA5V[\x90P` \x02\x81\x01\x90a$s\x91\x90a@\x9FV[a$\x81\x90`@\x81\x01\x90a?\x12V[\x82\x81\x81\x10a$\x91Wa$\x91a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a$\xA6\x91\x90a?\xF3V[`\x9F`\0a$\xB7`\xC0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a$\xC7Wa$\xC7a>\xA5V[\x90P` \x02\x81\x01\x90a$\xD9\x91\x90a@\x9FV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a$\xFA`\xC0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a%\nWa%\na>\xA5V[\x90P` \x02\x81\x01\x90a%\x1C\x91\x90a@\x9FV[a%*\x90` \x81\x01\x90a?\x12V[\x85\x81\x81\x10a%:Wa%:a>\xA5V[\x90P` \x02\x01` \x81\x01\x90a%O\x91\x90a7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a%\x8A\x81a?xV[\x91PPa#\xFEV[P\x80a%\x9D\x81a?xV[\x91PPa#HV[P`\0[a%\xB6`\xE0\x83\x01\x83a?\x12V[\x90P\x81\x10\x15a'iW`\0[a%\xCF`\xE0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a%\xDFWa%\xDFa>\xA5V[\x90P` \x02\x81\x01\x90a%\xF1\x91\x90a@\xBFV[a%\xFF\x90` \x81\x01\x90a?\x12V[\x90P\x81\x10\x15a'VWa&\x15`\xE0\x84\x01\x84a?\x12V[\x83\x81\x81\x10a&%Wa&%a>\xA5V[\x90P` \x02\x81\x01\x90a&7\x91\x90a@\xBFV[a&E\x90`@\x81\x01\x90a?\x12V[\x82\x81\x81\x10a&UWa&Ua>\xA5V[\x90P` \x02\x01` \x81\x01\x90a&j\x91\x90a?\xF3V[`\x9F`\0a&{`\xE0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a&\x8BWa&\x8Ba>\xA5V[\x90P` \x02\x81\x01\x90a&\x9D\x91\x90a@\xBFV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a&\xBE`\xE0\x87\x01\x87a?\x12V[\x86\x81\x81\x10a&\xCEWa&\xCEa>\xA5V[\x90P` \x02\x81\x01\x90a&\xE0\x91\x90a@\xBFV[a&\xEE\x90` \x81\x01\x90a?\x12V[\x85\x81\x81\x10a&\xFEWa&\xFEa>\xA5V[\x90P` \x02\x01` \x81\x01\x90a'\x13\x91\x90a7\xAFV[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a'N\x81a?xV[\x91PPa%\xC2V[P\x80a'a\x81a?xV[\x91PPa%\xA9V[P`\0[a'{a\x01\0\x83\x01\x83a@\x0EV[\x90P\x81\x10\x15a(\x1FWa'\x92a\x01\0\x83\x01\x83a@\x0EV[\x82\x81\x81\x10a'\xA2Wa'\xA2a>\xA5V[\x90P`@\x02\x01` \x01` \x81\x01\x90a'\xBA\x91\x90a7\xAFV[`\xA1`\0a'\xCCa\x01\0\x86\x01\x86a@\x0EV[\x85\x81\x81\x10a'\xDCWa'\xDCa>\xA5V[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a(\x17\x90a?xV[\x91PPa'mV[P`\xC0\x83\x015`\x9AUa(5` \x85\x01\x85a@\xE9V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua(_``\x85\x01`@\x86\x01a@\xE9V[`\x9B\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua(\x97`\x80\x85\x01\x85aA\x04V[a(\xA3\x91`\x9C\x91a6+V[Pa(\xB4`\xC0\x85\x01`\xA0\x86\x01a@\xE9V[`\x9D\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F\x9A\x12\x8F\xE74\x7F\x1E\x11\xCA\"\xAA\x9D\xEBc.\xC9\xAB\xB0\x96\x08\xC1:\x99L`\xF7zV/F\x01qa(\xFC` \x86\x01\x86a@\xE9V[a)\x0C``\x87\x01`@\x88\x01a@\xE9V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1[PPPPV[a);a-MV[`\x01`\x01`\xA0\x1B\x03\x81\x16a)\xA0W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x05\xABV[a\x05\xBD\x81a2FV[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a)\xC9WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a)\xE3WP0;\x15\x80\x15a)\xE3WP`\0T`\xFF\x16`\x01\x14[a*FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x05\xABV[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a*iW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a*t\x85`\0a4\x17V[a*}\x84a2FV[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x80\x86\x16`\x01`\x01`\xA0\x1B\x03\x19\x92\x83\x16\x17\x90\x92U`\x98\x80T\x92\x85\x16\x92\x90\x91\x16\x91\x90\x91\x17\x90U\x80\x15a*\xF3W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01`@Q\x80\x91\x03\x90\xA1[PPPPPV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a+MW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a+q\x91\x90a=\xCBV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a+\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x05\xAB\x90a=\xE8V[`fT\x19\x81\x19`fT\x19\x16\x14a,\x1FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\x1EV[`\x01`\x01`\xA0\x1B\x03\x81\x16a,\xE4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\nDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x05\xABV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra-\xC3a6\xAFV[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a-\xF6Wa-\xF8V[\xFE[P\x80a.6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xABV[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra.Za6\xCDV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a-\xF6WP\x80a.6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x05\xABV[a.\xDAa6\xEBV[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a/\xC2`\0\x80Q` aJ\x15\x839\x81Q\x91R\x86a>\xBBV[\x90P[a/\xCE\x81a5\x01V[\x90\x93P\x91P`\0\x80Q` aJ\x15\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a0\x08W`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aJ\x15\x839\x81Q\x91R`\x01\x82\x08\x90Pa/\xC5V[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a0Ta7\x10V[`\0[`\x02\x81\x10\x15a2\x19W`\0a0m\x82`\x06aI\xF5V[\x90P\x84\x82`\x02\x81\x10a0\x81Wa0\x81a>\xA5V[` \x02\x01QQ\x83a0\x93\x83`\0aH\xDBV[`\x0C\x81\x10a0\xA3Wa0\xA3a>\xA5V[` \x02\x01R\x84\x82`\x02\x81\x10a0\xBAWa0\xBAa>\xA5V[` \x02\x01Q` \x01Q\x83\x82`\x01a0\xD1\x91\x90aH\xDBV[`\x0C\x81\x10a0\xE1Wa0\xE1a>\xA5V[` \x02\x01R\x83\x82`\x02\x81\x10a0\xF8Wa0\xF8a>\xA5V[` \x02\x01QQQ\x83a1\x0B\x83`\x02aH\xDBV[`\x0C\x81\x10a1\x1BWa1\x1Ba>\xA5V[` \x02\x01R\x83\x82`\x02\x81\x10a12Wa12a>\xA5V[` \x02\x01QQ`\x01` \x02\x01Q\x83a1K\x83`\x03aH\xDBV[`\x0C\x81\x10a1[Wa1[a>\xA5V[` \x02\x01R\x83\x82`\x02\x81\x10a1rWa1ra>\xA5V[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a1\x8DWa1\x8Da>\xA5V[` \x02\x01Q\x83a1\x9E\x83`\x04aH\xDBV[`\x0C\x81\x10a1\xAEWa1\xAEa>\xA5V[` \x02\x01R\x83\x82`\x02\x81\x10a1\xC5Wa1\xC5a>\xA5V[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a1\xE0Wa1\xE0a>\xA5V[` \x02\x01Q\x83a1\xF1\x83`\x05aH\xDBV[`\x0C\x81\x10a2\x01Wa2\x01a>\xA5V[` \x02\x01RP\x80a2\x11\x81a?xV[\x91PPa0WV[Pa2\"a7/V[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a2\xF4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x05\xABV[\x81a\xFF\xFF\x16`\x01\x14\x15a3\x08WP\x81a\x1A\xF1V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a3qW`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a3TWa3Q\x84\x84a.>V[\x93P[a3^\x83\x84a.>V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a3$V[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a3\xA1WP` \x82\x01Q\x15[\x15a3\xBFWPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aJ\x15\x839\x81Q\x91R\x84` \x01Qa3\xF2\x91\x90a>\xBBV[a4\n\x90`\0\x80Q` aJ\x15\x839\x81Q\x91RaAJV[\x90R\x92\x91PPV[\x91\x90PV[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a48WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a4\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x05\xABV[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a4\xFD\x82a,VV[PPV[`\0\x80\x80`\0\x80Q` aJ\x15\x839\x81Q\x91R`\x03`\0\x80Q` aJ\x15\x839\x81Q\x91R\x86`\0\x80Q` aJ\x15\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a5w\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aJ\x15\x839\x81Q\x91Ra5\x83V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a5\x8Ea7/V[a5\x96a7MV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a-\xF6WP\x82a6 W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x05\xABV[PQ\x95\x94PPPPPV[\x82\x80Ta67\x90a>\xDDV[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a6YW`\0\x85Ua6\x9FV[\x82`\x1F\x10a6rW\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua6\x9FV[\x82\x80\x01`\x01\x01\x85U\x82\x15a6\x9FW\x91\x82\x01[\x82\x81\x11\x15a6\x9FW\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a6\x84V[Pa6\xAB\x92\x91Pa7kV[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a6\xFEa7\x80V[\x81R` \x01a7\x0Ba7\x80V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a6\xABW`\0\x81U`\x01\x01a7lV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a4\x12W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7\xC1W`\0\x80\xFD[a7\xCA\x82a7\x9EV[\x93\x92PPPV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x05\xBDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a7\xF8W`\0\x80\xFD[\x815a7\xCA\x81a7\xD1V[`\0` \x82\x84\x03\x12\x15a8\x15W`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a8TWa8Ta8\x1CV[`@R\x90V[`@Q``\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a8TWa8Ta8\x1CV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a8\xA4Wa8\xA4a8\x1CV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a8\xBEW`\0\x80\xFD[a8\xC6a82V[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12a8\xEDW`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15a9\x0FWa9\x0Fa8\x1CV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15a9&W`\0\x80\xFD[\x84[\x81\x81\x10\x15a3qW\x805\x83R` \x92\x83\x01\x92\x01a9(V[`\0`\x80\x82\x84\x03\x12\x15a9RW`\0\x80\xFD[a9Za82V[\x90Pa9f\x83\x83a8\xDCV[\x81Ra9u\x83`@\x84\x01a8\xDCV[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15a9\x97W`\0\x80\xFD[\x845\x93Pa9\xA8\x86` \x87\x01a8\xACV[\x92Pa9\xB7\x86``\x87\x01a9@V[\x91Pa9\xC6\x86`\xE0\x87\x01a8\xACV[\x90P\x92\x95\x91\x94P\x92PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15a9\xFEW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01a9\xE2V[\x81\x81\x11\x15a:\x10W`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[`\0\x80`@\x83\x85\x03\x12\x15a:9W`\0\x80\xFD[\x825\x91Pa:I` \x84\x01a7\x9EV[\x90P\x92P\x92\x90PV[`\0a\x01\0\x82\x84\x03\x12\x15a:eW`\0\x80\xFD[P\x91\x90PV[`\0`\xE0\x82\x84\x03\x12\x15a:eW`\0\x80\xFD[`\0a\x01 \x82\x84\x03\x12\x15a:eW`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a:\xA5W`\0\x80\xFD[\x835`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a:\xBCW`\0\x80\xFD[a:\xC8\x87\x83\x88\x01a:RV[\x94P` \x86\x015\x91P\x80\x82\x11\x15a:\xDEW`\0\x80\xFD[a:\xEA\x87\x83\x88\x01a:kV[\x93P`@\x86\x015\x91P\x80\x82\x11\x15a;\0W`\0\x80\xFD[Pa;\r\x86\x82\x87\x01a:}V[\x91PP\x92P\x92P\x92V[`\0`\xE0\x82\x84\x03\x12\x15a;)W`\0\x80\xFD[a;1a8ZV[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a;JW`\0\x80\xFD[\x81\x84\x01\x91P\x84`\x1F\x83\x01\x12a;^W`\0\x80\xFD[\x815` \x82\x82\x11\x15a;rWa;ra8\x1CV[a;\x80\x81\x83`\x05\x1B\x01a8|V[\x82\x81R\x81\x81\x01\x93P`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x87\x83\x11\x15a;\xA0W`\0\x80\xFD[\x93\x81\x01\x93[\x82\x85\x10\x15a;\xC9Wa;\xB7\x88\x86a8\xACV[\x84R\x81\x84\x01\x93P`@\x85\x01\x94Pa;\xA5V[\x85Ra;\xD7\x87\x87\x83\x01a9@V[\x81\x86\x01RPPPPa;\xEC\x83`\xA0\x84\x01a8\xACV[`@\x82\x01R\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15a<\nW`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a<'W`\0\x80\xFD[a<3\x85\x82\x86\x01a;\x17V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a<vW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a<QV[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01Ra<\x9D``\x84\x01\x82a<=V[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01Ra<\xBA\x82\x82a<=V[\x95\x94PPPPPV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a<\xD9W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a<\xF0W`\0\x80\xFD[a<\xFC\x88\x83\x89\x01a:RV[\x95P` \x87\x015\x91P\x80\x82\x11\x15a=\x12W`\0\x80\xFD[a=\x1E\x88\x83\x89\x01a:kV[\x94P`@\x87\x015\x91P\x80\x82\x11\x15a=4W`\0\x80\xFD[a=@\x88\x83\x89\x01a:kV[\x93P``\x87\x015\x91P\x80\x82\x11\x15a=VW`\0\x80\xFD[Pa=c\x87\x82\x88\x01a:}V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a=\x85W`\0\x80\xFD[\x845a=\x90\x81a7\xD1V[\x93P` \x85\x015a=\xA0\x81a7\xD1V[\x92P`@\x85\x015a=\xB0\x81a7\xD1V[\x91P``\x85\x015a=\xC0\x81a7\xD1V[\x93\x96\x92\x95P\x90\x93PPV[`\0` \x82\x84\x03\x12\x15a=\xDDW`\0\x80\xFD[\x81Qa7\xCA\x81a7\xD1V[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[\x80\x15\x15\x81\x14a\x05\xBDW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a>RW`\0\x80\xFD[\x81Qa7\xCA\x81a>2V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82a>\xD8WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\x01\x81\x81\x1C\x90\x82\x16\x80a>\xF1W`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15a:eWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a?)W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a?CW`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15a?[W`\0\x80\xFD[\x92P\x92\x90PV[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a?\x8CWa?\x8Ca?bV[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a?\xAAW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a?\xC4W`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15a?[W`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a4\x12W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a@\x05W`\0\x80\xFD[a7\xCA\x82a?\xDCV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a@%W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a@?W`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15a?[W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12a@nW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15a@\x88W`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15a?[W`\0\x80\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12a@\xB5W`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12a@\xB5W`\0\x80\xFD[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a4\x12W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a@\xFBW`\0\x80\xFD[a7\xCA\x82a@\xD5V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aA\x1BW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aA5W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15a?[W`\0\x80\xFD[`\0\x82\x82\x10\x15aA\\WaA\\a?bV[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aA\x81WaA\x81a?bV[\x03\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aA\xA0W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aA\xBFW`\0\x80\xFD[\x806\x03\x83\x13\x15a?[W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81Rc\xFF\xFF\xFF\xFFaB\t\x83a@\xD5V[\x16` \x82\x01R` \x82\x015`@\x82\x01R`\0aB'`@\x84\x01a@\xD5V[c\xFF\xFF\xFF\xFF\x81\x16``\x84\x01RPaB@``\x84\x01a@\xD5V[c\xFF\xFF\xFF\xFF\x81\x16`\x80\x84\x01RPaBZ`\x80\x84\x01\x84aA\x89V[a\x01\0\x80`\xA0\x86\x01RaBra\x01 \x86\x01\x83\x85aA\xCEV[\x92PaB\x80`\xA0\x87\x01a@\xD5V[c\xFF\xFF\xFF\xFF\x81\x16`\xC0\x87\x01R\x91PaB\x9B`\xC0\x87\x01\x87aA\x89V[\x86\x85\x03`\x1F\x19\x01`\xE0\x88\x01R\x92PaB\xB4\x84\x84\x83aA\xCEV[\x93PPaB\xC3`\xE0\x87\x01a@\xD5V[c\xFF\xFF\xFF\xFF\x16\x94\x01\x93\x90\x93R\x93\x92PPPV[\x805a4\x12\x81a>2V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aB\xF8W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aC\x17W`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15a?[W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<vW`\xFFaCL\x83a7\x9EV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aC9V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aCvW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aC\x95W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15a?[W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<vW`\xFFaC\xCA\x83a7\x9EV[\x16\x87R`\x01`\x01``\x1B\x03aC\xE0\x84\x84\x01a?\xDCV[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aC\xB7V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aD\"W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aDAW`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15a?[W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<vW`\xFFaDv\x83a7\x9EV[\x16\x87R`\x01`\x01``\x1B\x03aD\x8C\x84\x84\x01a?\xDCV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aDcV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aD\xBBW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aD\xDAW`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15a?[W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<vW`\xFFaE\x0F\x83a7\x9EV[\x16\x87RaE*\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aD\xFCV[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aEVW`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<vW`\x01`\x01``\x1B\x03aE\x9C\x83a?\xDCV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aE\x83V[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aF`W\x83\x85\x03\x8AR\x825`~\x19\x896\x03\x01\x81\x12aE\xEAW\x82\x83\xFD[\x88\x01\x805\x86R`\x80aE\xFE\x88\x83\x01\x83aB\xE1V[\x82\x8A\x8A\x01RaF\x10\x83\x8A\x01\x82\x84aC)V[\x92PPP`@aF\"\x81\x84\x01\x84aB\xE1V[\x89\x84\x03\x83\x8B\x01RaF4\x84\x82\x84aEsV[\x93PPPP```\xFFaFH\x82\x85\x01a7\x9EV[\x16\x97\x01\x96\x90\x96RP\x98\x85\x01\x98\x91\x85\x01\x91`\x01\x01aE\xCAV[P\x92\x98\x97PPPPPPPPV[\x81\x83R`\0` \x80\x85\x01\x80\x81\x96P\x85`\x05\x1B\x81\x01\x91P\x84`\0\x80[\x88\x81\x10\x15aF`W\x83\x85\x03\x8AR\x825`^\x19\x896\x03\x01\x81\x12aF\xA9W\x82\x83\xFD[\x88\x01\x805\x86R``aF\xBD\x88\x83\x01\x83aB\xE1V[\x82\x8A\x8A\x01RaF\xCF\x83\x8A\x01\x82\x84aC)V[\x92PPP`@aF\xE1\x81\x84\x01\x84aB\xE1V[\x93P\x88\x83\x03\x82\x8A\x01RaF\xF5\x83\x85\x83aEsV[\x9D\x8A\x01\x9D\x98PPP\x93\x87\x01\x93PP`\x01\x01aF\x89V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15a<vW\x815\x87R`\xFFaG4\x84\x84\x01a7\x9EV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aG\x1BV[` \x81RaGf` \x82\x01aG`\x84aB\xD6V[\x15\x15\x90RV[`\0aGu` \x84\x01\x84aB\xE1V[a\x01 \x80`@\x86\x01RaG\x8Da\x01@\x86\x01\x83\x85aC)V[\x92PaG\x9C`@\x87\x01\x87aC_V[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaG\xB6\x85\x85\x84aC\xA7V[\x94PaG\xC5``\x89\x01\x89aD\x0BV[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaG\xDE\x85\x85\x84aDSV[\x94PaG\xED`\x80\x89\x01\x89aD\xA4V[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaH\x06\x85\x85\x84aD\xECV[\x94PaH\x15`\xA0\x89\x01\x89aB\xE1V[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaH.\x85\x85\x84aE=V[\x94PaH=`\xC0\x89\x01\x89aB\xE1V[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaHV\x85\x85\x84aE\xAFV[\x94PaHe`\xE0\x89\x01\x89aB\xE1V[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaH\x80\x86\x86\x85aFnV[\x95PaH\x8E\x81\x8A\x01\x8AaD\x0BV[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaH\xA8\x84\x84\x83aG\x0BV[\x97\x96PPPPPPPV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aH\xD2WaH\xD2a?bV[\x01\x94\x93PPPPV[`\0\x82\x19\x82\x11\x15aH\xEEWaH\xEEa?bV[P\x01\x90V[`\0` \x80\x83Ra\x01\0\x83\x01c\xFF\xFF\xFF\xFFaI\r\x86a@\xD5V[\x16\x82\x85\x01R\x81\x85\x015`@\x85\x01RaI(`@\x86\x01\x86aB\xE1V[`\xE0``\x87\x01R\x91\x82\x90Ra\x01 `\x05\x83\x90\x1B\x86\x01\x81\x01\x92\x90\x86\x01\x82`\0[\x83\x81\x10\x15aI\x82W\x88\x86\x03a\x01\x1F\x19\x01\x83RaIc\x82\x86aB\xE1V[aIn\x88\x82\x84aE=V[\x97PPP\x91\x86\x01\x91\x90\x86\x01\x90`\x01\x01aIGV[PPPPP``\x85\x015`\x80\x85\x01R`\x80\x85\x015`\xA0\x85\x01R`\xA0\x85\x015`\xC0\x85\x01R`\xC0\x85\x015`\xE0\x85\x01R\x80\x92PPP\x92\x91PPV[`\0a\x1A\xF16\x83a;\x17V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aI\xECWaI\xECa?bV[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aJ\x0FWaJ\x0Fa?bV[P\x02\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 \x18\xFEc@\x0F\x0F~\xC4F#?\x9C\xAC\x83\t\xA7\x93G\x17\\.U\x90\x83\x96\xAF\xDE\x8B%`]\xC3dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct GaspMultiRollupService<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for GaspMultiRollupService<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for GaspMultiRollupService<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for GaspMultiRollupService<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for GaspMultiRollupService<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(::core::stringify!(GaspMultiRollupService))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> GaspMultiRollupService<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    GASPMULTIROLLUPSERVICE_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                GASPMULTIROLLUPSERVICE_ABI.clone(),
                GASPMULTIROLLUPSERVICE_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Calls the contract's `checkSignatures` (0xc4e1914c) function
        pub fn check_signatures(
            &self,
            msg_hash: [u8; 32],
            params: NonSignerStakesAndSignatureForOldState,
        ) -> ::ethers::contract::builders::ContractCall<M, QuorumStakeTotals> {
            self.0
                .method_hash([196, 225, 145, 76], (msg_hash, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xf8c8765e) function
        pub fn initialize(
            &self,
            pauser_registry: ::ethers::core::types::Address,
            initial_owner: ::ethers::core::types::Address,
            updater: ::ethers::core::types::Address,
            rolldown: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [248, 200, 118, 94],
                    (pauser_registry, initial_owner, updater, rolldown),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastUpdateBlockTimestamp` (0xe61db175) function
        pub fn last_update_block_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([230, 29, 177, 117], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestCompletedTaskCreatedBlock` (0xed5a04fe) function
        pub fn latest_completed_task_created_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([237, 90, 4, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestCompletedTaskNumber` (0xfc765dd5) function
        pub fn latest_completed_task_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([252, 118, 93, 213], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestPendingStateHash` (0x4ae6b203) function
        pub fn latest_pending_state_hash(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([74, 230, 178, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorAndQuorumToStakes` (0x499d6fb6) function
        pub fn operator_and_quorum_to_stakes(
            &self,
            p0: [u8; 32],
            p1: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([73, 157, 111, 182], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `operatorIdQuorumCount` (0x430d3b39) function
        pub fn operator_id_quorum_count(
            &self,
            p0: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([67, 13, 59, 57], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pause` (0x136439dd) function
        pub fn pause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 100, 57, 221], new_paused_status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauseAll` (0x595c6a67) function
        pub fn pause_all(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 92, 106, 103], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5ac86ab7) function
        pub fn paused_with_index(
            &self,
            index: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([90, 200, 106, 183], index)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `paused` (0x5c975abb) function
        pub fn paused(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([92, 151, 90, 187], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauserRegistry` (0x886f1195) function
        pub fn pauser_registry(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([136, 111, 17, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `process_eigen_reinit` (0x8140dfd3) function
        pub fn process_eigen_reinit(
            &self,
            task: Task,
            task_response: TaskResponse,
            operator_state_info: OperatorStateInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [129, 64, 223, 211],
                    (task, task_response, operator_state_info),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `process_eigen_update` (0xf1e98a5c) function
        pub fn process_eigen_update(
            &self,
            task: Task,
            task_response: TaskResponse,
            non_signer_stakes_and_signature_for_old_state: NonSignerStakesAndSignatureForOldState,
            operator_state_info: OperatorStateInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [241, 233, 138, 92],
                    (
                        task,
                        task_response,
                        non_signer_stakes_and_signature_for_old_state,
                        operator_state_info,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `qourumApk` (0x03d097d2) function
        pub fn qourum_apk(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([3, 208, 151, 210], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumNumbers` (0x2a8414fd) function
        pub fn quorum_numbers(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Bytes,
        > {
            self.0
                .method_hash([42, 132, 20, 253], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumThresholdPercentage` (0x4deabc21) function
        pub fn quorum_threshold_percentage(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([77, 234, 188, 33], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `quorumToStakes` (0x7ad75561) function
        pub fn quorum_to_stakes(
            &self,
            p0: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([122, 215, 85, 97], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `renounceOwnership` (0x715018a6) function
        pub fn renounce_ownership(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rolldown` (0x3d9fb00c) function
        pub fn rolldown(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([61, 159, 176, 12], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setPauserRegistry` (0x10d67a2f) function
        pub fn set_pauser_registry(
            &self,
            new_pauser_registry: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 214, 122, 47], new_pauser_registry)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `set_rolldown` (0x1e2d4bf7) function
        pub fn set_rolldown(
            &self,
            rolldown: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 45, 75, 247], rolldown)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `set_updater` (0x124648c9) function
        pub fn set_updater(
            &self,
            updater: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 70, 72, 201], updater)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stalled` (0x526e3e64) function
        pub fn stalled(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([82, 110, 62, 100], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `trySignatureAndApkVerification` (0x171f1d5b) function
        pub fn try_signature_and_apk_verification(
            &self,
            msg_hash: [u8; 32],
            apk: G1Point,
            apk_g2: G2Point,
            sigma: G1Point,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, bool)> {
            self.0
                .method_hash([23, 31, 29, 91], (msg_hash, apk, apk_g2, sigma))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `unpause` (0xfabc1cbc) function
        pub fn unpause(
            &self,
            new_paused_status: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 188, 28, 188], new_paused_status)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `updater` (0xdf034cd0) function
        pub fn updater(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([223, 3, 76, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EigenReinitProcessed` event
        pub fn eigen_reinit_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EigenReinitProcessedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `EigenUpdateProcessed` event
        pub fn eigen_update_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            EigenUpdateProcessedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            InitializedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Paused` event
        pub fn paused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PausedFilter> {
            self.0.event()
        }
        ///Gets the contract's `PauserRegistrySet` event
        pub fn pauser_registry_set_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            PauserRegistrySetFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            UnpausedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            GaspMultiRollupServiceEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for GaspMultiRollupService<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "EigenReinitProcessed",
        abi = "EigenReinitProcessed(uint32,uint32)"
    )]
    pub struct EigenReinitProcessedFilter {
        pub task_number: u32,
        pub task_created_block: u32,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "EigenUpdateProcessed",
        abi = "EigenUpdateProcessed(uint32,uint32)"
    )]
    pub struct EigenUpdateProcessedFilter {
        pub task_number: u32,
        pub task_created_block: u32,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Paused", abi = "Paused(address,uint256)")]
    pub struct PausedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub new_paused_status: ::ethers::core::types::U256,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "PauserRegistrySet", abi = "PauserRegistrySet(address,address)")]
    pub struct PauserRegistrySetFilter {
        pub pauser_registry: ::ethers::core::types::Address,
        pub new_pauser_registry: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "Unpaused", abi = "Unpaused(address,uint256)")]
    pub struct UnpausedFilter {
        #[ethevent(indexed)]
        pub account: ::ethers::core::types::Address,
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum GaspMultiRollupServiceEvents {
        EigenReinitProcessedFilter(EigenReinitProcessedFilter),
        EigenUpdateProcessedFilter(EigenUpdateProcessedFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for GaspMultiRollupServiceEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EigenReinitProcessedFilter::decode_log(log) {
                return Ok(
                    GaspMultiRollupServiceEvents::EigenReinitProcessedFilter(decoded),
                );
            }
            if let Ok(decoded) = EigenUpdateProcessedFilter::decode_log(log) {
                return Ok(
                    GaspMultiRollupServiceEvents::EigenUpdateProcessedFilter(decoded),
                );
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(
                    GaspMultiRollupServiceEvents::OwnershipTransferredFilter(decoded),
                );
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(
                    GaspMultiRollupServiceEvents::PauserRegistrySetFilter(decoded),
                );
            }
            if let Ok(decoded) = UnpausedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::UnpausedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for GaspMultiRollupServiceEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::EigenReinitProcessedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EigenUpdateProcessedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EigenReinitProcessedFilter>
    for GaspMultiRollupServiceEvents {
        fn from(value: EigenReinitProcessedFilter) -> Self {
            Self::EigenReinitProcessedFilter(value)
        }
    }
    impl ::core::convert::From<EigenUpdateProcessedFilter>
    for GaspMultiRollupServiceEvents {
        fn from(value: EigenUpdateProcessedFilter) -> Self {
            Self::EigenUpdateProcessedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter>
    for GaspMultiRollupServiceEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter>
    for GaspMultiRollupServiceEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `checkSignatures` function with signature `checkSignatures(bytes32,((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))` and selector `0xc4e1914c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "checkSignatures",
        abi = "checkSignatures(bytes32,((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))"
    )]
    pub struct CheckSignaturesCall {
        pub msg_hash: [u8; 32],
        pub params: NonSignerStakesAndSignatureForOldState,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address)` and selector `0xf8c8765e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "initialize", abi = "initialize(address,address,address,address)")]
    pub struct InitializeCall {
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_owner: ::ethers::core::types::Address,
        pub updater: ::ethers::core::types::Address,
        pub rolldown: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `lastUpdateBlockTimestamp` function with signature `lastUpdateBlockTimestamp()` and selector `0xe61db175`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "lastUpdateBlockTimestamp", abi = "lastUpdateBlockTimestamp()")]
    pub struct LastUpdateBlockTimestampCall;
    ///Container type for all input parameters for the `latestCompletedTaskCreatedBlock` function with signature `latestCompletedTaskCreatedBlock()` and selector `0xed5a04fe`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "latestCompletedTaskCreatedBlock",
        abi = "latestCompletedTaskCreatedBlock()"
    )]
    pub struct LatestCompletedTaskCreatedBlockCall;
    ///Container type for all input parameters for the `latestCompletedTaskNumber` function with signature `latestCompletedTaskNumber()` and selector `0xfc765dd5`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "latestCompletedTaskNumber", abi = "latestCompletedTaskNumber()")]
    pub struct LatestCompletedTaskNumberCall;
    ///Container type for all input parameters for the `latestPendingStateHash` function with signature `latestPendingStateHash()` and selector `0x4ae6b203`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "latestPendingStateHash", abi = "latestPendingStateHash()")]
    pub struct LatestPendingStateHashCall;
    ///Container type for all input parameters for the `operatorAndQuorumToStakes` function with signature `operatorAndQuorumToStakes(bytes32,uint8)` and selector `0x499d6fb6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "operatorAndQuorumToStakes",
        abi = "operatorAndQuorumToStakes(bytes32,uint8)"
    )]
    pub struct OperatorAndQuorumToStakesCall(pub [u8; 32], pub u8);
    ///Container type for all input parameters for the `operatorIdQuorumCount` function with signature `operatorIdQuorumCount(bytes32)` and selector `0x430d3b39`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "operatorIdQuorumCount", abi = "operatorIdQuorumCount(bytes32)")]
    pub struct OperatorIdQuorumCountCall(pub [u8; 32]);
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pause` function with signature `pause(uint256)` and selector `0x136439dd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pause", abi = "pause(uint256)")]
    pub struct PauseCall {
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `pauseAll` function with signature `pauseAll()` and selector `0x595c6a67`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pauseAll", abi = "pauseAll()")]
    pub struct PauseAllCall;
    ///Container type for all input parameters for the `paused` function with signature `paused(uint8)` and selector `0x5ac86ab7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "paused", abi = "paused(uint8)")]
    pub struct PausedWithIndexCall {
        pub index: u8,
    }
    ///Container type for all input parameters for the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "paused", abi = "paused()")]
    pub struct PausedCall;
    ///Container type for all input parameters for the `pauserRegistry` function with signature `pauserRegistry()` and selector `0x886f1195`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pauserRegistry", abi = "pauserRegistry()")]
    pub struct PauserRegistryCall;
    ///Container type for all input parameters for the `process_eigen_reinit` function with signature `process_eigen_reinit((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32[][],bytes32,bytes32,bytes32,bytes32),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))` and selector `0x8140dfd3`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "process_eigen_reinit",
        abi = "process_eigen_reinit((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32[][],bytes32,bytes32,bytes32,bytes32),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))"
    )]
    pub struct ProcessEigenReinitCall {
        pub task: Task,
        pub task_response: TaskResponse,
        pub operator_state_info: OperatorStateInfo,
    }
    ///Container type for all input parameters for the `process_eigen_update` function with signature `process_eigen_update((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32[][],bytes32,bytes32,bytes32,bytes32),((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))` and selector `0xf1e98a5c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "process_eigen_update",
        abi = "process_eigen_update((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32[][],bytes32,bytes32,bytes32,bytes32),((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))"
    )]
    pub struct ProcessEigenUpdateCall {
        pub task: Task,
        pub task_response: TaskResponse,
        pub non_signer_stakes_and_signature_for_old_state: NonSignerStakesAndSignatureForOldState,
        pub operator_state_info: OperatorStateInfo,
    }
    ///Container type for all input parameters for the `qourumApk` function with signature `qourumApk(uint8)` and selector `0x03d097d2`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "qourumApk", abi = "qourumApk(uint8)")]
    pub struct QourumApkCall(pub u8);
    ///Container type for all input parameters for the `quorumNumbers` function with signature `quorumNumbers()` and selector `0x2a8414fd`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "quorumNumbers", abi = "quorumNumbers()")]
    pub struct QuorumNumbersCall;
    ///Container type for all input parameters for the `quorumThresholdPercentage` function with signature `quorumThresholdPercentage()` and selector `0x4deabc21`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "quorumThresholdPercentage", abi = "quorumThresholdPercentage()")]
    pub struct QuorumThresholdPercentageCall;
    ///Container type for all input parameters for the `quorumToStakes` function with signature `quorumToStakes(uint8)` and selector `0x7ad75561`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "quorumToStakes", abi = "quorumToStakes(uint8)")]
    pub struct QuorumToStakesCall(pub u8);
    ///Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `0x715018a6`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    ///Container type for all input parameters for the `rolldown` function with signature `rolldown()` and selector `0x3d9fb00c`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rolldown", abi = "rolldown()")]
    pub struct RolldownCall;
    ///Container type for all input parameters for the `setPauserRegistry` function with signature `setPauserRegistry(address)` and selector `0x10d67a2f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setPauserRegistry", abi = "setPauserRegistry(address)")]
    pub struct SetPauserRegistryCall {
        pub new_pauser_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `set_rolldown` function with signature `set_rolldown(address)` and selector `0x1e2d4bf7`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "set_rolldown", abi = "set_rolldown(address)")]
    pub struct SetRolldownCall {
        pub rolldown: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `set_updater` function with signature `set_updater(address)` and selector `0x124648c9`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "set_updater", abi = "set_updater(address)")]
    pub struct SetUpdaterCall {
        pub updater: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `stalled` function with signature `stalled()` and selector `0x526e3e64`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "stalled", abi = "stalled()")]
    pub struct StalledCall;
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `trySignatureAndApkVerification` function with signature `trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))` and selector `0x171f1d5b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "trySignatureAndApkVerification",
        abi = "trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))"
    )]
    pub struct TrySignatureAndApkVerificationCall {
        pub msg_hash: [u8; 32],
        pub apk: G1Point,
        pub apk_g2: G2Point,
        pub sigma: G1Point,
    }
    ///Container type for all input parameters for the `unpause` function with signature `unpause(uint256)` and selector `0xfabc1cbc`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "unpause", abi = "unpause(uint256)")]
    pub struct UnpauseCall {
        pub new_paused_status: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `updater` function with signature `updater()` and selector `0xdf034cd0`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "updater", abi = "updater()")]
    pub struct UpdaterCall;
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        serde::Serialize,
        serde::Deserialize,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum GaspMultiRollupServiceCalls {
        CheckSignatures(CheckSignaturesCall),
        Initialize(InitializeCall),
        LastUpdateBlockTimestamp(LastUpdateBlockTimestampCall),
        LatestCompletedTaskCreatedBlock(LatestCompletedTaskCreatedBlockCall),
        LatestCompletedTaskNumber(LatestCompletedTaskNumberCall),
        LatestPendingStateHash(LatestPendingStateHashCall),
        OperatorAndQuorumToStakes(OperatorAndQuorumToStakesCall),
        OperatorIdQuorumCount(OperatorIdQuorumCountCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        ProcessEigenReinit(ProcessEigenReinitCall),
        ProcessEigenUpdate(ProcessEigenUpdateCall),
        QourumApk(QourumApkCall),
        QuorumNumbers(QuorumNumbersCall),
        QuorumThresholdPercentage(QuorumThresholdPercentageCall),
        QuorumToStakes(QuorumToStakesCall),
        RenounceOwnership(RenounceOwnershipCall),
        Rolldown(RolldownCall),
        SetPauserRegistry(SetPauserRegistryCall),
        SetRolldown(SetRolldownCall),
        SetUpdater(SetUpdaterCall),
        Stalled(StalledCall),
        TransferOwnership(TransferOwnershipCall),
        TrySignatureAndApkVerification(TrySignatureAndApkVerificationCall),
        Unpause(UnpauseCall),
        Updater(UpdaterCall),
    }
    impl ::ethers::core::abi::AbiDecode for GaspMultiRollupServiceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <CheckSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CheckSignatures(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) = <LastUpdateBlockTimestampCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LastUpdateBlockTimestamp(decoded));
            }
            if let Ok(decoded) = <LatestCompletedTaskCreatedBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestCompletedTaskCreatedBlock(decoded));
            }
            if let Ok(decoded) = <LatestCompletedTaskNumberCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestCompletedTaskNumber(decoded));
            }
            if let Ok(decoded) = <LatestPendingStateHashCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::LatestPendingStateHash(decoded));
            }
            if let Ok(decoded) = <OperatorAndQuorumToStakesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorAndQuorumToStakes(decoded));
            }
            if let Ok(decoded) = <OperatorIdQuorumCountCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OperatorIdQuorumCount(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PauseAllCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PauseAll(decoded));
            }
            if let Ok(decoded) = <PausedWithIndexCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PausedWithIndex(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) = <PauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PauserRegistry(decoded));
            }
            if let Ok(decoded) = <ProcessEigenReinitCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProcessEigenReinit(decoded));
            }
            if let Ok(decoded) = <ProcessEigenUpdateCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProcessEigenUpdate(decoded));
            }
            if let Ok(decoded) = <QourumApkCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QourumApk(decoded));
            }
            if let Ok(decoded) = <QuorumNumbersCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumNumbers(decoded));
            }
            if let Ok(decoded) = <QuorumThresholdPercentageCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumThresholdPercentage(decoded));
            }
            if let Ok(decoded) = <QuorumToStakesCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::QuorumToStakes(decoded));
            }
            if let Ok(decoded) = <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RolldownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Rolldown(decoded));
            }
            if let Ok(decoded) = <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) = <SetRolldownCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetRolldown(decoded));
            }
            if let Ok(decoded) = <SetUpdaterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetUpdater(decoded));
            }
            if let Ok(decoded) = <StalledCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Stalled(decoded));
            }
            if let Ok(decoded) = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) = <TrySignatureAndApkVerificationCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TrySignatureAndApkVerification(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <UpdaterCall as ::ethers::core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Updater(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GaspMultiRollupServiceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CheckSignatures(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LastUpdateBlockTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCompletedTaskCreatedBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCompletedTaskNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestPendingStateHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorAndQuorumToStakes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OperatorIdQuorumCount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Pause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauseAll(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PausedWithIndex(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessEigenReinit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessEigenUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QourumApk(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumNumbers(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumThresholdPercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumToStakes(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RenounceOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Rolldown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetPauserRegistry(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetRolldown(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SetUpdater(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Stalled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TrySignatureAndApkVerification(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Unpause(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Updater(element) => ::ethers::core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for GaspMultiRollupServiceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CheckSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastUpdateBlockTimestamp(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LatestCompletedTaskCreatedBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LatestCompletedTaskNumber(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LatestPendingStateHash(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorAndQuorumToStakes(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OperatorIdQuorumCount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessEigenReinit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProcessEigenUpdate(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QourumApk(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumNumbers(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumThresholdPercentage(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::QuorumToStakes(element) => ::core::fmt::Display::fmt(element, f),
                Self::RenounceOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::Rolldown(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetPauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetRolldown(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetUpdater(element) => ::core::fmt::Display::fmt(element, f),
                Self::Stalled(element) => ::core::fmt::Display::fmt(element, f),
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::TrySignatureAndApkVerification(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Unpause(element) => ::core::fmt::Display::fmt(element, f),
                Self::Updater(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CheckSignaturesCall> for GaspMultiRollupServiceCalls {
        fn from(value: CheckSignaturesCall) -> Self {
            Self::CheckSignatures(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for GaspMultiRollupServiceCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<LastUpdateBlockTimestampCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: LastUpdateBlockTimestampCall) -> Self {
            Self::LastUpdateBlockTimestamp(value)
        }
    }
    impl ::core::convert::From<LatestCompletedTaskCreatedBlockCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedTaskCreatedBlockCall) -> Self {
            Self::LatestCompletedTaskCreatedBlock(value)
        }
    }
    impl ::core::convert::From<LatestCompletedTaskNumberCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedTaskNumberCall) -> Self {
            Self::LatestCompletedTaskNumber(value)
        }
    }
    impl ::core::convert::From<LatestPendingStateHashCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: LatestPendingStateHashCall) -> Self {
            Self::LatestPendingStateHash(value)
        }
    }
    impl ::core::convert::From<OperatorAndQuorumToStakesCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: OperatorAndQuorumToStakesCall) -> Self {
            Self::OperatorAndQuorumToStakes(value)
        }
    }
    impl ::core::convert::From<OperatorIdQuorumCountCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: OperatorIdQuorumCountCall) -> Self {
            Self::OperatorIdQuorumCount(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for GaspMultiRollupServiceCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PauseCall> for GaspMultiRollupServiceCalls {
        fn from(value: PauseCall) -> Self {
            Self::Pause(value)
        }
    }
    impl ::core::convert::From<PauseAllCall> for GaspMultiRollupServiceCalls {
        fn from(value: PauseAllCall) -> Self {
            Self::PauseAll(value)
        }
    }
    impl ::core::convert::From<PausedWithIndexCall> for GaspMultiRollupServiceCalls {
        fn from(value: PausedWithIndexCall) -> Self {
            Self::PausedWithIndex(value)
        }
    }
    impl ::core::convert::From<PausedCall> for GaspMultiRollupServiceCalls {
        fn from(value: PausedCall) -> Self {
            Self::Paused(value)
        }
    }
    impl ::core::convert::From<PauserRegistryCall> for GaspMultiRollupServiceCalls {
        fn from(value: PauserRegistryCall) -> Self {
            Self::PauserRegistry(value)
        }
    }
    impl ::core::convert::From<ProcessEigenReinitCall> for GaspMultiRollupServiceCalls {
        fn from(value: ProcessEigenReinitCall) -> Self {
            Self::ProcessEigenReinit(value)
        }
    }
    impl ::core::convert::From<ProcessEigenUpdateCall> for GaspMultiRollupServiceCalls {
        fn from(value: ProcessEigenUpdateCall) -> Self {
            Self::ProcessEigenUpdate(value)
        }
    }
    impl ::core::convert::From<QourumApkCall> for GaspMultiRollupServiceCalls {
        fn from(value: QourumApkCall) -> Self {
            Self::QourumApk(value)
        }
    }
    impl ::core::convert::From<QuorumNumbersCall> for GaspMultiRollupServiceCalls {
        fn from(value: QuorumNumbersCall) -> Self {
            Self::QuorumNumbers(value)
        }
    }
    impl ::core::convert::From<QuorumThresholdPercentageCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: QuorumThresholdPercentageCall) -> Self {
            Self::QuorumThresholdPercentage(value)
        }
    }
    impl ::core::convert::From<QuorumToStakesCall> for GaspMultiRollupServiceCalls {
        fn from(value: QuorumToStakesCall) -> Self {
            Self::QuorumToStakes(value)
        }
    }
    impl ::core::convert::From<RenounceOwnershipCall> for GaspMultiRollupServiceCalls {
        fn from(value: RenounceOwnershipCall) -> Self {
            Self::RenounceOwnership(value)
        }
    }
    impl ::core::convert::From<RolldownCall> for GaspMultiRollupServiceCalls {
        fn from(value: RolldownCall) -> Self {
            Self::Rolldown(value)
        }
    }
    impl ::core::convert::From<SetPauserRegistryCall> for GaspMultiRollupServiceCalls {
        fn from(value: SetPauserRegistryCall) -> Self {
            Self::SetPauserRegistry(value)
        }
    }
    impl ::core::convert::From<SetRolldownCall> for GaspMultiRollupServiceCalls {
        fn from(value: SetRolldownCall) -> Self {
            Self::SetRolldown(value)
        }
    }
    impl ::core::convert::From<SetUpdaterCall> for GaspMultiRollupServiceCalls {
        fn from(value: SetUpdaterCall) -> Self {
            Self::SetUpdater(value)
        }
    }
    impl ::core::convert::From<StalledCall> for GaspMultiRollupServiceCalls {
        fn from(value: StalledCall) -> Self {
            Self::Stalled(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for GaspMultiRollupServiceCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<TrySignatureAndApkVerificationCall>
    for GaspMultiRollupServiceCalls {
        fn from(value: TrySignatureAndApkVerificationCall) -> Self {
            Self::TrySignatureAndApkVerification(value)
        }
    }
    impl ::core::convert::From<UnpauseCall> for GaspMultiRollupServiceCalls {
        fn from(value: UnpauseCall) -> Self {
            Self::Unpause(value)
        }
    }
    impl ::core::convert::From<UpdaterCall> for GaspMultiRollupServiceCalls {
        fn from(value: UpdaterCall) -> Self {
            Self::Updater(value)
        }
    }
    ///Container type for all return fields from the `checkSignatures` function with signature `checkSignatures(bytes32,((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)))` and selector `0xc4e1914c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CheckSignaturesReturn(pub QuorumStakeTotals);
    ///Container type for all return fields from the `lastUpdateBlockTimestamp` function with signature `lastUpdateBlockTimestamp()` and selector `0xe61db175`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LastUpdateBlockTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `latestCompletedTaskCreatedBlock` function with signature `latestCompletedTaskCreatedBlock()` and selector `0xed5a04fe`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LatestCompletedTaskCreatedBlockReturn(pub u32);
    ///Container type for all return fields from the `latestCompletedTaskNumber` function with signature `latestCompletedTaskNumber()` and selector `0xfc765dd5`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LatestCompletedTaskNumberReturn(pub u32);
    ///Container type for all return fields from the `latestPendingStateHash` function with signature `latestPendingStateHash()` and selector `0x4ae6b203`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LatestPendingStateHashReturn(pub [u8; 32]);
    ///Container type for all return fields from the `operatorAndQuorumToStakes` function with signature `operatorAndQuorumToStakes(bytes32,uint8)` and selector `0x499d6fb6`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OperatorAndQuorumToStakesReturn(pub u128);
    ///Container type for all return fields from the `operatorIdQuorumCount` function with signature `operatorIdQuorumCount(bytes32)` and selector `0x430d3b39`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OperatorIdQuorumCountReturn(pub u8);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `paused` function with signature `paused(uint8)` and selector `0x5ac86ab7`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PausedWithIndexReturn(pub bool);
    ///Container type for all return fields from the `paused` function with signature `paused()` and selector `0x5c975abb`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PausedReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `pauserRegistry` function with signature `pauserRegistry()` and selector `0x886f1195`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct PauserRegistryReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `qourumApk` function with signature `qourumApk(uint8)` and selector `0x03d097d2`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QourumApkReturn {
        pub x: ::ethers::core::types::U256,
        pub y: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `quorumNumbers` function with signature `quorumNumbers()` and selector `0x2a8414fd`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QuorumNumbersReturn(pub ::ethers::core::types::Bytes);
    ///Container type for all return fields from the `quorumThresholdPercentage` function with signature `quorumThresholdPercentage()` and selector `0x4deabc21`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QuorumThresholdPercentageReturn(pub u32);
    ///Container type for all return fields from the `quorumToStakes` function with signature `quorumToStakes(uint8)` and selector `0x7ad75561`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct QuorumToStakesReturn(pub u128);
    ///Container type for all return fields from the `rolldown` function with signature `rolldown()` and selector `0x3d9fb00c`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RolldownReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `stalled` function with signature `stalled()` and selector `0x526e3e64`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct StalledReturn(pub bool);
    ///Container type for all return fields from the `trySignatureAndApkVerification` function with signature `trySignatureAndApkVerification(bytes32,(uint256,uint256),(uint256[2],uint256[2]),(uint256,uint256))` and selector `0x171f1d5b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct TrySignatureAndApkVerificationReturn {
        pub pairing_successful: bool,
        pub siganature_is_valid: bool,
    }
    ///Container type for all return fields from the `updater` function with signature `updater()` and selector `0xdf034cd0`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        serde::Serialize,
        serde::Deserialize,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct UpdaterReturn(pub ::ethers::core::types::Address);
}
