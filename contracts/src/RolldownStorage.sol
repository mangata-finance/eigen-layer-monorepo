// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.9;

import "./IRolldown.sol";

abstract contract RolldownStorage is IRolldown {

    // Counter for mapping key
    uint256 public counter;
    // Counter for last processed request to ensure not reading and processing already processed
    uint256 public lastProcessedUpdate_origin_l1;
    // Counter for last processed updates comming from l2 to ensure not reading and processing already processed
    uint256 public lastProcessedUpdate_origin_l2;
    // Chain identificator
    ChainId public chain;
    // Chain identificator
    address public updaterAccount;

    mapping(uint256 => WithdrawalResolution) public withdrawalResolutions;
    mapping(uint256 => CancelResolution) public cancelResolutions;
    mapping(uint256 => Deposit) internal deposits;
    mapping(uint256 => L2UpdatesToRemove) internal l2UpdatesToRemove;
    mapping(address => uint) public pendingEthWithdrawals;   


    /**
     * @dev This empty reserved space is put in place to allow future versions to add new
     * variables without shifting down storage in the inheritance chain.
     * See https://docs.openzeppelin.com/contracts/4.x/upgradeable#storage_gaps
     */
    uint256[50] private __gap;
}
