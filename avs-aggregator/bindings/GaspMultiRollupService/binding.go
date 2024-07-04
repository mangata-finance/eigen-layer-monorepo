// Code generated - DO NOT EDIT.
// This file is a generated binding and any manual changes will be lost.

package contractGaspMultiRollupService

import (
	"errors"
	"math/big"
	"strings"

	ethereum "github.com/ethereum/go-ethereum"
	"github.com/ethereum/go-ethereum/accounts/abi"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"
)

// Reference imports to suppress errors if they are not otherwise used.
var (
	_ = errors.New
	_ = big.NewInt
	_ = strings.NewReader
	_ = ethereum.NotFound
	_ = bind.Bind
	_ = common.Big1
	_ = types.BloomLookup
	_ = event.NewSubscription
	_ = abi.ConvertType
)

// BN254G1Point is an auto generated low-level Go binding around an user-defined struct.
type BN254G1Point struct {
	X *big.Int
	Y *big.Int
}

// BN254G2Point is an auto generated low-level Go binding around an user-defined struct.
type BN254G2Point struct {
	X [2]*big.Int
	Y [2]*big.Int
}

// IBLSSignatureCheckerQuorumStakeTotals is an auto generated low-level Go binding around an user-defined struct.
type IBLSSignatureCheckerQuorumStakeTotals struct {
	SignedStakeForQuorum []*big.Int
	TotalStakeForQuorum  []*big.Int
}

// IFinalizerTaskManagerTask is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerTask struct {
	TaskNum                                    uint32
	BlockNumber                                *big.Int
	TaskCreatedBlock                           uint32
	LastCompletedTaskCreatedBlock              uint32
	QuorumNumbers                              []byte
	QuorumThresholdPercentage                  uint32
	LastCompletedTaskQuorumNumbers             []byte
	LastCompletedTaskQuorumThresholdPercentage uint32
}

// IFinalizerTaskManagerTaskResponse is an auto generated low-level Go binding around an user-defined struct.
type IFinalizerTaskManagerTaskResponse struct {
	ReferenceTaskIndex     uint32
	ReferenceTaskHash      [32]byte
	OperatorsStateInfoHash [32]byte
	BlockHash              [32]byte
	StorageProofHash       [32]byte
	PendingStateHash       [32]byte
}

// IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState struct {
	NonSignerG1PubkeysForOldState []BN254G1Point
	ApkG2ForOldState              BN254G2Point
	SigmaForOldState              BN254G1Point
}

// IGaspMultiRollupServicePrimitivesOperatorStateInfo is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesOperatorStateInfo struct {
	OperatorsStateChanged      bool
	QuorumsRemoved             []uint8
	QuorumsAdded               []IGaspMultiRollupServicePrimitivesQuorumsAdded
	QuorumsStakeUpdate         []IGaspMultiRollupServicePrimitivesQuorumsStakeUpdate
	QuorumsApkUpdate           []IGaspMultiRollupServicePrimitivesQuorumsApkUpdate
	OperatorsRemoved           [][32]byte
	OperatorsAdded             []IGaspMultiRollupServicePrimitivesOperatorsAdded
	OperatorsStakeUpdate       []IGaspMultiRollupServicePrimitivesOperatorsStakeUpdate
	OperatorsQuorumCountUpdate []IGaspMultiRollupServicePrimitivesOperatorsQuorumCountUpdate
}

// IGaspMultiRollupServicePrimitivesOperatorsAdded is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesOperatorsAdded struct {
	OperatorId      [32]byte
	QuorumForStakes []uint8
	QuorumStakes    []*big.Int
	QuorumCount     uint8
}

// IGaspMultiRollupServicePrimitivesOperatorsQuorumCountUpdate is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesOperatorsQuorumCountUpdate struct {
	OperatorId  [32]byte
	QuorumCount uint8
}

// IGaspMultiRollupServicePrimitivesOperatorsStakeUpdate is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesOperatorsStakeUpdate struct {
	OperatorId      [32]byte
	QuorumForStakes []uint8
	QuorumStakes    []*big.Int
}

// IGaspMultiRollupServicePrimitivesQuorumsAdded is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesQuorumsAdded struct {
	QuorumNumber uint8
	QuorumStake  *big.Int
	QuorumApk    BN254G1Point
}

// IGaspMultiRollupServicePrimitivesQuorumsApkUpdate is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesQuorumsApkUpdate struct {
	QuorumNumber uint8
	QuorumApk    BN254G1Point
}

// IGaspMultiRollupServicePrimitivesQuorumsStakeUpdate is an auto generated low-level Go binding around an user-defined struct.
type IGaspMultiRollupServicePrimitivesQuorumsStakeUpdate struct {
	QuorumNumber uint8
	QuorumStake  *big.Int
}

// ContractGaspMultiRollupServiceMetaData contains all meta data concerning the ContractGaspMultiRollupService contract.
var ContractGaspMultiRollupServiceMetaData = &bind.MetaData{
	ABI: "[{\"type\":\"function\",\"name\":\"checkSignatures\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"params\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState\",\"components\":[{\"name\":\"nonSignerG1PubkeysForOldState\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2ForOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigmaForOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]}],\"outputs\":[{\"name\":\"\",\"type\":\"tuple\",\"internalType\":\"structIBLSSignatureChecker.QuorumStakeTotals\",\"components\":[{\"name\":\"signedStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"totalStakeForQuorum\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"initialize\",\"inputs\":[{\"name\":\"_pauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"initialOwner\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_updater\",\"type\":\"address\",\"internalType\":\"address\"},{\"name\":\"_rolldown\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"lastUpdateBlockTimestamp\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestCompletedTaskCreatedBlock\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestCompletedTaskNumber\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"latestPendingStateHash\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorAndQuorumToStakes\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint96\",\"internalType\":\"uint96\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"operatorIdQuorumCount\",\"inputs\":[{\"name\":\"\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"owner\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"pauseAll\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[{\"name\":\"index\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"paused\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"pauserRegistry\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"process_eigen_reinit\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.Task\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.TaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"process_eigen_update\",\"inputs\":[{\"name\":\"task\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.Task\",\"components\":[{\"name\":\"taskNum\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"blockNumber\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskCreatedBlock\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"quorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"quorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"lastCompletedTaskQuorumNumbers\",\"type\":\"bytes\",\"internalType\":\"bytes\"},{\"name\":\"lastCompletedTaskQuorumThresholdPercentage\",\"type\":\"uint32\",\"internalType\":\"uint32\"}]},{\"name\":\"taskResponse\",\"type\":\"tuple\",\"internalType\":\"structIFinalizerTaskManager.TaskResponse\",\"components\":[{\"name\":\"referenceTaskIndex\",\"type\":\"uint32\",\"internalType\":\"uint32\"},{\"name\":\"referenceTaskHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"operatorsStateInfoHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"blockHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"storageProofHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"pendingStateHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"}]},{\"name\":\"nonSignerStakesAndSignatureForOldState\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.NonSignerStakesAndSignatureForOldState\",\"components\":[{\"name\":\"nonSignerG1PubkeysForOldState\",\"type\":\"tuple[]\",\"internalType\":\"structBN254.G1Point[]\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2ForOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigmaForOldState\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorStateInfo\",\"type\":\"tuple\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorStateInfo\",\"components\":[{\"name\":\"operatorsStateChanged\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"quorumsRemoved\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsAdded[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"quorumsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsStakeUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumStake\",\"type\":\"uint96\",\"internalType\":\"uint96\"}]},{\"name\":\"quorumsApkUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.QuorumsApkUpdate[]\",\"components\":[{\"name\":\"quorumNumber\",\"type\":\"uint8\",\"internalType\":\"uint8\"},{\"name\":\"quorumApk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}]},{\"name\":\"operatorsRemoved\",\"type\":\"bytes32[]\",\"internalType\":\"bytes32[]\"},{\"name\":\"operatorsAdded\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsAdded[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]},{\"name\":\"operatorsStakeUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsStakeUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumForStakes\",\"type\":\"uint8[]\",\"internalType\":\"uint8[]\"},{\"name\":\"quorumStakes\",\"type\":\"uint96[]\",\"internalType\":\"uint96[]\"}]},{\"name\":\"operatorsQuorumCountUpdate\",\"type\":\"tuple[]\",\"internalType\":\"structIGaspMultiRollupServicePrimitives.OperatorsQuorumCountUpdate[]\",\"components\":[{\"name\":\"operatorId\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"quorumCount\",\"type\":\"uint8\",\"internalType\":\"uint8\"}]}]}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"qourumApk\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"quorumNumbers\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bytes\",\"internalType\":\"bytes\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"quorumThresholdPercentage\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"uint32\",\"internalType\":\"uint32\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"quorumToStakes\",\"inputs\":[{\"name\":\"\",\"type\":\"uint8\",\"internalType\":\"uint8\"}],\"outputs\":[{\"name\":\"\",\"type\":\"uint96\",\"internalType\":\"uint96\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"renounceOwnership\",\"inputs\":[],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"rolldown\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"setPauserRegistry\",\"inputs\":[{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"internalType\":\"contractIPauserRegistry\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"set_rolldown\",\"inputs\":[{\"name\":\"_rolldown\",\"type\":\"address\",\"internalType\":\"contractIRolldown\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"set_updater\",\"inputs\":[{\"name\":\"_updater\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"stalled\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"transferOwnership\",\"inputs\":[{\"name\":\"newOwner\",\"type\":\"address\",\"internalType\":\"address\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"trySignatureAndApkVerification\",\"inputs\":[{\"name\":\"msgHash\",\"type\":\"bytes32\",\"internalType\":\"bytes32\"},{\"name\":\"apk\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]},{\"name\":\"apkG2\",\"type\":\"tuple\",\"internalType\":\"structBN254.G2Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"},{\"name\":\"Y\",\"type\":\"uint256[2]\",\"internalType\":\"uint256[2]\"}]},{\"name\":\"sigma\",\"type\":\"tuple\",\"internalType\":\"structBN254.G1Point\",\"components\":[{\"name\":\"X\",\"type\":\"uint256\",\"internalType\":\"uint256\"},{\"name\":\"Y\",\"type\":\"uint256\",\"internalType\":\"uint256\"}]}],\"outputs\":[{\"name\":\"pairingSuccessful\",\"type\":\"bool\",\"internalType\":\"bool\"},{\"name\":\"siganatureIsValid\",\"type\":\"bool\",\"internalType\":\"bool\"}],\"stateMutability\":\"view\"},{\"type\":\"function\",\"name\":\"unpause\",\"inputs\":[{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"internalType\":\"uint256\"}],\"outputs\":[],\"stateMutability\":\"nonpayable\"},{\"type\":\"function\",\"name\":\"updater\",\"inputs\":[],\"outputs\":[{\"name\":\"\",\"type\":\"address\",\"internalType\":\"address\"}],\"stateMutability\":\"view\"},{\"type\":\"event\",\"name\":\"EigenReinitProcessed\",\"inputs\":[{\"name\":\"taskNumber\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"EigenUpdateProcessed\",\"inputs\":[{\"name\":\"taskNumber\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"},{\"name\":\"taskCreatedBlock\",\"type\":\"uint32\",\"indexed\":false,\"internalType\":\"uint32\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Initialized\",\"inputs\":[{\"name\":\"version\",\"type\":\"uint8\",\"indexed\":false,\"internalType\":\"uint8\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"inputs\":[{\"name\":\"previousOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newOwner\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Paused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"PauserRegistrySet\",\"inputs\":[{\"name\":\"pauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"},{\"name\":\"newPauserRegistry\",\"type\":\"address\",\"indexed\":false,\"internalType\":\"contractIPauserRegistry\"}],\"anonymous\":false},{\"type\":\"event\",\"name\":\"Unpaused\",\"inputs\":[{\"name\":\"account\",\"type\":\"address\",\"indexed\":true,\"internalType\":\"address\"},{\"name\":\"newPausedStatus\",\"type\":\"uint256\",\"indexed\":false,\"internalType\":\"uint256\"}],\"anonymous\":false}]",
	Bin: "0x608060405234801561001057600080fd5b506149d5806100206000396000f3fe608060405234801561001057600080fd5b50600436106101da5760003560e01c80635ac86ab711610104578063c927feef116100a2578063f2fde38b11610071578063f2fde38b146104bb578063f8c8765e146104ce578063fabc1cbc146104e1578063fc765dd5146104f457600080fd5b8063c927feef14610474578063df034cd014610487578063e61db1751461049a578063ed5a04fe146104a357600080fd5b80637ad75561116100de5780637ad7556114610407578063886f1195146104305780638da5cb5b14610443578063c4e1914c1461045457600080fd5b80635ac86ab7146103d45780635c975abb146103f7578063715018a6146103ff57600080fd5b80633d9fb00c1161017c5780634ae6b2031161014b5780634ae6b2031461036c5780634deabc2114610383578063526e3e64146103a8578063595c6a67146103cc57600080fd5b80633d9fb00c146102ad5780633e404926146102d8578063430d3b39146102eb578063499d6fb61461032057600080fd5b8063136439dd116101b8578063136439dd14610248578063171f1d5b1461025b5780631e2d4bf7146102855780632a8414fd1461029857600080fd5b806303d097d2146101df57806310d67a2f14610220578063124648c914610235575b600080fd5b6102066101ed3660046137af565b60a0602052600090815260409020805460019091015482565b604080519283526020830191909152015b60405180910390f35b61023361022e3660046137e6565b610504565b005b6102336102433660046137e6565b6105c0565b610233610256366004613803565b6105ea565b61026e610269366004613980565b610729565b604080519215158352901515602083015201610217565b6102336102933660046137e6565b6108b3565b6102a06108dd565b60405161021791906139d1565b6098546102c0906001600160a01b031681565b6040516001600160a01b039091168152602001610217565b6102336102e6366004613a64565b61096b565b61030e6102f9366004613803565b60a16020526000908152604090205460ff1681565b60405160ff9091168152602001610217565b61035461032e366004613b07565b609f6020908152600092835260408084209091529082529020546001600160601b031681565b6040516001600160601b039091168152602001610217565b610375609a5481565b604051908152602001610217565b609d546103939063ffffffff1681565b60405163ffffffff9091168152602001610217565b6098546103bc90600160a01b900460ff1681565b6040519015158152602001610217565b6102336117a7565b6103bc6103e23660046137af565b606654600160ff9092169190911b9081161490565b606654610375565b61023361186e565b6103546104153660046137af565b609e602052600090815260409020546001600160601b031681565b6065546102c0906001600160a01b031681565b6033546001600160a01b03166102c0565b610467610462366004613c13565b611882565b6040516102179190613c9d565b610233610482366004613cdf565b611e6f565b6097546102c0906001600160a01b031681565b61037560995481565b609b5461039390640100000000900463ffffffff1681565b6102336104c93660046137e6565b612933565b6102336104dc366004613d54565b6129a9565b6102336104ef366004613803565b612afa565b609b546103939063ffffffff1681565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610557573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061057b9190613db0565b6001600160a01b0316336001600160a01b0316146105b45760405162461bcd60e51b81526004016105ab90613dcd565b60405180910390fd5b6105bd81612c56565b50565b6105c8612d4d565b609780546001600160a01b0319166001600160a01b0392909216919091179055565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa158015610632573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906106569190613e25565b6106725760405162461bcd60e51b81526004016105ab90613e42565b606654818116146106eb5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e70617573653a20696e76616c696420617474656d70742060448201527f746f20756e70617573652066756e6374696f6e616c697479000000000000000060648201526084016105ab565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d906020015b60405180910390a250565b60008060007f30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f00000018787600001518860200151886000015160006002811061077157610771613e8a565b60200201518951600160200201518a6020015160006002811061079657610796613e8a565b60200201518b602001516001600281106107b2576107b2613e8a565b602090810291909101518c518d83015160405161080f9a99989796959401988952602089019790975260408801959095526060870193909352608086019190915260a085015260c084015260e08301526101008201526101200190565b6040516020818303038152906040528051906020012060001c6108329190613ea0565b90506108a561084b6108448884612da7565b8690612e3e565b610853612ed2565b61089b61088c85610886604080518082018252600080825260209182015281518083019092526001825260029082015290565b90612da7565b6108958c612f92565b90612e3e565b886201d4c0613022565b909890975095505050505050565b6108bb612d4d565b609880546001600160a01b0319166001600160a01b0392909216919091179055565b609c80546108ea90613ec2565b80601f016020809104026020016040519081016040528092919081815260200182805461091690613ec2565b80156109635780601f1061093857610100808354040283529160200191610963565b820191906000526020600020905b81548152906001019060200180831161094657829003601f168201915b505050505081565b6097546001600160a01b031633146109c55760405162461bcd60e51b815260206004820152601a60248201527f55706461746572206d757374206265207468652063616c6c657200000000000060448201526064016105ab565b6109d56080850160608601613f0b565b609b54640100000000900463ffffffff908116911614610a375760405162461bcd60e51b815260206004820152601d60248201527f7265666572656e636520626c6f636b2068617368206d69736d6174636800000060448201526064016105ab565b83604051602001610a489190613f9b565b60405160208183030381529060405280519060200120836020013514610ab05760405162461bcd60e51b815260206004820152601f60248201527f7265666572656e63655461736b486173682068617368206d69736d617463680060448201526064016105ab565b80604051602001610ac19190614507565b60405160208183030381529060405280519060200120836040013514610b295760405162461bcd60e51b815260206004820152601f60248201527f6f70657261746f725374617465496e666f2068617368206d69736d617463680060448201526064016105ab565b609b54640100000000900463ffffffff1615610ce957610b4f6060850160408601613f0b565b609b5463ffffffff91821691610b7091640100000000900416613840614684565b63ffffffff1611610bb35760405162461bcd60e51b815260206004820152600d60248201526c07374616c65207374617465203609c1b60448201526064016105ab565b426099546203f480610bc591906146ac565b11610c025760405162461bcd60e51b815260206004820152600d60248201526c7374616c65207374617465203160981b60448201526064016105ab565b6000610c3884604051602001610c1891906146c4565b604051602081830303815290604052805190602001208461046290614711565b609d5490915063ffffffff1660005b609c8054610c5490613ec2565b9050811015610ce5578160ff1683602001518281518110610c7757610c77613e8a565b6020026020010151610c89919061471d565b6001600160601b0316606484600001518381518110610caa57610caa613e8a565b60200260200101516001600160601b0316610cc5919061474c565b1015610cd3575050506117a1565b80610cdd8161476b565b915050610c47565b5050505b60005b610cf96020830183614786565b9050811015610dbe57609e6000610d136020850185614786565b84818110610d2357610d23613e8a565b9050602002016020810190610d3891906137af565b60ff168152602080820192909252604001600090812080546001600160601b031916905560a091610d6b90850185614786565b84818110610d7b57610d7b613e8a565b9050602002016020810190610d9091906137af565b60ff168152602081019190915260400160009081208181556001015580610db68161476b565b915050610cec565b5060005b610dcf60408301836147cf565b9050811015610f0a57610de560408301836147cf565b82818110610df557610df5613e8a565b9050608002016020016020810190610e0d9190614818565b609e6000610e1e60408601866147cf565b85818110610e2e57610e2e613e8a565b610e4492602060809092020190810191506137af565b60ff1681526020810191909152604090810160002080546001600160601b0319166001600160601b039390931692909217909155610e84908301836147cf565b82818110610e9457610e94613e8a565b90506080020160400160a06000848060400190610eb191906147cf565b85818110610ec157610ec1613e8a565b610ed792602060809092020190810191506137af565b60ff1681526020808201929092526040016000208235815591013560019091015580610f028161476b565b915050610dc2565b5060005b610f1b6060830183614833565b9050811015610fd357610f316060830183614833565b82818110610f4157610f41613e8a565b9050604002016020016020810190610f599190614818565b609e6000610f6a6060860186614833565b85818110610f7a57610f7a613e8a565b610f9092602060409092020190810191506137af565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b039290921691909117905580610fcb8161476b565b915050610f0e565b5060005b610fe4608083018361487c565b905081101561108057610ffa608083018361487c565b8281811061100a5761100a613e8a565b90506060020160200160a06000848060800190611027919061487c565b8581811061103757611037613e8a565b61104d92602060609092020190810191506137af565b60ff16815260208082019290925260400160002082358155910135600190910155806110788161476b565b915050610fd7565b5060005b61109160a0830183614786565b90508110156111b85760005b609c80546110aa90613ec2565b905081101561116357609f60006110c460a0860186614786565b858181106110d4576110d4613e8a565b9050602002013581526020019081526020016000206000609c8381546110f990613ec2565b811061110757611107613e8a565b8154600116156111265790600052602060002090602091828204019190065b9054600160f81b911a0260f81c8152602081019190915260400160002080546001600160601b03191690558161115b8161476b565b92505061109d565b5060a1600061117560a0850185614786565b8481811061118557611185613e8a565b60209081029290920135835250810191909152604001600020805460ff19169055806111b08161476b565b915050611084565b5060005b6111c960c0830183614786565b9050811015611419576111df60c0830183614786565b828181106111ef576111ef613e8a565b905060200281019061120191906148c4565b6112129060808101906060016137af565b60a1600061122360c0860186614786565b8581811061123357611233613e8a565b905060200281019061124591906148c4565b60000135815260200190815260200160002060006101000a81548160ff021916908360ff16021790555060005b61127f60c0840184614786565b8381811061128f5761128f613e8a565b90506020028101906112a191906148c4565b6112af906020810190614786565b9050811015611406576112c560c0840184614786565b838181106112d5576112d5613e8a565b90506020028101906112e791906148c4565b6112f5906040810190614786565b8281811061130557611305613e8a565b905060200201602081019061131a9190614818565b609f600061132b60c0870187614786565b8681811061133b5761133b613e8a565b905060200281019061134d91906148c4565b358152602081019190915260400160009081209061136e60c0870187614786565b8681811061137e5761137e613e8a565b905060200281019061139091906148c4565b61139e906020810190614786565b858181106113ae576113ae613e8a565b90506020020160208101906113c391906137af565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b0392909216919091179055806113fe8161476b565b915050611272565b50806114118161476b565b9150506111bc565b5060005b61142a60e0830183614786565b90508110156115dd5760005b61144360e0840184614786565b8381811061145357611453613e8a565b905060200281019061146591906148e4565b611473906020810190614786565b90508110156115ca5761148960e0840184614786565b8381811061149957611499613e8a565b90506020028101906114ab91906148e4565b6114b9906040810190614786565b828181106114c9576114c9613e8a565b90506020020160208101906114de9190614818565b609f60006114ef60e0870187614786565b868181106114ff576114ff613e8a565b905060200281019061151191906148e4565b358152602081019190915260400160009081209061153260e0870187614786565b8681811061154257611542613e8a565b905060200281019061155491906148e4565b611562906020810190614786565b8581811061157257611572613e8a565b905060200201602081019061158791906137af565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b0392909216919091179055806115c28161476b565b915050611436565b50806115d58161476b565b91505061141d565b5060005b6115ef610100830183614833565b905081101561169357611606610100830183614833565b8281811061161657611616613e8a565b905060400201602001602081019061162e91906137af565b60a16000611640610100860186614833565b8581811061165057611650613e8a565b90506040020160000135815260200190815260200160002060006101000a81548160ff021916908360ff160217905550808061168b9061476b565b9150506115e1565b5060a0830135609a556116a96020850185613f0b565b609b805463ffffffff191663ffffffff929092169190911790556116d36060850160408601613f0b565b609b805463ffffffff929092166401000000000267ffffffff00000000199092169190911790554260995561170b60808501856148fa565b61171791609c9161362b565b5061172860c0850160a08601613f0b565b609d805463ffffffff191663ffffffff929092169190911790557f9a128fe7347f1e11ca22aa9deb632ec9abb09608c13a994c60f77a562f4601716117706020860186613f0b565b6117806060870160408801613f0b565b6040805163ffffffff93841681529290911660208301520160405180910390a15b50505050565b60655460405163237dfb4760e11b81523360048201526001600160a01b03909116906346fbf68e90602401602060405180830381865afa1580156117ef573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906118139190613e25565b61182f5760405162461bcd60e51b81526004016105ab90613e42565b600019606681905560405190815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a2565b611876612d4d565b6118806000613246565b565b604080518082019091526060808252602082015260408051808201909152600080825260208201819052609c80546118b990613ec2565b905090506118da604051806040016040528060608152602001606081525090565b816001600160401b038111156118f2576118f261381c565b60405190808252806020026020018201604052801561191b578160200160208202803683370190505b506020820152816001600160401b038111156119395761193961381c565b604051908082528060200260200182016040528015611962578160200160208202803683370190505b5081528451516000906001600160401b038111156119825761198261381c565b6040519080825280602002602001820160405280156119ab578160200160208202803683370190505b5090506000805b875151811015611b66576119f4886000015182815181106119d5576119d5613e8a565b6020026020010151805160009081526020918201519091526040902090565b838281518110611a0657611a06613e8a565b60209081029190910101528015611ae45782611a23600183614940565b81518110611a3357611a33613e8a565b602002602001015160001c838281518110611a5057611a50613e8a565b602002602001015160001c11611ae45760405162461bcd60e51b815260206004820152604d60248201527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a206e6f6e5369676e657247315075626b657973466f724f6c6453746160648201526c1d19481b9bdd081cdbdc9d1959609a1b608482015260a4016105ab565b611b52611b4b60a16000868581518110611b0057611b00613e8a565b6020908102919091018101518252810191909152604001600020548a51805160ff9092169185908110611b3557611b35613e8a565b602002602001015161329890919063ffffffff16565b8790612e3e565b955080611b5e8161476b565b9150506119b2565b50611b708561337c565b945060005b84811015611d5157609c818154611b8b90613ec2565b8110611b9957611b99613e8a565b815460011615611bb85790600052602060002090602091828204019190065b9054600160f81b911a0260f81c600081815260a060209081526040918290208251808401909352805483526001015490820152909250611bf9908790612e3e565b60ff83166000908152609e60209081526040909120549086015180519298506001600160601b039091169183908110611c3457611c34613e8a565b6001600160601b03909216602092830291909101820152840151805182908110611c6057611c60613e8a565b602002602001015184600001518281518110611c7e57611c7e613e8a565b60200260200101906001600160601b031690816001600160601b03168152505060005b885151811015611d3e57609f6000858381518110611cc157611cc1613e8a565b6020908102919091018101518252818101929092526040908101600090812060ff87168252909252902054855180516001600160601b039092169184908110611d0c57611d0c613e8a565b60200260200101818151611d209190614957565b6001600160601b031690525080611d368161476b565b915050611ca1565b5080611d498161476b565b915050611b75565b50600080611d698a888b602001518c60400151610729565b9150915081611dec5760405162461bcd60e51b815260206004820152604360248201527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a2070616972696e6720707265636f6d70696c652063616c6c206661696064820152621b195960ea1b608482015260a4016105ab565b80611e5f5760405162461bcd60e51b815260206004820152603960248201527f424c535369676e6174757265436865636b65722e636865636b5369676e61747560448201527f7265733a207369676e617475726520697320696e76616c69640000000000000060648201526084016105ab565b5092955050505050505b92915050565b611e77612d4d565b60005b611e876020830183614786565b9050811015611f4c57609e6000611ea16020850185614786565b84818110611eb157611eb1613e8a565b9050602002016020810190611ec691906137af565b60ff168152602080820192909252604001600090812080546001600160601b031916905560a091611ef990850185614786565b84818110611f0957611f09613e8a565b9050602002016020810190611f1e91906137af565b60ff168152602081019190915260400160009081208181556001015580611f448161476b565b915050611e7a565b5060005b611f5d60408301836147cf565b905081101561209857611f7360408301836147cf565b82818110611f8357611f83613e8a565b9050608002016020016020810190611f9b9190614818565b609e6000611fac60408601866147cf565b85818110611fbc57611fbc613e8a565b611fd292602060809092020190810191506137af565b60ff1681526020810191909152604090810160002080546001600160601b0319166001600160601b039390931692909217909155612012908301836147cf565b8281811061202257612022613e8a565b90506080020160400160a0600084806040019061203f91906147cf565b8581811061204f5761204f613e8a565b61206592602060809092020190810191506137af565b60ff16815260208082019290925260400160002082358155910135600190910155806120908161476b565b915050611f50565b5060005b6120a96060830183614833565b9050811015612161576120bf6060830183614833565b828181106120cf576120cf613e8a565b90506040020160200160208101906120e79190614818565b609e60006120f86060860186614833565b8581811061210857612108613e8a565b61211e92602060409092020190810191506137af565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b0392909216919091179055806121598161476b565b91505061209c565b5060005b612172608083018361487c565b905081101561220e57612188608083018361487c565b8281811061219857612198613e8a565b90506060020160200160a060008480608001906121b5919061487c565b858181106121c5576121c5613e8a565b6121db92602060609092020190810191506137af565b60ff16815260208082019290925260400160002082358155910135600190910155806122068161476b565b915050612165565b5060005b61221f60a0830183614786565b90508110156123465760005b609c805461223890613ec2565b90508110156122f157609f600061225260a0860186614786565b8581811061226257612262613e8a565b9050602002013581526020019081526020016000206000609c83815461228790613ec2565b811061229557612295613e8a565b8154600116156122b45790600052602060002090602091828204019190065b9054600160f81b911a0260f81c8152602081019190915260400160002080546001600160601b0319169055816122e98161476b565b92505061222b565b5060a1600061230360a0850185614786565b8481811061231357612313613e8a565b60209081029290920135835250810191909152604001600020805460ff191690558061233e8161476b565b915050612212565b5060005b61235760c0830183614786565b90508110156125a75761236d60c0830183614786565b8281811061237d5761237d613e8a565b905060200281019061238f91906148c4565b6123a09060808101906060016137af565b60a160006123b160c0860186614786565b858181106123c1576123c1613e8a565b90506020028101906123d391906148c4565b60000135815260200190815260200160002060006101000a81548160ff021916908360ff16021790555060005b61240d60c0840184614786565b8381811061241d5761241d613e8a565b905060200281019061242f91906148c4565b61243d906020810190614786565b90508110156125945761245360c0840184614786565b8381811061246357612463613e8a565b905060200281019061247591906148c4565b612483906040810190614786565b8281811061249357612493613e8a565b90506020020160208101906124a89190614818565b609f60006124b960c0870187614786565b868181106124c9576124c9613e8a565b90506020028101906124db91906148c4565b35815260208101919091526040016000908120906124fc60c0870187614786565b8681811061250c5761250c613e8a565b905060200281019061251e91906148c4565b61252c906020810190614786565b8581811061253c5761253c613e8a565b905060200201602081019061255191906137af565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b03929092169190911790558061258c8161476b565b915050612400565b508061259f8161476b565b91505061234a565b5060005b6125b860e0830183614786565b905081101561276b5760005b6125d160e0840184614786565b838181106125e1576125e1613e8a565b90506020028101906125f391906148e4565b612601906020810190614786565b90508110156127585761261760e0840184614786565b8381811061262757612627613e8a565b905060200281019061263991906148e4565b612647906040810190614786565b8281811061265757612657613e8a565b905060200201602081019061266c9190614818565b609f600061267d60e0870187614786565b8681811061268d5761268d613e8a565b905060200281019061269f91906148e4565b35815260208101919091526040016000908120906126c060e0870187614786565b868181106126d0576126d0613e8a565b90506020028101906126e291906148e4565b6126f0906020810190614786565b8581811061270057612700613e8a565b905060200201602081019061271591906137af565b60ff168152602081019190915260400160002080546001600160601b0319166001600160601b0392909216919091179055806127508161476b565b9150506125c4565b50806127638161476b565b9150506125ab565b5060005b61277d610100830183614833565b905081101561282157612794610100830183614833565b828181106127a4576127a4613e8a565b90506040020160200160208101906127bc91906137af565b60a160006127ce610100860186614833565b858181106127de576127de613e8a565b90506040020160000135815260200190815260200160002060006101000a81548160ff021916908360ff16021790555080806128199061476b565b91505061276f565b5060a0820135609a556128376020840184613f0b565b609b805463ffffffff191663ffffffff929092169190911790556128616060840160408501613f0b565b609b805463ffffffff929092166401000000000267ffffffff00000000199092169190911790554260995561289960808401846148fa565b6128a591609c9161362b565b506128b660c0840160a08501613f0b565b609d805463ffffffff191663ffffffff929092169190911790557f264965eb6bc436c6c473431d34af56e832ec344fdfd43ee6af6fce6d205e84af6128fe6020850185613f0b565b61290e6060860160408701613f0b565b6040805163ffffffff93841681529290911660208301520160405180910390a1505050565b61293b612d4d565b6001600160a01b0381166129a05760405162461bcd60e51b815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201526564647265737360d01b60648201526084016105ab565b6105bd81613246565b600054610100900460ff16158080156129c95750600054600160ff909116105b806129e35750303b1580156129e3575060005460ff166001145b612a465760405162461bcd60e51b815260206004820152602e60248201527f496e697469616c697a61626c653a20636f6e747261637420697320616c72656160448201526d191e481a5b9a5d1a585b1a5e995960921b60648201526084016105ab565b6000805460ff191660011790558015612a69576000805461ff0019166101001790555b612a74856000613417565b612a7d84613246565b609780546001600160a01b038086166001600160a01b03199283161790925560988054928516929091169190911790558015612af3576000805461ff0019169055604051600181527f7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb38474024989060200160405180910390a15b5050505050565b606560009054906101000a90046001600160a01b03166001600160a01b031663eab66d7a6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612b4d573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190612b719190613db0565b6001600160a01b0316336001600160a01b031614612ba15760405162461bcd60e51b81526004016105ab90613dcd565b606654198119606654191614612c1f5760405162461bcd60e51b815260206004820152603860248201527f5061757361626c652e756e70617573653a20696e76616c696420617474656d7060448201527f7420746f2070617573652066756e6374696f6e616c697479000000000000000060648201526084016105ab565b606681905560405181815233907f3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c9060200161071e565b6001600160a01b038116612ce45760405162461bcd60e51b815260206004820152604960248201527f5061757361626c652e5f73657450617573657252656769737472793a206e657760448201527f50617573657252656769737472792063616e6e6f7420626520746865207a65726064820152686f206164647265737360b81b608482015260a4016105ab565b606554604080516001600160a01b03928316815291831660208301527f6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6910160405180910390a1606580546001600160a01b0319166001600160a01b0392909216919091179055565b6033546001600160a01b031633146118805760405162461bcd60e51b815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e657260448201526064016105ab565b6040805180820190915260008082526020820152612dc36136af565b835181526020808501519082015260408082018490526000908360608460076107d05a03fa9050808015612df657612df8565bfe5b5080612e365760405162461bcd60e51b815260206004820152600d60248201526c1958cb5b5d5b0b59985a5b1959609a1b60448201526064016105ab565b505092915050565b6040805180820190915260008082526020820152612e5a6136cd565b835181526020808501518183015283516040808401919091529084015160608301526000908360808460066107d05a03fa9050808015612df6575080612e365760405162461bcd60e51b815260206004820152600d60248201526c1958cb5859190b59985a5b1959609a1b60448201526064016105ab565b612eda6136eb565b50604080516080810182527f198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c28183019081527f1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed6060830152815281518083019092527f275dc4a288d1afb3cbb1ac09187524c7db36395df7be3b99e673b13a075a65ec82527f1d9befcd05a5323e6da4d435f3b617cdb3af83285c2df711ef39c01571827f9d60208381019190915281019190915290565b604080518082019091526000808252602082015260008080612fc260008051602061498083398151915286613ea0565b90505b612fce81613501565b9093509150600080516020614980833981519152828309831415613008576040805180820190915290815260208101919091529392505050565b600080516020614980833981519152600182089050612fc5565b604080518082018252868152602080820186905282518084019093528683528201849052600091829190613054613710565b60005b600281101561321957600061306d82600661474c565b905084826002811061308157613081613e8a565b602002015151836130938360006146ac565b600c81106130a3576130a3613e8a565b60200201528482600281106130ba576130ba613e8a565b602002015160200151838260016130d191906146ac565b600c81106130e1576130e1613e8a565b60200201528382600281106130f8576130f8613e8a565b602002015151518361310b8360026146ac565b600c811061311b5761311b613e8a565b602002015283826002811061313257613132613e8a565b602002015151600160200201518361314b8360036146ac565b600c811061315b5761315b613e8a565b602002015283826002811061317257613172613e8a565b60200201516020015160006002811061318d5761318d613e8a565b60200201518361319e8360046146ac565b600c81106131ae576131ae613e8a565b60200201528382600281106131c5576131c5613e8a565b6020020151602001516001600281106131e0576131e0613e8a565b6020020151836131f18360056146ac565b600c811061320157613201613e8a565b602002015250806132118161476b565b915050613057565b5061322261372f565b60006020826101808560088cfa9151919c9115159b50909950505050505050505050565b603380546001600160a01b038381166001600160a01b0319831681179093556040519116919082907f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e090600090a35050565b60408051808201909152600080825260208201526102008261ffff16106132f45760405162461bcd60e51b815260206004820152601060248201526f7363616c61722d746f6f2d6c6172676560801b60448201526064016105ab565b8161ffff1660011415613308575081611e69565b6040805180820190915260008082526020820181905284906001905b8161ffff168661ffff161061337157600161ffff871660ff83161c81161415613354576133518484612e3e565b93505b61335e8384612e3e565b92506201fffe600192831b169101613324565b509195945050505050565b604080518082019091526000808252602082015281511580156133a157506020820151155b156133bf575050604080518082019091526000808252602082015290565b60405180604001604052808360000151815260200160008051602061498083398151915284602001516133f29190613ea0565b61340a90600080516020614980833981519152614940565b905292915050565b919050565b6065546001600160a01b031615801561343857506001600160a01b03821615155b6134ba5760405162461bcd60e51b815260206004820152604760248201527f5061757361626c652e5f696e697469616c697a655061757365723a205f696e6960448201527f7469616c697a6550617573657228292063616e206f6e6c792062652063616c6c6064820152666564206f6e636560c81b608482015260a4016105ab565b606681905560405181815233907fab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d9060200160405180910390a26134fd82612c56565b5050565b60008080600080516020614980833981519152600360008051602061498083398151915286600080516020614980833981519152888909090890506000613577827f0c19139cb84c680a6e14116da060561765e05aa45a1c72a34f082305b61f3f52600080516020614980833981519152613583565b91959194509092505050565b60008061358e61372f565b61359661374d565b602080825281810181905260408201819052606082018890526080820187905260a082018690528260c08360056107d05a03fa9250828015612df65750826136205760405162461bcd60e51b815260206004820152601a60248201527f424e3235342e6578704d6f643a2063616c6c206661696c75726500000000000060448201526064016105ab565b505195945050505050565b82805461363790613ec2565b90600052602060002090601f016020900481019282613659576000855561369f565b82601f106136725782800160ff1982351617855561369f565b8280016001018555821561369f579182015b8281111561369f578235825591602001919060010190613684565b506136ab92915061376b565b5090565b60405180606001604052806003906020820280368337509192915050565b60405180608001604052806004906020820280368337509192915050565b60405180604001604052806136fe613780565b815260200161370b613780565b905290565b604051806101800160405280600c906020820280368337509192915050565b60405180602001604052806001906020820280368337509192915050565b6040518060c001604052806006906020820280368337509192915050565b5b808211156136ab576000815560010161376c565b60405180604001604052806002906020820280368337509192915050565b803560ff8116811461341257600080fd5b6000602082840312156137c157600080fd5b6137ca8261379e565b9392505050565b6001600160a01b03811681146105bd57600080fd5b6000602082840312156137f857600080fd5b81356137ca816137d1565b60006020828403121561381557600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b03811182821017156138545761385461381c565b60405290565b604051606081016001600160401b03811182821017156138545761385461381c565b604051601f8201601f191681016001600160401b03811182821017156138a4576138a461381c565b604052919050565b6000604082840312156138be57600080fd5b6138c6613832565b9050813581526020820135602082015292915050565b600082601f8301126138ed57600080fd5b604051604081018181106001600160401b038211171561390f5761390f61381c565b806040525080604084018581111561392657600080fd5b845b81811015613371578035835260209283019201613928565b60006080828403121561395257600080fd5b61395a613832565b905061396683836138dc565b815261397583604084016138dc565b602082015292915050565b600080600080610120858703121561399757600080fd5b843593506139a886602087016138ac565b92506139b78660608701613940565b91506139c68660e087016138ac565b905092959194509250565b600060208083528351808285015260005b818110156139fe578581018301518582016040015282016139e2565b81811115613a10576000604083870101525b50601f01601f1916929092016040019392505050565b60006101008284031215613a3957600080fd5b50919050565b600060c08284031215613a3957600080fd5b60006101208284031215613a3957600080fd5b6000806000806101208587031215613a7b57600080fd5b84356001600160401b0380821115613a9257600080fd5b613a9e88838901613a26565b9550613aad8860208901613a3f565b945060e0870135915080821115613ac357600080fd5b9086019060e08289031215613ad757600080fd5b9092506101008601359080821115613aee57600080fd5b50613afb87828801613a51565b91505092959194509250565b60008060408385031215613b1a57600080fd5b82359150613b2a6020840161379e565b90509250929050565b600060e08284031215613b4557600080fd5b613b4d61385a565b905081356001600160401b0380821115613b6657600080fd5b818401915084601f830112613b7a57600080fd5b8135602082821115613b8e57613b8e61381c565b613b9c818360051b0161387c565b828152818101935060069290921b840181019187831115613bbc57600080fd5b938101935b82851015613be557613bd388866138ac565b84528184019350604085019450613bc1565b8552613bf387878301613940565b8186015250505050613c088360a084016138ac565b604082015292915050565b60008060408385031215613c2657600080fd5b8235915060208301356001600160401b03811115613c4357600080fd5b613c4f85828601613b33565b9150509250929050565b600081518084526020808501945080840160005b83811015613c925781516001600160601b031687529582019590820190600101613c6d565b509495945050505050565b602081526000825160406020840152613cb96060840182613c59565b90506020840151601f19848303016040850152613cd68282613c59565b95945050505050565b60008060006101008486031215613cf557600080fd5b83356001600160401b0380821115613d0c57600080fd5b613d1887838801613a26565b9450613d278760208801613a3f565b935060e0860135915080821115613d3d57600080fd5b50613d4a86828701613a51565b9150509250925092565b60008060008060808587031215613d6a57600080fd5b8435613d75816137d1565b93506020850135613d85816137d1565b92506040850135613d95816137d1565b91506060850135613da5816137d1565b939692955090935050565b600060208284031215613dc257600080fd5b81516137ca816137d1565b6020808252602a908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526939903ab73830bab9b2b960b11b606082015260800190565b80151581146105bd57600080fd5b600060208284031215613e3757600080fd5b81516137ca81613e17565b60208082526028908201527f6d73672e73656e646572206973206e6f74207065726d697373696f6e6564206160408201526739903830bab9b2b960c11b606082015260800190565b634e487b7160e01b600052603260045260246000fd5b600082613ebd57634e487b7160e01b600052601260045260246000fd5b500690565b600181811c90821680613ed657607f821691505b60208210811415613a3957634e487b7160e01b600052602260045260246000fd5b803563ffffffff8116811461341257600080fd5b600060208284031215613f1d57600080fd5b6137ca82613ef7565b6000808335601e19843603018112613f3d57600080fd5b83016020810192503590506001600160401b03811115613f5c57600080fd5b803603831315613f6b57600080fd5b9250929050565b81835281816020850137506000828201602090810191909152601f909101601f19169091010190565b6020815263ffffffff613fad83613ef7565b166020820152602082013560408201526000613fcb60408401613ef7565b63ffffffff8116606084015250613fe460608401613ef7565b63ffffffff8116608084015250613ffe6080840184613f26565b6101008060a086015261401661012086018385613f72565b925061402460a08701613ef7565b63ffffffff811660c0870152915061403f60c0870187613f26565b868503601f190160e08801529250614058848483613f72565b93505061406760e08701613ef7565b63ffffffff169401939093529392505050565b803561341281613e17565b6000808335601e1984360301811261409c57600080fd5b83016020810192503590506001600160401b038111156140bb57600080fd5b8060051b3603831315613f6b57600080fd5b8183526000602080850194508260005b85811015613c925760ff6140f08361379e565b16875295820195908201906001016140dd565b6000808335601e1984360301811261411a57600080fd5b83016020810192503590506001600160401b0381111561413957600080fd5b8060071b3603831315613f6b57600080fd5b80356001600160601b038116811461341257600080fd5b8183526000602080850194508260005b85811015613c925760ff6141858361379e565b1687526001600160601b0361419b84840161414b565b1683880152604082810135908801526060808301359088015260809687019690910190600101614172565b6000808335601e198436030181126141dd57600080fd5b83016020810192503590506001600160401b038111156141fc57600080fd5b8060061b3603831315613f6b57600080fd5b8183526000602080850194508260005b85811015613c925760ff6142318361379e565b1687526001600160601b0361424784840161414b565b1687840152604096870196919091019060010161421e565b6000808335601e1984360301811261427657600080fd5b83016020810192503590506001600160401b0381111561429557600080fd5b606081023603831315613f6b57600080fd5b8183526000602080850194508260005b85811015613c925760ff6142ca8361379e565b1687526142e583880184840180358252602090810135910152565b60609687019691909101906001016142b7565b81835260006001600160fb1b0383111561431157600080fd5b8260051b8083602087013760009401602001938452509192915050565b8183526000602080850194508260005b85811015613c92576001600160601b036143578361414b565b168752958201959082019060010161433e565b81835260006020808501808196508560051b81019150846000805b8881101561441b578385038a528235607e198936030181126143a5578283fd5b88018035865260806143b988830183614085565b828a8a01526143cb838a0182846140cd565b9250505060406143dd81840184614085565b898403838b01526143ef84828461432e565b9350505050606060ff61440382850161379e565b16970196909652509885019891850191600101614385565b509298975050505050505050565b81835260006020808501808196508560051b81019150846000805b8881101561441b578385038a528235605e19893603018112614464578283fd5b880180358652606061447888830183614085565b828a8a015261448a838a0182846140cd565b92505050604061449c81840184614085565b9350888303828a01526144b083858361432e565b9d8a019d98505050938701935050600101614444565b8183526000602080850194508260005b85811015613c92578135875260ff6144ef84840161379e565b168784015260409687019691909101906001016144d6565b602081526145216020820161451b8461407a565b15159052565b60006145306020840184614085565b610120806040860152614548610140860183856140cd565b92506145576040870187614103565b9250601f1980878603016060880152614571858584614162565b945061458060608901896141c6565b945091508087860301608088015261459985858461420e565b94506145a8608089018961425f565b94509150808786030160a08801526145c18585846142a7565b94506145d060a0890189614085565b94509150808786030160c08801526145e98585846142f8565b94506145f860c0890189614085565b94509150808786030160e088015261461185858461436a565b945061462060e0890189614085565b9450915061010081888703018189015261463b868685614429565b9550614649818a018a6141c6565b9550925050808786030183880152506146638484836144c6565b979650505050505050565b634e487b7160e01b600052601160045260246000fd5b600063ffffffff8083168185168083038211156146a3576146a361466e565b01949350505050565b600082198211156146bf576146bf61466e565b500190565b60c0810163ffffffff6146d684613ef7565b1682526020830135602083015260408301356040830152606083013560608301526080830135608083015260a083013560a083015292915050565b6000611e693683613b33565b60006001600160601b03808316818516818304811182151516156147435761474361466e565b02949350505050565b60008160001904831182151516156147665761476661466e565b500290565b600060001982141561477f5761477f61466e565b5060010190565b6000808335601e1984360301811261479d57600080fd5b8301803591506001600160401b038211156147b757600080fd5b6020019150600581901b3603821315613f6b57600080fd5b6000808335601e198436030181126147e657600080fd5b8301803591506001600160401b0382111561480057600080fd5b6020019150600781901b3603821315613f6b57600080fd5b60006020828403121561482a57600080fd5b6137ca8261414b565b6000808335601e1984360301811261484a57600080fd5b8301803591506001600160401b0382111561486457600080fd5b6020019150600681901b3603821315613f6b57600080fd5b6000808335601e1984360301811261489357600080fd5b8301803591506001600160401b038211156148ad57600080fd5b6020019150606081023603821315613f6b57600080fd5b60008235607e198336030181126148da57600080fd5b9190910192915050565b60008235605e198336030181126148da57600080fd5b6000808335601e1984360301811261491157600080fd5b8301803591506001600160401b0382111561492b57600080fd5b602001915036819003821315613f6b57600080fd5b6000828210156149525761495261466e565b500390565b60006001600160601b03838116908316818110156149775761497761466e565b03939250505056fe30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47a264697066735822122064c6150abc0756ab36b24ef7f40c5f6265cb4b261b604eddd1c4f4a45f55611f64736f6c634300080c0033",
}

// ContractGaspMultiRollupServiceABI is the input ABI used to generate the binding from.
// Deprecated: Use ContractGaspMultiRollupServiceMetaData.ABI instead.
var ContractGaspMultiRollupServiceABI = ContractGaspMultiRollupServiceMetaData.ABI

// ContractGaspMultiRollupServiceBin is the compiled bytecode used for deploying new contracts.
// Deprecated: Use ContractGaspMultiRollupServiceMetaData.Bin instead.
var ContractGaspMultiRollupServiceBin = ContractGaspMultiRollupServiceMetaData.Bin

// DeployContractGaspMultiRollupService deploys a new Ethereum contract, binding an instance of ContractGaspMultiRollupService to it.
func DeployContractGaspMultiRollupService(auth *bind.TransactOpts, backend bind.ContractBackend) (common.Address, *types.Transaction, *ContractGaspMultiRollupService, error) {
	parsed, err := ContractGaspMultiRollupServiceMetaData.GetAbi()
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	if parsed == nil {
		return common.Address{}, nil, nil, errors.New("GetABI returned nil")
	}

	address, tx, contract, err := bind.DeployContract(auth, *parsed, common.FromHex(ContractGaspMultiRollupServiceBin), backend)
	if err != nil {
		return common.Address{}, nil, nil, err
	}
	return address, tx, &ContractGaspMultiRollupService{ContractGaspMultiRollupServiceCaller: ContractGaspMultiRollupServiceCaller{contract: contract}, ContractGaspMultiRollupServiceTransactor: ContractGaspMultiRollupServiceTransactor{contract: contract}, ContractGaspMultiRollupServiceFilterer: ContractGaspMultiRollupServiceFilterer{contract: contract}}, nil
}

// ContractGaspMultiRollupService is an auto generated Go binding around an Ethereum contract.
type ContractGaspMultiRollupService struct {
	ContractGaspMultiRollupServiceCaller     // Read-only binding to the contract
	ContractGaspMultiRollupServiceTransactor // Write-only binding to the contract
	ContractGaspMultiRollupServiceFilterer   // Log filterer for contract events
}

// ContractGaspMultiRollupServiceCaller is an auto generated read-only Go binding around an Ethereum contract.
type ContractGaspMultiRollupServiceCaller struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractGaspMultiRollupServiceTransactor is an auto generated write-only Go binding around an Ethereum contract.
type ContractGaspMultiRollupServiceTransactor struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractGaspMultiRollupServiceFilterer is an auto generated log filtering Go binding around an Ethereum contract events.
type ContractGaspMultiRollupServiceFilterer struct {
	contract *bind.BoundContract // Generic contract wrapper for the low level calls
}

// ContractGaspMultiRollupServiceSession is an auto generated Go binding around an Ethereum contract,
// with pre-set call and transact options.
type ContractGaspMultiRollupServiceSession struct {
	Contract     *ContractGaspMultiRollupService // Generic contract binding to set the session for
	CallOpts     bind.CallOpts                   // Call options to use throughout this session
	TransactOpts bind.TransactOpts               // Transaction auth options to use throughout this session
}

// ContractGaspMultiRollupServiceCallerSession is an auto generated read-only Go binding around an Ethereum contract,
// with pre-set call options.
type ContractGaspMultiRollupServiceCallerSession struct {
	Contract *ContractGaspMultiRollupServiceCaller // Generic contract caller binding to set the session for
	CallOpts bind.CallOpts                         // Call options to use throughout this session
}

// ContractGaspMultiRollupServiceTransactorSession is an auto generated write-only Go binding around an Ethereum contract,
// with pre-set transact options.
type ContractGaspMultiRollupServiceTransactorSession struct {
	Contract     *ContractGaspMultiRollupServiceTransactor // Generic contract transactor binding to set the session for
	TransactOpts bind.TransactOpts                         // Transaction auth options to use throughout this session
}

// ContractGaspMultiRollupServiceRaw is an auto generated low-level Go binding around an Ethereum contract.
type ContractGaspMultiRollupServiceRaw struct {
	Contract *ContractGaspMultiRollupService // Generic contract binding to access the raw methods on
}

// ContractGaspMultiRollupServiceCallerRaw is an auto generated low-level read-only Go binding around an Ethereum contract.
type ContractGaspMultiRollupServiceCallerRaw struct {
	Contract *ContractGaspMultiRollupServiceCaller // Generic read-only contract binding to access the raw methods on
}

// ContractGaspMultiRollupServiceTransactorRaw is an auto generated low-level write-only Go binding around an Ethereum contract.
type ContractGaspMultiRollupServiceTransactorRaw struct {
	Contract *ContractGaspMultiRollupServiceTransactor // Generic write-only contract binding to access the raw methods on
}

// NewContractGaspMultiRollupService creates a new instance of ContractGaspMultiRollupService, bound to a specific deployed contract.
func NewContractGaspMultiRollupService(address common.Address, backend bind.ContractBackend) (*ContractGaspMultiRollupService, error) {
	contract, err := bindContractGaspMultiRollupService(address, backend, backend, backend)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupService{ContractGaspMultiRollupServiceCaller: ContractGaspMultiRollupServiceCaller{contract: contract}, ContractGaspMultiRollupServiceTransactor: ContractGaspMultiRollupServiceTransactor{contract: contract}, ContractGaspMultiRollupServiceFilterer: ContractGaspMultiRollupServiceFilterer{contract: contract}}, nil
}

// NewContractGaspMultiRollupServiceCaller creates a new read-only instance of ContractGaspMultiRollupService, bound to a specific deployed contract.
func NewContractGaspMultiRollupServiceCaller(address common.Address, caller bind.ContractCaller) (*ContractGaspMultiRollupServiceCaller, error) {
	contract, err := bindContractGaspMultiRollupService(address, caller, nil, nil)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceCaller{contract: contract}, nil
}

// NewContractGaspMultiRollupServiceTransactor creates a new write-only instance of ContractGaspMultiRollupService, bound to a specific deployed contract.
func NewContractGaspMultiRollupServiceTransactor(address common.Address, transactor bind.ContractTransactor) (*ContractGaspMultiRollupServiceTransactor, error) {
	contract, err := bindContractGaspMultiRollupService(address, nil, transactor, nil)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceTransactor{contract: contract}, nil
}

// NewContractGaspMultiRollupServiceFilterer creates a new log filterer instance of ContractGaspMultiRollupService, bound to a specific deployed contract.
func NewContractGaspMultiRollupServiceFilterer(address common.Address, filterer bind.ContractFilterer) (*ContractGaspMultiRollupServiceFilterer, error) {
	contract, err := bindContractGaspMultiRollupService(address, nil, nil, filterer)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceFilterer{contract: contract}, nil
}

// bindContractGaspMultiRollupService binds a generic wrapper to an already deployed contract.
func bindContractGaspMultiRollupService(address common.Address, caller bind.ContractCaller, transactor bind.ContractTransactor, filterer bind.ContractFilterer) (*bind.BoundContract, error) {
	parsed, err := ContractGaspMultiRollupServiceMetaData.GetAbi()
	if err != nil {
		return nil, err
	}
	return bind.NewBoundContract(address, *parsed, caller, transactor, filterer), nil
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractGaspMultiRollupService.Contract.ContractGaspMultiRollupServiceCaller.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ContractGaspMultiRollupServiceTransactor.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ContractGaspMultiRollupServiceTransactor.contract.Transact(opts, method, params...)
}

// Call invokes the (constant) contract method with params as input values and
// sets the output to result. The result type might be a single field for simple
// returns, a slice of interfaces for anonymous returns and a struct for named
// returns.
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerRaw) Call(opts *bind.CallOpts, result *[]interface{}, method string, params ...interface{}) error {
	return _ContractGaspMultiRollupService.Contract.contract.Call(opts, result, method, params...)
}

// Transfer initiates a plain transaction to move funds to the contract, calling
// its default method if one is available.
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorRaw) Transfer(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.contract.Transfer(opts)
}

// Transact invokes the (paid) contract method with params as input values.
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorRaw) Transact(opts *bind.TransactOpts, method string, params ...interface{}) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.contract.Transact(opts, method, params...)
}

// CheckSignatures is a free data retrieval call binding the contract method 0xc4e1914c.
//
// Solidity: function checkSignatures(bytes32 msgHash, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) params) view returns((uint96[],uint96[]))
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) CheckSignatures(opts *bind.CallOpts, msgHash [32]byte, params IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState) (IBLSSignatureCheckerQuorumStakeTotals, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "checkSignatures", msgHash, params)

	if err != nil {
		return *new(IBLSSignatureCheckerQuorumStakeTotals), err
	}

	out0 := *abi.ConvertType(out[0], new(IBLSSignatureCheckerQuorumStakeTotals)).(*IBLSSignatureCheckerQuorumStakeTotals)

	return out0, err

}

// CheckSignatures is a free data retrieval call binding the contract method 0xc4e1914c.
//
// Solidity: function checkSignatures(bytes32 msgHash, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) params) view returns((uint96[],uint96[]))
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) CheckSignatures(msgHash [32]byte, params IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState) (IBLSSignatureCheckerQuorumStakeTotals, error) {
	return _ContractGaspMultiRollupService.Contract.CheckSignatures(&_ContractGaspMultiRollupService.CallOpts, msgHash, params)
}

// CheckSignatures is a free data retrieval call binding the contract method 0xc4e1914c.
//
// Solidity: function checkSignatures(bytes32 msgHash, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) params) view returns((uint96[],uint96[]))
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) CheckSignatures(msgHash [32]byte, params IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState) (IBLSSignatureCheckerQuorumStakeTotals, error) {
	return _ContractGaspMultiRollupService.Contract.CheckSignatures(&_ContractGaspMultiRollupService.CallOpts, msgHash, params)
}

// LastUpdateBlockTimestamp is a free data retrieval call binding the contract method 0xe61db175.
//
// Solidity: function lastUpdateBlockTimestamp() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) LastUpdateBlockTimestamp(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "lastUpdateBlockTimestamp")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// LastUpdateBlockTimestamp is a free data retrieval call binding the contract method 0xe61db175.
//
// Solidity: function lastUpdateBlockTimestamp() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) LastUpdateBlockTimestamp() (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.LastUpdateBlockTimestamp(&_ContractGaspMultiRollupService.CallOpts)
}

// LastUpdateBlockTimestamp is a free data retrieval call binding the contract method 0xe61db175.
//
// Solidity: function lastUpdateBlockTimestamp() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) LastUpdateBlockTimestamp() (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.LastUpdateBlockTimestamp(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedTaskCreatedBlock is a free data retrieval call binding the contract method 0xed5a04fe.
//
// Solidity: function latestCompletedTaskCreatedBlock() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) LatestCompletedTaskCreatedBlock(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "latestCompletedTaskCreatedBlock")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LatestCompletedTaskCreatedBlock is a free data retrieval call binding the contract method 0xed5a04fe.
//
// Solidity: function latestCompletedTaskCreatedBlock() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) LatestCompletedTaskCreatedBlock() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedTaskCreatedBlock(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedTaskCreatedBlock is a free data retrieval call binding the contract method 0xed5a04fe.
//
// Solidity: function latestCompletedTaskCreatedBlock() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) LatestCompletedTaskCreatedBlock() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedTaskCreatedBlock(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedTaskNumber is a free data retrieval call binding the contract method 0xfc765dd5.
//
// Solidity: function latestCompletedTaskNumber() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) LatestCompletedTaskNumber(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "latestCompletedTaskNumber")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// LatestCompletedTaskNumber is a free data retrieval call binding the contract method 0xfc765dd5.
//
// Solidity: function latestCompletedTaskNumber() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) LatestCompletedTaskNumber() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedTaskNumber(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestCompletedTaskNumber is a free data retrieval call binding the contract method 0xfc765dd5.
//
// Solidity: function latestCompletedTaskNumber() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) LatestCompletedTaskNumber() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.LatestCompletedTaskNumber(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestPendingStateHash is a free data retrieval call binding the contract method 0x4ae6b203.
//
// Solidity: function latestPendingStateHash() view returns(bytes32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) LatestPendingStateHash(opts *bind.CallOpts) ([32]byte, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "latestPendingStateHash")

	if err != nil {
		return *new([32]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([32]byte)).(*[32]byte)

	return out0, err

}

// LatestPendingStateHash is a free data retrieval call binding the contract method 0x4ae6b203.
//
// Solidity: function latestPendingStateHash() view returns(bytes32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) LatestPendingStateHash() ([32]byte, error) {
	return _ContractGaspMultiRollupService.Contract.LatestPendingStateHash(&_ContractGaspMultiRollupService.CallOpts)
}

// LatestPendingStateHash is a free data retrieval call binding the contract method 0x4ae6b203.
//
// Solidity: function latestPendingStateHash() view returns(bytes32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) LatestPendingStateHash() ([32]byte, error) {
	return _ContractGaspMultiRollupService.Contract.LatestPendingStateHash(&_ContractGaspMultiRollupService.CallOpts)
}

// OperatorAndQuorumToStakes is a free data retrieval call binding the contract method 0x499d6fb6.
//
// Solidity: function operatorAndQuorumToStakes(bytes32 , uint8 ) view returns(uint96)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) OperatorAndQuorumToStakes(opts *bind.CallOpts, arg0 [32]byte, arg1 uint8) (*big.Int, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "operatorAndQuorumToStakes", arg0, arg1)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// OperatorAndQuorumToStakes is a free data retrieval call binding the contract method 0x499d6fb6.
//
// Solidity: function operatorAndQuorumToStakes(bytes32 , uint8 ) view returns(uint96)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) OperatorAndQuorumToStakes(arg0 [32]byte, arg1 uint8) (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.OperatorAndQuorumToStakes(&_ContractGaspMultiRollupService.CallOpts, arg0, arg1)
}

// OperatorAndQuorumToStakes is a free data retrieval call binding the contract method 0x499d6fb6.
//
// Solidity: function operatorAndQuorumToStakes(bytes32 , uint8 ) view returns(uint96)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) OperatorAndQuorumToStakes(arg0 [32]byte, arg1 uint8) (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.OperatorAndQuorumToStakes(&_ContractGaspMultiRollupService.CallOpts, arg0, arg1)
}

// OperatorIdQuorumCount is a free data retrieval call binding the contract method 0x430d3b39.
//
// Solidity: function operatorIdQuorumCount(bytes32 ) view returns(uint8)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) OperatorIdQuorumCount(opts *bind.CallOpts, arg0 [32]byte) (uint8, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "operatorIdQuorumCount", arg0)

	if err != nil {
		return *new(uint8), err
	}

	out0 := *abi.ConvertType(out[0], new(uint8)).(*uint8)

	return out0, err

}

// OperatorIdQuorumCount is a free data retrieval call binding the contract method 0x430d3b39.
//
// Solidity: function operatorIdQuorumCount(bytes32 ) view returns(uint8)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) OperatorIdQuorumCount(arg0 [32]byte) (uint8, error) {
	return _ContractGaspMultiRollupService.Contract.OperatorIdQuorumCount(&_ContractGaspMultiRollupService.CallOpts, arg0)
}

// OperatorIdQuorumCount is a free data retrieval call binding the contract method 0x430d3b39.
//
// Solidity: function operatorIdQuorumCount(bytes32 ) view returns(uint8)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) OperatorIdQuorumCount(arg0 [32]byte) (uint8, error) {
	return _ContractGaspMultiRollupService.Contract.OperatorIdQuorumCount(&_ContractGaspMultiRollupService.CallOpts, arg0)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) Owner(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "owner")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Owner() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.Owner(&_ContractGaspMultiRollupService.CallOpts)
}

// Owner is a free data retrieval call binding the contract method 0x8da5cb5b.
//
// Solidity: function owner() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) Owner() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.Owner(&_ContractGaspMultiRollupService.CallOpts)
}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) Paused(opts *bind.CallOpts, index uint8) (bool, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "paused", index)

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Paused(index uint8) (bool, error) {
	return _ContractGaspMultiRollupService.Contract.Paused(&_ContractGaspMultiRollupService.CallOpts, index)
}

// Paused is a free data retrieval call binding the contract method 0x5ac86ab7.
//
// Solidity: function paused(uint8 index) view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) Paused(index uint8) (bool, error) {
	return _ContractGaspMultiRollupService.Contract.Paused(&_ContractGaspMultiRollupService.CallOpts, index)
}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) Paused0(opts *bind.CallOpts) (*big.Int, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "paused0")

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Paused0() (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.Paused0(&_ContractGaspMultiRollupService.CallOpts)
}

// Paused0 is a free data retrieval call binding the contract method 0x5c975abb.
//
// Solidity: function paused() view returns(uint256)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) Paused0() (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.Paused0(&_ContractGaspMultiRollupService.CallOpts)
}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) PauserRegistry(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "pauserRegistry")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) PauserRegistry() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.PauserRegistry(&_ContractGaspMultiRollupService.CallOpts)
}

// PauserRegistry is a free data retrieval call binding the contract method 0x886f1195.
//
// Solidity: function pauserRegistry() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) PauserRegistry() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.PauserRegistry(&_ContractGaspMultiRollupService.CallOpts)
}

// QourumApk is a free data retrieval call binding the contract method 0x03d097d2.
//
// Solidity: function qourumApk(uint8 ) view returns(uint256 X, uint256 Y)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) QourumApk(opts *bind.CallOpts, arg0 uint8) (struct {
	X *big.Int
	Y *big.Int
}, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "qourumApk", arg0)

	outstruct := new(struct {
		X *big.Int
		Y *big.Int
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.X = *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)
	outstruct.Y = *abi.ConvertType(out[1], new(*big.Int)).(**big.Int)

	return *outstruct, err

}

// QourumApk is a free data retrieval call binding the contract method 0x03d097d2.
//
// Solidity: function qourumApk(uint8 ) view returns(uint256 X, uint256 Y)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) QourumApk(arg0 uint8) (struct {
	X *big.Int
	Y *big.Int
}, error) {
	return _ContractGaspMultiRollupService.Contract.QourumApk(&_ContractGaspMultiRollupService.CallOpts, arg0)
}

// QourumApk is a free data retrieval call binding the contract method 0x03d097d2.
//
// Solidity: function qourumApk(uint8 ) view returns(uint256 X, uint256 Y)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) QourumApk(arg0 uint8) (struct {
	X *big.Int
	Y *big.Int
}, error) {
	return _ContractGaspMultiRollupService.Contract.QourumApk(&_ContractGaspMultiRollupService.CallOpts, arg0)
}

// QuorumNumbers is a free data retrieval call binding the contract method 0x2a8414fd.
//
// Solidity: function quorumNumbers() view returns(bytes)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) QuorumNumbers(opts *bind.CallOpts) ([]byte, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "quorumNumbers")

	if err != nil {
		return *new([]byte), err
	}

	out0 := *abi.ConvertType(out[0], new([]byte)).(*[]byte)

	return out0, err

}

// QuorumNumbers is a free data retrieval call binding the contract method 0x2a8414fd.
//
// Solidity: function quorumNumbers() view returns(bytes)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) QuorumNumbers() ([]byte, error) {
	return _ContractGaspMultiRollupService.Contract.QuorumNumbers(&_ContractGaspMultiRollupService.CallOpts)
}

// QuorumNumbers is a free data retrieval call binding the contract method 0x2a8414fd.
//
// Solidity: function quorumNumbers() view returns(bytes)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) QuorumNumbers() ([]byte, error) {
	return _ContractGaspMultiRollupService.Contract.QuorumNumbers(&_ContractGaspMultiRollupService.CallOpts)
}

// QuorumThresholdPercentage is a free data retrieval call binding the contract method 0x4deabc21.
//
// Solidity: function quorumThresholdPercentage() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) QuorumThresholdPercentage(opts *bind.CallOpts) (uint32, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "quorumThresholdPercentage")

	if err != nil {
		return *new(uint32), err
	}

	out0 := *abi.ConvertType(out[0], new(uint32)).(*uint32)

	return out0, err

}

// QuorumThresholdPercentage is a free data retrieval call binding the contract method 0x4deabc21.
//
// Solidity: function quorumThresholdPercentage() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) QuorumThresholdPercentage() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.QuorumThresholdPercentage(&_ContractGaspMultiRollupService.CallOpts)
}

// QuorumThresholdPercentage is a free data retrieval call binding the contract method 0x4deabc21.
//
// Solidity: function quorumThresholdPercentage() view returns(uint32)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) QuorumThresholdPercentage() (uint32, error) {
	return _ContractGaspMultiRollupService.Contract.QuorumThresholdPercentage(&_ContractGaspMultiRollupService.CallOpts)
}

// QuorumToStakes is a free data retrieval call binding the contract method 0x7ad75561.
//
// Solidity: function quorumToStakes(uint8 ) view returns(uint96)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) QuorumToStakes(opts *bind.CallOpts, arg0 uint8) (*big.Int, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "quorumToStakes", arg0)

	if err != nil {
		return *new(*big.Int), err
	}

	out0 := *abi.ConvertType(out[0], new(*big.Int)).(**big.Int)

	return out0, err

}

// QuorumToStakes is a free data retrieval call binding the contract method 0x7ad75561.
//
// Solidity: function quorumToStakes(uint8 ) view returns(uint96)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) QuorumToStakes(arg0 uint8) (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.QuorumToStakes(&_ContractGaspMultiRollupService.CallOpts, arg0)
}

// QuorumToStakes is a free data retrieval call binding the contract method 0x7ad75561.
//
// Solidity: function quorumToStakes(uint8 ) view returns(uint96)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) QuorumToStakes(arg0 uint8) (*big.Int, error) {
	return _ContractGaspMultiRollupService.Contract.QuorumToStakes(&_ContractGaspMultiRollupService.CallOpts, arg0)
}

// Rolldown is a free data retrieval call binding the contract method 0x3d9fb00c.
//
// Solidity: function rolldown() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) Rolldown(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "rolldown")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Rolldown is a free data retrieval call binding the contract method 0x3d9fb00c.
//
// Solidity: function rolldown() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Rolldown() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.Rolldown(&_ContractGaspMultiRollupService.CallOpts)
}

// Rolldown is a free data retrieval call binding the contract method 0x3d9fb00c.
//
// Solidity: function rolldown() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) Rolldown() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.Rolldown(&_ContractGaspMultiRollupService.CallOpts)
}

// Stalled is a free data retrieval call binding the contract method 0x526e3e64.
//
// Solidity: function stalled() view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) Stalled(opts *bind.CallOpts) (bool, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "stalled")

	if err != nil {
		return *new(bool), err
	}

	out0 := *abi.ConvertType(out[0], new(bool)).(*bool)

	return out0, err

}

// Stalled is a free data retrieval call binding the contract method 0x526e3e64.
//
// Solidity: function stalled() view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Stalled() (bool, error) {
	return _ContractGaspMultiRollupService.Contract.Stalled(&_ContractGaspMultiRollupService.CallOpts)
}

// Stalled is a free data retrieval call binding the contract method 0x526e3e64.
//
// Solidity: function stalled() view returns(bool)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) Stalled() (bool, error) {
	return _ContractGaspMultiRollupService.Contract.Stalled(&_ContractGaspMultiRollupService.CallOpts)
}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) TrySignatureAndApkVerification(opts *bind.CallOpts, msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "trySignatureAndApkVerification", msgHash, apk, apkG2, sigma)

	outstruct := new(struct {
		PairingSuccessful bool
		SiganatureIsValid bool
	})
	if err != nil {
		return *outstruct, err
	}

	outstruct.PairingSuccessful = *abi.ConvertType(out[0], new(bool)).(*bool)
	outstruct.SiganatureIsValid = *abi.ConvertType(out[1], new(bool)).(*bool)

	return *outstruct, err

}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) TrySignatureAndApkVerification(msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	return _ContractGaspMultiRollupService.Contract.TrySignatureAndApkVerification(&_ContractGaspMultiRollupService.CallOpts, msgHash, apk, apkG2, sigma)
}

// TrySignatureAndApkVerification is a free data retrieval call binding the contract method 0x171f1d5b.
//
// Solidity: function trySignatureAndApkVerification(bytes32 msgHash, (uint256,uint256) apk, (uint256[2],uint256[2]) apkG2, (uint256,uint256) sigma) view returns(bool pairingSuccessful, bool siganatureIsValid)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) TrySignatureAndApkVerification(msgHash [32]byte, apk BN254G1Point, apkG2 BN254G2Point, sigma BN254G1Point) (struct {
	PairingSuccessful bool
	SiganatureIsValid bool
}, error) {
	return _ContractGaspMultiRollupService.Contract.TrySignatureAndApkVerification(&_ContractGaspMultiRollupService.CallOpts, msgHash, apk, apkG2, sigma)
}

// Updater is a free data retrieval call binding the contract method 0xdf034cd0.
//
// Solidity: function updater() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCaller) Updater(opts *bind.CallOpts) (common.Address, error) {
	var out []interface{}
	err := _ContractGaspMultiRollupService.contract.Call(opts, &out, "updater")

	if err != nil {
		return *new(common.Address), err
	}

	out0 := *abi.ConvertType(out[0], new(common.Address)).(*common.Address)

	return out0, err

}

// Updater is a free data retrieval call binding the contract method 0xdf034cd0.
//
// Solidity: function updater() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Updater() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.Updater(&_ContractGaspMultiRollupService.CallOpts)
}

// Updater is a free data retrieval call binding the contract method 0xdf034cd0.
//
// Solidity: function updater() view returns(address)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceCallerSession) Updater() (common.Address, error) {
	return _ContractGaspMultiRollupService.Contract.Updater(&_ContractGaspMultiRollupService.CallOpts)
}

// Initialize is a paid mutator transaction binding the contract method 0xf8c8765e.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _updater, address _rolldown) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) Initialize(opts *bind.TransactOpts, _pauserRegistry common.Address, initialOwner common.Address, _updater common.Address, _rolldown common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "initialize", _pauserRegistry, initialOwner, _updater, _rolldown)
}

// Initialize is a paid mutator transaction binding the contract method 0xf8c8765e.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _updater, address _rolldown) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _updater common.Address, _rolldown common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Initialize(&_ContractGaspMultiRollupService.TransactOpts, _pauserRegistry, initialOwner, _updater, _rolldown)
}

// Initialize is a paid mutator transaction binding the contract method 0xf8c8765e.
//
// Solidity: function initialize(address _pauserRegistry, address initialOwner, address _updater, address _rolldown) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) Initialize(_pauserRegistry common.Address, initialOwner common.Address, _updater common.Address, _rolldown common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Initialize(&_ContractGaspMultiRollupService.TransactOpts, _pauserRegistry, initialOwner, _updater, _rolldown)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) Pause(opts *bind.TransactOpts, newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "pause", newPausedStatus)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Pause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Pause(&_ContractGaspMultiRollupService.TransactOpts, newPausedStatus)
}

// Pause is a paid mutator transaction binding the contract method 0x136439dd.
//
// Solidity: function pause(uint256 newPausedStatus) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) Pause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Pause(&_ContractGaspMultiRollupService.TransactOpts, newPausedStatus)
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) PauseAll(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "pauseAll")
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) PauseAll() (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.PauseAll(&_ContractGaspMultiRollupService.TransactOpts)
}

// PauseAll is a paid mutator transaction binding the contract method 0x595c6a67.
//
// Solidity: function pauseAll() returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) PauseAll() (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.PauseAll(&_ContractGaspMultiRollupService.TransactOpts)
}

// ProcessEigenReinit is a paid mutator transaction binding the contract method 0xc927feef.
//
// Solidity: function process_eigen_reinit((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) ProcessEigenReinit(opts *bind.TransactOpts, task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "process_eigen_reinit", task, taskResponse, operatorStateInfo)
}

// ProcessEigenReinit is a paid mutator transaction binding the contract method 0xc927feef.
//
// Solidity: function process_eigen_reinit((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) ProcessEigenReinit(task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenReinit(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, operatorStateInfo)
}

// ProcessEigenReinit is a paid mutator transaction binding the contract method 0xc927feef.
//
// Solidity: function process_eigen_reinit((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) ProcessEigenReinit(task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenReinit(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, operatorStateInfo)
}

// ProcessEigenUpdate is a paid mutator transaction binding the contract method 0x3e404926.
//
// Solidity: function process_eigen_update((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) nonSignerStakesAndSignatureForOldState, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) ProcessEigenUpdate(opts *bind.TransactOpts, task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, nonSignerStakesAndSignatureForOldState IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "process_eigen_update", task, taskResponse, nonSignerStakesAndSignatureForOldState, operatorStateInfo)
}

// ProcessEigenUpdate is a paid mutator transaction binding the contract method 0x3e404926.
//
// Solidity: function process_eigen_update((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) nonSignerStakesAndSignatureForOldState, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) ProcessEigenUpdate(task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, nonSignerStakesAndSignatureForOldState IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenUpdate(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, nonSignerStakesAndSignatureForOldState, operatorStateInfo)
}

// ProcessEigenUpdate is a paid mutator transaction binding the contract method 0x3e404926.
//
// Solidity: function process_eigen_update((uint32,uint256,uint32,uint32,bytes,uint32,bytes,uint32) task, (uint32,bytes32,bytes32,bytes32,bytes32,bytes32) taskResponse, ((uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256)) nonSignerStakesAndSignatureForOldState, (bool,uint8[],(uint8,uint96,(uint256,uint256))[],(uint8,uint96)[],(uint8,(uint256,uint256))[],bytes32[],(bytes32,uint8[],uint96[],uint8)[],(bytes32,uint8[],uint96[])[],(bytes32,uint8)[]) operatorStateInfo) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) ProcessEigenUpdate(task IFinalizerTaskManagerTask, taskResponse IFinalizerTaskManagerTaskResponse, nonSignerStakesAndSignatureForOldState IGaspMultiRollupServicePrimitivesNonSignerStakesAndSignatureForOldState, operatorStateInfo IGaspMultiRollupServicePrimitivesOperatorStateInfo) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.ProcessEigenUpdate(&_ContractGaspMultiRollupService.TransactOpts, task, taskResponse, nonSignerStakesAndSignatureForOldState, operatorStateInfo)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) RenounceOwnership(opts *bind.TransactOpts) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "renounceOwnership")
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.RenounceOwnership(&_ContractGaspMultiRollupService.TransactOpts)
}

// RenounceOwnership is a paid mutator transaction binding the contract method 0x715018a6.
//
// Solidity: function renounceOwnership() returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) RenounceOwnership() (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.RenounceOwnership(&_ContractGaspMultiRollupService.TransactOpts)
}

// SetPauserRegistry is a paid mutator transaction binding the contract method 0x10d67a2f.
//
// Solidity: function setPauserRegistry(address newPauserRegistry) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) SetPauserRegistry(opts *bind.TransactOpts, newPauserRegistry common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "setPauserRegistry", newPauserRegistry)
}

// SetPauserRegistry is a paid mutator transaction binding the contract method 0x10d67a2f.
//
// Solidity: function setPauserRegistry(address newPauserRegistry) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) SetPauserRegistry(newPauserRegistry common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.SetPauserRegistry(&_ContractGaspMultiRollupService.TransactOpts, newPauserRegistry)
}

// SetPauserRegistry is a paid mutator transaction binding the contract method 0x10d67a2f.
//
// Solidity: function setPauserRegistry(address newPauserRegistry) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) SetPauserRegistry(newPauserRegistry common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.SetPauserRegistry(&_ContractGaspMultiRollupService.TransactOpts, newPauserRegistry)
}

// SetRolldown is a paid mutator transaction binding the contract method 0x1e2d4bf7.
//
// Solidity: function set_rolldown(address _rolldown) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) SetRolldown(opts *bind.TransactOpts, _rolldown common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "set_rolldown", _rolldown)
}

// SetRolldown is a paid mutator transaction binding the contract method 0x1e2d4bf7.
//
// Solidity: function set_rolldown(address _rolldown) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) SetRolldown(_rolldown common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.SetRolldown(&_ContractGaspMultiRollupService.TransactOpts, _rolldown)
}

// SetRolldown is a paid mutator transaction binding the contract method 0x1e2d4bf7.
//
// Solidity: function set_rolldown(address _rolldown) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) SetRolldown(_rolldown common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.SetRolldown(&_ContractGaspMultiRollupService.TransactOpts, _rolldown)
}

// SetUpdater is a paid mutator transaction binding the contract method 0x124648c9.
//
// Solidity: function set_updater(address _updater) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) SetUpdater(opts *bind.TransactOpts, _updater common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "set_updater", _updater)
}

// SetUpdater is a paid mutator transaction binding the contract method 0x124648c9.
//
// Solidity: function set_updater(address _updater) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) SetUpdater(_updater common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.SetUpdater(&_ContractGaspMultiRollupService.TransactOpts, _updater)
}

// SetUpdater is a paid mutator transaction binding the contract method 0x124648c9.
//
// Solidity: function set_updater(address _updater) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) SetUpdater(_updater common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.SetUpdater(&_ContractGaspMultiRollupService.TransactOpts, _updater)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) TransferOwnership(opts *bind.TransactOpts, newOwner common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "transferOwnership", newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.TransferOwnership(&_ContractGaspMultiRollupService.TransactOpts, newOwner)
}

// TransferOwnership is a paid mutator transaction binding the contract method 0xf2fde38b.
//
// Solidity: function transferOwnership(address newOwner) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) TransferOwnership(newOwner common.Address) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.TransferOwnership(&_ContractGaspMultiRollupService.TransactOpts, newOwner)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactor) Unpause(opts *bind.TransactOpts, newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.contract.Transact(opts, "unpause", newPausedStatus)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceSession) Unpause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Unpause(&_ContractGaspMultiRollupService.TransactOpts, newPausedStatus)
}

// Unpause is a paid mutator transaction binding the contract method 0xfabc1cbc.
//
// Solidity: function unpause(uint256 newPausedStatus) returns()
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceTransactorSession) Unpause(newPausedStatus *big.Int) (*types.Transaction, error) {
	return _ContractGaspMultiRollupService.Contract.Unpause(&_ContractGaspMultiRollupService.TransactOpts, newPausedStatus)
}

// ContractGaspMultiRollupServiceEigenReinitProcessedIterator is returned from FilterEigenReinitProcessed and is used to iterate over the raw logs and unpacked data for EigenReinitProcessed events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceEigenReinitProcessedIterator struct {
	Event *ContractGaspMultiRollupServiceEigenReinitProcessed // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractGaspMultiRollupServiceEigenReinitProcessedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServiceEigenReinitProcessed)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractGaspMultiRollupServiceEigenReinitProcessed)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractGaspMultiRollupServiceEigenReinitProcessedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServiceEigenReinitProcessedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServiceEigenReinitProcessed represents a EigenReinitProcessed event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceEigenReinitProcessed struct {
	TaskNumber       uint32
	TaskCreatedBlock uint32
	Raw              types.Log // Blockchain specific contextual infos
}

// FilterEigenReinitProcessed is a free log retrieval operation binding the contract event 0x264965eb6bc436c6c473431d34af56e832ec344fdfd43ee6af6fce6d205e84af.
//
// Solidity: event EigenReinitProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterEigenReinitProcessed(opts *bind.FilterOpts) (*ContractGaspMultiRollupServiceEigenReinitProcessedIterator, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "EigenReinitProcessed")
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceEigenReinitProcessedIterator{contract: _ContractGaspMultiRollupService.contract, event: "EigenReinitProcessed", logs: logs, sub: sub}, nil
}

// WatchEigenReinitProcessed is a free log subscription operation binding the contract event 0x264965eb6bc436c6c473431d34af56e832ec344fdfd43ee6af6fce6d205e84af.
//
// Solidity: event EigenReinitProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchEigenReinitProcessed(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServiceEigenReinitProcessed) (event.Subscription, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "EigenReinitProcessed")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServiceEigenReinitProcessed)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "EigenReinitProcessed", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseEigenReinitProcessed is a log parse operation binding the contract event 0x264965eb6bc436c6c473431d34af56e832ec344fdfd43ee6af6fce6d205e84af.
//
// Solidity: event EigenReinitProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParseEigenReinitProcessed(log types.Log) (*ContractGaspMultiRollupServiceEigenReinitProcessed, error) {
	event := new(ContractGaspMultiRollupServiceEigenReinitProcessed)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "EigenReinitProcessed", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractGaspMultiRollupServiceEigenUpdateProcessedIterator is returned from FilterEigenUpdateProcessed and is used to iterate over the raw logs and unpacked data for EigenUpdateProcessed events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceEigenUpdateProcessedIterator struct {
	Event *ContractGaspMultiRollupServiceEigenUpdateProcessed // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractGaspMultiRollupServiceEigenUpdateProcessedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServiceEigenUpdateProcessed)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractGaspMultiRollupServiceEigenUpdateProcessed)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractGaspMultiRollupServiceEigenUpdateProcessedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServiceEigenUpdateProcessedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServiceEigenUpdateProcessed represents a EigenUpdateProcessed event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceEigenUpdateProcessed struct {
	TaskNumber       uint32
	TaskCreatedBlock uint32
	Raw              types.Log // Blockchain specific contextual infos
}

// FilterEigenUpdateProcessed is a free log retrieval operation binding the contract event 0x9a128fe7347f1e11ca22aa9deb632ec9abb09608c13a994c60f77a562f460171.
//
// Solidity: event EigenUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterEigenUpdateProcessed(opts *bind.FilterOpts) (*ContractGaspMultiRollupServiceEigenUpdateProcessedIterator, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "EigenUpdateProcessed")
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceEigenUpdateProcessedIterator{contract: _ContractGaspMultiRollupService.contract, event: "EigenUpdateProcessed", logs: logs, sub: sub}, nil
}

// WatchEigenUpdateProcessed is a free log subscription operation binding the contract event 0x9a128fe7347f1e11ca22aa9deb632ec9abb09608c13a994c60f77a562f460171.
//
// Solidity: event EigenUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchEigenUpdateProcessed(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServiceEigenUpdateProcessed) (event.Subscription, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "EigenUpdateProcessed")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServiceEigenUpdateProcessed)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "EigenUpdateProcessed", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseEigenUpdateProcessed is a log parse operation binding the contract event 0x9a128fe7347f1e11ca22aa9deb632ec9abb09608c13a994c60f77a562f460171.
//
// Solidity: event EigenUpdateProcessed(uint32 taskNumber, uint32 taskCreatedBlock)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParseEigenUpdateProcessed(log types.Log) (*ContractGaspMultiRollupServiceEigenUpdateProcessed, error) {
	event := new(ContractGaspMultiRollupServiceEigenUpdateProcessed)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "EigenUpdateProcessed", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractGaspMultiRollupServiceInitializedIterator is returned from FilterInitialized and is used to iterate over the raw logs and unpacked data for Initialized events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceInitializedIterator struct {
	Event *ContractGaspMultiRollupServiceInitialized // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractGaspMultiRollupServiceInitializedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServiceInitialized)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractGaspMultiRollupServiceInitialized)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractGaspMultiRollupServiceInitializedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServiceInitializedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServiceInitialized represents a Initialized event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceInitialized struct {
	Version uint8
	Raw     types.Log // Blockchain specific contextual infos
}

// FilterInitialized is a free log retrieval operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterInitialized(opts *bind.FilterOpts) (*ContractGaspMultiRollupServiceInitializedIterator, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceInitializedIterator{contract: _ContractGaspMultiRollupService.contract, event: "Initialized", logs: logs, sub: sub}, nil
}

// WatchInitialized is a free log subscription operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchInitialized(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServiceInitialized) (event.Subscription, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "Initialized")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServiceInitialized)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "Initialized", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseInitialized is a log parse operation binding the contract event 0x7f26b83ff96e1f2b6a682f133852f6798a09c465da95921460cefb3847402498.
//
// Solidity: event Initialized(uint8 version)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParseInitialized(log types.Log) (*ContractGaspMultiRollupServiceInitialized, error) {
	event := new(ContractGaspMultiRollupServiceInitialized)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "Initialized", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractGaspMultiRollupServiceOwnershipTransferredIterator is returned from FilterOwnershipTransferred and is used to iterate over the raw logs and unpacked data for OwnershipTransferred events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceOwnershipTransferredIterator struct {
	Event *ContractGaspMultiRollupServiceOwnershipTransferred // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractGaspMultiRollupServiceOwnershipTransferredIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServiceOwnershipTransferred)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractGaspMultiRollupServiceOwnershipTransferred)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractGaspMultiRollupServiceOwnershipTransferredIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServiceOwnershipTransferredIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServiceOwnershipTransferred represents a OwnershipTransferred event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceOwnershipTransferred struct {
	PreviousOwner common.Address
	NewOwner      common.Address
	Raw           types.Log // Blockchain specific contextual infos
}

// FilterOwnershipTransferred is a free log retrieval operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterOwnershipTransferred(opts *bind.FilterOpts, previousOwner []common.Address, newOwner []common.Address) (*ContractGaspMultiRollupServiceOwnershipTransferredIterator, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceOwnershipTransferredIterator{contract: _ContractGaspMultiRollupService.contract, event: "OwnershipTransferred", logs: logs, sub: sub}, nil
}

// WatchOwnershipTransferred is a free log subscription operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchOwnershipTransferred(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServiceOwnershipTransferred, previousOwner []common.Address, newOwner []common.Address) (event.Subscription, error) {

	var previousOwnerRule []interface{}
	for _, previousOwnerItem := range previousOwner {
		previousOwnerRule = append(previousOwnerRule, previousOwnerItem)
	}
	var newOwnerRule []interface{}
	for _, newOwnerItem := range newOwner {
		newOwnerRule = append(newOwnerRule, newOwnerItem)
	}

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "OwnershipTransferred", previousOwnerRule, newOwnerRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServiceOwnershipTransferred)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseOwnershipTransferred is a log parse operation binding the contract event 0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0.
//
// Solidity: event OwnershipTransferred(address indexed previousOwner, address indexed newOwner)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParseOwnershipTransferred(log types.Log) (*ContractGaspMultiRollupServiceOwnershipTransferred, error) {
	event := new(ContractGaspMultiRollupServiceOwnershipTransferred)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "OwnershipTransferred", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractGaspMultiRollupServicePausedIterator is returned from FilterPaused and is used to iterate over the raw logs and unpacked data for Paused events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServicePausedIterator struct {
	Event *ContractGaspMultiRollupServicePaused // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractGaspMultiRollupServicePausedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServicePaused)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractGaspMultiRollupServicePaused)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractGaspMultiRollupServicePausedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServicePausedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServicePaused represents a Paused event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServicePaused struct {
	Account         common.Address
	NewPausedStatus *big.Int
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterPaused is a free log retrieval operation binding the contract event 0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d.
//
// Solidity: event Paused(address indexed account, uint256 newPausedStatus)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterPaused(opts *bind.FilterOpts, account []common.Address) (*ContractGaspMultiRollupServicePausedIterator, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "Paused", accountRule)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServicePausedIterator{contract: _ContractGaspMultiRollupService.contract, event: "Paused", logs: logs, sub: sub}, nil
}

// WatchPaused is a free log subscription operation binding the contract event 0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d.
//
// Solidity: event Paused(address indexed account, uint256 newPausedStatus)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchPaused(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServicePaused, account []common.Address) (event.Subscription, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "Paused", accountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServicePaused)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "Paused", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParsePaused is a log parse operation binding the contract event 0xab40a374bc51de372200a8bc981af8c9ecdc08dfdaef0bb6e09f88f3c616ef3d.
//
// Solidity: event Paused(address indexed account, uint256 newPausedStatus)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParsePaused(log types.Log) (*ContractGaspMultiRollupServicePaused, error) {
	event := new(ContractGaspMultiRollupServicePaused)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "Paused", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractGaspMultiRollupServicePauserRegistrySetIterator is returned from FilterPauserRegistrySet and is used to iterate over the raw logs and unpacked data for PauserRegistrySet events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServicePauserRegistrySetIterator struct {
	Event *ContractGaspMultiRollupServicePauserRegistrySet // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractGaspMultiRollupServicePauserRegistrySetIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServicePauserRegistrySet)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractGaspMultiRollupServicePauserRegistrySet)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractGaspMultiRollupServicePauserRegistrySetIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServicePauserRegistrySetIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServicePauserRegistrySet represents a PauserRegistrySet event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServicePauserRegistrySet struct {
	PauserRegistry    common.Address
	NewPauserRegistry common.Address
	Raw               types.Log // Blockchain specific contextual infos
}

// FilterPauserRegistrySet is a free log retrieval operation binding the contract event 0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6.
//
// Solidity: event PauserRegistrySet(address pauserRegistry, address newPauserRegistry)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterPauserRegistrySet(opts *bind.FilterOpts) (*ContractGaspMultiRollupServicePauserRegistrySetIterator, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "PauserRegistrySet")
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServicePauserRegistrySetIterator{contract: _ContractGaspMultiRollupService.contract, event: "PauserRegistrySet", logs: logs, sub: sub}, nil
}

// WatchPauserRegistrySet is a free log subscription operation binding the contract event 0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6.
//
// Solidity: event PauserRegistrySet(address pauserRegistry, address newPauserRegistry)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchPauserRegistrySet(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServicePauserRegistrySet) (event.Subscription, error) {

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "PauserRegistrySet")
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServicePauserRegistrySet)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "PauserRegistrySet", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParsePauserRegistrySet is a log parse operation binding the contract event 0x6e9fcd539896fca60e8b0f01dd580233e48a6b0f7df013b89ba7f565869acdb6.
//
// Solidity: event PauserRegistrySet(address pauserRegistry, address newPauserRegistry)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParsePauserRegistrySet(log types.Log) (*ContractGaspMultiRollupServicePauserRegistrySet, error) {
	event := new(ContractGaspMultiRollupServicePauserRegistrySet)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "PauserRegistrySet", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}

// ContractGaspMultiRollupServiceUnpausedIterator is returned from FilterUnpaused and is used to iterate over the raw logs and unpacked data for Unpaused events raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceUnpausedIterator struct {
	Event *ContractGaspMultiRollupServiceUnpaused // Event containing the contract specifics and raw log

	contract *bind.BoundContract // Generic contract to use for unpacking event data
	event    string              // Event name to use for unpacking event data

	logs chan types.Log        // Log channel receiving the found contract events
	sub  ethereum.Subscription // Subscription for errors, completion and termination
	done bool                  // Whether the subscription completed delivering logs
	fail error                 // Occurred error to stop iteration
}

// Next advances the iterator to the subsequent event, returning whether there
// are any more events found. In case of a retrieval or parsing error, false is
// returned and Error() can be queried for the exact failure.
func (it *ContractGaspMultiRollupServiceUnpausedIterator) Next() bool {
	// If the iterator failed, stop iterating
	if it.fail != nil {
		return false
	}
	// If the iterator completed, deliver directly whatever's available
	if it.done {
		select {
		case log := <-it.logs:
			it.Event = new(ContractGaspMultiRollupServiceUnpaused)
			if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
				it.fail = err
				return false
			}
			it.Event.Raw = log
			return true

		default:
			return false
		}
	}
	// Iterator still in progress, wait for either a data or an error event
	select {
	case log := <-it.logs:
		it.Event = new(ContractGaspMultiRollupServiceUnpaused)
		if err := it.contract.UnpackLog(it.Event, it.event, log); err != nil {
			it.fail = err
			return false
		}
		it.Event.Raw = log
		return true

	case err := <-it.sub.Err():
		it.done = true
		it.fail = err
		return it.Next()
	}
}

// Error returns any retrieval or parsing error occurred during filtering.
func (it *ContractGaspMultiRollupServiceUnpausedIterator) Error() error {
	return it.fail
}

// Close terminates the iteration process, releasing any pending underlying
// resources.
func (it *ContractGaspMultiRollupServiceUnpausedIterator) Close() error {
	it.sub.Unsubscribe()
	return nil
}

// ContractGaspMultiRollupServiceUnpaused represents a Unpaused event raised by the ContractGaspMultiRollupService contract.
type ContractGaspMultiRollupServiceUnpaused struct {
	Account         common.Address
	NewPausedStatus *big.Int
	Raw             types.Log // Blockchain specific contextual infos
}

// FilterUnpaused is a free log retrieval operation binding the contract event 0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c.
//
// Solidity: event Unpaused(address indexed account, uint256 newPausedStatus)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) FilterUnpaused(opts *bind.FilterOpts, account []common.Address) (*ContractGaspMultiRollupServiceUnpausedIterator, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractGaspMultiRollupService.contract.FilterLogs(opts, "Unpaused", accountRule)
	if err != nil {
		return nil, err
	}
	return &ContractGaspMultiRollupServiceUnpausedIterator{contract: _ContractGaspMultiRollupService.contract, event: "Unpaused", logs: logs, sub: sub}, nil
}

// WatchUnpaused is a free log subscription operation binding the contract event 0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c.
//
// Solidity: event Unpaused(address indexed account, uint256 newPausedStatus)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) WatchUnpaused(opts *bind.WatchOpts, sink chan<- *ContractGaspMultiRollupServiceUnpaused, account []common.Address) (event.Subscription, error) {

	var accountRule []interface{}
	for _, accountItem := range account {
		accountRule = append(accountRule, accountItem)
	}

	logs, sub, err := _ContractGaspMultiRollupService.contract.WatchLogs(opts, "Unpaused", accountRule)
	if err != nil {
		return nil, err
	}
	return event.NewSubscription(func(quit <-chan struct{}) error {
		defer sub.Unsubscribe()
		for {
			select {
			case log := <-logs:
				// New log arrived, parse the event and forward to the user
				event := new(ContractGaspMultiRollupServiceUnpaused)
				if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "Unpaused", log); err != nil {
					return err
				}
				event.Raw = log

				select {
				case sink <- event:
				case err := <-sub.Err():
					return err
				case <-quit:
					return nil
				}
			case err := <-sub.Err():
				return err
			case <-quit:
				return nil
			}
		}
	}), nil
}

// ParseUnpaused is a log parse operation binding the contract event 0x3582d1828e26bf56bd801502bc021ac0bc8afb57c826e4986b45593c8fad389c.
//
// Solidity: event Unpaused(address indexed account, uint256 newPausedStatus)
func (_ContractGaspMultiRollupService *ContractGaspMultiRollupServiceFilterer) ParseUnpaused(log types.Log) (*ContractGaspMultiRollupServiceUnpaused, error) {
	event := new(ContractGaspMultiRollupServiceUnpaused)
	if err := _ContractGaspMultiRollupService.contract.UnpackLog(event, "Unpaused", log); err != nil {
		return nil, err
	}
	event.Raw = log
	return event, nil
}
