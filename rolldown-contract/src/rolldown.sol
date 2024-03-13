pragma solidity ^0.8.0;

// Import ERC-20 interface
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/utils/Strings.sol";
import "forge-std/console.sol";

contract RollDown {
    //tbr
    address owner;

    // Counter for mapping key
    uint256 public counter;
    // Counter for last processed request to ensure not reading and processing already processed
    uint256 public lastProcessedUpdate_origin_l1;
    // Counter for last processed updates comming from l2 to ensure not reading and processing already processed
    uint256 public lastProcessedUpdate_origin_l2;

    event DepositAcceptedIntoQueue(
        uint256 requestId,
        address depositRecipient,
        address tokenAddress,
        uint256 amount
    );
    event DisputeResolutionAcceptedIntoQueue(
        uint256 requestId,
        bool cancelJustified
    );
    event L2UpdatesToRemovedAcceptedIntoQueue(
        uint256 requestId,
        uint256[] l2UpdatesToRemove
    );
    event FundsWithdrawn(
        address withdrawRecipient,
        address tokenAddress,
        uint256 amount
    );
    event cancelAndCalculatedHash(bytes32 cancelHash, bytes32 calculatedHash);

    enum Origin {
        L1,
        L2
    }

    struct RequestId {
        Origin origin;
        uint256 id;
    }

    struct Deposit {
        RequestId requestId;
        address depositRecipient;
        address tokenAddress;
        uint256 amount;
        bytes32 blockHash;
    }

    struct L2UpdatesToRemove {
        RequestId requestId;
        uint256[] l2UpdatesToRemove;
        bytes32 blockHash;
    }

    struct CancelResolution {
        RequestId requestId;
        uint256 l2RequestId;
        bool cancelJustified;
        bytes32 blockHash;
    }

    struct WithdrawalResolutions {
        RequestId requestId;
        uint256 l2RequestId;
        bool status;
        bytes32 blockHash;
    }

    enum PendingRequestType {
        DEPOSIT,
        WITHDRAWAL,
        CANCEL_RESOLUTION,
        L2_UPDATES_TO_REMOVE
    }

    struct L1Update {
        Deposit[] pendingDeposits;
        CancelResolution[] pendingCancelResultions;
        WithdrawalResolutions[] pendingWithdrawalResolutions;
        L2UpdatesToRemove[] pendingL2UpdatesToRemove;
    }

    mapping(uint256 => WithdrawalResolutions) public withdrawalResolutions;
    mapping(uint256 => CancelResolution) public cancelResolutions;
    mapping(uint256 => Deposit) private deposits;
    mapping(uint256 => L2UpdatesToRemove) private l2UpdatesToRemove;
    /// PENING REQUESTS TYPES (L1)

    enum UpdateType {
        DEPOSIT,
        WITHDRAWAL,
        INDEX_UPDATE,
        CANCEL_RESOLUTION
    }

    struct RequestResult {
        RequestId requestId;
        uint256 originRequestId;
        UpdateType updateType;
        bool status;
    }

    struct L2Update {
        Cancel[] cancels;
        Withdrawal[] withdrawals;
        RequestResult[] results;
    }

    struct Range {
        uint256 start;
        uint256 end;
    }

    struct Cancel {
        RequestId requestId;
        Range range;
        bytes32 hash;
    }

    struct Withdrawal {
        RequestId requestId;
        address withdrawalRecipient;
        address tokenAddress;
        uint256 amount;
    }

    constructor() {
        lastProcessedUpdate_origin_l1 = 0;
        counter = 1;
        lastProcessedUpdate_origin_l2 = 0;
        owner = msg.sender;
    }

    function deposit(address tokenAddress, uint256 amount) public {
        require(tokenAddress != address(0), "Invalid token address");
        require(amount > 0, "Amount must be greater than zero");
        address depositRecipient = msg.sender;

        IERC20 token = IERC20(tokenAddress);
        require(
            token.transferFrom(msg.sender, address(this), amount),
            "Token transfer failed"
        );

        bytes32 blockHash = blockhash(block.number);
        Deposit memory depositRequest = Deposit({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            depositRecipient: depositRecipient,
            tokenAddress: tokenAddress,
            amount: amount,
            blockHash: blockHash
        });
        // Add the new request to the mapping
        deposits[depositRequest.requestId.id] = depositRequest;
        emit DepositAcceptedIntoQueue(
            depositRequest.requestId.id,
            depositRecipient,
            tokenAddress,
            amount
        );
    }

    function getUpdateForL2() public view returns (L1Update memory) {
        return
            getPendingRequests(lastProcessedUpdate_origin_l1 + 1, counter - 1);
    }

    function getOrderOfRequestsOriginatingOnL2(L2Update calldata update)
        private
        view
        returns (UpdateType[] memory)
    {
        if (update.cancels.length == 0 && update.withdrawals.length == 0) {
            return new UpdateType[](0);
        }

        uint256 withdrawalId = 0;
        uint256 cancelId = 0;
        uint256 orderId = 0;
        uint256 firstId;
        unchecked {
            firstId = uint256(0) - 1;
        }
        uint256 updatesAmount = update.cancels.length +
            update.withdrawals.length;
        UpdateType[] memory order = new UpdateType[](updatesAmount);

        if (update.cancels.length > 0) {
            firstId = update.cancels[0].requestId.id < firstId
                ? update.cancels[0].requestId.id
                : firstId;
        }

        if (update.withdrawals.length > 0) {
            firstId = update.withdrawals[0].requestId.id < firstId
                ? update.withdrawals[0].requestId.id
                : firstId;
        }

        require(
            firstId <= lastProcessedUpdate_origin_l2 + 1,
            "Invalid L2Update"
        );
        for (uint256 i = firstId; i < firstId + updatesAmount; i++) {
            if (
                withdrawalId < update.withdrawals.length &&
                update.withdrawals[withdrawalId].requestId.id == i
            ) {
                order[orderId] = UpdateType.WITHDRAWAL;
                withdrawalId++;
                orderId++;
            } else if (
                cancelId < update.cancels.length &&
                update.cancels[cancelId].requestId.id == i
            ) {
                order[orderId] = UpdateType.CANCEL_RESOLUTION;
                cancelId++;
                orderId++;
            } else {
                revert("invalide L2Update");
            }
        }
        return order;
    }

    function processRequestsOriginatingOnL2(L2Update calldata inputArray)
        private
    {
        UpdateType[] memory order = getOrderOfRequestsOriginatingOnL2(
            inputArray
        );
        uint256 cancelId = 0;
        uint256 withdrawalId = 0;

        for (uint256 i = 0; i < order.length; i++) {
            if (order[i] == UpdateType.WITHDRAWAL) {
                process_l2_update_withdrawal(inputArray.withdrawals[withdrawalId++]);
            } else if (order[i] == UpdateType.CANCEL_RESOLUTION) {
                process_l2_update_cancels(inputArray.cancels[cancelId++]);
            } else {
                revert("unknown update type");
            }
            lastProcessedUpdate_origin_l2 = i;
        }
    }

    function update_l1_from_l2(L2Update calldata inputArray) external {
        //1st iteration, security comes from ensuring dedicated acc
        //Ensure sender is dedic acc
        // TODO: uncomment & fix UT
        // require(msg.sender == owner, "Not the owner");
        require(
            inputArray.results.length >= 1 ||
                inputArray.cancels.length >= 1 ||
                inputArray.withdrawals.length >= 1,
            "Array must have at least 1 update"
        );

        uint256[]
            memory l2UpdatesToBeRemoved = process_l2_update_requests_results(
                inputArray.results
            );

        processRequestsOriginatingOnL2(inputArray);

        // Create a new array with the correct size
        if (l2UpdatesToBeRemoved.length > 0) {
            uint256 rid = counter++;
            l2UpdatesToRemove[rid].requestId = RequestId({
                origin: Origin.L1,
                id: rid
            });
            l2UpdatesToRemove[rid].l2UpdatesToRemove = l2UpdatesToBeRemoved;
            lastProcessedUpdate_origin_l1 += l2UpdatesToBeRemoved.length;
            emit L2UpdatesToRemovedAcceptedIntoQueue(rid, l2UpdatesToBeRemoved);
        }
    }

    function process_l2_update_requests_results(
        RequestResult[] calldata results
    ) private view returns (uint256[] memory) {
        uint256 updatesToBeRemovedCounter = 0;
        if (results.length == 0) {
            return new uint256[](0);
        }
        uint256 oderCounter = results[0].requestId.id;
        uint256[] memory l2UpdatesToBeRemovedTemp = new uint256[](
            results.length
        );

        for (uint256 idx = 1; idx < results.length; idx++) {
            if (results[idx].requestId.id != oderCounter + 1) {
                revert("Requests are not in order");
            }
            oderCounter = results[idx].requestId.id;
        }

        for (uint256 idx = 0; idx < results.length; idx++) {
            RequestResult calldata element = results[idx];

            if (element.requestId.id <= lastProcessedUpdate_origin_l1) {
                continue;
            }

            if (element.updateType == UpdateType.DEPOSIT) {
                l2UpdatesToBeRemovedTemp[updatesToBeRemovedCounter++] = (
                    element.originRequestId
                );
            } else if (element.updateType == UpdateType.INDEX_UPDATE) {
                l2UpdatesToBeRemovedTemp[updatesToBeRemovedCounter++] = (
                    element.originRequestId
                );
            } else {
                revert("unknown request type");
            }
        }

        uint256[] memory l2UpdatesToBeRemoved = new uint256[](
            updatesToBeRemovedCounter
        );

        for (uint256 i = 0; i < updatesToBeRemovedCounter; i++) {
            l2UpdatesToBeRemoved[i] = l2UpdatesToBeRemovedTemp[i];
        }

        return l2UpdatesToBeRemoved;
    }

    function process_l2_update_cancels(Cancel calldata cancel) private {
        if (cancel.requestId.id <= lastProcessedUpdate_origin_l1) {
          return;
        }
        L1Update memory pending = getPendingRequests(
            cancel.range.start,
            cancel.range.end
        );
        bytes32 correct_hash = keccak256(abi.encode(pending));
        bytes32 blockHash = blockhash(block.number);

        CancelResolution memory resolution = CancelResolution({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            l2RequestId: cancel.requestId.id,
            cancelJustified: correct_hash == cancel.hash,
            blockHash: blockHash
        });

        cancelResolutions[counter++] = resolution;
        emit DisputeResolutionAcceptedIntoQueue(
            resolution.l2RequestId,
            resolution.cancelJustified
        );
    }

    function process_l2_update_withdrawal(Withdrawal calldata withdrawal)
        private
    {
        if (withdrawal.requestId.id <= lastProcessedUpdate_origin_l1) {
          return;
        }
        IERC20 token = IERC20(withdrawal.tokenAddress);
        bool status = token.balanceOf(address(this)) >= withdrawal.amount;
        bytes32 blockHash = blockhash(block.number);

        WithdrawalResolutions memory resolution = WithdrawalResolutions({
            requestId: RequestId({origin: Origin.L1, id: counter++}),
            l2RequestId: withdrawal.requestId.id,
            status: status,
            blockHash: blockHash
        });

        withdrawalResolutions[counter++] = resolution;

        if (status) {
            token.transfer(withdrawal.withdrawalRecipient, withdrawal.amount);
            emit FundsWithdrawn(
                withdrawal.withdrawalRecipient,
                withdrawal.tokenAddress,
                withdrawal.amount
            );
        }
    }

    function getPendingRequests(uint256 start, uint256 end)
        private
        view
        returns (L1Update memory)
    {
        L1Update memory result;

        uint256 depositsCounter = 0;
        uint256 withdrawalsCounter = 0;
        uint256 cancelsCounter = 0;
        uint256 updatesToBeRemovedCounter = 0;

        for (uint256 requestId = start; requestId <= end; requestId++) {
            if (deposits[requestId].requestId.id != 0) {
                depositsCounter++;
            } else if (l2UpdatesToRemove[requestId].requestId.id != 0) {
                updatesToBeRemovedCounter++;
            } else if (withdrawalResolutions[requestId].requestId.id != 0) {
                withdrawalsCounter++;
            } else if (cancelResolutions[requestId].requestId.id != 0) {
                cancelsCounter++;
            }
        }

        result.pendingDeposits = new Deposit[](depositsCounter);
        result.pendingCancelResultions = new CancelResolution[](cancelsCounter);
        result.pendingL2UpdatesToRemove = new L2UpdatesToRemove[](
            updatesToBeRemovedCounter
        );

        withdrawalsCounter = 0;
        depositsCounter = 0;
        cancelsCounter = 0;
        updatesToBeRemovedCounter = 0;

        for (uint256 requestId = start; requestId <= end; requestId++) {
            if (deposits[requestId].requestId.id > 0) {
                result.pendingDeposits[depositsCounter++] = deposits[requestId];
            } else if (withdrawalResolutions[requestId].requestId.id > 0) {
              result.pendingWithdrawalResolutions[withdrawalsCounter++] = withdrawalResolutions[requestId];
            } else if (l2UpdatesToRemove[requestId].requestId.id > 0) {
              result.pendingL2UpdatesToRemove[updatesToBeRemovedCounter++] = l2UpdatesToRemove[requestId];
            } else if (cancelResolutions[requestId].l2RequestId > 0) {
              result.pendingCancelResultions[cancelsCounter++] = cancelResolutions[requestId];
            } else {
                break;
            }
        }

        return result;
    }
}

// TODO: counter l2 check
// TODO: align types again
