pub use gasp_multi_rollup_service::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types
)]
pub mod gasp_multi_rollup_service {
    pub use super::super::shared_types::*;
    #[allow(deprecated)]
    fn __abi() -> ::ethers::core::abi::Abi {
        ::ethers::core::abi::ethabi::Contract {
            constructor: ::core::option::Option::None,
            functions: ::core::convert::From::from([
                (
                    ::std::borrow::ToOwned::to_owned("allowNonRootInit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("allowNonRootInit"),
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
                    ::std::borrow::ToOwned::to_owned("chainId"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("chainId"),
                            inputs: ::std::vec![],
                            outputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::string::String::new(),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IRolldownPrimitives.ChainId",
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
                    ::std::borrow::ToOwned::to_owned("chainRdBatchNonce"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("chainRdBatchNonce"),
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
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
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
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSSignatureChecker.NonSignerStakesAndSignature",
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
                                    name: ::std::borrow::ToOwned::to_owned("_allowNonRootInit"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Bool,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bool"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_rolldown"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Address,
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("contract IRolldown"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("_chainId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "enum IRolldownPrimitives.ChainId",
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
                    ::std::borrow::ToOwned::to_owned("lastOpUpdateBlockTimestamp"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "lastOpUpdateBlockTimestamp",
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
                    ::std::borrow::ToOwned::to_owned(
                        "latestCompletedOpTaskCreatedBlock",
                    ),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestCompletedOpTaskCreatedBlock",
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
                    ::std::borrow::ToOwned::to_owned("latestCompletedOpTaskNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestCompletedOpTaskNumber",
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
                    ::std::borrow::ToOwned::to_owned("latestCompletedRdTaskNumber"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "latestCompletedRdTaskNumber",
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
                    ::std::borrow::ToOwned::to_owned("processEigenOpUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "processEigenOpUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("task"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
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
                                            "struct IFinalizerTaskManager.OpTask",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("taskResponse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.OpTaskResponse",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "nonSignerStakesAndSignature",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
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
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSSignatureChecker.NonSignerStakesAndSignature",
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
                    ::std::borrow::ToOwned::to_owned("processEigenRdUpdate"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned(
                                "processEigenRdUpdate",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("task"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Bytes,
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.RdTask",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("taskResponse"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(8usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                            ::ethers::core::abi::ethabi::ParamType::Address,
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IFinalizerTaskManager.RdTaskResponse",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned(
                                        "nonSignerStakesAndSignature",
                                    ),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
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
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                ),
                                            ),
                                            ::ethers::core::abi::ethabi::ParamType::Array(
                                                ::std::boxed::Box::new(
                                                    ::ethers::core::abi::ethabi::ParamType::Array(
                                                        ::std::boxed::Box::new(
                                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ],
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IBLSSignatureChecker.NonSignerStakesAndSignature",
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
                    ::std::borrow::ToOwned::to_owned("processEigenReinit"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("processEigenReinit"),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("task"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Tuple(
                                        ::std::vec![
                                            ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
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
                                            "struct IFinalizerTaskManager.OpTask",
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
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("merkleRoots"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::FixedBytes(32usize),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("bytes32[]"),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("ranges"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Array(
                                        ::std::boxed::Box::new(
                                            ::ethers::core::abi::ethabi::ParamType::Tuple(
                                                ::std::vec![
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                    ::ethers::core::abi::ethabi::ParamType::Uint(256usize),
                                                ],
                                            ),
                                        ),
                                    ),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned(
                                            "struct IRolldownPrimitives.Range[]",
                                        ),
                                    ),
                                },
                                ::ethers::core::abi::ethabi::Param {
                                    name: ::std::borrow::ToOwned::to_owned("lastBatchId"),
                                    kind: ::ethers::core::abi::ethabi::ParamType::Uint(32usize),
                                    internal_type: ::core::option::Option::Some(
                                        ::std::borrow::ToOwned::to_owned("uint32"),
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
                    ::std::borrow::ToOwned::to_owned("setRolldown"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setRolldown"),
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
                    ::std::borrow::ToOwned::to_owned("setUpdater"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Function {
                            name: ::std::borrow::ToOwned::to_owned("setUpdater"),
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
                    ::std::borrow::ToOwned::to_owned("EigenOpUpdateProcessed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EigenOpUpdateProcessed",
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
                    ::std::borrow::ToOwned::to_owned("EigenRdUpdateProcessed"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "EigenRdUpdateProcessed",
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
                    ::std::borrow::ToOwned::to_owned("RolldownTargetUpdated"),
                    ::std::vec![
                        ::ethers::core::abi::ethabi::Event {
                            name: ::std::borrow::ToOwned::to_owned(
                                "RolldownTargetUpdated",
                            ),
                            inputs: ::std::vec![
                                ::ethers::core::abi::ethabi::EventParam {
                                    name: ::std::borrow::ToOwned::to_owned("rolldownAddress"),
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
    pub static GASPMULTIROLLUPSERVICE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> =
        ::ethers::contract::Lazy::new(__abi);
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[PaV\xE7\x80b\0\0!`\09`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x06W`\x005`\xE0\x1C\x80co\x0C0\xA4\x11a\x01\x1AW\x80c\xD0:\x07\xB2\x11a\0\xADW\x80c\xED9\xE5\x02\x11a\0|W\x80c\xED9\xE5\x02\x14a\x05DW\x80c\xF2\xFD\xE3\x8B\x14a\x05WW\x80c\xF8N\x91\xFC\x14a\x05jW\x80c\xFA\xBC\x1C\xBC\x14a\x05sW\x80c\xFD\xC1]\xE8\x14a\x05\x86W`\0\x80\xFD[\x80c\xD0:\x07\xB2\x14a\x04\xF2W\x80c\xDE\xB4\x03}\x14a\x05\x02W\x80c\xDF\x03L\xD0\x14a\x05\x19W\x80c\xE2\xA7\xCBf\x14a\x05,W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\0\xE9W\x80c\x88o\x11\x95\x14a\x04\x9AW\x80c\x8D\xA5\xCB[\x14a\x04\xADW\x80c\x9A\x8A\x05\x92\x14a\x04\xBEW\x80c\x9DT\xF4\x19\x14a\x04\xDFW`\0\x80\xFD[\x80co\x0C0\xA4\x14a\x042W\x80cqP\x18\xA6\x14a\x04IW\x80cz\xD7Ua\x14a\x04QW\x80c}\x97\x88\x97\x14a\x04zW`\0\x80\xFD[\x80c4\xFA\xDB\xEA\x11a\x01\x9DW\x80cM\xEA\xBC!\x11a\x01lW\x80cM\xEA\xBC!\x14a\x03\xBCW\x80cRn>d\x14a\x03\xE1W\x80cY\\jg\x14a\x03\xF5W\x80cZ\xC8j\xB7\x14a\x03\xFDW\x80c\\\x97Z\xBB\x14a\x04 W`\0\x80\xFD[\x80c4\xFA\xDB\xEA\x14a\x02\xFDW\x80c=\x9F\xB0\x0C\x14a\x03\x10W\x80cC\r;9\x14a\x03;W\x80cI\x9Do\xB6\x14a\x03pW`\0\x80\xFD[\x80c\x17\x1F\x1D[\x11a\x01\xD9W\x80c\x17\x1F\x1D[\x14a\x02\x98W\x80c&5\xE7N\x14a\x02\xC2W\x80c*\x84\x14\xFD\x14a\x02\xD5W\x80c0\xC4}\x8E\x14a\x02\xEAW`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x02\x0BW\x80c\x0E\xE0\xFD\xBD\x14a\x02LW\x80c\x10\xD6z/\x14a\x02pW\x80c\x13d9\xDD\x14a\x02\x85W[`\0\x80\xFD[a\x022a\x02\x196`\x04a?\xF0V[`\x9F` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[`\x98Ta\x02`\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02CV[a\x02\x83a\x02~6`\x04a@ V[a\x05\x99V[\0[a\x02\x83a\x02\x936`\x04a@=V[a\x06UV[a\x02\xABa\x02\xA66`\x04aA\xBBV[a\x07\x94V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02CV[a\x02\x83a\x02\xD06`\x04aB\x96V[a\t\x1EV[a\x02\xDDa\x15\x18V[`@Qa\x02C\x91\x90aC\x90V[a\x02\x83a\x02\xF86`\x04aD\rV[a\x15\xA6V[a\x02\x83a\x03\x0B6`\x04aD\x9EV[a\x178V[`\x97Ta\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02CV[a\x03^a\x03I6`\x04a@=V[`\xA0` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02CV[a\x03\xA4a\x03~6`\x04aEBV[`\x9E` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02CV[`\x9CTa\x03\xCC\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02CV[`\x98Ta\x02`\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x02\x83a%\xCDV[a\x02`a\x04\x0B6`\x04a?\xF0V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fT[`@Q\x90\x81R` \x01a\x02CV[`\x9ATa\x03\xCC\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\x83a&\x94V[a\x03\xA4a\x04_6`\x04a?\xF0V[`\x9D` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x04\x8Da\x04\x886`\x04aH\x10V[a&\xA8V[`@Qa\x02C\x91\x90aH\x9AV[`eTa\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03#V[`\x97Ta\x04\xD2\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16\x81V[`@Qa\x02C\x91\x90aI\x14V[a\x02\x83a\x04\xED6`\x04a@ V[a,\x92V[`\x9ATa\x03\xCC\x90c\xFF\xFF\xFF\xFF\x16\x81V[`\x97Ta\x03\xCC\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9ATa\x03\xCC\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\x83a\x05R6`\x04aI\"V[a,\xBCV[a\x02\x83a\x05e6`\x04a@ V[a2\x17V[a\x04$`\x99T\x81V[a\x02\x83a\x05\x816`\x04a@=V[a2\x8DV[a\x02\x83a\x05\x946`\x04a@ V[a3\xE9V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x10\x91\x90aI\xA4V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06IW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90aI\xC1V[`@Q\x80\x91\x03\x90\xFD[a\x06R\x81a4\x95V[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC1\x91\x90aJ\x0BV[a\x06\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90aJ(V[`fT\x81\x81\x16\x14a\x07VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06@V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07\xDCWa\x07\xDCaJpV[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x08\x01Wa\x08\x01aJpV[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x08\x1DWa\x08\x1DaJpV[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08z\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x08\x9D\x91\x90aJ\x86V[\x90Pa\t\x10a\x08\xB6a\x08\xAF\x88\x84a5\x8CV[\x86\x90a6#V[a\x08\xBEa6\xB7V[a\t\x06a\x08\xF7\x85a\x08\xF1`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a5\x8CV[a\t\0\x8Ca7wV[\x90a6#V[\x88b\x01\xD4\xC0a8\x07V[\x90\x98\x90\x97P\x95PPPPPPV[a\t&a:+V[\x83\x82\x14a\tuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FrdUpdate info length mismatch\0\0\0`D\x82\x01R`d\x01a\x06@V[`\0[a\t\x85` \x88\x01\x88aJ\xA8V[\x90P\x81\x10\x15a\nJW`\x9D`\0a\t\x9F` \x8A\x01\x8AaJ\xA8V[\x84\x81\x81\x10a\t\xAFWa\t\xAFaJpV[\x90P` \x02\x01` \x81\x01\x90a\t\xC4\x91\x90a?\xF0V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\t\xF7\x90\x8A\x01\x8AaJ\xA8V[\x84\x81\x81\x10a\n\x07Wa\n\x07aJpV[\x90P` \x02\x01` \x81\x01\x90a\n\x1C\x91\x90a?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\nB\x81aK\x07V[\x91PPa\txV[P`\0[a\n[`@\x88\x01\x88aK\"V[\x90P\x81\x10\x15a\x0B\x96Wa\nq`@\x88\x01\x88aK\"V[\x82\x81\x81\x10a\n\x81Wa\n\x81aJpV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\n\x99\x91\x90aK\x82V[`\x9D`\0a\n\xAA`@\x8B\x01\x8BaK\"V[\x85\x81\x81\x10a\n\xBAWa\n\xBAaJpV[a\n\xD0\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x0B\x10\x90\x88\x01\x88aK\"V[\x82\x81\x81\x10a\x0B Wa\x0B aJpV[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x89\x80`@\x01\x90a\x0B=\x91\x90aK\"V[\x85\x81\x81\x10a\x0BMWa\x0BMaJpV[a\x0Bc\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x0B\x8E\x81aK\x07V[\x91PPa\nNV[P`\0[a\x0B\xA7``\x88\x01\x88aK\x9DV[\x90P\x81\x10\x15a\x0C_Wa\x0B\xBD``\x88\x01\x88aK\x9DV[\x82\x81\x81\x10a\x0B\xCDWa\x0B\xCDaJpV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x0B\xE5\x91\x90aK\x82V[`\x9D`\0a\x0B\xF6``\x8B\x01\x8BaK\x9DV[\x85\x81\x81\x10a\x0C\x06Wa\x0C\x06aJpV[a\x0C\x1C\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x0CW\x81aK\x07V[\x91PPa\x0B\x9AV[P`\0[a\x0Cp`\x80\x88\x01\x88aK\xE6V[\x90P\x81\x10\x15a\r\x0CWa\x0C\x86`\x80\x88\x01\x88aK\xE6V[\x82\x81\x81\x10a\x0C\x96Wa\x0C\x96aJpV[\x90P``\x02\x01` \x01`\x9F`\0\x89\x80`\x80\x01\x90a\x0C\xB3\x91\x90aK\xE6V[\x85\x81\x81\x10a\x0C\xC3Wa\x0C\xC3aJpV[a\x0C\xD9\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\r\x04\x81aK\x07V[\x91PPa\x0CcV[P`\0[a\r\x1D`\xA0\x88\x01\x88aJ\xA8V[\x90P\x81\x10\x15a\x0ECW`\0[`\x9B\x80Ta\r6\x90aL.V[\x90P\x81\x10\x15a\r\xEFW`\x9E`\0a\rP`\xA0\x8B\x01\x8BaJ\xA8V[\x85\x81\x81\x10a\r`Wa\r`aJpV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta\r\x85\x90aL.V[\x81\x10a\r\x93Wa\r\x93aJpV[\x81T`\x01\x16\x15a\r\xB2W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\r\xE7\x81aK\x07V[\x92PPa\r)V[P`\xA0`\0a\x0E\0\x89\x83\x01\x8AaJ\xA8V[\x84\x81\x81\x10a\x0E\x10Wa\x0E\x10aJpV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x0E;\x81aK\x07V[\x91PPa\r\x10V[P`\0[a\x0ET`\xC0\x88\x01\x88aJ\xA8V[\x90P\x81\x10\x15a\x10\xA4Wa\x0Ej`\xC0\x88\x01\x88aJ\xA8V[\x82\x81\x81\x10a\x0EzWa\x0EzaJpV[\x90P` \x02\x81\x01\x90a\x0E\x8C\x91\x90aLcV[a\x0E\x9D\x90`\x80\x81\x01\x90``\x01a?\xF0V[`\xA0`\0a\x0E\xAE`\xC0\x8B\x01\x8BaJ\xA8V[\x85\x81\x81\x10a\x0E\xBEWa\x0E\xBEaJpV[\x90P` \x02\x81\x01\x90a\x0E\xD0\x91\x90aLcV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x0F\n`\xC0\x89\x01\x89aJ\xA8V[\x83\x81\x81\x10a\x0F\x1AWa\x0F\x1AaJpV[\x90P` \x02\x81\x01\x90a\x0F,\x91\x90aLcV[a\x0F:\x90` \x81\x01\x90aJ\xA8V[\x90P\x81\x10\x15a\x10\x91Wa\x0FP`\xC0\x89\x01\x89aJ\xA8V[\x83\x81\x81\x10a\x0F`Wa\x0F`aJpV[\x90P` \x02\x81\x01\x90a\x0Fr\x91\x90aLcV[a\x0F\x80\x90`@\x81\x01\x90aJ\xA8V[\x82\x81\x81\x10a\x0F\x90Wa\x0F\x90aJpV[\x90P` \x02\x01` \x81\x01\x90a\x0F\xA5\x91\x90aK\x82V[`\x9E`\0a\x0F\xB6`\xC0\x8C\x01\x8CaJ\xA8V[\x86\x81\x81\x10a\x0F\xC6Wa\x0F\xC6aJpV[\x90P` \x02\x81\x01\x90a\x0F\xD8\x91\x90aLcV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x0F\xF9`\xC0\x8C\x01\x8CaJ\xA8V[\x86\x81\x81\x10a\x10\tWa\x10\taJpV[\x90P` \x02\x81\x01\x90a\x10\x1B\x91\x90aLcV[a\x10)\x90` \x81\x01\x90aJ\xA8V[\x85\x81\x81\x10a\x109Wa\x109aJpV[\x90P` \x02\x01` \x81\x01\x90a\x10N\x91\x90a?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x10\x89\x81aK\x07V[\x91PPa\x0E\xFDV[P\x80a\x10\x9C\x81aK\x07V[\x91PPa\x0EGV[P`\0[a\x10\xB5`\xE0\x88\x01\x88aJ\xA8V[\x90P\x81\x10\x15a\x12hW`\0[a\x10\xCE`\xE0\x89\x01\x89aJ\xA8V[\x83\x81\x81\x10a\x10\xDEWa\x10\xDEaJpV[\x90P` \x02\x81\x01\x90a\x10\xF0\x91\x90aL\x83V[a\x10\xFE\x90` \x81\x01\x90aJ\xA8V[\x90P\x81\x10\x15a\x12UWa\x11\x14`\xE0\x89\x01\x89aJ\xA8V[\x83\x81\x81\x10a\x11$Wa\x11$aJpV[\x90P` \x02\x81\x01\x90a\x116\x91\x90aL\x83V[a\x11D\x90`@\x81\x01\x90aJ\xA8V[\x82\x81\x81\x10a\x11TWa\x11TaJpV[\x90P` \x02\x01` \x81\x01\x90a\x11i\x91\x90aK\x82V[`\x9E`\0a\x11z`\xE0\x8C\x01\x8CaJ\xA8V[\x86\x81\x81\x10a\x11\x8AWa\x11\x8AaJpV[\x90P` \x02\x81\x01\x90a\x11\x9C\x91\x90aL\x83V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x11\xBD`\xE0\x8C\x01\x8CaJ\xA8V[\x86\x81\x81\x10a\x11\xCDWa\x11\xCDaJpV[\x90P` \x02\x81\x01\x90a\x11\xDF\x91\x90aL\x83V[a\x11\xED\x90` \x81\x01\x90aJ\xA8V[\x85\x81\x81\x10a\x11\xFDWa\x11\xFDaJpV[\x90P` \x02\x01` \x81\x01\x90a\x12\x12\x91\x90a?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x12M\x81aK\x07V[\x91PPa\x10\xC1V[P\x80a\x12`\x81aK\x07V[\x91PPa\x10\xA8V[P`\0[a\x12za\x01\0\x88\x01\x88aK\x9DV[\x90P\x81\x10\x15a\x13\x1EWa\x12\x91a\x01\0\x88\x01\x88aK\x9DV[\x82\x81\x81\x10a\x12\xA1Wa\x12\xA1aJpV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x12\xB9\x91\x90a?\xF0V[`\xA0`\0a\x12\xCBa\x01\0\x8B\x01\x8BaK\x9DV[\x85\x81\x81\x10a\x12\xDBWa\x12\xDBaJpV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x13\x16\x90aK\x07V[\x91PPa\x12lV[Pa\x13,` \x88\x01\x88aL\x99V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x13c`@\x88\x01` \x89\x01aL\x99V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\x13\x99``\x88\x01\x88aL\xB4V[a\x13\xA5\x91`\x9B\x91a>lV[Pa\x13\xB6`\xA0\x88\x01`\x80\x89\x01aL\x99V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\0[\x84\x81\x10\x15a\x14\x7FW`\x97T`\x01`\x01`\xA0\x1B\x03\x16c\x08\xF4-@\x87\x87\x84\x81\x81\x10a\x13\xFEWa\x13\xFEaJpV[\x90P` \x02\x015\x86\x86\x85\x81\x81\x10a\x14\x17Wa\x14\x17aJpV[\x90P`@\x02\x01`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14:\x92\x91\x90aL\xFAV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14TW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14hW=`\0\x80>=`\0\xFD[PPPP\x80\x80a\x14w\x90aK\x07V[\x91PPa\x13\xD3V[Pa\x14\x8B\x81`\x01aM\x18V[`\x97\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa\x14\xDE` \x89\x01\x89aL\x99V[a\x14\xEE`@\x8A\x01` \x8B\x01aL\x99V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x9B\x80Ta\x15%\x90aL.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15Q\x90aL.V[\x80\x15a\x15\x9EW\x80`\x1F\x10a\x15sWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\x9EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x81W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x15\xC6WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x15\xE0WP0;\x15\x80\x15a\x15\xE0WP`\0T`\xFF\x16`\x01\x14[a\x16CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06@V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x16fW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x16q\x87`\0a:\x85V[a\x16z\x86a;oV[`\x98\x80T\x85\x15\x15`\x01`\xA8\x1B\x02`\x01a\xFF\x01`\xA0\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x80\x89\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90\x91U`\x97\x80T\x91\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x82U\x84\x92`\x01d\xFF\0\0\0\x01`\xA0\x1B\x03\x19\x16\x17`\x01`\xC0\x1B\x83`\x01\x81\x11\x15a\x16\xE8Wa\x16\xE8aH\xDCV[\x02\x17\x90UP\x80\x15a\x17/W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x15\x07V[PPPPPPPV[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80\x15a\x18DW`\x98T`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15a\x18\x02W`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x06@V[a\x18\x86V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd \xBA\xBA4\x19`\xD9\x1B`D\x82\x01R`d\x01a\x06@V[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x06@V[\x84`@Q` \x01a\x18\x97\x91\x90aM\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x015\x14a\x18\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x06@V[\x81`@Q` \x01a\x19\x10\x91\x90aR\xEAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`@\x015\x14a\x19xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x06@V[\x80a\x1B\rWa\x19\x8D``\x86\x01`@\x87\x01aL\x99V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x19\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Freference block mismatch\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\0a\x1A$\x85`@Q` \x01a\x1A\x04\x91\x90aTQV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85a\x04\x88\x90aT\x80V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x1A@\x90aL.V[\x90P\x81\x10\x15a\x1B\tW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x1AcWa\x1AcaJpV[` \x02` \x01\x01Qa\x1Au\x91\x90aT\x8CV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x1A\x96Wa\x1A\x96aJpV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x1A\xB1\x91\x90aT\xBBV[\x10\x15a\x1A\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtFailed to meet quorum`X\x1B`D\x82\x01R`d\x01a\x06@V[\x80a\x1B\x01\x81aK\x07V[\x91PPa\x1A3V[PPP[`\0[a\x1B\x1D` \x84\x01\x84aJ\xA8V[\x90P\x81\x10\x15a\x1B\xE2W`\x9D`\0a\x1B7` \x86\x01\x86aJ\xA8V[\x84\x81\x81\x10a\x1BGWa\x1BGaJpV[\x90P` \x02\x01` \x81\x01\x90a\x1B\\\x91\x90a?\xF0V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\x1B\x8F\x90\x86\x01\x86aJ\xA8V[\x84\x81\x81\x10a\x1B\x9FWa\x1B\x9FaJpV[\x90P` \x02\x01` \x81\x01\x90a\x1B\xB4\x91\x90a?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\x1B\xDA\x81aK\x07V[\x91PPa\x1B\x10V[P`\0[a\x1B\xF3`@\x84\x01\x84aK\"V[\x90P\x81\x10\x15a\x1D.Wa\x1C\t`@\x84\x01\x84aK\"V[\x82\x81\x81\x10a\x1C\x19Wa\x1C\x19aJpV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x1C1\x91\x90aK\x82V[`\x9D`\0a\x1CB`@\x87\x01\x87aK\"V[\x85\x81\x81\x10a\x1CRWa\x1CRaJpV[a\x1Ch\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x1C\xA8\x90\x84\x01\x84aK\"V[\x82\x81\x81\x10a\x1C\xB8Wa\x1C\xB8aJpV[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x85\x80`@\x01\x90a\x1C\xD5\x91\x90aK\"V[\x85\x81\x81\x10a\x1C\xE5Wa\x1C\xE5aJpV[a\x1C\xFB\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x1D&\x81aK\x07V[\x91PPa\x1B\xE6V[P`\0[a\x1D?``\x84\x01\x84aK\x9DV[\x90P\x81\x10\x15a\x1D\xF7Wa\x1DU``\x84\x01\x84aK\x9DV[\x82\x81\x81\x10a\x1DeWa\x1DeaJpV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x1D}\x91\x90aK\x82V[`\x9D`\0a\x1D\x8E``\x87\x01\x87aK\x9DV[\x85\x81\x81\x10a\x1D\x9EWa\x1D\x9EaJpV[a\x1D\xB4\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x1D\xEF\x81aK\x07V[\x91PPa\x1D2V[P`\0[a\x1E\x08`\x80\x84\x01\x84aK\xE6V[\x90P\x81\x10\x15a\x1E\xA4Wa\x1E\x1E`\x80\x84\x01\x84aK\xE6V[\x82\x81\x81\x10a\x1E.Wa\x1E.aJpV[\x90P``\x02\x01` \x01`\x9F`\0\x85\x80`\x80\x01\x90a\x1EK\x91\x90aK\xE6V[\x85\x81\x81\x10a\x1E[Wa\x1E[aJpV[a\x1Eq\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x1E\x9C\x81aK\x07V[\x91PPa\x1D\xFBV[P`\0[a\x1E\xB5`\xA0\x84\x01\x84aJ\xA8V[\x90P\x81\x10\x15a\x1F\xDBW`\0[`\x9B\x80Ta\x1E\xCE\x90aL.V[\x90P\x81\x10\x15a\x1F\x87W`\x9E`\0a\x1E\xE8`\xA0\x87\x01\x87aJ\xA8V[\x85\x81\x81\x10a\x1E\xF8Wa\x1E\xF8aJpV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta\x1F\x1D\x90aL.V[\x81\x10a\x1F+Wa\x1F+aJpV[\x81T`\x01\x16\x15a\x1FJW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x1F\x7F\x81aK\x07V[\x92PPa\x1E\xC1V[P`\xA0`\0a\x1F\x98\x85\x83\x01\x86aJ\xA8V[\x84\x81\x81\x10a\x1F\xA8Wa\x1F\xA8aJpV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x1F\xD3\x81aK\x07V[\x91PPa\x1E\xA8V[P`\0[a\x1F\xEC`\xC0\x84\x01\x84aJ\xA8V[\x90P\x81\x10\x15a\"<Wa \x02`\xC0\x84\x01\x84aJ\xA8V[\x82\x81\x81\x10a \x12Wa \x12aJpV[\x90P` \x02\x81\x01\x90a $\x91\x90aLcV[a 5\x90`\x80\x81\x01\x90``\x01a?\xF0V[`\xA0`\0a F`\xC0\x87\x01\x87aJ\xA8V[\x85\x81\x81\x10a VWa VaJpV[\x90P` \x02\x81\x01\x90a h\x91\x90aLcV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a \xA2`\xC0\x85\x01\x85aJ\xA8V[\x83\x81\x81\x10a \xB2Wa \xB2aJpV[\x90P` \x02\x81\x01\x90a \xC4\x91\x90aLcV[a \xD2\x90` \x81\x01\x90aJ\xA8V[\x90P\x81\x10\x15a\")Wa \xE8`\xC0\x85\x01\x85aJ\xA8V[\x83\x81\x81\x10a \xF8Wa \xF8aJpV[\x90P` \x02\x81\x01\x90a!\n\x91\x90aLcV[a!\x18\x90`@\x81\x01\x90aJ\xA8V[\x82\x81\x81\x10a!(Wa!(aJpV[\x90P` \x02\x01` \x81\x01\x90a!=\x91\x90aK\x82V[`\x9E`\0a!N`\xC0\x88\x01\x88aJ\xA8V[\x86\x81\x81\x10a!^Wa!^aJpV[\x90P` \x02\x81\x01\x90a!p\x91\x90aLcV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a!\x91`\xC0\x88\x01\x88aJ\xA8V[\x86\x81\x81\x10a!\xA1Wa!\xA1aJpV[\x90P` \x02\x81\x01\x90a!\xB3\x91\x90aLcV[a!\xC1\x90` \x81\x01\x90aJ\xA8V[\x85\x81\x81\x10a!\xD1Wa!\xD1aJpV[\x90P` \x02\x01` \x81\x01\x90a!\xE6\x91\x90a?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\"!\x81aK\x07V[\x91PPa \x95V[P\x80a\"4\x81aK\x07V[\x91PPa\x1F\xDFV[P`\0[a\"M`\xE0\x84\x01\x84aJ\xA8V[\x90P\x81\x10\x15a$\0W`\0[a\"f`\xE0\x85\x01\x85aJ\xA8V[\x83\x81\x81\x10a\"vWa\"vaJpV[\x90P` \x02\x81\x01\x90a\"\x88\x91\x90aL\x83V[a\"\x96\x90` \x81\x01\x90aJ\xA8V[\x90P\x81\x10\x15a#\xEDWa\"\xAC`\xE0\x85\x01\x85aJ\xA8V[\x83\x81\x81\x10a\"\xBCWa\"\xBCaJpV[\x90P` \x02\x81\x01\x90a\"\xCE\x91\x90aL\x83V[a\"\xDC\x90`@\x81\x01\x90aJ\xA8V[\x82\x81\x81\x10a\"\xECWa\"\xECaJpV[\x90P` \x02\x01` \x81\x01\x90a#\x01\x91\x90aK\x82V[`\x9E`\0a#\x12`\xE0\x88\x01\x88aJ\xA8V[\x86\x81\x81\x10a#\"Wa#\"aJpV[\x90P` \x02\x81\x01\x90a#4\x91\x90aL\x83V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a#U`\xE0\x88\x01\x88aJ\xA8V[\x86\x81\x81\x10a#eWa#eaJpV[\x90P` \x02\x81\x01\x90a#w\x91\x90aL\x83V[a#\x85\x90` \x81\x01\x90aJ\xA8V[\x85\x81\x81\x10a#\x95Wa#\x95aJpV[\x90P` \x02\x01` \x81\x01\x90a#\xAA\x91\x90a?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a#\xE5\x81aK\x07V[\x91PPa\"YV[P\x80a#\xF8\x81aK\x07V[\x91PPa\"@V[P`\0[a$\x12a\x01\0\x84\x01\x84aK\x9DV[\x90P\x81\x10\x15a$\xB6Wa$)a\x01\0\x84\x01\x84aK\x9DV[\x82\x81\x81\x10a$9Wa$9aJpV[\x90P`@\x02\x01` \x01` \x81\x01\x90a$Q\x91\x90a?\xF0V[`\xA0`\0a$ca\x01\0\x87\x01\x87aK\x9DV[\x85\x81\x81\x10a$sWa$saJpV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a$\xAE\x90aK\x07V[\x91PPa$\x04V[Pa$\xC4` \x86\x01\x86aL\x99V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua$\xFB`@\x86\x01` \x87\x01aL\x99V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua%1``\x86\x01\x86aL\xB4V[a%=\x91`\x9B\x91a>lV[Pa%N`\xA0\x86\x01`\x80\x87\x01aL\x99V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F6\xA1\xFD{\xD5T\xF5\xC4(\xC9\x82\x9C\t\xC6`kL\x89;\x1F\xAD\xC8sZz\x12yW\x97D}\xEDa%\x96` \x87\x01\x87aL\x99V[a%\xA6`@\x88\x01` \x89\x01aL\x99V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&9\x91\x90aJ\x0BV[a&UW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90aJ(V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a&\x9Ca:+V[a&\xA6`\0a;oV[V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9B\x80Ta&\xDF\x90aL.V[\x90P\x90Pa'\0`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a'\x18Wa'\x18a@VV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'AW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a'_Wa'_a@VV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'\x88W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x85\x01QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xABWa'\xABa@VV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'\xD4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87` \x01QQ\x81\x10\x15a)\x86Wa( \x88` \x01Q\x82\x81Q\x81\x10a(\x01Wa(\x01aJpV[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a(2Wa(2aJpV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a(\xFCW\x82a(O`\x01\x83aT\xDAV[\x81Q\x81\x10a(_Wa(_aJpV[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a(|Wa(|aJpV[` \x02` \x01\x01Q`\0\x1C\x11a(\xFCW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x06@V[a)ra)k`\xA0`\0\x86\x85\x81Q\x81\x10a)\x18Wa)\x18aJpV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x8A` \x01Q\x84\x81Q\x81\x10a)UWa)UaJpV[` \x02` \x01\x01Qa;\xC1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a6#V[\x95P\x80a)~\x81aK\x07V[\x91PPa'\xDBV[Pa)\x90\x85a<\xA5V[\x94P`\0[\x84\x81\x10\x15a+tW`\x9B\x81\x81Ta)\xAB\x90aL.V[\x81\x10a)\xB9Wa)\xB9aJpV[\x81T`\x01\x16\x15a)\xD8W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\x9F` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa*\x19\x90\x87\x90a6#V[`\xFF\x83\x16`\0\x90\x81R`\x9D` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a*TWa*TaJpV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a*\x80Wa*\x80aJpV[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a*\x9EWa*\x9EaJpV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88` \x01QQ\x81\x10\x15a+aW`\x9E`\0\x85\x83\x81Q\x81\x10a*\xE4Wa*\xE4aJpV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a+/Wa+/aJpV[` \x02` \x01\x01\x81\x81Qa+C\x91\x90aT\xF1V[`\x01`\x01``\x1B\x03\x16\x90RP\x80a+Y\x81aK\x07V[\x91PPa*\xC1V[P\x80a+l\x81aK\x07V[\x91PPa)\x95V[P`\0\x80a+\x8C\x8A\x88\x8B``\x01Q\x8C`\x80\x01Qa\x07\x94V[\x91P\x91P\x81a,\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06@V[\x80a,\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06@V[P\x92\x95PPPPPP[\x92\x91PPV[a,\x9Aa:+V[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a-\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\x97T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a-6`\x80\x84\x01``\x85\x01aL\x99V[c\xFF\xFF\xFF\xFF\x16\x14a-\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FchainRdBatchNonce mismatch\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[a-\x99`@\x84\x01` \x85\x01aU\x19V[`\x01\x81\x11\x15a-\xAAWa-\xAAaH\xDCV[`\x97T`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a-\xC8Wa-\xC8aH\xDCV[\x14a.\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x15\xDC\x9B\xDB\x99\xC8\x18\xDA\x18Z[\x92Y`\x9A\x1B`D\x82\x01R`d\x01a\x06@V[`\x9ATc\xFF\xFF\xFF\xFF\x16\x15\x80a.1WPa.\"` \x84\x01\x84aL\x99V[`\x9ATc\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x10[a.lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkStale RdTask`\xA0\x1B`D\x82\x01R`d\x01a\x06@V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a.\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x1C\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x06@V[a.\xCA`\xA0\x84\x01`\x80\x85\x01aL\x99V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a/+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x06@V[\x82`@Q` \x01a/<\x91\x90aU4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82` \x015\x14a/\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x06@V[`\0a/\xDA\x83`@Q` \x01a/\xBA\x91\x90aU\xE6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83a\x04\x88\x90aT\x80V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta/\xF6\x90aL.V[\x90P\x81\x10\x15a0\xBFW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a0\x19Wa0\x19aJpV[` \x02` \x01\x01Qa0+\x91\x90aT\x8CV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a0LWa0LaJpV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a0g\x91\x90aT\xBBV[\x10\x15a0\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtFailed to meet quorum`X\x1B`D\x82\x01R`d\x01a\x06@V[\x80a0\xB7\x81aK\x07V[\x91PPa/\xE9V[P`@\x80Q\x80\x82\x01\x82R`\xA0\x86\x015\x81R`\xC0\x86\x015` \x82\x01\x90\x81R`\x97T\x92Qb#\xD0\xB5`\xE6\x1B\x81R`\x80\x88\x015`\x04\x82\x01R\x82Q`$\x82\x01R\x90Q`D\x82\x01R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x08\xF4-@\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a11W`\0\x80\xFD[PZ\xF1\x15\x80\x15a1EW=`\0\x80>=`\0\xFD[Pa1Z\x92PPP`\x80\x86\x01``\x87\x01aL\x99V[a1e\x90`\x01aM\x18V[`\x97\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua1\x97` \x87\x01\x87aL\x99V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F\xECh\xDB9\x18y\xB0\xF9\xF4 \xD1\xCD\xF3Gj\xFB\xDF\x08Z$b\xBFM+\x11\xDFxFb\x95\xCB\x17a1\xDF` \x88\x01\x88aL\x99V[a1\xEF`\x80\x89\x01``\x8A\x01aL\x99V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[a2\x1Fa:+V[`\x01`\x01`\xA0\x1B\x03\x81\x16a2\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06@V[a\x06R\x81a;oV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x04\x91\x90aI\xA4V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a34W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90aI\xC1V[`fT\x19\x81\x19`fT\x19\x16\x14a3\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06@V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\x89V[`fT\x15a49W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x06@V[a4Aa:+V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F/ \xCF\x1B\xDAgs\x90D\xC5\xBFWsS\x97\x0C=\xBC\x18;,rt\xD1\xE8XJ\x10&\x922g\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01`\x01`\xA0\x1B\x03\x81\x16a5#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06@V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra5\xA8a>\xF0V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a5\xDBWa5\xDDV[\xFE[P\x80a6\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06@V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra6?a?\x0EV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a5\xDBWP\x80a6\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06@V[a6\xBFa?,V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a7\xA7`\0\x80Q` aV\x92\x839\x81Q\x91R\x86aJ\x86V[\x90P[a7\xB3\x81a=@V[\x90\x93P\x91P`\0\x80Q` aV\x92\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a7\xEDW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aV\x92\x839\x81Q\x91R`\x01\x82\x08\x90Pa7\xAAV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a89a?QV[`\0[`\x02\x81\x10\x15a9\xFEW`\0a8R\x82`\x06aT\xBBV[\x90P\x84\x82`\x02\x81\x10a8fWa8faJpV[` \x02\x01QQ\x83a8x\x83`\0aVyV[`\x0C\x81\x10a8\x88Wa8\x88aJpV[` \x02\x01R\x84\x82`\x02\x81\x10a8\x9FWa8\x9FaJpV[` \x02\x01Q` \x01Q\x83\x82`\x01a8\xB6\x91\x90aVyV[`\x0C\x81\x10a8\xC6Wa8\xC6aJpV[` \x02\x01R\x83\x82`\x02\x81\x10a8\xDDWa8\xDDaJpV[` \x02\x01QQQ\x83a8\xF0\x83`\x02aVyV[`\x0C\x81\x10a9\0Wa9\0aJpV[` \x02\x01R\x83\x82`\x02\x81\x10a9\x17Wa9\x17aJpV[` \x02\x01QQ`\x01` \x02\x01Q\x83a90\x83`\x03aVyV[`\x0C\x81\x10a9@Wa9@aJpV[` \x02\x01R\x83\x82`\x02\x81\x10a9WWa9WaJpV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a9rWa9raJpV[` \x02\x01Q\x83a9\x83\x83`\x04aVyV[`\x0C\x81\x10a9\x93Wa9\x93aJpV[` \x02\x01R\x83\x82`\x02\x81\x10a9\xAAWa9\xAAaJpV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a9\xC5Wa9\xC5aJpV[` \x02\x01Q\x83a9\xD6\x83`\x05aVyV[`\x0C\x81\x10a9\xE6Wa9\xE6aJpV[` \x02\x01RP\x80a9\xF6\x81aK\x07V[\x91PPa8<V[Pa:\x07a?pV[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a&\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06@V[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a:\xA6WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a;(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06@V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a;k\x82a4\x95V[PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a<\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x06@V[\x81a\xFF\xFF\x16`\x01\x14\x15a<1WP\x81a,\x8CV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a<\x9AW`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a<}Wa<z\x84\x84a6#V[\x93P[a<\x87\x83\x84a6#V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a<MV[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a<\xCAWP` \x82\x01Q\x15[\x15a<\xE8WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aV\x92\x839\x81Q\x91R\x84` \x01Qa=\x1B\x91\x90aJ\x86V[a=3\x90`\0\x80Q` aV\x92\x839\x81Q\x91RaT\xDAV[\x90R\x92\x91PPV[\x91\x90PV[`\0\x80\x80`\0\x80Q` aV\x92\x839\x81Q\x91R`\x03`\0\x80Q` aV\x92\x839\x81Q\x91R\x86`\0\x80Q` aV\x92\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a=\xB6\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aV\x92\x839\x81Q\x91Ra=\xC2V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a=\xCDa?pV[a=\xD5a?\x8EV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a5\xDBWP\x82a>_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[PQ\x91PP[\x93\x92PPPV[\x82\x80Ta>x\x90aL.V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a>\x9AW`\0\x85Ua>\xE0V[\x82`\x1F\x10a>\xB3W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua>\xE0V[\x82\x80\x01`\x01\x01\x85U\x82\x15a>\xE0W\x91\x82\x01[\x82\x81\x11\x15a>\xE0W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a>\xC5V[Pa>\xEC\x92\x91Pa?\xACV[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a??a?\xC1V[\x81R` \x01a?La?\xC1V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a>\xECW`\0\x81U`\x01\x01a?\xADV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a=;W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a@\x02W`\0\x80\xFD[a>e\x82a?\xDFV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06RW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a@2W`\0\x80\xFD[\x815a>e\x81a@\x0BV[`\0` \x82\x84\x03\x12\x15a@OW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@\x8EWa@\x8Ea@VV[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@\x8EWa@\x8Ea@VV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@\xDFWa@\xDFa@VV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a@\xF9W`\0\x80\xFD[aA\x01a@lV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aA(W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15aAJWaAJa@VV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15aAaW`\0\x80\xFD[\x84[\x81\x81\x10\x15a<\x9AW\x805\x83R` \x92\x83\x01\x92\x01aAcV[`\0`\x80\x82\x84\x03\x12\x15aA\x8DW`\0\x80\xFD[aA\x95a@lV[\x90PaA\xA1\x83\x83aA\x17V[\x81RaA\xB0\x83`@\x84\x01aA\x17V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15aA\xD2W`\0\x80\xFD[\x845\x93PaA\xE3\x86` \x87\x01a@\xE7V[\x92PaA\xF2\x86``\x87\x01aA{V[\x91PaB\x01\x86`\xE0\x87\x01a@\xE7V[\x90P\x92\x95\x91\x94P\x92PV[`\0`\xE0\x82\x84\x03\x12\x15aB\x1EW`\0\x80\xFD[P\x91\x90PV[`\0a\x01 \x82\x84\x03\x12\x15aB\x1EW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12aBIW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aB`W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15aB{W`\0\x80\xFD[\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a=;W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15aB\xB1W`\0\x80\xFD[\x875`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aB\xC8W`\0\x80\xFD[aB\xD4\x8B\x83\x8C\x01aB\x0CV[\x98P` \x8A\x015\x91P\x80\x82\x11\x15aB\xEAW`\0\x80\xFD[aB\xF6\x8B\x83\x8C\x01aB$V[\x97P`@\x8A\x015\x91P\x80\x82\x11\x15aC\x0CW`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12aC W`\0\x80\xFD[\x815\x81\x81\x11\x15aC/W`\0\x80\xFD[\x8B` \x82`\x05\x1B\x85\x01\x01\x11\x15aCDW`\0\x80\xFD[` \x83\x01\x97P\x80\x96PP``\x8A\x015\x91P\x80\x82\x11\x15aCbW`\0\x80\xFD[PaCo\x8A\x82\x8B\x01aB7V[\x90\x94P\x92PaC\x82\x90P`\x80\x89\x01aB\x82V[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15aC\xBDW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01aC\xA1V[\x81\x81\x11\x15aC\xCFW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[\x80\x15\x15\x81\x14a\x06RW`\0\x80\xFD[\x805a=;\x81aC\xE5V[\x805`\x02\x81\x10a=;W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aD&W`\0\x80\xFD[\x865aD1\x81a@\x0BV[\x95P` \x87\x015aDA\x81a@\x0BV[\x94P`@\x87\x015aDQ\x81a@\x0BV[\x93P``\x87\x015aDa\x81aC\xE5V[\x92P`\x80\x87\x015aDq\x81a@\x0BV[\x91PaD\x7F`\xA0\x88\x01aC\xFEV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0a\x01\x80\x82\x84\x03\x12\x15aB\x1EW`\0\x80\xFD[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15aD\xB5W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aD\xCCW`\0\x80\xFD[aD\xD8\x89\x83\x8A\x01aB\x0CV[\x96P```\x1F\x19\x84\x01\x12\x15aD\xECW`\0\x80\xFD[` \x88\x01\x95P`\x80\x88\x015\x92P\x80\x83\x11\x15aE\x06W`\0\x80\xFD[aE\x12\x89\x84\x8A\x01aD\x8BV[\x94P`\xA0\x88\x015\x92P\x80\x83\x11\x15aE(W`\0\x80\xFD[PPaE6\x87\x82\x88\x01aB$V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15aEUW`\0\x80\xFD[\x825\x91PaEe` \x84\x01a?\xDFV[\x90P\x92P\x92\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aE\x87WaE\x87a@VV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aE\xA2W`\0\x80\xFD[\x815` aE\xB7aE\xB2\x83aEnV[a@\xB7V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aE\xD6W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aE\xF8WaE\xEB\x81aB\x82V[\x83R\x91\x83\x01\x91\x83\x01aE\xDAV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aF\x14W`\0\x80\xFD[\x815` aF$aE\xB2\x83aEnV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aFCW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aE\xF8WaFY\x88\x82a@\xE7V[\x83R\x91\x83\x01\x91`@\x01aFGV[`\0\x82`\x1F\x83\x01\x12aFxW`\0\x80\xFD[\x815` aF\x88aE\xB2\x83aEnV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aF\xA7W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aE\xF8W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xCAW`\0\x80\x81\xFD[aF\xD8\x89\x86\x83\x8B\x01\x01aE\x91V[\x84RP\x91\x83\x01\x91\x83\x01aF\xABV[`\0a\x01\x80\x82\x84\x03\x12\x15aF\xF9W`\0\x80\xFD[aG\x01a@\x94V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aG\x1AW`\0\x80\xFD[aG&\x85\x83\x86\x01aE\x91V[\x83R` \x84\x015\x91P\x80\x82\x11\x15aG<W`\0\x80\xFD[aGH\x85\x83\x86\x01aF\x03V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15aGaW`\0\x80\xFD[aGm\x85\x83\x86\x01aF\x03V[`@\x84\x01RaG\x7F\x85``\x86\x01aA{V[``\x84\x01RaG\x91\x85`\xE0\x86\x01a@\xE7V[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15aG\xABW`\0\x80\xFD[aG\xB7\x85\x83\x86\x01aE\x91V[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15aG\xD1W`\0\x80\xFD[aG\xDD\x85\x83\x86\x01aE\x91V[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15aG\xF7W`\0\x80\xFD[PaH\x04\x84\x82\x85\x01aFgV[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aH#W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aH@W`\0\x80\xFD[aHL\x85\x82\x86\x01aF\xE6V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aH\x8FW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aHjV[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01RaH\xB6``\x84\x01\x82aHVV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01RaH\xD3\x82\x82aHVV[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10aI\x10WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a,\x8C\x82\x84aH\xF2V[`\0\x80`\0\x83\x85\x03a\x01@\x81\x12\x15aI9W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aIPW`\0\x80\xFD[aI\\\x88\x83\x89\x01aB\x0CV[\x95Pa\x01\0`\x1F\x19\x84\x01\x12\x15aIqW`\0\x80\xFD[` \x87\x01\x94Pa\x01 \x87\x015\x92P\x80\x83\x11\x15aI\x8CW`\0\x80\xFD[PPaI\x9A\x86\x82\x87\x01aD\x8BV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15aI\xB6W`\0\x80\xFD[\x81Qa>e\x81a@\x0BV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aJ\x1DW`\0\x80\xFD[\x81Qa>e\x81aC\xE5V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82aJ\xA3WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aJ\xBFW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aJ\xD9W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aB{W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aK\x1BWaK\x1BaJ\xF1V[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aK9W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aKSW`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15aB{W`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a=;W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aK\x94W`\0\x80\xFD[a>e\x82aKkV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aK\xB4W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aK\xCEW`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15aB{W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aK\xFDW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\x17W`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15aB{W`\0\x80\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80aLBW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15aB\x1EWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aLyW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aLyW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aL\xABW`\0\x80\xFD[a>e\x82aB\x82V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aL\xCBW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\xE5W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aB{W`\0\x80\xFD[\x82\x81R``\x81\x01a>e` \x83\x01\x84\x805\x82R` \x90\x81\x015\x91\x01RV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aM7WaM7aJ\xF1V[\x01\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aMWW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aMvW`\0\x80\xFD[\x806\x03\x83\x13\x15aB{W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aM\xC3\x85aB\x82V[\x16` \x84\x01R\x80aM\xD6` \x86\x01aB\x82V[\x16`@\x84\x01R\x80aM\xE9`@\x86\x01aB\x82V[\x16``\x84\x01RPaM\xFD``\x84\x01\x84aM@V[`\xE0`\x80\x85\x01RaN\x13a\x01\0\x85\x01\x82\x84aM\x85V[\x91PPaN\"`\x80\x85\x01aB\x82V[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x85\x01RPaN<`\xA0\x85\x01\x85aM@V[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01RaNS\x83\x82\x84aM\x85V[\x92PPPaNc`\xC0\x85\x01aB\x82V[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aN\x8EW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xADW`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15aB{W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aH\x8FW`\xFFaN\xE2\x83a?\xDFV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aN\xCFV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aO\x0CW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aO+W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15aB{W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aH\x8FW`\xFFaO`\x83a?\xDFV[\x16\x87R`\x01`\x01``\x1B\x03aOv\x84\x84\x01aKkV[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aOMV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aO\xB8W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xD7W`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15aB{W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aH\x8FW`\xFFaP\x0C\x83a?\xDFV[\x16\x87R`\x01`\x01``\x1B\x03aP\"\x84\x84\x01aKkV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aO\xF9V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aPQW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aPpW`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15aB{W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aH\x8FW`\xFFaP\xA5\x83a?\xDFV[\x16\x87RaP\xC0\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aP\x92V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aP\xECW`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aH\x8FW`\x01`\x01``\x1B\x03aQ2\x83aKkV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aQ\x19V[`\0\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84`\0\x80[\x88\x81\x10\x15aQ\xFAW\x85\x84\x03`\x1F\x19\x01\x8AR\x8256\x89\x90\x03`~\x19\x01\x81\x12aQ\x84W\x82\x83\xFD[\x88\x01\x805\x85R`\x80aQ\x98\x87\x83\x01\x83aNwV[\x82\x89\x89\x01RaQ\xAA\x83\x89\x01\x82\x84aN\xBFV[\x92PPP`@aQ\xBC\x81\x84\x01\x84aNwV[\x88\x84\x03\x83\x8A\x01RaQ\xCE\x84\x82\x84aQ\tV[\x93PPPP```\xFFaQ\xE2\x82\x85\x01a?\xDFV[\x16\x96\x01\x95\x90\x95RP\x98\x84\x01\x98\x91\x84\x01\x91`\x01\x01aQ_V[P\x91\x98\x97PPPPPPPPV[`\0\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84`\0\x80[\x88\x81\x10\x15aQ\xFAW\x85\x84\x03`\x1F\x19\x01\x8AR\x8256\x89\x90\x03`^\x19\x01\x81\x12aRGW\x82\x83\xFD[\x88\x01\x805\x85R``aR[\x87\x83\x01\x83aNwV[\x82\x89\x89\x01RaRm\x83\x89\x01\x82\x84aN\xBFV[\x92PPP`@aR\x7F\x81\x84\x01\x84aNwV[\x93P\x87\x83\x03\x82\x89\x01RaR\x93\x83\x85\x83aQ\tV[\x9D\x89\x01\x9D\x97PPP\x93\x86\x01\x93PP`\x01\x01aR\"V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aH\x8FW\x815\x87R`\xFFaR\xD2\x84\x84\x01a?\xDFV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aR\xB9V[` \x81RaS\x04` \x82\x01aR\xFE\x84aC\xF3V[\x15\x15\x90RV[`\0aS\x13` \x84\x01\x84aNwV[a\x01 \x80`@\x86\x01RaS+a\x01@\x86\x01\x83\x85aN\xBFV[\x92PaS:`@\x87\x01\x87aN\xF5V[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaST\x85\x85\x84aO=V[\x94PaSc``\x89\x01\x89aO\xA1V[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaS|\x85\x85\x84aO\xE9V[\x94PaS\x8B`\x80\x89\x01\x89aP:V[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaS\xA4\x85\x85\x84aP\x82V[\x94PaS\xB3`\xA0\x89\x01\x89aNwV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaS\xCC\x85\x85\x84aP\xD3V[\x94PaS\xDB`\xC0\x89\x01\x89aNwV[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaS\xF4\x85\x85\x84aQEV[\x94PaT\x03`\xE0\x89\x01\x89aNwV[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaT\x1E\x86\x86\x85aR\x08V[\x95PaT,\x81\x8A\x01\x8AaO\xA1V[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaTF\x84\x84\x83aR\xA9V[\x97\x96PPPPPPPV[``\x81\x01c\xFF\xFF\xFF\xFFaTc\x84aB\x82V[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R\x92\x91PPV[`\0a,\x8C6\x83aF\xE6V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aT\xB2WaT\xB2aJ\xF1V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aT\xD5WaT\xD5aJ\xF1V[P\x02\x90V[`\0\x82\x82\x10\x15aT\xECWaT\xECaJ\xF1V[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aU\x11WaU\x11aJ\xF1V[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aU+W`\0\x80\xFD[a>e\x82aC\xFEV[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aUI\x85aB\x82V[\x16` \x84\x01RaU[` \x85\x01aC\xFEV[aUh`@\x85\x01\x82aH\xF2V[P\x80aUv`@\x86\x01aB\x82V[\x16``\x84\x01R\x80aU\x89``\x86\x01aB\x82V[\x16`\x80\x84\x01R\x80aU\x9C`\x80\x86\x01aB\x82V[\x16`\xA0\x84\x01RaU\xAF`\xA0\x85\x01\x85aM@V[`\xE0`\xC0\x86\x01RaU\xC5a\x01\0\x86\x01\x82\x84aM\x85V[\x91PP\x81aU\xD5`\xC0\x87\x01aB\x82V[\x16`\xE0\x85\x01R\x80\x92PPP\x92\x91PPV[a\x01\0\x81\x01c\xFF\xFF\xFF\xFF\x80aU\xFA\x85aB\x82V[\x16\x83R` \x84\x015` \x84\x01RaV\x13`@\x85\x01aC\xFEV[aV `@\x85\x01\x82aH\xF2V[P\x80aV.``\x86\x01aB\x82V[\x16``\x84\x01RP`\x80\x83\x015`\x80\x83\x01R`\xA0\x83\x015`\xA0\x83\x01R`\xC0\x83\x015`\xC0\x83\x01R`\xE0\x83\x015aVa\x81a@\x0BV[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x92\x90\x92\x01\x91\x90\x91R\x91\x90PV[`\0\x82\x19\x82\x11\x15aV\x8CWaV\x8CaJ\xF1V[P\x01\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 U\xE5:\xF2\xBAFon\x13\xA0o9\x9980\x08\xD4\xD8\x85\xE9\xE5\xB0\x1B\xF6\xCA;\x1Au\xCF\xC6\xA6\x19dsolcC\0\x08\x0C\x003";
    /// The bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICE_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__BYTECODE);
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x06W`\x005`\xE0\x1C\x80co\x0C0\xA4\x11a\x01\x1AW\x80c\xD0:\x07\xB2\x11a\0\xADW\x80c\xED9\xE5\x02\x11a\0|W\x80c\xED9\xE5\x02\x14a\x05DW\x80c\xF2\xFD\xE3\x8B\x14a\x05WW\x80c\xF8N\x91\xFC\x14a\x05jW\x80c\xFA\xBC\x1C\xBC\x14a\x05sW\x80c\xFD\xC1]\xE8\x14a\x05\x86W`\0\x80\xFD[\x80c\xD0:\x07\xB2\x14a\x04\xF2W\x80c\xDE\xB4\x03}\x14a\x05\x02W\x80c\xDF\x03L\xD0\x14a\x05\x19W\x80c\xE2\xA7\xCBf\x14a\x05,W`\0\x80\xFD[\x80c\x88o\x11\x95\x11a\0\xE9W\x80c\x88o\x11\x95\x14a\x04\x9AW\x80c\x8D\xA5\xCB[\x14a\x04\xADW\x80c\x9A\x8A\x05\x92\x14a\x04\xBEW\x80c\x9DT\xF4\x19\x14a\x04\xDFW`\0\x80\xFD[\x80co\x0C0\xA4\x14a\x042W\x80cqP\x18\xA6\x14a\x04IW\x80cz\xD7Ua\x14a\x04QW\x80c}\x97\x88\x97\x14a\x04zW`\0\x80\xFD[\x80c4\xFA\xDB\xEA\x11a\x01\x9DW\x80cM\xEA\xBC!\x11a\x01lW\x80cM\xEA\xBC!\x14a\x03\xBCW\x80cRn>d\x14a\x03\xE1W\x80cY\\jg\x14a\x03\xF5W\x80cZ\xC8j\xB7\x14a\x03\xFDW\x80c\\\x97Z\xBB\x14a\x04 W`\0\x80\xFD[\x80c4\xFA\xDB\xEA\x14a\x02\xFDW\x80c=\x9F\xB0\x0C\x14a\x03\x10W\x80cC\r;9\x14a\x03;W\x80cI\x9Do\xB6\x14a\x03pW`\0\x80\xFD[\x80c\x17\x1F\x1D[\x11a\x01\xD9W\x80c\x17\x1F\x1D[\x14a\x02\x98W\x80c&5\xE7N\x14a\x02\xC2W\x80c*\x84\x14\xFD\x14a\x02\xD5W\x80c0\xC4}\x8E\x14a\x02\xEAW`\0\x80\xFD[\x80c\x03\xD0\x97\xD2\x14a\x02\x0BW\x80c\x0E\xE0\xFD\xBD\x14a\x02LW\x80c\x10\xD6z/\x14a\x02pW\x80c\x13d9\xDD\x14a\x02\x85W[`\0\x80\xFD[a\x022a\x02\x196`\x04a?\xF0V[`\x9F` R`\0\x90\x81R`@\x90 \x80T`\x01\x90\x91\x01T\x82V[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01[`@Q\x80\x91\x03\x90\xF3[`\x98Ta\x02`\x90`\x01`\xA8\x1B\x90\x04`\xFF\x16\x81V[`@Q\x90\x15\x15\x81R` \x01a\x02CV[a\x02\x83a\x02~6`\x04a@ V[a\x05\x99V[\0[a\x02\x83a\x02\x936`\x04a@=V[a\x06UV[a\x02\xABa\x02\xA66`\x04aA\xBBV[a\x07\x94V[`@\x80Q\x92\x15\x15\x83R\x90\x15\x15` \x83\x01R\x01a\x02CV[a\x02\x83a\x02\xD06`\x04aB\x96V[a\t\x1EV[a\x02\xDDa\x15\x18V[`@Qa\x02C\x91\x90aC\x90V[a\x02\x83a\x02\xF86`\x04aD\rV[a\x15\xA6V[a\x02\x83a\x03\x0B6`\x04aD\x9EV[a\x178V[`\x97Ta\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02CV[a\x03^a\x03I6`\x04a@=V[`\xA0` R`\0\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02CV[a\x03\xA4a\x03~6`\x04aEBV[`\x9E` \x90\x81R`\0\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\x01`\x01``\x1B\x03\x16\x81V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02CV[`\x9CTa\x03\xCC\x90c\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02CV[`\x98Ta\x02`\x90`\x01`\xA0\x1B\x90\x04`\xFF\x16\x81V[a\x02\x83a%\xCDV[a\x02`a\x04\x0B6`\x04a?\xF0V[`fT`\x01`\xFF\x90\x92\x16\x91\x90\x91\x1B\x90\x81\x16\x14\x90V[`fT[`@Q\x90\x81R` \x01a\x02CV[`\x9ATa\x03\xCC\x90`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\x83a&\x94V[a\x03\xA4a\x04_6`\x04a?\xF0V[`\x9D` R`\0\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x04\x8Da\x04\x886`\x04aH\x10V[a&\xA8V[`@Qa\x02C\x91\x90aH\x9AV[`eTa\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`3T`\x01`\x01`\xA0\x1B\x03\x16a\x03#V[`\x97Ta\x04\xD2\x90`\x01`\xC0\x1B\x90\x04`\xFF\x16\x81V[`@Qa\x02C\x91\x90aI\x14V[a\x02\x83a\x04\xED6`\x04a@ V[a,\x92V[`\x9ATa\x03\xCC\x90c\xFF\xFF\xFF\xFF\x16\x81V[`\x97Ta\x03\xCC\x90`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x98Ta\x03#\x90`\x01`\x01`\xA0\x1B\x03\x16\x81V[`\x9ATa\x03\xCC\x90d\x01\0\0\0\0\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[a\x02\x83a\x05R6`\x04aI\"V[a,\xBCV[a\x02\x83a\x05e6`\x04a@ V[a2\x17V[a\x04$`\x99T\x81V[a\x02\x83a\x05\x816`\x04a@=V[a2\x8DV[a\x02\x83a\x05\x946`\x04a@ V[a3\xE9V[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x05\xECW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\x10\x91\x90aI\xA4V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x06IW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90aI\xC1V[`@Q\x80\x91\x03\x90\xFD[a\x06R\x81a4\x95V[PV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x06\x9DW=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x06\xC1\x91\x90aJ\x0BV[a\x06\xDDW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90aJ(V[`fT\x81\x81\x16\x14a\x07VW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.pause: invalid attempt `D\x82\x01R\x7Fto unpause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06@V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01[`@Q\x80\x91\x03\x90\xA2PV[`\0\x80`\0\x7F0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X](3\xE8Hy\xB9p\x91C\xE1\xF5\x93\xF0\0\0\x01\x87\x87`\0\x01Q\x88` \x01Q\x88`\0\x01Q`\0`\x02\x81\x10a\x07\xDCWa\x07\xDCaJpV[` \x02\x01Q\x89Q`\x01` \x02\x01Q\x8A` \x01Q`\0`\x02\x81\x10a\x08\x01Wa\x08\x01aJpV[` \x02\x01Q\x8B` \x01Q`\x01`\x02\x81\x10a\x08\x1DWa\x08\x1DaJpV[` \x90\x81\x02\x91\x90\x91\x01Q\x8CQ\x8D\x83\x01Q`@Qa\x08z\x9A\x99\x98\x97\x96\x95\x94\x01\x98\x89R` \x89\x01\x97\x90\x97R`@\x88\x01\x95\x90\x95R``\x87\x01\x93\x90\x93R`\x80\x86\x01\x91\x90\x91R`\xA0\x85\x01R`\xC0\x84\x01R`\xE0\x83\x01Ra\x01\0\x82\x01Ra\x01 \x01\x90V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 `\0\x1Ca\x08\x9D\x91\x90aJ\x86V[\x90Pa\t\x10a\x08\xB6a\x08\xAF\x88\x84a5\x8CV[\x86\x90a6#V[a\x08\xBEa6\xB7V[a\t\x06a\x08\xF7\x85a\x08\xF1`@\x80Q\x80\x82\x01\x82R`\0\x80\x82R` \x91\x82\x01R\x81Q\x80\x83\x01\x90\x92R`\x01\x82R`\x02\x90\x82\x01R\x90V[\x90a5\x8CV[a\t\0\x8Ca7wV[\x90a6#V[\x88b\x01\xD4\xC0a8\x07V[\x90\x98\x90\x97P\x95PPPPPPV[a\t&a:+V[\x83\x82\x14a\tuW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7FrdUpdate info length mismatch\0\0\0`D\x82\x01R`d\x01a\x06@V[`\0[a\t\x85` \x88\x01\x88aJ\xA8V[\x90P\x81\x10\x15a\nJW`\x9D`\0a\t\x9F` \x8A\x01\x8AaJ\xA8V[\x84\x81\x81\x10a\t\xAFWa\t\xAFaJpV[\x90P` \x02\x01` \x81\x01\x90a\t\xC4\x91\x90a?\xF0V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\t\xF7\x90\x8A\x01\x8AaJ\xA8V[\x84\x81\x81\x10a\n\x07Wa\n\x07aJpV[\x90P` \x02\x01` \x81\x01\x90a\n\x1C\x91\x90a?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\nB\x81aK\x07V[\x91PPa\txV[P`\0[a\n[`@\x88\x01\x88aK\"V[\x90P\x81\x10\x15a\x0B\x96Wa\nq`@\x88\x01\x88aK\"V[\x82\x81\x81\x10a\n\x81Wa\n\x81aJpV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\n\x99\x91\x90aK\x82V[`\x9D`\0a\n\xAA`@\x8B\x01\x8BaK\"V[\x85\x81\x81\x10a\n\xBAWa\n\xBAaJpV[a\n\xD0\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x0B\x10\x90\x88\x01\x88aK\"V[\x82\x81\x81\x10a\x0B Wa\x0B aJpV[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x89\x80`@\x01\x90a\x0B=\x91\x90aK\"V[\x85\x81\x81\x10a\x0BMWa\x0BMaJpV[a\x0Bc\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x0B\x8E\x81aK\x07V[\x91PPa\nNV[P`\0[a\x0B\xA7``\x88\x01\x88aK\x9DV[\x90P\x81\x10\x15a\x0C_Wa\x0B\xBD``\x88\x01\x88aK\x9DV[\x82\x81\x81\x10a\x0B\xCDWa\x0B\xCDaJpV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x0B\xE5\x91\x90aK\x82V[`\x9D`\0a\x0B\xF6``\x8B\x01\x8BaK\x9DV[\x85\x81\x81\x10a\x0C\x06Wa\x0C\x06aJpV[a\x0C\x1C\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x0CW\x81aK\x07V[\x91PPa\x0B\x9AV[P`\0[a\x0Cp`\x80\x88\x01\x88aK\xE6V[\x90P\x81\x10\x15a\r\x0CWa\x0C\x86`\x80\x88\x01\x88aK\xE6V[\x82\x81\x81\x10a\x0C\x96Wa\x0C\x96aJpV[\x90P``\x02\x01` \x01`\x9F`\0\x89\x80`\x80\x01\x90a\x0C\xB3\x91\x90aK\xE6V[\x85\x81\x81\x10a\x0C\xC3Wa\x0C\xC3aJpV[a\x0C\xD9\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\r\x04\x81aK\x07V[\x91PPa\x0CcV[P`\0[a\r\x1D`\xA0\x88\x01\x88aJ\xA8V[\x90P\x81\x10\x15a\x0ECW`\0[`\x9B\x80Ta\r6\x90aL.V[\x90P\x81\x10\x15a\r\xEFW`\x9E`\0a\rP`\xA0\x8B\x01\x8BaJ\xA8V[\x85\x81\x81\x10a\r`Wa\r`aJpV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta\r\x85\x90aL.V[\x81\x10a\r\x93Wa\r\x93aJpV[\x81T`\x01\x16\x15a\r\xB2W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\r\xE7\x81aK\x07V[\x92PPa\r)V[P`\xA0`\0a\x0E\0\x89\x83\x01\x8AaJ\xA8V[\x84\x81\x81\x10a\x0E\x10Wa\x0E\x10aJpV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x0E;\x81aK\x07V[\x91PPa\r\x10V[P`\0[a\x0ET`\xC0\x88\x01\x88aJ\xA8V[\x90P\x81\x10\x15a\x10\xA4Wa\x0Ej`\xC0\x88\x01\x88aJ\xA8V[\x82\x81\x81\x10a\x0EzWa\x0EzaJpV[\x90P` \x02\x81\x01\x90a\x0E\x8C\x91\x90aLcV[a\x0E\x9D\x90`\x80\x81\x01\x90``\x01a?\xF0V[`\xA0`\0a\x0E\xAE`\xC0\x8B\x01\x8BaJ\xA8V[\x85\x81\x81\x10a\x0E\xBEWa\x0E\xBEaJpV[\x90P` \x02\x81\x01\x90a\x0E\xD0\x91\x90aLcV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a\x0F\n`\xC0\x89\x01\x89aJ\xA8V[\x83\x81\x81\x10a\x0F\x1AWa\x0F\x1AaJpV[\x90P` \x02\x81\x01\x90a\x0F,\x91\x90aLcV[a\x0F:\x90` \x81\x01\x90aJ\xA8V[\x90P\x81\x10\x15a\x10\x91Wa\x0FP`\xC0\x89\x01\x89aJ\xA8V[\x83\x81\x81\x10a\x0F`Wa\x0F`aJpV[\x90P` \x02\x81\x01\x90a\x0Fr\x91\x90aLcV[a\x0F\x80\x90`@\x81\x01\x90aJ\xA8V[\x82\x81\x81\x10a\x0F\x90Wa\x0F\x90aJpV[\x90P` \x02\x01` \x81\x01\x90a\x0F\xA5\x91\x90aK\x82V[`\x9E`\0a\x0F\xB6`\xC0\x8C\x01\x8CaJ\xA8V[\x86\x81\x81\x10a\x0F\xC6Wa\x0F\xC6aJpV[\x90P` \x02\x81\x01\x90a\x0F\xD8\x91\x90aLcV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x0F\xF9`\xC0\x8C\x01\x8CaJ\xA8V[\x86\x81\x81\x10a\x10\tWa\x10\taJpV[\x90P` \x02\x81\x01\x90a\x10\x1B\x91\x90aLcV[a\x10)\x90` \x81\x01\x90aJ\xA8V[\x85\x81\x81\x10a\x109Wa\x109aJpV[\x90P` \x02\x01` \x81\x01\x90a\x10N\x91\x90a?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x10\x89\x81aK\x07V[\x91PPa\x0E\xFDV[P\x80a\x10\x9C\x81aK\x07V[\x91PPa\x0EGV[P`\0[a\x10\xB5`\xE0\x88\x01\x88aJ\xA8V[\x90P\x81\x10\x15a\x12hW`\0[a\x10\xCE`\xE0\x89\x01\x89aJ\xA8V[\x83\x81\x81\x10a\x10\xDEWa\x10\xDEaJpV[\x90P` \x02\x81\x01\x90a\x10\xF0\x91\x90aL\x83V[a\x10\xFE\x90` \x81\x01\x90aJ\xA8V[\x90P\x81\x10\x15a\x12UWa\x11\x14`\xE0\x89\x01\x89aJ\xA8V[\x83\x81\x81\x10a\x11$Wa\x11$aJpV[\x90P` \x02\x81\x01\x90a\x116\x91\x90aL\x83V[a\x11D\x90`@\x81\x01\x90aJ\xA8V[\x82\x81\x81\x10a\x11TWa\x11TaJpV[\x90P` \x02\x01` \x81\x01\x90a\x11i\x91\x90aK\x82V[`\x9E`\0a\x11z`\xE0\x8C\x01\x8CaJ\xA8V[\x86\x81\x81\x10a\x11\x8AWa\x11\x8AaJpV[\x90P` \x02\x81\x01\x90a\x11\x9C\x91\x90aL\x83V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a\x11\xBD`\xE0\x8C\x01\x8CaJ\xA8V[\x86\x81\x81\x10a\x11\xCDWa\x11\xCDaJpV[\x90P` \x02\x81\x01\x90a\x11\xDF\x91\x90aL\x83V[a\x11\xED\x90` \x81\x01\x90aJ\xA8V[\x85\x81\x81\x10a\x11\xFDWa\x11\xFDaJpV[\x90P` \x02\x01` \x81\x01\x90a\x12\x12\x91\x90a?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x12M\x81aK\x07V[\x91PPa\x10\xC1V[P\x80a\x12`\x81aK\x07V[\x91PPa\x10\xA8V[P`\0[a\x12za\x01\0\x88\x01\x88aK\x9DV[\x90P\x81\x10\x15a\x13\x1EWa\x12\x91a\x01\0\x88\x01\x88aK\x9DV[\x82\x81\x81\x10a\x12\xA1Wa\x12\xA1aJpV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x12\xB9\x91\x90a?\xF0V[`\xA0`\0a\x12\xCBa\x01\0\x8B\x01\x8BaK\x9DV[\x85\x81\x81\x10a\x12\xDBWa\x12\xDBaJpV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a\x13\x16\x90aK\x07V[\x91PPa\x12lV[Pa\x13,` \x88\x01\x88aL\x99V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua\x13c`@\x88\x01` \x89\x01aL\x99V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua\x13\x99``\x88\x01\x88aL\xB4V[a\x13\xA5\x91`\x9B\x91a>lV[Pa\x13\xB6`\xA0\x88\x01`\x80\x89\x01aL\x99V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U`\0[\x84\x81\x10\x15a\x14\x7FW`\x97T`\x01`\x01`\xA0\x1B\x03\x16c\x08\xF4-@\x87\x87\x84\x81\x81\x10a\x13\xFEWa\x13\xFEaJpV[\x90P` \x02\x015\x86\x86\x85\x81\x81\x10a\x14\x17Wa\x14\x17aJpV[\x90P`@\x02\x01`@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a\x14:\x92\x91\x90aL\xFAV[`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a\x14TW`\0\x80\xFD[PZ\xF1\x15\x80\x15a\x14hW=`\0\x80>=`\0\xFD[PPPP\x80\x80a\x14w\x90aK\x07V[\x91PPa\x13\xD3V[Pa\x14\x8B\x81`\x01aM\x18V[`\x97\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90U\x7F&Ie\xEBk\xC46\xC6\xC4sC\x1D4\xAFV\xE82\xEC4O\xDF\xD4>\xE6\xAFo\xCEm ^\x84\xAFa\x14\xDE` \x89\x01\x89aL\x99V[a\x14\xEE`@\x8A\x01` \x8B\x01aL\x99V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01[`@Q\x80\x91\x03\x90\xA1PPPPPPPV[`\x9B\x80Ta\x15%\x90aL.V[\x80`\x1F\x01` \x80\x91\x04\x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01\x82\x80Ta\x15Q\x90aL.V[\x80\x15a\x15\x9EW\x80`\x1F\x10a\x15sWa\x01\0\x80\x83T\x04\x02\x83R\x91` \x01\x91a\x15\x9EV[\x82\x01\x91\x90`\0R` `\0 \x90[\x81T\x81R\x90`\x01\x01\x90` \x01\x80\x83\x11a\x15\x81W\x82\x90\x03`\x1F\x16\x82\x01\x91[PPPPP\x81V[`\0Ta\x01\0\x90\x04`\xFF\x16\x15\x80\x80\x15a\x15\xC6WP`\0T`\x01`\xFF\x90\x91\x16\x10[\x80a\x15\xE0WP0;\x15\x80\x15a\x15\xE0WP`\0T`\xFF\x16`\x01\x14[a\x16CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`.`$\x82\x01R\x7FInitializable: contract is alrea`D\x82\x01Rm\x19\x1EH\x1A[\x9A]\x1AX[\x1A^\x99Y`\x92\x1B`d\x82\x01R`\x84\x01a\x06@V[`\0\x80T`\xFF\x19\x16`\x01\x17\x90U\x80\x15a\x16fW`\0\x80Ta\xFF\0\x19\x16a\x01\0\x17\x90U[a\x16q\x87`\0a:\x85V[a\x16z\x86a;oV[`\x98\x80T\x85\x15\x15`\x01`\xA8\x1B\x02`\x01a\xFF\x01`\xA0\x1B\x03\x19\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x80\x89\x16\x91\x90\x91\x17\x91\x90\x91\x17\x90\x91U`\x97\x80T\x91\x85\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x82U\x84\x92`\x01d\xFF\0\0\0\x01`\xA0\x1B\x03\x19\x16\x17`\x01`\xC0\x1B\x83`\x01\x81\x11\x15a\x16\xE8Wa\x16\xE8aH\xDCV[\x02\x17\x90UP\x80\x15a\x17/W`\0\x80Ta\xFF\0\x19\x16\x90U`@Q`\x01\x81R\x7F\x7F&\xB8?\xF9n\x1F+jh/\x138R\xF6y\x8A\t\xC4e\xDA\x95\x92\x14`\xCE\xFB8G@$\x98\x90` \x01a\x15\x07V[PPPPPPPV[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\x92W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16\x15\x80\x15a\x18DW`\x98T`\x01`\xA8\x1B\x90\x04`\xFF\x16\x15a\x18\x02W`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x06@V[a\x18\x86V[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x17\xFDW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd \xBA\xBA4\x19`\xD9\x1B`D\x82\x01R`d\x01a\x06@V[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a\x18\x86W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x05`$\x82\x01Rd\x04\x17WF\x83`\xDC\x1B`D\x82\x01R`d\x01a\x06@V[\x84`@Q` \x01a\x18\x97\x91\x90aM\xAEV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84` \x015\x14a\x18\xFFW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x06@V[\x81`@Q` \x01a\x19\x10\x91\x90aR\xEAV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x84`@\x015\x14a\x19xW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FoperatorStateInfo hash mismatch\0`D\x82\x01R`d\x01a\x06@V[\x80a\x1B\rWa\x19\x8D``\x86\x01`@\x87\x01aL\x99V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a\x19\xEEW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x18`$\x82\x01R\x7Freference block mismatch\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\0a\x1A$\x85`@Q` \x01a\x1A\x04\x91\x90aTQV[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x85a\x04\x88\x90aT\x80V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta\x1A@\x90aL.V[\x90P\x81\x10\x15a\x1B\tW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a\x1AcWa\x1AcaJpV[` \x02` \x01\x01Qa\x1Au\x91\x90aT\x8CV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a\x1A\x96Wa\x1A\x96aJpV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a\x1A\xB1\x91\x90aT\xBBV[\x10\x15a\x1A\xF7W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtFailed to meet quorum`X\x1B`D\x82\x01R`d\x01a\x06@V[\x80a\x1B\x01\x81aK\x07V[\x91PPa\x1A3V[PPP[`\0[a\x1B\x1D` \x84\x01\x84aJ\xA8V[\x90P\x81\x10\x15a\x1B\xE2W`\x9D`\0a\x1B7` \x86\x01\x86aJ\xA8V[\x84\x81\x81\x10a\x1BGWa\x1BGaJpV[\x90P` \x02\x01` \x81\x01\x90a\x1B\\\x91\x90a?\xF0V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0\x90\x81 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U`\x9F\x91a\x1B\x8F\x90\x86\x01\x86aJ\xA8V[\x84\x81\x81\x10a\x1B\x9FWa\x1B\x9FaJpV[\x90P` \x02\x01` \x81\x01\x90a\x1B\xB4\x91\x90a?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x81\x81U`\x01\x01U\x80a\x1B\xDA\x81aK\x07V[\x91PPa\x1B\x10V[P`\0[a\x1B\xF3`@\x84\x01\x84aK\"V[\x90P\x81\x10\x15a\x1D.Wa\x1C\t`@\x84\x01\x84aK\"V[\x82\x81\x81\x10a\x1C\x19Wa\x1C\x19aJpV[\x90P`\x80\x02\x01` \x01` \x81\x01\x90a\x1C1\x91\x90aK\x82V[`\x9D`\0a\x1CB`@\x87\x01\x87aK\"V[\x85\x81\x81\x10a\x1CRWa\x1CRaJpV[a\x1Ch\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x90\x81\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x93\x90\x93\x16\x92\x90\x92\x17\x90\x91Ua\x1C\xA8\x90\x84\x01\x84aK\"V[\x82\x81\x81\x10a\x1C\xB8Wa\x1C\xB8aJpV[\x90P`\x80\x02\x01`@\x01`\x9F`\0\x85\x80`@\x01\x90a\x1C\xD5\x91\x90aK\"V[\x85\x81\x81\x10a\x1C\xE5Wa\x1C\xE5aJpV[a\x1C\xFB\x92` `\x80\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x1D&\x81aK\x07V[\x91PPa\x1B\xE6V[P`\0[a\x1D?``\x84\x01\x84aK\x9DV[\x90P\x81\x10\x15a\x1D\xF7Wa\x1DU``\x84\x01\x84aK\x9DV[\x82\x81\x81\x10a\x1DeWa\x1DeaJpV[\x90P`@\x02\x01` \x01` \x81\x01\x90a\x1D}\x91\x90aK\x82V[`\x9D`\0a\x1D\x8E``\x87\x01\x87aK\x9DV[\x85\x81\x81\x10a\x1D\x9EWa\x1D\x9EaJpV[a\x1D\xB4\x92` `@\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\x1D\xEF\x81aK\x07V[\x91PPa\x1D2V[P`\0[a\x1E\x08`\x80\x84\x01\x84aK\xE6V[\x90P\x81\x10\x15a\x1E\xA4Wa\x1E\x1E`\x80\x84\x01\x84aK\xE6V[\x82\x81\x81\x10a\x1E.Wa\x1E.aJpV[\x90P``\x02\x01` \x01`\x9F`\0\x85\x80`\x80\x01\x90a\x1EK\x91\x90aK\xE6V[\x85\x81\x81\x10a\x1E[Wa\x1E[aJpV[a\x1Eq\x92` ``\x90\x92\x02\x01\x90\x81\x01\x91Pa?\xF0V[`\xFF\x16\x81R` \x80\x82\x01\x92\x90\x92R`@\x01`\0 \x825\x81U\x91\x015`\x01\x90\x91\x01U\x80a\x1E\x9C\x81aK\x07V[\x91PPa\x1D\xFBV[P`\0[a\x1E\xB5`\xA0\x84\x01\x84aJ\xA8V[\x90P\x81\x10\x15a\x1F\xDBW`\0[`\x9B\x80Ta\x1E\xCE\x90aL.V[\x90P\x81\x10\x15a\x1F\x87W`\x9E`\0a\x1E\xE8`\xA0\x87\x01\x87aJ\xA8V[\x85\x81\x81\x10a\x1E\xF8Wa\x1E\xF8aJpV[\x90P` \x02\x015\x81R` \x01\x90\x81R` \x01`\0 `\0`\x9B\x83\x81Ta\x1F\x1D\x90aL.V[\x81\x10a\x1F+Wa\x1F+aJpV[\x81T`\x01\x16\x15a\x1FJW\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16\x90U\x81a\x1F\x7F\x81aK\x07V[\x92PPa\x1E\xC1V[P`\xA0`\0a\x1F\x98\x85\x83\x01\x86aJ\xA8V[\x84\x81\x81\x10a\x1F\xA8Wa\x1F\xA8aJpV[` \x90\x81\x02\x92\x90\x92\x015\x83RP\x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\xFF\x19\x16\x90U\x80a\x1F\xD3\x81aK\x07V[\x91PPa\x1E\xA8V[P`\0[a\x1F\xEC`\xC0\x84\x01\x84aJ\xA8V[\x90P\x81\x10\x15a\"<Wa \x02`\xC0\x84\x01\x84aJ\xA8V[\x82\x81\x81\x10a \x12Wa \x12aJpV[\x90P` \x02\x81\x01\x90a $\x91\x90aLcV[a 5\x90`\x80\x81\x01\x90``\x01a?\xF0V[`\xA0`\0a F`\xC0\x87\x01\x87aJ\xA8V[\x85\x81\x81\x10a VWa VaJpV[\x90P` \x02\x81\x01\x90a h\x91\x90aLcV[`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP`\0[a \xA2`\xC0\x85\x01\x85aJ\xA8V[\x83\x81\x81\x10a \xB2Wa \xB2aJpV[\x90P` \x02\x81\x01\x90a \xC4\x91\x90aLcV[a \xD2\x90` \x81\x01\x90aJ\xA8V[\x90P\x81\x10\x15a\")Wa \xE8`\xC0\x85\x01\x85aJ\xA8V[\x83\x81\x81\x10a \xF8Wa \xF8aJpV[\x90P` \x02\x81\x01\x90a!\n\x91\x90aLcV[a!\x18\x90`@\x81\x01\x90aJ\xA8V[\x82\x81\x81\x10a!(Wa!(aJpV[\x90P` \x02\x01` \x81\x01\x90a!=\x91\x90aK\x82V[`\x9E`\0a!N`\xC0\x88\x01\x88aJ\xA8V[\x86\x81\x81\x10a!^Wa!^aJpV[\x90P` \x02\x81\x01\x90a!p\x91\x90aLcV[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a!\x91`\xC0\x88\x01\x88aJ\xA8V[\x86\x81\x81\x10a!\xA1Wa!\xA1aJpV[\x90P` \x02\x81\x01\x90a!\xB3\x91\x90aLcV[a!\xC1\x90` \x81\x01\x90aJ\xA8V[\x85\x81\x81\x10a!\xD1Wa!\xD1aJpV[\x90P` \x02\x01` \x81\x01\x90a!\xE6\x91\x90a?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a\"!\x81aK\x07V[\x91PPa \x95V[P\x80a\"4\x81aK\x07V[\x91PPa\x1F\xDFV[P`\0[a\"M`\xE0\x84\x01\x84aJ\xA8V[\x90P\x81\x10\x15a$\0W`\0[a\"f`\xE0\x85\x01\x85aJ\xA8V[\x83\x81\x81\x10a\"vWa\"vaJpV[\x90P` \x02\x81\x01\x90a\"\x88\x91\x90aL\x83V[a\"\x96\x90` \x81\x01\x90aJ\xA8V[\x90P\x81\x10\x15a#\xEDWa\"\xAC`\xE0\x85\x01\x85aJ\xA8V[\x83\x81\x81\x10a\"\xBCWa\"\xBCaJpV[\x90P` \x02\x81\x01\x90a\"\xCE\x91\x90aL\x83V[a\"\xDC\x90`@\x81\x01\x90aJ\xA8V[\x82\x81\x81\x10a\"\xECWa\"\xECaJpV[\x90P` \x02\x01` \x81\x01\x90a#\x01\x91\x90aK\x82V[`\x9E`\0a#\x12`\xE0\x88\x01\x88aJ\xA8V[\x86\x81\x81\x10a#\"Wa#\"aJpV[\x90P` \x02\x81\x01\x90a#4\x91\x90aL\x83V[5\x81R` \x81\x01\x91\x90\x91R`@\x01`\0\x90\x81 \x90a#U`\xE0\x88\x01\x88aJ\xA8V[\x86\x81\x81\x10a#eWa#eaJpV[\x90P` \x02\x81\x01\x90a#w\x91\x90aL\x83V[a#\x85\x90` \x81\x01\x90aJ\xA8V[\x85\x81\x81\x10a#\x95Wa#\x95aJpV[\x90P` \x02\x01` \x81\x01\x90a#\xAA\x91\x90a?\xF0V[`\xFF\x16\x81R` \x81\x01\x91\x90\x91R`@\x01`\0 \x80T`\x01`\x01``\x1B\x03\x19\x16`\x01`\x01``\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x80a#\xE5\x81aK\x07V[\x91PPa\"YV[P\x80a#\xF8\x81aK\x07V[\x91PPa\"@V[P`\0[a$\x12a\x01\0\x84\x01\x84aK\x9DV[\x90P\x81\x10\x15a$\xB6Wa$)a\x01\0\x84\x01\x84aK\x9DV[\x82\x81\x81\x10a$9Wa$9aJpV[\x90P`@\x02\x01` \x01` \x81\x01\x90a$Q\x91\x90a?\xF0V[`\xA0`\0a$ca\x01\0\x87\x01\x87aK\x9DV[\x85\x81\x81\x10a$sWa$saJpV[\x90P`@\x02\x01`\0\x015\x81R` \x01\x90\x81R` \x01`\0 `\0a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\xFF\x16\x02\x17\x90UP\x80\x80a$\xAE\x90aK\x07V[\x91PPa$\x04V[Pa$\xC4` \x86\x01\x86aL\x99V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16d\x01\0\0\0\0\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua$\xFB`@\x86\x01` \x87\x01aL\x99V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`@\x1B\x02c\xFF\xFF\xFF\xFF`@\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90UB`\x99Ua%1``\x86\x01\x86aL\xB4V[a%=\x91`\x9B\x91a>lV[Pa%N`\xA0\x86\x01`\x80\x87\x01aL\x99V[`\x9C\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F6\xA1\xFD{\xD5T\xF5\xC4(\xC9\x82\x9C\t\xC6`kL\x89;\x1F\xAD\xC8sZz\x12yW\x97D}\xEDa%\x96` \x87\x01\x87aL\x99V[a%\xA6`@\x88\x01` \x89\x01aL\x99V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPPPV[`eT`@Qc#}\xFBG`\xE1\x1B\x81R3`\x04\x82\x01R`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x90cF\xFB\xF6\x8E\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&9\x91\x90aJ\x0BV[a&UW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90aJ(V[`\0\x19`f\x81\x90U`@Q\x90\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2V[a&\x9Ca:+V[a&\xA6`\0a;oV[V[`@\x80Q\x80\x82\x01\x90\x91R``\x80\x82R` \x82\x01R`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R`\x9B\x80Ta&\xDF\x90aL.V[\x90P\x90Pa'\0`@Q\x80`@\x01`@R\x80``\x81R` \x01``\x81RP\x90V[\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a'\x18Wa'\x18a@VV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'AW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P` \x82\x01R\x81`\x01`\x01`@\x1B\x03\x81\x11\x15a'_Wa'_a@VV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'\x88W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x81R` \x85\x01QQ`\0\x90`\x01`\x01`@\x1B\x03\x81\x11\x15a'\xABWa'\xABa@VV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a'\xD4W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x80[\x87` \x01QQ\x81\x10\x15a)\x86Wa( \x88` \x01Q\x82\x81Q\x81\x10a(\x01Wa(\x01aJpV[` \x02` \x01\x01Q\x80Q`\0\x90\x81R` \x91\x82\x01Q\x90\x91R`@\x90 \x90V[\x83\x82\x81Q\x81\x10a(2Wa(2aJpV[` \x90\x81\x02\x91\x90\x91\x01\x01R\x80\x15a(\xFCW\x82a(O`\x01\x83aT\xDAV[\x81Q\x81\x10a(_Wa(_aJpV[` \x02` \x01\x01Q`\0\x1C\x83\x82\x81Q\x81\x10a(|Wa(|aJpV[` \x02` \x01\x01Q`\0\x1C\x11a(\xFCW`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: nonSignerPubkeys not sorted`d\x82\x01R`\x84\x01a\x06@V[a)ra)k`\xA0`\0\x86\x85\x81Q\x81\x10a)\x18Wa)\x18aJpV[` \x02` \x01\x01Q\x81R` \x01\x90\x81R` \x01`\0 `\0\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\xFF\x16\x8A` \x01Q\x84\x81Q\x81\x10a)UWa)UaJpV[` \x02` \x01\x01Qa;\xC1\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x87\x90a6#V[\x95P\x80a)~\x81aK\x07V[\x91PPa'\xDBV[Pa)\x90\x85a<\xA5V[\x94P`\0[\x84\x81\x10\x15a+tW`\x9B\x81\x81Ta)\xAB\x90aL.V[\x81\x10a)\xB9Wa)\xB9aJpV[\x81T`\x01\x16\x15a)\xD8W\x90`\0R` `\0 \x90` \x91\x82\x82\x04\x01\x91\x90\x06[\x90T`\x01`\xF8\x1B\x91\x1A\x02`\xF8\x1C`\0\x81\x81R`\x9F` \x90\x81R`@\x91\x82\x90 \x82Q\x80\x84\x01\x90\x93R\x80T\x83R`\x01\x01T\x90\x82\x01R\x90\x92Pa*\x19\x90\x87\x90a6#V[`\xFF\x83\x16`\0\x90\x81R`\x9D` \x90\x81R`@\x90\x91 T\x90\x86\x01Q\x80Q\x92\x98P`\x01`\x01``\x1B\x03\x90\x91\x16\x91\x83\x90\x81\x10a*TWa*TaJpV[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R\x84\x01Q\x80Q\x82\x90\x81\x10a*\x80Wa*\x80aJpV[` \x02` \x01\x01Q\x84`\0\x01Q\x82\x81Q\x81\x10a*\x9EWa*\x9EaJpV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPP`\0[\x88` \x01QQ\x81\x10\x15a+aW`\x9E`\0\x85\x83\x81Q\x81\x10a*\xE4Wa*\xE4aJpV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82R\x81\x81\x01\x92\x90\x92R`@\x90\x81\x01`\0\x90\x81 `\xFF\x87\x16\x82R\x90\x92R\x90 T\x85Q\x80Q`\x01`\x01``\x1B\x03\x90\x92\x16\x91\x84\x90\x81\x10a+/Wa+/aJpV[` \x02` \x01\x01\x81\x81Qa+C\x91\x90aT\xF1V[`\x01`\x01``\x1B\x03\x16\x90RP\x80a+Y\x81aK\x07V[\x91PPa*\xC1V[P\x80a+l\x81aK\x07V[\x91PPa)\x95V[P`\0\x80a+\x8C\x8A\x88\x8B``\x01Q\x8C`\x80\x01Qa\x07\x94V[\x91P\x91P\x81a,\x0FW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`C`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: pairing precompile call fai`d\x82\x01Rb\x1B\x19Y`\xEA\x1B`\x84\x82\x01R`\xA4\x01a\x06@V[\x80a,\x82W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FBLSSignatureChecker.checkSignatu`D\x82\x01R\x7Fres: signature is invalid\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06@V[P\x92\x95PPPPPP[\x92\x91PPV[a,\x9Aa:+V[`\x98\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`\x98T`\x01`\x01`\xA0\x1B\x03\x163\x14a-\x16W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FUpdater must be the caller\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[`\x97T`\x01`\xA0\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a-6`\x80\x84\x01``\x85\x01aL\x99V[c\xFF\xFF\xFF\xFF\x16\x14a-\x89W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FchainRdBatchNonce mismatch\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[a-\x99`@\x84\x01` \x85\x01aU\x19V[`\x01\x81\x11\x15a-\xAAWa-\xAAaH\xDCV[`\x97T`\x01`\xC0\x1B\x90\x04`\xFF\x16`\x01\x81\x11\x15a-\xC8Wa-\xC8aH\xDCV[\x14a.\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x15\xDC\x9B\xDB\x99\xC8\x18\xDA\x18Z[\x92Y`\x9A\x1B`D\x82\x01R`d\x01a\x06@V[`\x9ATc\xFF\xFF\xFF\xFF\x16\x15\x80a.1WPa.\"` \x84\x01\x84aL\x99V[`\x9ATc\xFF\xFF\xFF\xFF\x91\x82\x16\x91\x16\x10[a.lW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0C`$\x82\x01RkStale RdTask`\xA0\x1B`D\x82\x01R`d\x01a\x06@V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x16a.\xBAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x0F`$\x82\x01Rn\x13\xDC\x08\x1C\xDD\x18]\x19H\x1D[\x9A[\x9A]`\x8A\x1B`D\x82\x01R`d\x01a\x06@V[a.\xCA`\xA0\x84\x01`\x80\x85\x01aL\x99V[`\x9AT`\x01`@\x1B\x90\x04c\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14a/+W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1D`$\x82\x01R\x7Freference block hash mismatch\0\0\0`D\x82\x01R`d\x01a\x06@V[\x82`@Q` \x01a/<\x91\x90aU4V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x82` \x015\x14a/\xA4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1F`$\x82\x01R\x7FreferenceTaskHash hash mismatch\0`D\x82\x01R`d\x01a\x06@V[`\0a/\xDA\x83`@Q` \x01a/\xBA\x91\x90aU\xE6V[`@Q` \x81\x83\x03\x03\x81R\x90`@R\x80Q\x90` \x01 \x83a\x04\x88\x90aT\x80V[`\x9CT\x90\x91Pc\xFF\xFF\xFF\xFF\x16`\0[`\x9B\x80Ta/\xF6\x90aL.V[\x90P\x81\x10\x15a0\xBFW\x81`\xFF\x16\x83` \x01Q\x82\x81Q\x81\x10a0\x19Wa0\x19aJpV[` \x02` \x01\x01Qa0+\x91\x90aT\x8CV[`\x01`\x01``\x1B\x03\x16`d\x84`\0\x01Q\x83\x81Q\x81\x10a0LWa0LaJpV[` \x02` \x01\x01Q`\x01`\x01``\x1B\x03\x16a0g\x91\x90aT\xBBV[\x10\x15a0\xADW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x15`$\x82\x01RtFailed to meet quorum`X\x1B`D\x82\x01R`d\x01a\x06@V[\x80a0\xB7\x81aK\x07V[\x91PPa/\xE9V[P`@\x80Q\x80\x82\x01\x82R`\xA0\x86\x015\x81R`\xC0\x86\x015` \x82\x01\x90\x81R`\x97T\x92Qb#\xD0\xB5`\xE6\x1B\x81R`\x80\x88\x015`\x04\x82\x01R\x82Q`$\x82\x01R\x90Q`D\x82\x01R\x90\x91`\x01`\x01`\xA0\x1B\x03\x16\x90c\x08\xF4-@\x90`d\x01`\0`@Q\x80\x83\x03\x81`\0\x87\x80;\x15\x80\x15a11W`\0\x80\xFD[PZ\xF1\x15\x80\x15a1EW=`\0\x80>=`\0\xFD[Pa1Z\x92PPP`\x80\x86\x01``\x87\x01aL\x99V[a1e\x90`\x01aM\x18V[`\x97\x80Tc\xFF\xFF\xFF\xFF\x92\x90\x92\x16`\x01`\xA0\x1B\x02c\xFF\xFF\xFF\xFF`\xA0\x1B\x19\x90\x92\x16\x91\x90\x91\x17\x90Ua1\x97` \x87\x01\x87aL\x99V[`\x9A\x80Tc\xFF\xFF\xFF\xFF\x19\x16c\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90U\x7F\xECh\xDB9\x18y\xB0\xF9\xF4 \xD1\xCD\xF3Gj\xFB\xDF\x08Z$b\xBFM+\x11\xDFxFb\x95\xCB\x17a1\xDF` \x88\x01\x88aL\x99V[a1\xEF`\x80\x89\x01``\x8A\x01aL\x99V[`@\x80Qc\xFF\xFF\xFF\xFF\x93\x84\x16\x81R\x92\x90\x91\x16` \x83\x01R\x01`@Q\x80\x91\x03\x90\xA1PPPPPPV[a2\x1Fa:+V[`\x01`\x01`\xA0\x1B\x03\x81\x16a2\x84W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`&`$\x82\x01R\x7FOwnable: new owner is the zero a`D\x82\x01Reddress`\xD0\x1B`d\x82\x01R`\x84\x01a\x06@V[a\x06R\x81a;oV[`e`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16`\x01`\x01`\xA0\x1B\x03\x16c\xEA\xB6mz`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a2\xE0W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a3\x04\x91\x90aI\xA4V[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a34W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06@\x90aI\xC1V[`fT\x19\x81\x19`fT\x19\x16\x14a3\xB2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FPausable.unpause: invalid attemp`D\x82\x01R\x7Ft to pause functionality\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06@V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F5\x82\xD1\x82\x8E&\xBFV\xBD\x80\x15\x02\xBC\x02\x1A\xC0\xBC\x8A\xFBW\xC8&\xE4\x98kEY<\x8F\xAD8\x9C\x90` \x01a\x07\x89V[`fT\x15a49W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1C`$\x82\x01R\x7FPausable: contract is paused\0\0\0\0`D\x82\x01R`d\x01a\x06@V[a4Aa:+V[`\x97\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x83\x16\x90\x81\x17\x90\x91U`@Q\x90\x81R\x7F/ \xCF\x1B\xDAgs\x90D\xC5\xBFWsS\x97\x0C=\xBC\x18;,rt\xD1\xE8XJ\x10&\x922g\x90` \x01`@Q\x80\x91\x03\x90\xA1PV[`\x01`\x01`\xA0\x1B\x03\x81\x16a5#W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`I`$\x82\x01R\x7FPausable._setPauserRegistry: new`D\x82\x01R\x7FPauserRegistry cannot be the zer`d\x82\x01Rho address`\xB8\x1B`\x84\x82\x01R`\xA4\x01a\x06@V[`eT`@\x80Q`\x01`\x01`\xA0\x1B\x03\x92\x83\x16\x81R\x91\x83\x16` \x83\x01R\x7Fn\x9F\xCDS\x98\x96\xFC\xA6\x0E\x8B\x0F\x01\xDDX\x023\xE4\x8Ak\x0F}\xF0\x13\xB8\x9B\xA7\xF5e\x86\x9A\xCD\xB6\x91\x01`@Q\x80\x91\x03\x90\xA1`e\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90UV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra5\xA8a>\xF0V[\x83Q\x81R` \x80\x85\x01Q\x90\x82\x01R`@\x80\x82\x01\x84\x90R`\0\x90\x83``\x84`\x07a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a5\xDBWa5\xDDV[\xFE[P\x80a6\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCB[][\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06@V[PP\x92\x91PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra6?a?\x0EV[\x83Q\x81R` \x80\x85\x01Q\x81\x83\x01R\x83Q`@\x80\x84\x01\x91\x90\x91R\x90\x84\x01Q``\x83\x01R`\0\x90\x83`\x80\x84`\x06a\x07\xD0Z\x03\xFA\x90P\x80\x80\x15a5\xDBWP\x80a6\x1BW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\r`$\x82\x01Rl\x19X\xCBXY\x19\x0BY\x98Z[\x19Y`\x9A\x1B`D\x82\x01R`d\x01a\x06@V[a6\xBFa?,V[P`@\x80Q`\x80\x81\x01\x82R\x7F\x19\x8E\x93\x93\x92\rH:r`\xBF\xB71\xFB]%\xF1\xAAI35\xA9\xE7\x12\x97\xE4\x85\xB7\xAE\xF3\x12\xC2\x81\x83\x01\x90\x81R\x7F\x18\0\xDE\xEF\x12\x1F\x1EvBj\0f^\\DygC\"\xD4\xF7^\xDA\xDDF\xDE\xBD\\\xD9\x92\xF6\xED``\x83\x01R\x81R\x81Q\x80\x83\x01\x90\x92R\x7F']\xC4\xA2\x88\xD1\xAF\xB3\xCB\xB1\xAC\t\x18u$\xC7\xDB69]\xF7\xBE;\x99\xE6s\xB1:\x07Ze\xEC\x82R\x7F\x1D\x9B\xEF\xCD\x05\xA52>m\xA4\xD45\xF3\xB6\x17\xCD\xB3\xAF\x83(\\-\xF7\x11\xEF9\xC0\x15q\x82\x7F\x9D` \x83\x81\x01\x91\x90\x91R\x81\x01\x91\x90\x91R\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\0\x80\x80a7\xA7`\0\x80Q` aV\x92\x839\x81Q\x91R\x86aJ\x86V[\x90P[a7\xB3\x81a=@V[\x90\x93P\x91P`\0\x80Q` aV\x92\x839\x81Q\x91R\x82\x83\t\x83\x14\x15a7\xEDW`@\x80Q\x80\x82\x01\x90\x91R\x90\x81R` \x81\x01\x91\x90\x91R\x93\x92PPPV[`\0\x80Q` aV\x92\x839\x81Q\x91R`\x01\x82\x08\x90Pa7\xAAV[`@\x80Q\x80\x82\x01\x82R\x86\x81R` \x80\x82\x01\x86\x90R\x82Q\x80\x84\x01\x90\x93R\x86\x83R\x82\x01\x84\x90R`\0\x91\x82\x91\x90a89a?QV[`\0[`\x02\x81\x10\x15a9\xFEW`\0a8R\x82`\x06aT\xBBV[\x90P\x84\x82`\x02\x81\x10a8fWa8faJpV[` \x02\x01QQ\x83a8x\x83`\0aVyV[`\x0C\x81\x10a8\x88Wa8\x88aJpV[` \x02\x01R\x84\x82`\x02\x81\x10a8\x9FWa8\x9FaJpV[` \x02\x01Q` \x01Q\x83\x82`\x01a8\xB6\x91\x90aVyV[`\x0C\x81\x10a8\xC6Wa8\xC6aJpV[` \x02\x01R\x83\x82`\x02\x81\x10a8\xDDWa8\xDDaJpV[` \x02\x01QQQ\x83a8\xF0\x83`\x02aVyV[`\x0C\x81\x10a9\0Wa9\0aJpV[` \x02\x01R\x83\x82`\x02\x81\x10a9\x17Wa9\x17aJpV[` \x02\x01QQ`\x01` \x02\x01Q\x83a90\x83`\x03aVyV[`\x0C\x81\x10a9@Wa9@aJpV[` \x02\x01R\x83\x82`\x02\x81\x10a9WWa9WaJpV[` \x02\x01Q` \x01Q`\0`\x02\x81\x10a9rWa9raJpV[` \x02\x01Q\x83a9\x83\x83`\x04aVyV[`\x0C\x81\x10a9\x93Wa9\x93aJpV[` \x02\x01R\x83\x82`\x02\x81\x10a9\xAAWa9\xAAaJpV[` \x02\x01Q` \x01Q`\x01`\x02\x81\x10a9\xC5Wa9\xC5aJpV[` \x02\x01Q\x83a9\xD6\x83`\x05aVyV[`\x0C\x81\x10a9\xE6Wa9\xE6aJpV[` \x02\x01RP\x80a9\xF6\x81aK\x07V[\x91PPa8<V[Pa:\x07a?pV[`\0` \x82a\x01\x80\x85`\x08\x8C\xFA\x91Q\x91\x9C\x91\x15\x15\x9BP\x90\x99PPPPPPPPPPV[`3T`\x01`\x01`\xA0\x1B\x03\x163\x14a&\xA6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01\x81\x90R`$\x82\x01R\x7FOwnable: caller is not the owner`D\x82\x01R`d\x01a\x06@V[`eT`\x01`\x01`\xA0\x1B\x03\x16\x15\x80\x15a:\xA6WP`\x01`\x01`\xA0\x1B\x03\x82\x16\x15\x15[a;(W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`G`$\x82\x01R\x7FPausable._initializePauser: _ini`D\x82\x01R\x7FtializePauser() can only be call`d\x82\x01Rfed once`\xC8\x1B`\x84\x82\x01R`\xA4\x01a\x06@V[`f\x81\x90U`@Q\x81\x81R3\x90\x7F\xAB@\xA3t\xBCQ\xDE7\"\0\xA8\xBC\x98\x1A\xF8\xC9\xEC\xDC\x08\xDF\xDA\xEF\x0B\xB6\xE0\x9F\x88\xF3\xC6\x16\xEF=\x90` \x01`@Q\x80\x91\x03\x90\xA2a;k\x82a4\x95V[PPV[`3\x80T`\x01`\x01`\xA0\x1B\x03\x83\x81\x16`\x01`\x01`\xA0\x1B\x03\x19\x83\x16\x81\x17\x90\x93U`@Q\x91\x16\x91\x90\x82\x90\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90`\0\x90\xA3PPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01Ra\x02\0\x82a\xFF\xFF\x16\x10a<\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x10`$\x82\x01Roscalar-too-large`\x80\x1B`D\x82\x01R`d\x01a\x06@V[\x81a\xFF\xFF\x16`\x01\x14\x15a<1WP\x81a,\x8CV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01\x81\x90R\x84\x90`\x01\x90[\x81a\xFF\xFF\x16\x86a\xFF\xFF\x16\x10a<\x9AW`\x01a\xFF\xFF\x87\x16`\xFF\x83\x16\x1C\x81\x16\x14\x15a<}Wa<z\x84\x84a6#V[\x93P[a<\x87\x83\x84a6#V[\x92Pb\x01\xFF\xFE`\x01\x92\x83\x1B\x16\x91\x01a<MV[P\x91\x95\x94PPPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x81Q\x15\x80\x15a<\xCAWP` \x82\x01Q\x15[\x15a<\xE8WPP`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90V[`@Q\x80`@\x01`@R\x80\x83`\0\x01Q\x81R` \x01`\0\x80Q` aV\x92\x839\x81Q\x91R\x84` \x01Qa=\x1B\x91\x90aJ\x86V[a=3\x90`\0\x80Q` aV\x92\x839\x81Q\x91RaT\xDAV[\x90R\x92\x91PPV[\x91\x90PV[`\0\x80\x80`\0\x80Q` aV\x92\x839\x81Q\x91R`\x03`\0\x80Q` aV\x92\x839\x81Q\x91R\x86`\0\x80Q` aV\x92\x839\x81Q\x91R\x88\x89\t\t\x08\x90P`\0a=\xB6\x82\x7F\x0C\x19\x13\x9C\xB8Lh\nn\x14\x11m\xA0`V\x17e\xE0Z\xA4Z\x1Cr\xA3O\x08#\x05\xB6\x1F?R`\0\x80Q` aV\x92\x839\x81Q\x91Ra=\xC2V[\x91\x95\x91\x94P\x90\x92PPPV[`\0\x80a=\xCDa?pV[a=\xD5a?\x8EV[` \x80\x82R\x81\x81\x01\x81\x90R`@\x82\x01\x81\x90R``\x82\x01\x88\x90R`\x80\x82\x01\x87\x90R`\xA0\x82\x01\x86\x90R\x82`\xC0\x83`\x05a\x07\xD0Z\x03\xFA\x92P\x82\x80\x15a5\xDBWP\x82a>_W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x1A`$\x82\x01R\x7FBN254.expMod: call failure\0\0\0\0\0\0`D\x82\x01R`d\x01a\x06@V[PQ\x91PP[\x93\x92PPPV[\x82\x80Ta>x\x90aL.V[\x90`\0R` `\0 \x90`\x1F\x01` \x90\x04\x81\x01\x92\x82a>\x9AW`\0\x85Ua>\xE0V[\x82`\x1F\x10a>\xB3W\x82\x80\x01`\xFF\x19\x825\x16\x17\x85Ua>\xE0V[\x82\x80\x01`\x01\x01\x85U\x82\x15a>\xE0W\x91\x82\x01[\x82\x81\x11\x15a>\xE0W\x825\x82U\x91` \x01\x91\x90`\x01\x01\x90a>\xC5V[Pa>\xEC\x92\x91Pa?\xACV[P\x90V[`@Q\x80``\x01`@R\x80`\x03\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\x80\x01`@R\x80`\x04\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`@\x01`@R\x80a??a?\xC1V[\x81R` \x01a?La?\xC1V[\x90R\x90V[`@Q\x80a\x01\x80\x01`@R\x80`\x0C\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80` \x01`@R\x80`\x01\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[`@Q\x80`\xC0\x01`@R\x80`\x06\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[[\x80\x82\x11\x15a>\xECW`\0\x81U`\x01\x01a?\xADV[`@Q\x80`@\x01`@R\x80`\x02\x90` \x82\x02\x806\x837P\x91\x92\x91PPV[\x805`\xFF\x81\x16\x81\x14a=;W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a@\x02W`\0\x80\xFD[a>e\x82a?\xDFV[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x06RW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15a@2W`\0\x80\xFD[\x815a>e\x81a@\x0BV[`\0` \x82\x84\x03\x12\x15a@OW`\0\x80\xFD[P5\x91\x90PV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@\x8EWa@\x8Ea@VV[`@R\x90V[`@Qa\x01\0\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@\x8EWa@\x8Ea@VV[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a@\xDFWa@\xDFa@VV[`@R\x91\x90PV[`\0`@\x82\x84\x03\x12\x15a@\xF9W`\0\x80\xFD[aA\x01a@lV[\x90P\x815\x81R` \x82\x015` \x82\x01R\x92\x91PPV[`\0\x82`\x1F\x83\x01\x12aA(W`\0\x80\xFD[`@Q`@\x81\x01\x81\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17\x15aAJWaAJa@VV[\x80`@RP\x80`@\x84\x01\x85\x81\x11\x15aAaW`\0\x80\xFD[\x84[\x81\x81\x10\x15a<\x9AW\x805\x83R` \x92\x83\x01\x92\x01aAcV[`\0`\x80\x82\x84\x03\x12\x15aA\x8DW`\0\x80\xFD[aA\x95a@lV[\x90PaA\xA1\x83\x83aA\x17V[\x81RaA\xB0\x83`@\x84\x01aA\x17V[` \x82\x01R\x92\x91PPV[`\0\x80`\0\x80a\x01 \x85\x87\x03\x12\x15aA\xD2W`\0\x80\xFD[\x845\x93PaA\xE3\x86` \x87\x01a@\xE7V[\x92PaA\xF2\x86``\x87\x01aA{V[\x91PaB\x01\x86`\xE0\x87\x01a@\xE7V[\x90P\x92\x95\x91\x94P\x92PV[`\0`\xE0\x82\x84\x03\x12\x15aB\x1EW`\0\x80\xFD[P\x91\x90PV[`\0a\x01 \x82\x84\x03\x12\x15aB\x1EW`\0\x80\xFD[`\0\x80\x83`\x1F\x84\x01\x12aBIW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15aB`W`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x06\x1B\x85\x01\x01\x11\x15aB{W`\0\x80\xFD[\x92P\x92\x90PV[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a=;W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\0`\xA0\x88\x8A\x03\x12\x15aB\xB1W`\0\x80\xFD[\x875`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aB\xC8W`\0\x80\xFD[aB\xD4\x8B\x83\x8C\x01aB\x0CV[\x98P` \x8A\x015\x91P\x80\x82\x11\x15aB\xEAW`\0\x80\xFD[aB\xF6\x8B\x83\x8C\x01aB$V[\x97P`@\x8A\x015\x91P\x80\x82\x11\x15aC\x0CW`\0\x80\xFD[\x81\x8A\x01\x91P\x8A`\x1F\x83\x01\x12aC W`\0\x80\xFD[\x815\x81\x81\x11\x15aC/W`\0\x80\xFD[\x8B` \x82`\x05\x1B\x85\x01\x01\x11\x15aCDW`\0\x80\xFD[` \x83\x01\x97P\x80\x96PP``\x8A\x015\x91P\x80\x82\x11\x15aCbW`\0\x80\xFD[PaCo\x8A\x82\x8B\x01aB7V[\x90\x94P\x92PaC\x82\x90P`\x80\x89\x01aB\x82V[\x90P\x92\x95\x98\x91\x94\x97P\x92\x95PV[`\0` \x80\x83R\x83Q\x80\x82\x85\x01R`\0[\x81\x81\x10\x15aC\xBDW\x85\x81\x01\x83\x01Q\x85\x82\x01`@\x01R\x82\x01aC\xA1V[\x81\x81\x11\x15aC\xCFW`\0`@\x83\x87\x01\x01R[P`\x1F\x01`\x1F\x19\x16\x92\x90\x92\x01`@\x01\x93\x92PPPV[\x80\x15\x15\x81\x14a\x06RW`\0\x80\xFD[\x805a=;\x81aC\xE5V[\x805`\x02\x81\x10a=;W`\0\x80\xFD[`\0\x80`\0\x80`\0\x80`\xC0\x87\x89\x03\x12\x15aD&W`\0\x80\xFD[\x865aD1\x81a@\x0BV[\x95P` \x87\x015aDA\x81a@\x0BV[\x94P`@\x87\x015aDQ\x81a@\x0BV[\x93P``\x87\x015aDa\x81aC\xE5V[\x92P`\x80\x87\x015aDq\x81a@\x0BV[\x91PaD\x7F`\xA0\x88\x01aC\xFEV[\x90P\x92\x95P\x92\x95P\x92\x95V[`\0a\x01\x80\x82\x84\x03\x12\x15aB\x1EW`\0\x80\xFD[`\0\x80`\0\x80\x84\x86\x03`\xC0\x81\x12\x15aD\xB5W`\0\x80\xFD[\x855`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aD\xCCW`\0\x80\xFD[aD\xD8\x89\x83\x8A\x01aB\x0CV[\x96P```\x1F\x19\x84\x01\x12\x15aD\xECW`\0\x80\xFD[` \x88\x01\x95P`\x80\x88\x015\x92P\x80\x83\x11\x15aE\x06W`\0\x80\xFD[aE\x12\x89\x84\x8A\x01aD\x8BV[\x94P`\xA0\x88\x015\x92P\x80\x83\x11\x15aE(W`\0\x80\xFD[PPaE6\x87\x82\x88\x01aB$V[\x91PP\x92\x95\x91\x94P\x92PV[`\0\x80`@\x83\x85\x03\x12\x15aEUW`\0\x80\xFD[\x825\x91PaEe` \x84\x01a?\xDFV[\x90P\x92P\x92\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15aE\x87WaE\x87a@VV[P`\x05\x1B` \x01\x90V[`\0\x82`\x1F\x83\x01\x12aE\xA2W`\0\x80\xFD[\x815` aE\xB7aE\xB2\x83aEnV[a@\xB7V[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aE\xD6W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aE\xF8WaE\xEB\x81aB\x82V[\x83R\x91\x83\x01\x91\x83\x01aE\xDAV[P\x96\x95PPPPPPV[`\0\x82`\x1F\x83\x01\x12aF\x14W`\0\x80\xFD[\x815` aF$aE\xB2\x83aEnV[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aFCW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aE\xF8WaFY\x88\x82a@\xE7V[\x83R\x91\x83\x01\x91`@\x01aFGV[`\0\x82`\x1F\x83\x01\x12aFxW`\0\x80\xFD[\x815` aF\x88aE\xB2\x83aEnV[\x82\x81R`\x05\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15aF\xA7W`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15aE\xF8W\x805`\x01`\x01`@\x1B\x03\x81\x11\x15aF\xCAW`\0\x80\x81\xFD[aF\xD8\x89\x86\x83\x8B\x01\x01aE\x91V[\x84RP\x91\x83\x01\x91\x83\x01aF\xABV[`\0a\x01\x80\x82\x84\x03\x12\x15aF\xF9W`\0\x80\xFD[aG\x01a@\x94V[\x90P\x815`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aG\x1AW`\0\x80\xFD[aG&\x85\x83\x86\x01aE\x91V[\x83R` \x84\x015\x91P\x80\x82\x11\x15aG<W`\0\x80\xFD[aGH\x85\x83\x86\x01aF\x03V[` \x84\x01R`@\x84\x015\x91P\x80\x82\x11\x15aGaW`\0\x80\xFD[aGm\x85\x83\x86\x01aF\x03V[`@\x84\x01RaG\x7F\x85``\x86\x01aA{V[``\x84\x01RaG\x91\x85`\xE0\x86\x01a@\xE7V[`\x80\x84\x01Ra\x01 \x84\x015\x91P\x80\x82\x11\x15aG\xABW`\0\x80\xFD[aG\xB7\x85\x83\x86\x01aE\x91V[`\xA0\x84\x01Ra\x01@\x84\x015\x91P\x80\x82\x11\x15aG\xD1W`\0\x80\xFD[aG\xDD\x85\x83\x86\x01aE\x91V[`\xC0\x84\x01Ra\x01`\x84\x015\x91P\x80\x82\x11\x15aG\xF7W`\0\x80\xFD[PaH\x04\x84\x82\x85\x01aFgV[`\xE0\x83\x01RP\x92\x91PPV[`\0\x80`@\x83\x85\x03\x12\x15aH#W`\0\x80\xFD[\x825\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15aH@W`\0\x80\xFD[aHL\x85\x82\x86\x01aF\xE6V[\x91PP\x92P\x92\x90PV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15aH\x8FW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aHjV[P\x94\x95\x94PPPPPV[` \x81R`\0\x82Q`@` \x84\x01RaH\xB6``\x84\x01\x82aHVV[\x90P` \x84\x01Q`\x1F\x19\x84\x83\x03\x01`@\x85\x01RaH\xD3\x82\x82aHVV[\x95\x94PPPPPV[cNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[`\x02\x81\x10aI\x10WcNH{q`\xE0\x1B`\0R`!`\x04R`$`\0\xFD[\x90RV[` \x81\x01a,\x8C\x82\x84aH\xF2V[`\0\x80`\0\x83\x85\x03a\x01@\x81\x12\x15aI9W`\0\x80\xFD[\x845`\x01`\x01`@\x1B\x03\x80\x82\x11\x15aIPW`\0\x80\xFD[aI\\\x88\x83\x89\x01aB\x0CV[\x95Pa\x01\0`\x1F\x19\x84\x01\x12\x15aIqW`\0\x80\xFD[` \x87\x01\x94Pa\x01 \x87\x015\x92P\x80\x83\x11\x15aI\x8CW`\0\x80\xFD[PPaI\x9A\x86\x82\x87\x01aD\x8BV[\x91PP\x92P\x92P\x92V[`\0` \x82\x84\x03\x12\x15aI\xB6W`\0\x80\xFD[\x81Qa>e\x81a@\x0BV[` \x80\x82R`*\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Ri9\x90:\xB780\xBA\xB9\xB2\xB9`\xB1\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15aJ\x1DW`\0\x80\xFD[\x81Qa>e\x81aC\xE5V[` \x80\x82R`(\x90\x82\x01R\x7Fmsg.sender is not permissioned a`@\x82\x01Rg9\x9080\xBA\xB9\xB2\xB9`\xC1\x1B``\x82\x01R`\x80\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0\x82aJ\xA3WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x06\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aJ\xBFW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aJ\xD9W`\0\x80\xFD[` \x01\x91P`\x05\x81\x90\x1B6\x03\x82\x13\x15aB{W`\0\x80\xFD[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15aK\x1BWaK\x1BaJ\xF1V[P`\x01\x01\x90V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aK9W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aKSW`\0\x80\xFD[` \x01\x91P`\x07\x81\x90\x1B6\x03\x82\x13\x15aB{W`\0\x80\xFD[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a=;W`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aK\x94W`\0\x80\xFD[a>e\x82aKkV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aK\xB4W`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aK\xCEW`\0\x80\xFD[` \x01\x91P`\x06\x81\x90\x1B6\x03\x82\x13\x15aB{W`\0\x80\xFD[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aK\xFDW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\x17W`\0\x80\xFD[` \x01\x91P``\x81\x026\x03\x82\x13\x15aB{W`\0\x80\xFD[`\x01\x81\x81\x1C\x90\x82\x16\x80aLBW`\x7F\x82\x16\x91P[` \x82\x10\x81\x14\x15aB\x1EWcNH{q`\xE0\x1B`\0R`\"`\x04R`$`\0\xFD[`\0\x825`~\x19\x836\x03\x01\x81\x12aLyW`\0\x80\xFD[\x91\x90\x91\x01\x92\x91PPV[`\0\x825`^\x19\x836\x03\x01\x81\x12aLyW`\0\x80\xFD[`\0` \x82\x84\x03\x12\x15aL\xABW`\0\x80\xFD[a>e\x82aB\x82V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aL\xCBW`\0\x80\xFD[\x83\x01\x805\x91P`\x01`\x01`@\x1B\x03\x82\x11\x15aL\xE5W`\0\x80\xFD[` \x01\x91P6\x81\x90\x03\x82\x13\x15aB{W`\0\x80\xFD[\x82\x81R``\x81\x01a>e` \x83\x01\x84\x805\x82R` \x90\x81\x015\x91\x01RV[`\0c\xFF\xFF\xFF\xFF\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15aM7WaM7aJ\xF1V[\x01\x94\x93PPPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aMWW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aMvW`\0\x80\xFD[\x806\x03\x83\x13\x15aB{W`\0\x80\xFD[\x81\x83R\x81\x81` \x85\x017P`\0\x82\x82\x01` \x90\x81\x01\x91\x90\x91R`\x1F\x90\x91\x01`\x1F\x19\x16\x90\x91\x01\x01\x90V[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aM\xC3\x85aB\x82V[\x16` \x84\x01R\x80aM\xD6` \x86\x01aB\x82V[\x16`@\x84\x01R\x80aM\xE9`@\x86\x01aB\x82V[\x16``\x84\x01RPaM\xFD``\x84\x01\x84aM@V[`\xE0`\x80\x85\x01RaN\x13a\x01\0\x85\x01\x82\x84aM\x85V[\x91PPaN\"`\x80\x85\x01aB\x82V[c\xFF\xFF\xFF\xFF\x81\x16`\xA0\x85\x01RPaN<`\xA0\x85\x01\x85aM@V[\x84\x83\x03`\x1F\x19\x01`\xC0\x86\x01RaNS\x83\x82\x84aM\x85V[\x92PPPaNc`\xC0\x85\x01aB\x82V[c\xFF\xFF\xFF\xFF\x81\x16`\xE0\x85\x01RP\x93\x92PPPV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aN\x8EW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aN\xADW`\0\x80\xFD[\x80`\x05\x1B6\x03\x83\x13\x15aB{W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aH\x8FW`\xFFaN\xE2\x83a?\xDFV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aN\xCFV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aO\x0CW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aO+W`\0\x80\xFD[\x80`\x07\x1B6\x03\x83\x13\x15aB{W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aH\x8FW`\xFFaO`\x83a?\xDFV[\x16\x87R`\x01`\x01``\x1B\x03aOv\x84\x84\x01aKkV[\x16\x83\x88\x01R`@\x82\x81\x015\x90\x88\x01R``\x80\x83\x015\x90\x88\x01R`\x80\x96\x87\x01\x96\x90\x91\x01\x90`\x01\x01aOMV[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aO\xB8W`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aO\xD7W`\0\x80\xFD[\x80`\x06\x1B6\x03\x83\x13\x15aB{W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aH\x8FW`\xFFaP\x0C\x83a?\xDFV[\x16\x87R`\x01`\x01``\x1B\x03aP\"\x84\x84\x01aKkV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aO\xF9V[`\0\x80\x835`\x1E\x19\x846\x03\x01\x81\x12aPQW`\0\x80\xFD[\x83\x01` \x81\x01\x92P5\x90P`\x01`\x01`@\x1B\x03\x81\x11\x15aPpW`\0\x80\xFD[``\x81\x026\x03\x83\x13\x15aB{W`\0\x80\xFD[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aH\x8FW`\xFFaP\xA5\x83a?\xDFV[\x16\x87RaP\xC0\x83\x88\x01\x84\x84\x01\x805\x82R` \x90\x81\x015\x91\x01RV[``\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aP\x92V[\x81\x83R`\0`\x01`\x01`\xFB\x1B\x03\x83\x11\x15aP\xECW`\0\x80\xFD[\x82`\x05\x1B\x80\x83` \x87\x017`\0\x94\x01` \x01\x93\x84RP\x91\x92\x91PPV[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aH\x8FW`\x01`\x01``\x1B\x03aQ2\x83aKkV[\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01aQ\x19V[`\0\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84`\0\x80[\x88\x81\x10\x15aQ\xFAW\x85\x84\x03`\x1F\x19\x01\x8AR\x8256\x89\x90\x03`~\x19\x01\x81\x12aQ\x84W\x82\x83\xFD[\x88\x01\x805\x85R`\x80aQ\x98\x87\x83\x01\x83aNwV[\x82\x89\x89\x01RaQ\xAA\x83\x89\x01\x82\x84aN\xBFV[\x92PPP`@aQ\xBC\x81\x84\x01\x84aNwV[\x88\x84\x03\x83\x8A\x01RaQ\xCE\x84\x82\x84aQ\tV[\x93PPPP```\xFFaQ\xE2\x82\x85\x01a?\xDFV[\x16\x96\x01\x95\x90\x95RP\x98\x84\x01\x98\x91\x84\x01\x91`\x01\x01aQ_V[P\x91\x98\x97PPPPPPPPV[`\0\x83\x83\x85R` \x80\x86\x01\x95P\x80\x85`\x05\x1B\x83\x01\x01\x84`\0\x80[\x88\x81\x10\x15aQ\xFAW\x85\x84\x03`\x1F\x19\x01\x8AR\x8256\x89\x90\x03`^\x19\x01\x81\x12aRGW\x82\x83\xFD[\x88\x01\x805\x85R``aR[\x87\x83\x01\x83aNwV[\x82\x89\x89\x01RaRm\x83\x89\x01\x82\x84aN\xBFV[\x92PPP`@aR\x7F\x81\x84\x01\x84aNwV[\x93P\x87\x83\x03\x82\x89\x01RaR\x93\x83\x85\x83aQ\tV[\x9D\x89\x01\x9D\x97PPP\x93\x86\x01\x93PP`\x01\x01aR\"V[\x81\x83R`\0` \x80\x85\x01\x94P\x82`\0[\x85\x81\x10\x15aH\x8FW\x815\x87R`\xFFaR\xD2\x84\x84\x01a?\xDFV[\x16\x87\x84\x01R`@\x96\x87\x01\x96\x91\x90\x91\x01\x90`\x01\x01aR\xB9V[` \x81RaS\x04` \x82\x01aR\xFE\x84aC\xF3V[\x15\x15\x90RV[`\0aS\x13` \x84\x01\x84aNwV[a\x01 \x80`@\x86\x01RaS+a\x01@\x86\x01\x83\x85aN\xBFV[\x92PaS:`@\x87\x01\x87aN\xF5V[\x92P`\x1F\x19\x80\x87\x86\x03\x01``\x88\x01RaST\x85\x85\x84aO=V[\x94PaSc``\x89\x01\x89aO\xA1V[\x94P\x91P\x80\x87\x86\x03\x01`\x80\x88\x01RaS|\x85\x85\x84aO\xE9V[\x94PaS\x8B`\x80\x89\x01\x89aP:V[\x94P\x91P\x80\x87\x86\x03\x01`\xA0\x88\x01RaS\xA4\x85\x85\x84aP\x82V[\x94PaS\xB3`\xA0\x89\x01\x89aNwV[\x94P\x91P\x80\x87\x86\x03\x01`\xC0\x88\x01RaS\xCC\x85\x85\x84aP\xD3V[\x94PaS\xDB`\xC0\x89\x01\x89aNwV[\x94P\x91P\x80\x87\x86\x03\x01`\xE0\x88\x01RaS\xF4\x85\x85\x84aQEV[\x94PaT\x03`\xE0\x89\x01\x89aNwV[\x94P\x91Pa\x01\0\x81\x88\x87\x03\x01\x81\x89\x01RaT\x1E\x86\x86\x85aR\x08V[\x95PaT,\x81\x8A\x01\x8AaO\xA1V[\x95P\x92PP\x80\x87\x86\x03\x01\x83\x88\x01RPaTF\x84\x84\x83aR\xA9V[\x97\x96PPPPPPPV[``\x81\x01c\xFF\xFF\xFF\xFFaTc\x84aB\x82V[\x16\x82R` \x83\x015` \x83\x01R`@\x83\x015`@\x83\x01R\x92\x91PPV[`\0a,\x8C6\x83aF\xE6V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x81\x83\x04\x81\x11\x82\x15\x15\x16\x15aT\xB2WaT\xB2aJ\xF1V[\x02\x94\x93PPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15aT\xD5WaT\xD5aJ\xF1V[P\x02\x90V[`\0\x82\x82\x10\x15aT\xECWaT\xECaJ\xF1V[P\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15aU\x11WaU\x11aJ\xF1V[\x03\x93\x92PPPV[`\0` \x82\x84\x03\x12\x15aU+W`\0\x80\xFD[a>e\x82aC\xFEV[` \x81R`\0c\xFF\xFF\xFF\xFF\x80aUI\x85aB\x82V[\x16` \x84\x01RaU[` \x85\x01aC\xFEV[aUh`@\x85\x01\x82aH\xF2V[P\x80aUv`@\x86\x01aB\x82V[\x16``\x84\x01R\x80aU\x89``\x86\x01aB\x82V[\x16`\x80\x84\x01R\x80aU\x9C`\x80\x86\x01aB\x82V[\x16`\xA0\x84\x01RaU\xAF`\xA0\x85\x01\x85aM@V[`\xE0`\xC0\x86\x01RaU\xC5a\x01\0\x86\x01\x82\x84aM\x85V[\x91PP\x81aU\xD5`\xC0\x87\x01aB\x82V[\x16`\xE0\x85\x01R\x80\x92PPP\x92\x91PPV[a\x01\0\x81\x01c\xFF\xFF\xFF\xFF\x80aU\xFA\x85aB\x82V[\x16\x83R` \x84\x015` \x84\x01RaV\x13`@\x85\x01aC\xFEV[aV `@\x85\x01\x82aH\xF2V[P\x80aV.``\x86\x01aB\x82V[\x16``\x84\x01RP`\x80\x83\x015`\x80\x83\x01R`\xA0\x83\x015`\xA0\x83\x01R`\xC0\x83\x015`\xC0\x83\x01R`\xE0\x83\x015aVa\x81a@\x0BV[`\x01`\x01`\xA0\x1B\x03\x16`\xE0\x92\x90\x92\x01\x91\x90\x91R\x91\x90PV[`\0\x82\x19\x82\x11\x15aV\x8CWaV\x8CaJ\xF1V[P\x01\x90V\xFE0dNr\xE11\xA0)\xB8PE\xB6\x81\x81X]\x97\x81j\x91hq\xCA\x8D< \x8C\x16\xD8|\xFDG\xA2dipfsX\"\x12 U\xE5:\xF2\xBAFon\x13\xA0o9\x9980\x08\xD4\xD8\x85\xE9\xE5\xB0\x1B\xF6\xCA;\x1Au\xCF\xC6\xA6\x19dsolcC\0\x08\x0C\x003";
    /// The deployed bytecode of the contract.
    pub static GASPMULTIROLLUPSERVICE_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes =
        ::ethers::core::types::Bytes::from_static(__DEPLOYED_BYTECODE);
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
            Self(::ethers::contract::Contract::new(
                address.into(),
                GASPMULTIROLLUPSERVICE_ABI.clone(),
                client,
            ))
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
        ///Calls the contract's `allowNonRootInit` (0x0ee0fdbd) function
        pub fn allow_non_root_init(&self) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([14, 224, 253, 189], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainId` (0x9a8a0592) function
        pub fn chain_id(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([154, 138, 5, 146], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainRdBatchNonce` (0xdeb4037d) function
        pub fn chain_rd_batch_nonce(&self) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([222, 180, 3, 125], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `checkSignatures` (0x7d978897) function
        pub fn check_signatures(
            &self,
            msg_hash: [u8; 32],
            params: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, QuorumStakeTotals> {
            self.0
                .method_hash([125, 151, 136, 151], (msg_hash, params))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0x30c47d8e) function
        pub fn initialize(
            &self,
            pauser_registry: ::ethers::core::types::Address,
            initial_owner: ::ethers::core::types::Address,
            updater: ::ethers::core::types::Address,
            allow_non_root_init: bool,
            rolldown: ::ethers::core::types::Address,
            chain_id: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [48, 196, 125, 142],
                    (
                        pauser_registry,
                        initial_owner,
                        updater,
                        allow_non_root_init,
                        rolldown,
                        chain_id,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `lastOpUpdateBlockTimestamp` (0xf84e91fc) function
        pub fn last_op_update_block_timestamp(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([248, 78, 145, 252], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestCompletedOpTaskCreatedBlock` (0x6f0c30a4) function
        pub fn latest_completed_op_task_created_block(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([111, 12, 48, 164], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestCompletedOpTaskNumber` (0xe2a7cb66) function
        pub fn latest_completed_op_task_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([226, 167, 203, 102], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `latestCompletedRdTaskNumber` (0xd03a07b2) function
        pub fn latest_completed_rd_task_number(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, u32> {
            self.0
                .method_hash([208, 58, 7, 178], ())
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([136, 111, 17, 149], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processEigenOpUpdate` (0x34fadbea) function
        pub fn process_eigen_op_update(
            &self,
            task: OpTask,
            task_response: OpTaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
            operator_state_info: OperatorStateInfo,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [52, 250, 219, 234],
                    (
                        task,
                        task_response,
                        non_signer_stakes_and_signature,
                        operator_state_info,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processEigenRdUpdate` (0xed39e502) function
        pub fn process_eigen_rd_update(
            &self,
            task: RdTask,
            task_response: RdTaskResponse,
            non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [237, 57, 229, 2],
                    (task, task_response, non_signer_stakes_and_signature),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `processEigenReinit` (0x2635e74e) function
        pub fn process_eigen_reinit(
            &self,
            task: OpTask,
            operator_state_info: OperatorStateInfo,
            merkle_roots: ::std::vec::Vec<[u8; 32]>,
            ranges: ::std::vec::Vec<Range>,
            last_batch_id: u32,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [38, 53, 231, 78],
                    (
                        task,
                        operator_state_info,
                        merkle_roots,
                        ranges,
                        last_batch_id,
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Bytes> {
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
        pub fn renounce_ownership(&self) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rolldown` (0x3d9fb00c) function
        pub fn rolldown(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
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
        ///Calls the contract's `setRolldown` (0xfdc15de8) function
        pub fn set_rolldown(
            &self,
            rolldown: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 193, 93, 232], rolldown)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setUpdater` (0x9d54f419) function
        pub fn set_updater(
            &self,
            updater: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 84, 244, 25], updater)
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
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::Address> {
            self.0
                .method_hash([223, 3, 76, 208], ())
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `EigenOpUpdateProcessed` event
        pub fn eigen_op_update_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EigenOpUpdateProcessedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `EigenRdUpdateProcessed` event
        pub fn eigen_rd_update_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EigenRdUpdateProcessedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `EigenReinitProcessed` event
        pub fn eigen_reinit_processed_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, EigenReinitProcessedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Initialized` event
        pub fn initialized_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, InitializedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, OwnershipTransferredFilter>
        {
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
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, PauserRegistrySetFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `RolldownTargetUpdated` event
        pub fn rolldown_target_updated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, RolldownTargetUpdatedFilter>
        {
            self.0.event()
        }
        ///Gets the contract's `Unpaused` event
        pub fn unpaused_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, UnpausedFilter> {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<::std::sync::Arc<M>, M, GaspMultiRollupServiceEvents>
        {
            self.0
                .event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
        for GaspMultiRollupService<M>
    {
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
        Hash,
    )]
    #[ethevent(
        name = "EigenOpUpdateProcessed",
        abi = "EigenOpUpdateProcessed(uint32,uint32)"
    )]
    pub struct EigenOpUpdateProcessedFilter {
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
        Hash,
    )]
    #[ethevent(
        name = "EigenRdUpdateProcessed",
        abi = "EigenRdUpdateProcessed(uint32,uint32)"
    )]
    pub struct EigenRdUpdateProcessedFilter {
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethevent(name = "RolldownTargetUpdated", abi = "RolldownTargetUpdated(address)")]
    pub struct RolldownTargetUpdatedFilter {
        pub rolldown_address: ::ethers::core::types::Address,
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
        Hash,
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
        Hash,
    )]
    pub enum GaspMultiRollupServiceEvents {
        EigenOpUpdateProcessedFilter(EigenOpUpdateProcessedFilter),
        EigenRdUpdateProcessedFilter(EigenRdUpdateProcessedFilter),
        EigenReinitProcessedFilter(EigenReinitProcessedFilter),
        InitializedFilter(InitializedFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        PausedFilter(PausedFilter),
        PauserRegistrySetFilter(PauserRegistrySetFilter),
        RolldownTargetUpdatedFilter(RolldownTargetUpdatedFilter),
        UnpausedFilter(UnpausedFilter),
    }
    impl ::ethers::contract::EthLogDecode for GaspMultiRollupServiceEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = EigenOpUpdateProcessedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::EigenOpUpdateProcessedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EigenRdUpdateProcessedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::EigenRdUpdateProcessedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = EigenReinitProcessedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::EigenReinitProcessedFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::OwnershipTransferredFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = PausedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::PausedFilter(decoded));
            }
            if let Ok(decoded) = PauserRegistrySetFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::PauserRegistrySetFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = RolldownTargetUpdatedFilter::decode_log(log) {
                return Ok(GaspMultiRollupServiceEvents::RolldownTargetUpdatedFilter(
                    decoded,
                ));
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
                Self::EigenOpUpdateProcessedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EigenRdUpdateProcessedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EigenReinitProcessedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::InitializedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::OwnershipTransferredFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistrySetFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::RolldownTargetUpdatedFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::UnpausedFilter(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<EigenOpUpdateProcessedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: EigenOpUpdateProcessedFilter) -> Self {
            Self::EigenOpUpdateProcessedFilter(value)
        }
    }
    impl ::core::convert::From<EigenRdUpdateProcessedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: EigenRdUpdateProcessedFilter) -> Self {
            Self::EigenRdUpdateProcessedFilter(value)
        }
    }
    impl ::core::convert::From<EigenReinitProcessedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: EigenReinitProcessedFilter) -> Self {
            Self::EigenReinitProcessedFilter(value)
        }
    }
    impl ::core::convert::From<InitializedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: InitializedFilter) -> Self {
            Self::InitializedFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for GaspMultiRollupServiceEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<PausedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: PausedFilter) -> Self {
            Self::PausedFilter(value)
        }
    }
    impl ::core::convert::From<PauserRegistrySetFilter> for GaspMultiRollupServiceEvents {
        fn from(value: PauserRegistrySetFilter) -> Self {
            Self::PauserRegistrySetFilter(value)
        }
    }
    impl ::core::convert::From<RolldownTargetUpdatedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: RolldownTargetUpdatedFilter) -> Self {
            Self::RolldownTargetUpdatedFilter(value)
        }
    }
    impl ::core::convert::From<UnpausedFilter> for GaspMultiRollupServiceEvents {
        fn from(value: UnpausedFilter) -> Self {
            Self::UnpausedFilter(value)
        }
    }
    ///Container type for all input parameters for the `allowNonRootInit` function with signature `allowNonRootInit()` and selector `0x0ee0fdbd`
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
        Hash,
    )]
    #[ethcall(name = "allowNonRootInit", abi = "allowNonRootInit()")]
    pub struct AllowNonRootInitCall;
    ///Container type for all input parameters for the `chainId` function with signature `chainId()` and selector `0x9a8a0592`
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
        Hash,
    )]
    #[ethcall(name = "chainId", abi = "chainId()")]
    pub struct ChainIdCall;
    ///Container type for all input parameters for the `chainRdBatchNonce` function with signature `chainRdBatchNonce()` and selector `0xdeb4037d`
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
        Hash,
    )]
    #[ethcall(name = "chainRdBatchNonce", abi = "chainRdBatchNonce()")]
    pub struct ChainRdBatchNonceCall;
    ///Container type for all input parameters for the `checkSignatures` function with signature `checkSignatures(bytes32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x7d978897`
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
        Hash,
    )]
    #[ethcall(
        name = "checkSignatures",
        abi = "checkSignatures(bytes32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
    )]
    pub struct CheckSignaturesCall {
        pub msg_hash: [u8; 32],
        pub params: NonSignerStakesAndSignature,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,bool,address,uint8)` and selector `0x30c47d8e`
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
        Hash,
    )]
    #[ethcall(
        name = "initialize",
        abi = "initialize(address,address,address,bool,address,uint8)"
    )]
    pub struct InitializeCall {
        pub pauser_registry: ::ethers::core::types::Address,
        pub initial_owner: ::ethers::core::types::Address,
        pub updater: ::ethers::core::types::Address,
        pub allow_non_root_init: bool,
        pub rolldown: ::ethers::core::types::Address,
        pub chain_id: u8,
    }
    ///Container type for all input parameters for the `lastOpUpdateBlockTimestamp` function with signature `lastOpUpdateBlockTimestamp()` and selector `0xf84e91fc`
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
        Hash,
    )]
    #[ethcall(
        name = "lastOpUpdateBlockTimestamp",
        abi = "lastOpUpdateBlockTimestamp()"
    )]
    pub struct LastOpUpdateBlockTimestampCall;
    ///Container type for all input parameters for the `latestCompletedOpTaskCreatedBlock` function with signature `latestCompletedOpTaskCreatedBlock()` and selector `0x6f0c30a4`
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
        Hash,
    )]
    #[ethcall(
        name = "latestCompletedOpTaskCreatedBlock",
        abi = "latestCompletedOpTaskCreatedBlock()"
    )]
    pub struct LatestCompletedOpTaskCreatedBlockCall;
    ///Container type for all input parameters for the `latestCompletedOpTaskNumber` function with signature `latestCompletedOpTaskNumber()` and selector `0xe2a7cb66`
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
        Hash,
    )]
    #[ethcall(
        name = "latestCompletedOpTaskNumber",
        abi = "latestCompletedOpTaskNumber()"
    )]
    pub struct LatestCompletedOpTaskNumberCall;
    ///Container type for all input parameters for the `latestCompletedRdTaskNumber` function with signature `latestCompletedRdTaskNumber()` and selector `0xd03a07b2`
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
        Hash,
    )]
    #[ethcall(
        name = "latestCompletedRdTaskNumber",
        abi = "latestCompletedRdTaskNumber()"
    )]
    pub struct LatestCompletedRdTaskNumberCall;
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "pauserRegistry", abi = "pauserRegistry()")]
    pub struct PauserRegistryCall;
    ///Container type for all input parameters for the `processEigenOpUpdate` function with signature `processEigenOpUpdate((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))` and selector `0x34fadbea`
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
        Hash,
    )]
    #[ethcall(
        name = "processEigenOpUpdate",
        abi = "processEigenOpUpdate((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(uint32,bytes32,bytes32),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]))"
    )]
    pub struct ProcessEigenOpUpdateCall {
        pub task: OpTask,
        pub task_response: OpTaskResponse,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
        pub operator_state_info: OperatorStateInfo,
    }
    ///Container type for all input parameters for the `processEigenRdUpdate` function with signature `processEigenRdUpdate((uint32,uint8,uint32,uint32,uint32,bytes,uint32),(uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0xed39e502`
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
        Hash,
    )]
    #[ethcall(
        name = "processEigenRdUpdate",
        abi = "processEigenRdUpdate((uint32,uint8,uint32,uint32,uint32,bytes,uint32),(uint32,bytes32,uint8,uint32,bytes32,uint256,uint256,address),(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))"
    )]
    pub struct ProcessEigenRdUpdateCall {
        pub task: RdTask,
        pub task_response: RdTaskResponse,
        pub non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    }
    ///Container type for all input parameters for the `processEigenReinit` function with signature `processEigenReinit((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]),bytes32[],(uint256,uint256)[],uint32)` and selector `0x2635e74e`
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
        Hash,
    )]
    #[ethcall(
        name = "processEigenReinit",
        abi = "processEigenReinit((uint32,uint32,uint32,bytes,uint32,bytes,uint32),(bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]),bytes32[],(uint256,uint256)[],uint32)"
    )]
    pub struct ProcessEigenReinitCall {
        pub task: OpTask,
        pub operator_state_info: OperatorStateInfo,
        pub merkle_roots: ::std::vec::Vec<[u8; 32]>,
        pub ranges: ::std::vec::Vec<Range>,
        pub last_batch_id: u32,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(
        name = "quorumThresholdPercentage",
        abi = "quorumThresholdPercentage()"
    )]
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    #[ethcall(name = "setPauserRegistry", abi = "setPauserRegistry(address)")]
    pub struct SetPauserRegistryCall {
        pub new_pauser_registry: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setRolldown` function with signature `setRolldown(address)` and selector `0xfdc15de8`
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
        Hash,
    )]
    #[ethcall(name = "setRolldown", abi = "setRolldown(address)")]
    pub struct SetRolldownCall {
        pub rolldown: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `setUpdater` function with signature `setUpdater(address)` and selector `0x9d54f419`
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
        Hash,
    )]
    #[ethcall(name = "setUpdater", abi = "setUpdater(address)")]
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub enum GaspMultiRollupServiceCalls {
        AllowNonRootInit(AllowNonRootInitCall),
        ChainId(ChainIdCall),
        ChainRdBatchNonce(ChainRdBatchNonceCall),
        CheckSignatures(CheckSignaturesCall),
        Initialize(InitializeCall),
        LastOpUpdateBlockTimestamp(LastOpUpdateBlockTimestampCall),
        LatestCompletedOpTaskCreatedBlock(LatestCompletedOpTaskCreatedBlockCall),
        LatestCompletedOpTaskNumber(LatestCompletedOpTaskNumberCall),
        LatestCompletedRdTaskNumber(LatestCompletedRdTaskNumberCall),
        OperatorAndQuorumToStakes(OperatorAndQuorumToStakesCall),
        OperatorIdQuorumCount(OperatorIdQuorumCountCall),
        Owner(OwnerCall),
        Pause(PauseCall),
        PauseAll(PauseAllCall),
        PausedWithIndex(PausedWithIndexCall),
        Paused(PausedCall),
        PauserRegistry(PauserRegistryCall),
        ProcessEigenOpUpdate(ProcessEigenOpUpdateCall),
        ProcessEigenRdUpdate(ProcessEigenRdUpdateCall),
        ProcessEigenReinit(ProcessEigenReinitCall),
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
            if let Ok(decoded) =
                <AllowNonRootInitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::AllowNonRootInit(decoded));
            }
            if let Ok(decoded) = <ChainIdCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChainId(decoded));
            }
            if let Ok(decoded) =
                <ChainRdBatchNonceCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ChainRdBatchNonce(decoded));
            }
            if let Ok(decoded) =
                <CheckSignaturesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::CheckSignatures(decoded));
            }
            if let Ok(decoded) = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded) =
                <LastOpUpdateBlockTimestampCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LastOpUpdateBlockTimestamp(decoded));
            }
            if let Ok(decoded) =
                <LatestCompletedOpTaskCreatedBlockCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                )
            {
                return Ok(Self::LatestCompletedOpTaskCreatedBlock(decoded));
            }
            if let Ok(decoded) =
                <LatestCompletedOpTaskNumberCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestCompletedOpTaskNumber(decoded));
            }
            if let Ok(decoded) =
                <LatestCompletedRdTaskNumberCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::LatestCompletedRdTaskNumber(decoded));
            }
            if let Ok(decoded) =
                <OperatorAndQuorumToStakesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorAndQuorumToStakes(decoded));
            }
            if let Ok(decoded) =
                <OperatorIdQuorumCountCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::OperatorIdQuorumCount(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded) = <PauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Pause(decoded));
            }
            if let Ok(decoded) = <PauseAllCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PauseAll(decoded));
            }
            if let Ok(decoded) =
                <PausedWithIndexCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PausedWithIndex(decoded));
            }
            if let Ok(decoded) = <PausedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Paused(decoded));
            }
            if let Ok(decoded) =
                <PauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::PauserRegistry(decoded));
            }
            if let Ok(decoded) =
                <ProcessEigenOpUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessEigenOpUpdate(decoded));
            }
            if let Ok(decoded) =
                <ProcessEigenRdUpdateCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessEigenRdUpdate(decoded));
            }
            if let Ok(decoded) =
                <ProcessEigenReinitCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::ProcessEigenReinit(decoded));
            }
            if let Ok(decoded) = <QourumApkCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::QourumApk(decoded));
            }
            if let Ok(decoded) = <QuorumNumbersCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::QuorumNumbers(decoded));
            }
            if let Ok(decoded) =
                <QuorumThresholdPercentageCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::QuorumThresholdPercentage(decoded));
            }
            if let Ok(decoded) =
                <QuorumToStakesCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::QuorumToStakes(decoded));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::RenounceOwnership(decoded));
            }
            if let Ok(decoded) = <RolldownCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Rolldown(decoded));
            }
            if let Ok(decoded) =
                <SetPauserRegistryCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::SetPauserRegistry(decoded));
            }
            if let Ok(decoded) = <SetRolldownCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetRolldown(decoded));
            }
            if let Ok(decoded) = <SetUpdaterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SetUpdater(decoded));
            }
            if let Ok(decoded) = <StalledCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Stalled(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TrySignatureAndApkVerificationCall as ::ethers::core::abi::AbiDecode>::decode(data)
            {
                return Ok(Self::TrySignatureAndApkVerification(decoded));
            }
            if let Ok(decoded) = <UnpauseCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Unpause(decoded));
            }
            if let Ok(decoded) = <UpdaterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Updater(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for GaspMultiRollupServiceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AllowNonRootInit(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChainId(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChainRdBatchNonce(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::CheckSignatures(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Initialize(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::LastOpUpdateBlockTimestamp(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCompletedOpTaskCreatedBlock(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCompletedOpTaskNumber(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::LatestCompletedRdTaskNumber(element) => {
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
                Self::PauseAll(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PausedWithIndex(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Paused(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProcessEigenOpUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessEigenRdUpdate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProcessEigenReinit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QourumApk(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QuorumNumbers(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::QuorumThresholdPercentage(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::QuorumToStakes(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::RenounceOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Rolldown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetPauserRegistry(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetRolldown(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::SetUpdater(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Stalled(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::TransferOwnership(element) => ::ethers::core::abi::AbiEncode::encode(element),
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
                Self::AllowNonRootInit(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainRdBatchNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::CheckSignatures(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::LastOpUpdateBlockTimestamp(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestCompletedOpTaskCreatedBlock(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::LatestCompletedOpTaskNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::LatestCompletedRdTaskNumber(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorAndQuorumToStakes(element) => ::core::fmt::Display::fmt(element, f),
                Self::OperatorIdQuorumCount(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::Pause(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseAll(element) => ::core::fmt::Display::fmt(element, f),
                Self::PausedWithIndex(element) => ::core::fmt::Display::fmt(element, f),
                Self::Paused(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauserRegistry(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessEigenOpUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessEigenRdUpdate(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProcessEigenReinit(element) => ::core::fmt::Display::fmt(element, f),
                Self::QourumApk(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumNumbers(element) => ::core::fmt::Display::fmt(element, f),
                Self::QuorumThresholdPercentage(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<AllowNonRootInitCall> for GaspMultiRollupServiceCalls {
        fn from(value: AllowNonRootInitCall) -> Self {
            Self::AllowNonRootInit(value)
        }
    }
    impl ::core::convert::From<ChainIdCall> for GaspMultiRollupServiceCalls {
        fn from(value: ChainIdCall) -> Self {
            Self::ChainId(value)
        }
    }
    impl ::core::convert::From<ChainRdBatchNonceCall> for GaspMultiRollupServiceCalls {
        fn from(value: ChainRdBatchNonceCall) -> Self {
            Self::ChainRdBatchNonce(value)
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
    impl ::core::convert::From<LastOpUpdateBlockTimestampCall> for GaspMultiRollupServiceCalls {
        fn from(value: LastOpUpdateBlockTimestampCall) -> Self {
            Self::LastOpUpdateBlockTimestamp(value)
        }
    }
    impl ::core::convert::From<LatestCompletedOpTaskCreatedBlockCall> for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedOpTaskCreatedBlockCall) -> Self {
            Self::LatestCompletedOpTaskCreatedBlock(value)
        }
    }
    impl ::core::convert::From<LatestCompletedOpTaskNumberCall> for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedOpTaskNumberCall) -> Self {
            Self::LatestCompletedOpTaskNumber(value)
        }
    }
    impl ::core::convert::From<LatestCompletedRdTaskNumberCall> for GaspMultiRollupServiceCalls {
        fn from(value: LatestCompletedRdTaskNumberCall) -> Self {
            Self::LatestCompletedRdTaskNumber(value)
        }
    }
    impl ::core::convert::From<OperatorAndQuorumToStakesCall> for GaspMultiRollupServiceCalls {
        fn from(value: OperatorAndQuorumToStakesCall) -> Self {
            Self::OperatorAndQuorumToStakes(value)
        }
    }
    impl ::core::convert::From<OperatorIdQuorumCountCall> for GaspMultiRollupServiceCalls {
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
    impl ::core::convert::From<ProcessEigenOpUpdateCall> for GaspMultiRollupServiceCalls {
        fn from(value: ProcessEigenOpUpdateCall) -> Self {
            Self::ProcessEigenOpUpdate(value)
        }
    }
    impl ::core::convert::From<ProcessEigenRdUpdateCall> for GaspMultiRollupServiceCalls {
        fn from(value: ProcessEigenRdUpdateCall) -> Self {
            Self::ProcessEigenRdUpdate(value)
        }
    }
    impl ::core::convert::From<ProcessEigenReinitCall> for GaspMultiRollupServiceCalls {
        fn from(value: ProcessEigenReinitCall) -> Self {
            Self::ProcessEigenReinit(value)
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
    impl ::core::convert::From<QuorumThresholdPercentageCall> for GaspMultiRollupServiceCalls {
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
    impl ::core::convert::From<TrySignatureAndApkVerificationCall> for GaspMultiRollupServiceCalls {
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
    ///Container type for all return fields from the `allowNonRootInit` function with signature `allowNonRootInit()` and selector `0x0ee0fdbd`
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
        Hash,
    )]
    pub struct AllowNonRootInitReturn(pub bool);
    ///Container type for all return fields from the `chainId` function with signature `chainId()` and selector `0x9a8a0592`
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
        Hash,
    )]
    pub struct ChainIdReturn(pub u8);
    ///Container type for all return fields from the `chainRdBatchNonce` function with signature `chainRdBatchNonce()` and selector `0xdeb4037d`
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
        Hash,
    )]
    pub struct ChainRdBatchNonceReturn(pub u32);
    ///Container type for all return fields from the `checkSignatures` function with signature `checkSignatures(bytes32,(uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]))` and selector `0x7d978897`
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
        Hash,
    )]
    pub struct CheckSignaturesReturn(pub QuorumStakeTotals);
    ///Container type for all return fields from the `lastOpUpdateBlockTimestamp` function with signature `lastOpUpdateBlockTimestamp()` and selector `0xf84e91fc`
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
        Hash,
    )]
    pub struct LastOpUpdateBlockTimestampReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `latestCompletedOpTaskCreatedBlock` function with signature `latestCompletedOpTaskCreatedBlock()` and selector `0x6f0c30a4`
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
        Hash,
    )]
    pub struct LatestCompletedOpTaskCreatedBlockReturn(pub u32);
    ///Container type for all return fields from the `latestCompletedOpTaskNumber` function with signature `latestCompletedOpTaskNumber()` and selector `0xe2a7cb66`
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
        Hash,
    )]
    pub struct LatestCompletedOpTaskNumberReturn(pub u32);
    ///Container type for all return fields from the `latestCompletedRdTaskNumber` function with signature `latestCompletedRdTaskNumber()` and selector `0xd03a07b2`
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
        Hash,
    )]
    pub struct LatestCompletedRdTaskNumberReturn(pub u32);
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
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
        Hash,
    )]
    pub struct UpdaterReturn(pub ::ethers::core::types::Address);
    ///`Range(uint256,uint256)`
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
        Hash,
    )]
    pub struct Range {
        pub start: ::ethers::core::types::U256,
        pub end: ::ethers::core::types::U256,
    }
}
