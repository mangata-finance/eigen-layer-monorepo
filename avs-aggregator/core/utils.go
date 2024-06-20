package core

import (
	"math/big"

	"fmt"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	"github.com/ethereum/go-ethereum/accounts/abi"
	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
	"golang.org/x/crypto/sha3"
)

// this hardcodes abi.encode() for taskmanager.IFinalizerTaskManagerTaskResponse
// unclear why abigen doesn't provide this out of the box...
func AbiEncodeTaskResponse(h *taskmanager.IFinalizerTaskManagerTaskResponse) ([]byte, error) {

	// The order here has to match the field ordering of taskmanager.IFinalizerTaskManagerTaskResponse
	taskResponseType, err := abi.NewType("tuple", "", []abi.ArgumentMarshaling{
		{
			Name: "referenceTaskIndex",
			Type: "uint32",
		},
		{
			Name: "BlockHash",
			Type: "bytes32",
		},
		{
			Name: "StorageProofHash",
			Type: "bytes32",
		},
		{
			Name: "PendingStateHash",
			Type: "bytes32",
		},
	})
	fmt.Print("AbiEncodeTaskResponse - taskResponseType:",taskResponseType, "\n")
	if err != nil {
		return nil, err
	}
	arguments := abi.Arguments{
		{
			Type: taskResponseType,
		},
	}
	fmt.Print("AbiEncodeTaskResponse - arguments:",arguments, "\n")

	// // agg.logger.Info("Received signed task response", "response", signedTaskResponse, "operatorId", signedTaskResponse.OperatorId.LogValue())
	// // var taskResponse taskmanager.IFinalizerTaskManagerTaskResponse
	// // // taskResponseType, err := abi.NewType("tuple", "", []abi.ArgumentMarshaling{
	// // // 	{
	// // // 		Name: "TaskResponse",
	// // // 		Type: "taskmanager.IFinalizerTaskManagerTaskResponse",
	// // // 	},
	// // // })
	// // // arguments := abi.Arguments{
	// // // 	{
	// // // 		Type: taskResponseType,
	// // // 	},
	// // // }
	// // // unpacked, err := arguments.Unpack(signedTaskResponse.TaskResponse)
	// // // arguments.Copy(taskResponse, unpacked)
	// // // agg.logger.Info("ProcessSignedTaskResponse", "taskResponseType", taskResponseType)

	// taskResponseTypeASDF, err := abi.NewType("tuple", "", []abi.ArgumentMarshaling{
	// 	{
	// 		Name: "TaskResponse",
	// 		Type: "taskmanager.IFinalizerTaskManagerTaskResponse",
	// 	},
	// })
	// argumentsASDF := abi.Arguments{
	// 	{
	// 		Type: taskResponseTypeASDF,
	// 	},
	// }
	// packedASDF, err := argumentsASDF.Pack(h)
	// fmt.Print("AbiEncodeTaskResponse - packedASDF:",packedASDF, "\n")

	parsedAbi, err := taskmanager.ContractFinalizerTaskManagerMetaData.GetAbi()
	inputParameters := parsedAbi.Methods["respondToTask"].Inputs

	// args := inputParameters[1:2]

	args := abi.Arguments{
		{
			Type: inputParameters[1].Type,
		},
	}
	fmt.Print("AbiEncodeTaskResponse - args:",args, "\n")
	// // args := abi.Arguments{inputParameters[1]}
	// // args := parsedAbi.GetArguments(inputParameters[1])
	// agg.logger.Info("ProcessSignedTaskResponse", "inputParameters", inputParameters)
	// agg.logger.Info("ProcessSignedTaskResponse", "args", args)
	packed, err := args.Pack(h)
	fmt.Print("AbiEncodeTaskResponse - packed:",packed, "\n")
	unpacked_by_args, err:= args.Unpack(packed)
	fmt.Print("AbiEncodeTaskResponse - unpacked_by_args:",unpacked_by_args, "\n")
	// unpacked, err := args.Unpack(signedTaskResponse.TaskResponse)
	// if err != nil {
	// 	agg.logger.Error("Failed to get task response", "err", err)
	// 	return TaskResponseDigestNotFoundError500
	// }
	// args.Copy(taskResponse, unpacked)
	// agg.logger.Info("ProcessSignedTaskResponse", "taskResponse", taskResponse)

	// // taskResponse := abi.toGoType(0, taskmanager.IFinalizerTaskManagerTaskResponse, signedTaskResponse.TaskResponse)

	fmt.Print("AbiEncodeTaskResponse - h:",h, "\n")
	bytes, err := arguments.Pack(h)
	fmt.Print("AbiEncodeTaskResponse - bytes:",bytes, "\n")

	unpacked_by_arguments, err:= arguments.Unpack(bytes)
	fmt.Print("AbiEncodeTaskResponse - unpacked_by_arguments:",unpacked_by_arguments, "\n")

	if err != nil {
		return nil, err
	}

	return bytes, nil
}

// GetTaskResponseDigest returns the hash of the TaskResponse, which is what operators sign over
func GetTaskResponseDigest(h *taskmanager.IFinalizerTaskManagerTaskResponse) ([32]byte, error) {

	encodeTaskResponseByte, err := AbiEncodeTaskResponse(h)
	if err != nil {
		return [32]byte{}, err
	}

	var taskResponseDigest [32]byte
	hasher := sha3.NewLegacyKeccak256()
	hasher.Write(encodeTaskResponseByte)
	copy(taskResponseDigest[:], hasher.Sum(nil)[:32])

	return taskResponseDigest, nil
}

// BINDING UTILS - conversion from contract structs to golang structs

// BN254.sol is a library, so bindings for G1 Points and G2 Points are only generated
// in every contract that imports that library. Thus the output here will need to be
// type casted if G1Point is needed to interface with another contract (eg: BLSPublicKeyCompendium.sol)
func ConvertToBN254G1Point(input *bls.G1Point) taskmanager.BN254G1Point {
	output := taskmanager.BN254G1Point{
		X: input.X.BigInt(big.NewInt(0)),
		Y: input.Y.BigInt(big.NewInt(0)),
	}
	return output
}

func ConvertToBN254G2Point(input *bls.G2Point) taskmanager.BN254G2Point {
	output := taskmanager.BN254G2Point{
		X: [2]*big.Int{input.X.A1.BigInt(big.NewInt(0)), input.X.A0.BigInt(big.NewInt(0))},
		Y: [2]*big.Int{input.Y.A1.BigInt(big.NewInt(0)), input.Y.A0.BigInt(big.NewInt(0))},
	}
	return output
}

// [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 2 32 181 19 80 132 110 159 170 101 197 17 57 134 160 214 109 235 64 46 35 123 58 220 123 76 182 202 107 98 196 207 32 197 206 183 62 234 178 9 168 232 180 157 33 105 85 120 73 54 55 202 30 255 244 84 143 81 156 108 197 235 228 72 82 31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]
// avs-aggregator-1  | AbiEncodeTaskResponse - h:&{2 [32 181 19 80 132 110 159 170 101 197 17 57 134 160 214 109 235 64 46 35 123 58 220 123 76 182 202 107 98 196 207 32] [197 206 183 62 234 178 9 168 232 180 157 33 105 85 120 73 54 55 202 30 255 244 84 143 81 156 108 197 235 228 72 82] [31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]}
// avs-aggregator-1  | AbiEncodeTaskResponse - bytes:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 2 32 181 19 80 132 110 159 170 101 197 17 57 134 160 214 109 235 64 46 35 123 58 220 123 76 182 202 107 98 196 207 32 197 206 183 62 234 178 9 168 232 180 157 33 105 85 120 73 54 55 202 30 255 244 84 143 81 156 108 197 235 228 72 82 31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]