// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.9;

import "@openzeppelin-upgrades/contracts/proxy/utils/Initializable.sol";
import "@openzeppelin-upgrades/contracts/access/OwnableUpgradeable.sol";

import "@eigenlayer/contracts/permissions/Pausable.sol";

import "@eigenlayer-middleware/src/libraries/BN254.sol";
import "@eigenlayer-middleware/src/interfaces/IServiceManager.sol";

import {BLSApkRegistry} from "@eigenlayer-middleware/src/BLSApkRegistry.sol";
import {RegistryCoordinator} from "@eigenlayer-middleware/src/RegistryCoordinator.sol";
import {BLSSignatureChecker, IRegistryCoordinator, IBLSSignatureChecker, IBLSApkRegistry, IStakeRegistry, IDelegationManager} from "@eigenlayer-middleware/src/BLSSignatureChecker.sol";
import {OperatorStateRetriever} from "@eigenlayer-middleware/src/OperatorStateRetriever.sol";

import "./IGaspMultiRollupServicePrimitives.sol";
import "./IFinalizerTaskManager.sol";
import {IRolldown} from "./IRolldown.sol";
import {IRolldownPrimitives} from "./IRolldownPrimitives.sol";

contract FinalizerTaskManager is
    Initializable,
    OwnableUpgradeable,
    Pausable,
    IBLSSignatureChecker,
    OperatorStateRetriever,
    IFinalizerTaskManager
{
    using BN254 for BN254.G1Point;

    BLSSignatureChecker public blsSignatureChecker;
    address public operatorStateRetrieverExtended;

    /* CONSTANT */
    // The number of blocks from the task initialization within which the aggregator has to respond to
    uint32 public taskResponseWindowBlock;
    uint256 public constant THRESHOLD_DENOMINATOR = 100;

    /* STORAGE */
    // The latest task index
    uint32 public latestOpTaskNum;
    uint32 public latestRdTaskNum;

    // mapping of task indices to all tasks hashes
    // when a task is created, task hash is stored here,
    // and responses need to pass the actual task,
    // which is hashed onchain and checked against this mapping
    mapping(TaskType => mapping(uint32 => bytes32)) public allTaskHashes;

    // mapping of task indices to hash of abi.encode(taskResponse, taskResponseMetadata)
    mapping(TaskType => mapping(uint32 => bytes32)) public allTaskResponses;

    mapping(TaskType => mapping(uint32 => TaskStatus)) public idToTaskStatus;

    uint32 public lastOpTaskCreatedBlock;
    uint32 public lastRdTaskCreatedBlock;
    uint32 public lastCompletedOpTaskNum;
    // If zero then no opTask has yet been completed
    // And hence no reference opState has been established
    uint32 public lastCompletedOpTaskCreatedBlock;
    // uint32 lastCompletedTaskNum;
    bytes public lastCompletedOpTaskQuorumNumbers;
    uint32 public lastCompletedOpTaskQuorumThresholdPercentage;

    address public aggregator;
    address public generator;

    IRolldown public rolldown;

    bool public isTaskPending;

    mapping(IRolldown.ChainId => uint32) public chainRdBatchNonce;

    // TODO
    // Maybe skip storing this
    bytes32 public operatorsStateInfoHash;

    bool public allowNonRootInit;


    /* MODIFIERS */
    modifier onlyAggregator() {
        require(msg.sender == aggregator, "Auth0");
        _;
    }

    // onlyTaskGenerator is used to restrict createNewTask from only being called by a permissioned entity
    modifier onlyTaskGenerator() {
        require(msg.sender == generator, "Auth1");
        _;
    }

    function initialize(IPauserRegistry _pauserRegistry, address initialOwner, address _aggregator, address _generator, bool _allowNonRootInit, address _blsSignatureCheckerAddress, uint32 _taskResponseWindowBlock, address _operatorStateRetrieverExtended, IRolldown _rolldown)
        public
        initializer
    {
        _initializePauser(_pauserRegistry, UNPAUSE_ALL);
        _transferOwnership(initialOwner);
        aggregator = _aggregator;
        generator = _generator;
        allowNonRootInit = _allowNonRootInit;
        blsSignatureChecker = BLSSignatureChecker(_blsSignatureCheckerAddress);
        operatorStateRetrieverExtended = _operatorStateRetrieverExtended;
        taskResponseWindowBlock = _taskResponseWindowBlock;
        rolldown = _rolldown;
    }

    function setRolldown(IRolldown _rolldown) external whenNotPaused onlyOwner {
      rolldown = _rolldown;
      emit RolldownTargetUpdated(address(_rolldown));
    }

    function pauseTrackingOpState()
        public
        onlyOwner
    {
        emit PauseTrackingOpState();
    }

    function resumeTrackingQuorums(bool resetTrackedQuorums)
        public
        onlyOwner
    {
        emit ResumeTrackingOpState(resetTrackedQuorums);
    }

    function updateBlsSignatureCheckerAddress(address _blsSignatureCheckerAddress) external onlyOwner{
        blsSignatureChecker = BLSSignatureChecker(_blsSignatureCheckerAddress);
        emit BLSSignatureCheckerAddressUpdated(_blsSignatureCheckerAddress);
    }

    // TODO!!!
    // DEDUP ALL THIS!
    
    /* FUNCTIONS */
    function _createNewOpTask(uint32 quorumThresholdPercentage, bytes calldata quorumNumbers)
        internal
    {
        require(
            lastCompletedOpTaskCreatedBlock != block.number && block.number != 0,
            "Can't in lastCompletedOpTaskCreatedBlock"
        );

        // create a new task struct
        OpTask memory newTask;
        newTask.taskNum = latestOpTaskNum;
        newTask.taskCreatedBlock = uint32(block.number);
        newTask.quorumThresholdPercentage = quorumThresholdPercentage;
        newTask.quorumNumbers = quorumNumbers;
        // This is to help the aggregator function as it currently is while 
        // being compatible with past op state verficiation
        if (lastCompletedOpTaskCreatedBlock == 0) {
            newTask.lastCompletedOpTaskCreatedBlock = uint32(block.number);
            newTask.lastCompletedOpTaskQuorumNumbers = quorumNumbers;
            newTask.lastCompletedOpTaskQuorumThresholdPercentage = quorumThresholdPercentage;
        } else {
            newTask.lastCompletedOpTaskCreatedBlock = lastCompletedOpTaskCreatedBlock;
            newTask.lastCompletedOpTaskQuorumNumbers = lastCompletedOpTaskQuorumNumbers;
            newTask.lastCompletedOpTaskQuorumThresholdPercentage = lastCompletedOpTaskQuorumThresholdPercentage;
        }

        // store hash of task onchain, emit event, and increase taskNum
        allTaskHashes[TaskType.OP_TASK][latestOpTaskNum] = keccak256(abi.encode(newTask));
        idToTaskStatus[TaskType.OP_TASK][latestOpTaskNum] = TaskStatus.INITIALIZED;
        lastOpTaskCreatedBlock = uint32(block.number);
        isTaskPending = true;
        emit NewOpTaskCreated(latestOpTaskNum, newTask);
        latestOpTaskNum = latestOpTaskNum + 1;
    }

    function createNewOpTask(uint32 quorumThresholdPercentage, bytes calldata quorumNumbers)
        external
        onlyTaskGenerator
    {
        require(
            isTaskPending == false,
            "Task already pending"
        );
        _createNewOpTask(quorumThresholdPercentage, quorumNumbers);
    }

    function validateTaskResponse(bytes32 taskHash, TaskType taskType, uint32 referenceTaskIndex, uint32 taskCreatedBlock) public view {

        require(
                isTaskPending == true, "No task pending");
        // check that the task is valid, hasn't been responsed yet, and is being responsed in time
        require(
            taskHash == allTaskHashes[taskType][referenceTaskIndex],
            "Task mismatch"
        );
        // some logical checks
        require(
            idToTaskStatus[taskType][referenceTaskIndex] == TaskStatus.INITIALIZED,
            "Not Init state"
        );
        require(
            allTaskResponses[taskType][referenceTaskIndex] == bytes32(0),
            "Alrdy Resp"
        );
        require(
            uint32(block.number) <= taskCreatedBlock + taskResponseWindowBlock,
            "Too late"
        );
    }

    // NOTE: this function responds to existing tasks.
    function respondToOpTask(
        OpTask calldata task,
        OpTaskResponse calldata taskResponse,
        IBLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) external {

        require(!(lastCompletedOpTaskCreatedBlock == 0) || allowNonRootInit, "use root init");

        bytes calldata quorumNumbers = task.lastCompletedOpTaskQuorumNumbers;
        uint32 quorumThresholdPercentage = task.lastCompletedOpTaskQuorumThresholdPercentage;

        validateTaskResponse(keccak256(abi.encode(task)), TaskType.OP_TASK, taskResponse.referenceTaskIndex, task.taskCreatedBlock);

        // Maybe also redundantly check here that taskResponse.referenceTaskIndex == lastestTaskNum - 1 ( safe since createNewTask increments latestTaskNum and the only task that should be INITIALIZED is the last created task)

        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */
        // calculate message which operators signed
        bytes32 message = keccak256(abi.encode(taskResponse));

        IBLSSignatureChecker.QuorumStakeTotals memory quorumStakeTotals; bytes32 hashOfNonSigners;

        // check the BLS signature
        (quorumStakeTotals, hashOfNonSigners) =
            blsSignatureChecker.checkSignatures(message, quorumNumbers, task.lastCompletedOpTaskCreatedBlock, nonSignerStakesAndSignature);

        TaskResponseMetadata memory taskResponseMetadata = TaskResponseMetadata(
            uint32(block.number),
            hashOfNonSigners,
            quorumStakeTotals.totalStakeForQuorum,
            quorumStakeTotals.signedStakeForQuorum
        );

        // updating the storage with task responsea
        allTaskResponses[TaskType.OP_TASK][taskResponse.referenceTaskIndex] = keccak256(abi.encode(taskResponse, taskResponseMetadata));
        idToTaskStatus[TaskType.OP_TASK][taskResponse.referenceTaskIndex] = TaskStatus.RESPONDED;
        isTaskPending = false;
        // emitting event
        emit OpTaskResponded(task.taskNum, taskResponse, taskResponseMetadata);

        // check that signatories own at least a threshold percentage of each quourm
        for (uint256 i = 0; i < quorumNumbers.length; i++) {
            // we don't check that the quorumThresholdPercentages are not >100 because a greater value would trivially fail the check, implying
            // signed stake > total stake
            if (
                quorumStakeTotals.signedStakeForQuorum[i] * THRESHOLD_DENOMINATOR
                    < quorumStakeTotals.totalStakeForQuorum[i] * uint8(quorumThresholdPercentage)
            ) {
                // "Signatories do not own at least threshold percentage of a quorum"
                return;
            }
        }

        operatorsStateInfoHash = taskResponse.operatorsStateInfoHash;
        idToTaskStatus[TaskType.OP_TASK][taskResponse.referenceTaskIndex] = TaskStatus.COMPLETED;
        lastCompletedOpTaskCreatedBlock = task.taskCreatedBlock;
        lastCompletedOpTaskQuorumNumbers = task.quorumNumbers;
        lastCompletedOpTaskQuorumThresholdPercentage = task.quorumThresholdPercentage;
        lastCompletedOpTaskNum = task.taskNum;
        // emitting completed event
        emit OpTaskCompleted(taskResponse.referenceTaskIndex, taskResponse);
    }

    function _cancelPendingTasks()
    internal {
        if (latestOpTaskNum > 0) {
            uint32 lastTaskNum = latestOpTaskNum - 1;
            if (idToTaskStatus[TaskType.OP_TASK][lastTaskNum] == TaskStatus.INITIALIZED){
                idToTaskStatus[TaskType.OP_TASK][lastTaskNum] = TaskStatus.CANCELLED;
                emit OpTaskCancelled(lastTaskNum);
            }
        }

        if (latestRdTaskNum > 0) {
            uint32 lastTaskNum = latestRdTaskNum - 1;
            if (idToTaskStatus[TaskType.RD_TASK][lastTaskNum] == TaskStatus.INITIALIZED){
                idToTaskStatus[TaskType.RD_TASK][lastTaskNum] = TaskStatus.CANCELLED;
                emit RdTaskCancelled(lastTaskNum);
            }
        }
        isTaskPending = false;
    }

    function forceCancelPendingTasks()
        external
        onlyOwner
    {
        require(isTaskPending == true, "No task pending");
        _cancelPendingTasks();
    }

    function forceCreateNewOpTask(uint32 quorumThresholdPercentage, bytes calldata quorumNumbers)
        external
        onlyOwner
    {
        if (isTaskPending) {
        _cancelPendingTasks();
        }

        _createNewOpTask(quorumThresholdPercentage, quorumNumbers);

        emit NewOpTaskForceCreated();
    }

    function forceRespondToOpTask(
        OpTask calldata task,
        OpTaskResponse calldata taskResponse
    ) external onlyOwner {

        validateTaskResponse(keccak256(abi.encode(task)), TaskType.OP_TASK, taskResponse.referenceTaskIndex, task.taskCreatedBlock);

        IBLSSignatureChecker.QuorumStakeTotals memory quorumStakeTotals; bytes32 hashOfNonSigners;

        TaskResponseMetadata memory taskResponseMetadata = TaskResponseMetadata(
            uint32(block.number),
            hashOfNonSigners,
            quorumStakeTotals.totalStakeForQuorum,
            quorumStakeTotals.signedStakeForQuorum
        );
        // updating the storage with task responsea
        allTaskResponses[TaskType.OP_TASK][taskResponse.referenceTaskIndex] = keccak256(abi.encode(taskResponse, taskResponseMetadata));

        operatorsStateInfoHash = taskResponse.operatorsStateInfoHash;
        idToTaskStatus[TaskType.OP_TASK][taskResponse.referenceTaskIndex] = TaskStatus.COMPLETED;
        lastCompletedOpTaskCreatedBlock = task.taskCreatedBlock;
        lastCompletedOpTaskQuorumNumbers = task.quorumNumbers;
        lastCompletedOpTaskQuorumThresholdPercentage = task.quorumThresholdPercentage;
        lastCompletedOpTaskNum = task.taskNum;
        isTaskPending = false;
        // emitting completed event
        emit OpTaskCompleted(taskResponse.referenceTaskIndex, taskResponse);
        emit OpTaskForceCompleted(taskResponse.referenceTaskIndex, taskResponse);
    }


    /* FUNCTIONS */
    function createNewRdTask(IRolldown.ChainId chainId, uint32 batchId)
        external
        onlyTaskGenerator
    {
        require(
            isTaskPending == false,
            "Task already pending"
        );
        require(
            lastCompletedOpTaskCreatedBlock != 0 && block.number != 0,
            "Op State uninit"
        );
        uint32 latestRdTaskNumMem = latestRdTaskNum;
        // create a new task struct
        RdTask memory newTask = RdTask({
            taskNum: latestRdTaskNumMem,
            chainId: chainId,
            batchId: batchId,
            taskCreatedBlock: uint32(block.number),
            lastCompletedOpTaskQuorumThresholdPercentage: lastCompletedOpTaskQuorumThresholdPercentage,
            lastCompletedOpTaskQuorumNumbers: lastCompletedOpTaskQuorumNumbers,
            lastCompletedOpTaskCreatedBlock: lastCompletedOpTaskCreatedBlock
        });

        // store hash of task onchain, emit event, and increase taskNum
        allTaskHashes[TaskType.RD_TASK][latestRdTaskNumMem] = keccak256(abi.encode(newTask));
        idToTaskStatus[TaskType.RD_TASK][latestRdTaskNumMem] = TaskStatus.INITIALIZED;
        lastRdTaskCreatedBlock = uint32(block.number);
        isTaskPending = true;
        emit NewRdTaskCreated(latestRdTaskNumMem, newTask);
        latestRdTaskNum = latestRdTaskNumMem + 1;
    }

    // NOTE: this function responds to existing tasks.
    function respondToRdTask(
        RdTask calldata task,
        RdTaskResponse calldata taskResponse,
        IBLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) external onlyAggregator {
        bytes calldata quorumNumbers = task.lastCompletedOpTaskQuorumNumbers;
        uint32 quorumThresholdPercentage = task.lastCompletedOpTaskQuorumThresholdPercentage;

        // TODO
        // Maybe this belongs in createNewRdTask
        require(chainRdBatchNonce[taskResponse.chainId] ==0 || taskResponse.batchId == chainRdBatchNonce[taskResponse.chainId], "chainRdBatchNonce mismatch"); 
        
        validateTaskResponse(keccak256(abi.encode(task)), TaskType.RD_TASK, taskResponse.referenceTaskIndex, task.taskCreatedBlock);
        
        // Maybe also redundantly check here that taskResponse.referenceTaskIndex == lastestTaskNum - 1 ( safe since createNewTask increments latestTaskNum and the only task that should be INITIALIZED is the last created task)

        /* CHECKING SIGNATURES & WHETHER THRESHOLD IS MET OR NOT */
        // calculate message which operators signed
        bytes32 message = keccak256(abi.encode(taskResponse));

        // check the BLS signature
        (IBLSSignatureChecker.QuorumStakeTotals memory quorumStakeTotals, bytes32 hashOfNonSigners) =
            blsSignatureChecker.checkSignatures(message, quorumNumbers, task.lastCompletedOpTaskCreatedBlock, nonSignerStakesAndSignature);

        TaskResponseMetadata memory taskResponseMetadata = TaskResponseMetadata(
            uint32(block.number),
            hashOfNonSigners,
            quorumStakeTotals.totalStakeForQuorum,
            quorumStakeTotals.signedStakeForQuorum
        );
        // updating the storage with task responses
        allTaskResponses[TaskType.RD_TASK][taskResponse.referenceTaskIndex] = keccak256(abi.encode(taskResponse, taskResponseMetadata));
        idToTaskStatus[TaskType.RD_TASK][taskResponse.referenceTaskIndex] = TaskStatus.RESPONDED;
        isTaskPending = false;

        // TODO
        // Optimize the following

        // emitting event
        emit RdTaskResponded(task.taskNum, taskResponse, taskResponseMetadata);

        // check that signatories own at least a threshold percentage of each quourm
        for (uint256 i = 0; i < quorumNumbers.length; i++) {
            // we don't check that the quorumThresholdPercentages are not >100 because a greater value would trivially fail the check, implying
            // signed stake > total stake
            if (
                quorumStakeTotals.signedStakeForQuorum[i] * THRESHOLD_DENOMINATOR
                    < quorumStakeTotals.totalStakeForQuorum[i] * uint8(quorumThresholdPercentage)
            ) {
                // "Signatories do not own at least threshold percentage of a quorum"
                return;
            }
        }

        idToTaskStatus[TaskType.RD_TASK][taskResponse.referenceTaskIndex] = TaskStatus.COMPLETED;

        IRolldown.ChainId ethChainId = IRolldownPrimitives.ChainId.Ethereum;
        if (taskResponse.chainId == ethChainId){
            IRolldown.Range memory range;
            range.start = taskResponse.rangeStart;
            range.end = taskResponse.rangeEnd;
            rolldown.update_l1_from_l2(taskResponse.rdUpdate, range);
        }
        chainRdBatchNonce[taskResponse.chainId] = taskResponse.batchId + 1;

        // emitting completed event
        emit RdTaskCompleted(taskResponse.referenceTaskIndex, taskResponse);
    }
    
    function dummyForOperatorStateInfoType(IGaspMultiRollupServicePrimitives.OperatorStateInfo calldata _operatorStateInfo) external view {
    }
    function dummyForQuorumStakeTotalsType(BLSSignatureChecker.QuorumStakeTotals calldata _quorumStakeTotals) external view {
    }

    function registryCoordinator() external view returns (IRegistryCoordinator){
        return blsSignatureChecker.registryCoordinator();
    }
    function stakeRegistry() external view returns (IStakeRegistry){
        return blsSignatureChecker.stakeRegistry();
    }
    function blsApkRegistry() external view returns (IBLSApkRegistry){
        return blsSignatureChecker.blsApkRegistry();
    }
    function delegation() external view returns (IDelegationManager){
        return blsSignatureChecker.delegation();
    }

    function checkSignatures(
        bytes32 msgHash, 
        bytes calldata quorumNumbers,
        uint32 referenceBlockNumber, 
        IBLSSignatureChecker.NonSignerStakesAndSignature memory nonSignerStakesAndSignature
    ) 
        external 
        view
        returns (
            QuorumStakeTotals memory,
            bytes32
        )
    {
        return blsSignatureChecker.checkSignatures(msgHash, quorumNumbers, referenceBlockNumber, nonSignerStakesAndSignature);
    }
}
