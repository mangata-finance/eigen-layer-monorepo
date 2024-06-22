package aggregator

import (
	"context"
	"encoding/json"
	"errors"
	"net/http"
	"fmt"
	"encoding/hex"
	// "io"
	// "strings"

	"github.com/ethereum/go-ethereum/accounts/abi"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/core"

	"github.com/Layr-Labs/eigensdk-go/crypto/bls"
	"github.com/Layr-Labs/eigensdk-go/services/bls_aggregation"
	"github.com/Layr-Labs/eigensdk-go/types"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
)

var (
	TaskNotFoundError400                     = errors.New("400. Task not found")
	OperatorNotPartOfTaskQuorum400           = errors.New("400. Operator not part of quorum")
	OperatorNotRegistered400                 = errors.New("400. Operator not registered in AVS")
	TaskResponseDigestNotFoundError500       = errors.New("500. Failed to get task response digest")
	UnknownErrorWhileVerifyingSignature400   = errors.New("400. Failed to verify signature")
	SignatureVerificationFailed400           = errors.New("400. Signature verification failed")
	CallToGetCheckSignaturesIndicesFailed500 = errors.New("500. Failed to get check signatures indices")
)

func (agg *Aggregator) startServer(ctx context.Context) error {
	http.HandleFunc("/", agg.handler)
	err := http.ListenAndServe(agg.serverIpPortAddr, nil)
	if err != nil {
		agg.logger.Fatal("ListenAndServe", "err", err)
	}

	return nil
}

func (agg *Aggregator) handler(w http.ResponseWriter, req *http.Request) {
	switch req.Method {
	case http.MethodConnect:
		http.Error(w, "Operator not supported, please upgrade to latest", http.StatusUpgradeRequired)
		return
	case http.MethodPost:
		break
	default:
		http.Error(w, "Method not supported", http.StatusMethodNotAllowed)
		return
	}


	// buf := new(strings.Builder)
	// io.Copy(buf, req.Body)

	// agg.logger.Info("handler", "req.Body", buf.String())

	var response SignedTaskResponse
	if err := json.NewDecoder(req.Body).Decode(&response); err != nil {
		http.Error(w, "Error parsing request body", http.StatusBadRequest)
		
		fmt.Print("handler - err:",err, "\n")
		return
	}

	if err := agg.ProcessSignedTaskResponse(&response, nil); err != nil {
		var status int
		switch err {
		case TaskResponseDigestNotFoundError500, CallToGetCheckSignaturesIndicesFailed500:
			status = http.StatusInternalServerError
		default:
			switch err.Error() {
			case blsagg.TaskNotFoundErrorFn(0).Error():
				status = http.StatusNotFound
			default:
				status = http.StatusBadRequest
			}
		}
		http.Error(w, err.Error(), status)
		return
	}
}

type SignedTaskResponse struct {
	TaskResponseEncoded string
	// TaskResponse []byte
	// TaskResponse taskmanager.IFinalizerTaskManagerTaskResponse
	BlsSignature bls.Signature
	OperatorId   types.OperatorId
}

// type SignedTaskResponse struct {
// 	TaskResponse taskmanager.IFinalizerTaskManagerTaskResponse
// 	BlsSignature bls.Signature
// 	OperatorId   types.OperatorId
// }

// rpc endpoint which is called by operator
// reply doesn't need to be checked. If there are no errors, the task response is accepted
// rpc framework forces a reply type to exist, so we put bool as a placeholder
func (agg *Aggregator) ProcessSignedTaskResponse(signedTaskResponse *SignedTaskResponse, reply *bool) error {
	agg.logger.Info("Received signed task response", signedTaskResponse, "operatorId", signedTaskResponse.OperatorId.LogValue())

	taskResponseTrimmed:=signedTaskResponse.TaskResponseEncoded[2:]
	fmt.Print("ProcessSignedTaskResponse - taskResponseTrimmed:",taskResponseTrimmed, "\n")
	task_response_bytes, err := hex.DecodeString(taskResponseTrimmed)
	fmt.Print("ProcessSignedTaskResponse - task_response_bytes:",task_response_bytes, "\n")
	if err != nil {
		agg.logger.Error("Failed to get task_response_bytes", "err", err)
		return TaskResponseDigestNotFoundError500
	}

	parsedAbi, err := taskmanager.ContractFinalizerTaskManagerMetaData.GetAbi()
	if err != nil {
		agg.logger.Error("Failed to get parsedAbi", "err", err)
		return TaskResponseDigestNotFoundError500
	}

	// inputParameters := parsedAbi.Methods["respondToTask"].Inputs
	inputParameters := parsedAbi.Methods["dummyForTaskResponseType"].Inputs
	fmt.Print("ProcessSignedTaskResponse - inputParameters:",inputParameters, "\n")
	// args := inputParameters[1:2]
	args := inputParameters
	fmt.Print("ProcessSignedTaskResponse - args:",args, "\n")
	// unpacked, err := args.Unpack(task_response_bytes)
	unpacked, err := args.UnpackValues(task_response_bytes)
	if err != nil {
		agg.logger.Error("Failed to get unpacked", "err", err)
		return TaskResponseDigestNotFoundError500
	}
	fmt.Print("ProcessSignedTaskResponse - unpacked:",unpacked, "\n")

	// unpacked, err := parsedAbi.Unpack("dummyForTaskResponseType", task_response_bytes)
	// if err != nil {
	// 	agg.logger.Error("Failed to get taskResponse", "err", err)
	// 	return TaskResponseDigestNotFoundError500
	// }
	// fmt.Print("ProcessSignedTaskResponse - unpacked:",unpacked, "\n")

	// err = args.Copy(&taskResponse, unpacked)
	// if err != nil {
	// 	agg.logger.Error("Failed to get taskResponse", "err", err)
	// 	return TaskResponseDigestNotFoundError500
	// }
	// fmt.Print("ProcessSignedTaskResponse - taskResponse:",taskResponse, "\n")

	// var x taskmanager.IFinalizerTaskManagerTaskResponse


	var taskResponse taskmanager.IFinalizerTaskManagerTaskResponse
	// x := abi.ConvertType(unpacked[0], taskResponse)
	fmt.Print("ProcessSignedTaskResponse - unpacked[0]:",unpacked[0], "\n")
	x := abi.ConvertType(unpacked[0], taskResponse)
	fmt.Print("ProcessSignedTaskResponse - x:",x, "\n")
	cx, ok := x.(taskmanager.IFinalizerTaskManagerTaskResponse)
	fmt.Print("ProcessSignedTaskResponse - ok:",ok, "\n")
	fmt.Print("ProcessSignedTaskResponse - cx:",cx, "\n")

	taskResponse = cx
	fmt.Print("ProcessSignedTaskResponse - taskResponse:",taskResponse, "\n")


	// var taskResponse taskmanager.IFinalizerTaskManagerTaskResponse
	// taskResponse = signedTaskResponse.TaskResponse

	taskIndex := taskResponse.ReferenceTaskIndex
	taskResponseDigest, err := core.GetTaskResponseDigest(&taskResponse)
	fmt.Print("ProcessSignedTaskResponse - taskResponseDigest:",taskResponseDigest, "\n")
	if err != nil {
		agg.logger.Error("Failed to get task response digest", "err", err)
		return TaskResponseDigestNotFoundError500
	}
	if signedTaskResponse.OperatorId == [32]byte{} {
		agg.logger.Error("Operator not registered", "err", err)
		return OperatorNotRegistered400
	}
	agg.taskResponsesMu.Lock()
	if _, ok := agg.taskResponses[taskIndex]; !ok {
		agg.taskResponses[taskIndex] = make(map[sdktypes.TaskResponseDigest]taskmanager.IFinalizerTaskManagerTaskResponse)
	}
	if _, ok := agg.taskResponses[taskIndex][taskResponseDigest]; !ok {
		agg.taskResponses[taskIndex][taskResponseDigest] = taskResponse
	}
	agg.taskResponsesMu.Unlock()

	err = agg.blsAggregationService.ProcessNewSignature(
		context.Background(), taskIndex, taskResponseDigest,
		&signedTaskResponse.BlsSignature, signedTaskResponse.OperatorId,
	)
	return err
}

// taskResponseTrimmed:0000000000000000000000000000000000000000000000000000000000000002d279e396ed520e326354bde3a68416b89a977394c74da2a493d4478a3adc0afe6e07278d636e7f25c70d801960cfb033a2900543822e633d8d32514d4f69a83b1fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919
// avs-aggregator-1  | AbiEncodeTaskResponse - task_response_bytes:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 2 210 121 227 150 237 82 14 50 99 84 189 227 166 132 22 184 154 151 115 148 199 77 162 164 147 212 71 138 58 220 10 254 110 7 39 141 99 110 127 37 199 13 128 25 96 207 176 51 162 144 5 67 130 46 99 61 141 50 81 77 79 105 168 59 31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]
// avs-aggregator-1  | AbiEncodeTaskResponse - unpacked:[{2 [210 121 227 150 237 82 14 50 99 84 189 227 166 132 22 184 154 151 115 148 199 77 162 164 147 212 71 138 58 220 10 254] [110 7 39 141 99 110 127 37 199 13 128 25 96 207 176 51 162 144 5 67 130 46 99 61 141 50 81 77 79 105 168 59] [31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]}]
// avs-aggregator-1  | 2024-06-20T08:30:09.338Z	INFO	avs-aggregator/rpc_server.go:94	Received signed task response	{"taskResponse": "0x0000000000000000000000000000000000000000000000000000000000000002d279e396ed520e326354bde3a68416b89a977394c74da2a493d4478a3adc0afe6e07278d636e7f25c70d801960cfb033a2900543822e633d8d32514d4f69a83b1fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919", "response": {"TaskResponse":"0x0000000000000000000000000000000000000000000000000000000000000002d279e396ed520e326354bde3a68416b89a977394c74da2a493d4478a3adc0afe6e07278d636e7f25c70d801960cfb033a2900543822e633d8d32514d4f69a83b1fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919","BlsSignature":{"g1_point":{"X":"4107897427722126060664475803072658570098398804302363895911419782038565918816","Y":"14712596803426563710188172335965214018268740832262615642554451938542724354912"}},"OperatorId":[21,111,33,161,232,98,184,87,251,115,226,124,195,231,251,166,107,244,71,175,97,196,78,130,192,6,19,4,31,206,209,244]}, "operatorId": "156f21a1e862b857fb73e27cc3e7fba66bf447af61c44e82c00613041fced1f4"}
// avs-aggregator-1  | AbiEncodeTaskResponse - taskResponse:{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}
// avs-aggregator-1  | AbiEncodeTaskResponse - taskResponseTrimmed:0000000000000000000000000000000000000000000000000000000000000002d279e396ed520e326354bde3a68416b89a977394c74da2a493d4478a3adc0afe6e07278d636e7f25c70d801960cfb033a2900543822e633d8d32514d4f69a83b1fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919
// avs-aggregator-1  | AbiEncodeTaskResponse - task_response_bytes:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 2 210 121 227 150 237 82 14 50 99 84 189 227 166 132 22 184 154 151 115 148 199 77 162 164 147 212 71 138 58 220 10 254 110 7 39 141 99 110 127 37 199 13 128 25 96 207 176 51 162 144 5 67 130 46 99 61 141 50 81 77 79 105 168 59 31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]
// avs-aggregator-1  | AbiEncodeTaskResponse - unpacked:[{2 [210 121 227 150 237 82 14 50 99 84 189 227 166 132 22 184 154 151 115 148 199 77 162 164 147 212 71 138 58 220 10 254] [110 7 39 141 99 110 127 37 199 13 128 25 96 207 176 51 162 144 5 67 130 46 99 61 141 50 81 77 79 105 168 59] [31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]}]
// avs-aggregator-1  | AbiEncodeTaskResponse - taskResponse:{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}
// avs-aggregator-1  | 2024-06-20T08:30:10.211Z	INFO	avs-aggregator/rpc_server.go:94	Received signed task response	{"taskResponse": "0x0000000000000000000000000000000000000000000000000000000000000002d279e396ed520e326354bde3a68416b89a977394c74da2a493d4478a3adc0afe6e07278d636e7f25c70d801960cfb033a2900543822e633d8d32514d4f69a83b1fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919", "response": {"TaskResponse":"0x0000000000000000000000000000000000000000000000000000000000000002d279e396ed520e326354bde3a68416b89a977394c74da2a493d4478a3adc0afe6e07278d636e7f25c70d801960cfb033a2900543822e633d8d32514d4f69a83b1fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919","BlsSignature":{"g1_point":{"X":"4107897427722126060664475803072658570098398804302363895911419782038565918816","Y":"14712596803426563710188172335965214018268740832262615642554451938542724354912"}},"OperatorId":[21,111,33,161,232,98,184,87,251,115,226,124,195,231,251,166,107,244,71,175,97,196,78,130,192,6,19,4,31,206,209,244]}, "operatorId": "156f21a1e862b857fb73e27cc3e7fba66bf447af61c44e82c00613041fced1f4"}
// avs-aggregator-1  | 2024-06-20T08:30:10.791Z	INFO	avs-aggregator/rpc_server.go:94	Received signed task response	{"taskResponse": "0x0000000000000000000000000000000000000000000000000000000000000002d279e396ed520e326354bde3a68416b89a977394c74da2a493d4478a3adc0afe6e07278d636e7f25c70d801960cfb033a2900543822e633d8d32514d4f69a83b1fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919", "response": {"TaskResponse":"0x0000000000000000000000000000000000000000000000000000000000000002d279e396ed520e326354bde3a68416b89a977394c74da2a493d4478a3adc0afe6e07278d636e7f25c70d801960cfb033a2900543822e633d8d32514d4f69a83b1fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919","BlsSignature":{"g1_point":{"X":"4107897427722126060664475803072658570098398804302363895911419782038565918816","Y":"14712596803426563710188172335965214018268740832262615642554451938542724354912"}},"OperatorId":[21,111,33,161,232,98,184,87,251,115,226,124,195,231,251,166,107,244,71,175,97,196,78,130,192,6,19,4,31,206,209,244]}, "operatorId": "156f21a1e862b857fb73e27cc3e7fba66bf447af61c44e82c00613041fced1f4"}
// avs-aggregator-1  | AbiEncodeTaskResponse - taskResponseTrimmed:0000000000000000000000000000000000000000000000000000000000000002d279e396ed520e326354bde3a68416b89a977394c74da2a493d4478a3adc0afe6e07278d636e7f25c70d801960cfb033a2900543822e633d8d32514d4f69a83b1fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919
// avs-aggregator-1  | AbiEncodeTaskResponse - task_response_bytes:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 2 210 121 227 150 237 82 14 50 99 84 189 227 166 132 22 184 154 151 115 148 199 77 162 164 147 212 71 138 58 220 10 254 110 7 39 141 99 110 127 37 199 13 128 25 96 207 176 51 162 144 5 67 130 46 99 61 141 50 81 77 79 105 168 59 31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]
// avs-aggregator-1  | AbiEncodeTaskResponse - unpacked:[{2 [210 121 227 150 237 82 14 50 99 84 189 227 166 132 22 184 154 151 115 148 199 77 162 164 147 212 71 138 58 220 10 254] [110 7 39 141 99 110 127 37 199 13 128 25 96 207 176 51 162 144 5 67 130 46 99 61 141 50 81 77 79 105 168 59] [31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]}]
// avs-aggregator-1  | AbiEncodeTaskResponse - taskResponse:{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}
// avs-aggregator-1  | 2024-06-20T08:30:11.334Z	INFO	avs-aggregator/rpc_server.go:94	Received signed task response	{"taskResponse": "0x0000000000000000000000000000000000000000000000000000000000000002d279e396ed520e326354bde3a68416b89a977394c74da2a493d4478a3adc0afe6e07278d636e7f25c70d801960cfb033a2900543822e633d8d32514d4f69a83b1fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919", "response": {"TaskResponse":"0x0000000000000000000000000000000000000000000000000000000000000002d279e396ed520e326354bde3a68416b89a977394c74da2a493d4478a3adc0afe6e07278d636e7f25c70d801960cfb033a2900543822e633d8d32514d4f69a83b1fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919","BlsSignature":{"g1_point":{"X":"4107897427722126060664475803072658570098398804302363895911419782038565918816","Y":"14712596803426563710188172335965214018268740832262615642554451938542724354912"}},"OperatorId":[21,111,33,161,232,98,184,87,251,115,226,124,195,231,251,166,107,244,71,175,97,196,78,130,192,6,19,4,31,206,209,244]}, "operatorId": "156f21a1e862b857fb73e27cc3e7fba66bf447af61c44e82c00613041fced1f4"}
// avs-aggregator-1  | AbiEncodeTaskResponse - taskResponseTrimmed:0000000000000000000000000000000000000000000000000000000000000002d279e396ed520e326354bde3a68416b89a977394c74da2a493d4478a3adc0afe6e07278d636e7f25c70d801960cfb033a2900543822e633d8d32514d4f69a83b1fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919
// avs-aggregator-1  | AbiEncodeTaskResponse - task_response_bytes:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 2 210 121 227 150 237 82 14 50 99 84 189 227 166 132 22 184 154 151 115 148 199 77 162 164 147 212 71 138 58 220 10 254 110 7 39 141 99 110 127 37 199 13 128 25 96 207 176 51 162 144 5 67 130 46 99 61 141 50 81 77 79 105 168 59 31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]
// avs-aggregator-1  | AbiEncodeTaskResponse - unpacked:[{2 [210 121 227 150 237 82 14 50 99 84 189 227 166 132 22 184 154 151 115 148 199 77 162 164 147 212 71 138 58 220 10 254] [110 7 39 141 99 110 127 37 199 13 128 25 96 207 176 51 162 144 5 67 130 46 99 61 141 50 81 77 79 105 168 59] [31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]}]
// avs-aggregator-1  | AbiEncodeTaskResponse - taskResponse:{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}

// 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,2,210,121,227,150,237,82,14,50,99,84,189,227,166,132,22,184,154,151,115,148,199,77,162,164,147,212,71,138,58,220,10,254,110,7,39,141,99,110,127,37,199,13,128,25,96,207,176,51,162,144,5,67,130,46,99,61,141,50,81,77,79,105,168,59,31,188,19,31,78,175,205,220,101,13,225,81,155,55,247,31,107,154,134,69,35,200,63,22,57,47,71,152,204,46,185,25


// ProcessSignedTaskResponse - taskResponseTrimmed:0000000000000000000000000000000000000000000000000000000000000002d661ba915dad9b54e313816f90cdc5b3f5fd6139badaee83cd87f6d2144345cb0b7c153ab612213d32f978ec14311f8210f247838da6cdae79e9d1497352ac681fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919
// avs-aggregator-1  | 2024-06-20T08:45:57.345Z	INFO	avs-aggregator/rpc_server.go:94	Received signed task response	{"taskResponse": "0x0000000000000000000000000000000000000000000000000000000000000002d661ba915dad9b54e313816f90cdc5b3f5fd6139badaee83cd87f6d2144345cb0b7c153ab612213d32f978ec14311f8210f247838da6cdae79e9d1497352ac681fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919", "response": {"TaskResponse":"0x0000000000000000000000000000000000000000000000000000000000000002d661ba915dad9b54e313816f90cdc5b3f5fd6139badaee83cd87f6d2144345cb0b7c153ab612213d32f978ec14311f8210f247838da6cdae79e9d1497352ac681fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919","TaskResponseWire":{"ReferenceTaskIndex":0,"BlockHash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"StorageProofHash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"PendingStateHash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]},"BlsSignature":{"g1_point":{"X":"11188729249708916585755830932161601683233882821907761660802161281717029935487","Y":"13939739560611588903714149476107880688324941114388110476170326624791581451470"}},"OperatorId":[250,151,44,130,171,164,68,231,130,119,166,234,40,231,174,46,198,221,208,244,108,177,173,237,151,137,108,55,27,20,9,128]}, "operatorId": "fa972c82aba444e78277a6ea28e7ae2ec6ddd0f46cb1aded97896c371b140980"}
// avs-aggregator-1  | ProcessSignedTaskResponse - task_response_bytes:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 2 214 97 186 145 93 173 155 84 227 19 129 111 144 205 197 179 245 253 97 57 186 218 238 131 205 135 246 210 20 67 69 203 11 124 21 58 182 18 33 61 50 249 120 236 20 49 31 130 16 242 71 131 141 166 205 174 121 233 209 73 115 82 172 104 31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]
// avs-aggregator-1  | ProcessSignedTaskResponse - inputParameters:[{task (uint256,uint32,bytes,uint32) false} {taskResponse (uint32,bytes32,bytes32,bytes32) false} {nonSignerStakesAndSignature (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) false}]
// avs-aggregator-1  | ProcessSignedTaskResponse - args:[{taskResponse (uint32,bytes32,bytes32,bytes32) false}]
// avs-aggregator-1  | ProcessSignedTaskResponse - unpacked:[{2 [214 97 186 145 93 173 155 84 227 19 129 111 144 205 197 179 245 253 97 57 186 218 238 131 205 135 246 210 20 67 69 203] [11 124 21 58 182 18 33 61 50 249 120 236 20 49 31 130 16 242 71 131 141 166 205 174 121 233 209 73 115 82 172 104] [31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]}]
// avs-aggregator-1  | ProcessSignedTaskResponse - taskResponse:{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}
// avs-aggregator-1  | AbiEncodeTaskResponse - args:[{taskResponse (uint32,bytes32,bytes32,bytes32) false}]
// avs-aggregator-1  | AbiEncodeTaskResponse - packed:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]
// avs-aggregator-1  | AbiEncodeTaskResponse - unpacked_by_args:[{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}]
// avs-aggregator-1  | AbiEncodeTaskResponse - h:&{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}
// avs-aggregator-1  | AbiEncodeTaskResponse - bytes:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]
// avs-aggregator-1  | AbiEncodeTaskResponse - unpacked_by_arguments:[{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}]
// avs-aggregator-1  | 2024-06-20T08:45:57.893Z	INFO	avs-aggregator/rpc_server.go:94	Received signed task response	{"taskResponse": "0x0000000000000000000000000000000000000000000000000000000000000002d661ba915dad9b54e313816f90cdc5b3f5fd6139badaee83cd87f6d2144345cb0b7c153ab612213d32f978ec14311f8210f247838da6cdae79e9d1497352ac681fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919", "response": {"TaskResponse":"0x0000000000000000000000000000000000000000000000000000000000000002d661ba915dad9b54e313816f90cdc5b3f5fd6139badaee83cd87f6d2144345cb0b7c153ab612213d32f978ec14311f8210f247838da6cdae79e9d1497352ac681fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919","TaskResponseWire":{"ReferenceTaskIndex":0,"BlockHash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"StorageProofHash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"PendingStateHash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]},"BlsSignature":{"g1_point":{"X":"11188729249708916585755830932161601683233882821907761660802161281717029935487","Y":"13939739560611588903714149476107880688324941114388110476170326624791581451470"}},"OperatorId":[250,151,44,130,171,164,68,231,130,119,166,234,40,231,174,46,198,221,208,244,108,177,173,237,151,137,108,55,27,20,9,128]}, "operatorId": "fa972c82aba444e78277a6ea28e7ae2ec6ddd0f46cb1aded97896c371b140980"}
// avs-aggregator-1  | ProcessSignedTaskResponse - taskResponseTrimmed:0000000000000000000000000000000000000000000000000000000000000002d661ba915dad9b54e313816f90cdc5b3f5fd6139badaee83cd87f6d2144345cb0b7c153ab612213d32f978ec14311f8210f247838da6cdae79e9d1497352ac681fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919
// avs-aggregator-1  | ProcessSignedTaskResponse - task_response_bytes:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 2 214 97 186 145 93 173 155 84 227 19 129 111 144 205 197 179 245 253 97 57 186 218 238 131 205 135 246 210 20 67 69 203 11 124 21 58 182 18 33 61 50 249 120 236 20 49 31 130 16 242 71 131 141 166 205 174 121 233 209 73 115 82 172 104 31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]
// avs-aggregator-1  | ProcessSignedTaskResponse - inputParameters:[{task (uint256,uint32,bytes,uint32) false} {taskResponse (uint32,bytes32,bytes32,bytes32) false} {nonSignerStakesAndSignature (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) false}]
// avs-aggregator-1  | ProcessSignedTaskResponse - args:[{taskResponse (uint32,bytes32,bytes32,bytes32) false}]
// avs-aggregator-1  | ProcessSignedTaskResponse - unpacked:[{2 [214 97 186 145 93 173 155 84 227 19 129 111 144 205 197 179 245 253 97 57 186 218 238 131 205 135 246 210 20 67 69 203] [11 124 21 58 182 18 33 61 50 249 120 236 20 49 31 130 16 242 71 131 141 166 205 174 121 233 209 73 115 82 172 104] [31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]}]
// avs-aggregator-1  | ProcessSignedTaskResponse - taskResponse:{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}
// avs-aggregator-1  | AbiEncodeTaskResponse - args:[{taskResponse (uint32,bytes32,bytes32,bytes32) false}]
// avs-aggregator-1  | AbiEncodeTaskResponse - packed:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]
// avs-aggregator-1  | AbiEncodeTaskResponse - unpacked_by_args:[{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}]
// avs-aggregator-1  | AbiEncodeTaskResponse - h:&{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}
// avs-aggregator-1  | AbiEncodeTaskResponse - bytes:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]
// avs-aggregator-1  | AbiEncodeTaskResponse - unpacked_by_arguments:[{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}]
// avs-aggregator-1  | ProcessSignedTaskResponse - taskResponseTrimmed:0000000000000000000000000000000000000000000000000000000000000002d661ba915dad9b54e313816f90cdc5b3f5fd6139badaee83cd87f6d2144345cb0b7c153ab612213d32f978ec14311f8210f247838da6cdae79e9d1497352ac681fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919
// avs-aggregator-1  | ProcessSignedTaskResponse - task_response_bytes:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 2 214 97 186 145 93 173 155 84 227 19 129 111 144 205 197 179 245 253 97 57 186 218 238 131 205 135 246 210 20 67 69 203 11 124 21 58 182 18 33 61 50 249 120 236 20 49 31 130 16 242 71 131 141 166 205 174 121 233 209 73 115 82 172 104 31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]
// avs-aggregator-1  | 2024-06-20T08:45:59.460Z	INFO	avs-aggregator/rpc_server.go:94	Received signed task response	{"taskResponse": "0x0000000000000000000000000000000000000000000000000000000000000002d661ba915dad9b54e313816f90cdc5b3f5fd6139badaee83cd87f6d2144345cb0b7c153ab612213d32f978ec14311f8210f247838da6cdae79e9d1497352ac681fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919", "response": {"TaskResponse":"0x0000000000000000000000000000000000000000000000000000000000000002d661ba915dad9b54e313816f90cdc5b3f5fd6139badaee83cd87f6d2144345cb0b7c153ab612213d32f978ec14311f8210f247838da6cdae79e9d1497352ac681fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919","TaskResponseWire":{"ReferenceTaskIndex":0,"BlockHash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"StorageProofHash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"PendingStateHash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]},"BlsSignature":{"g1_point":{"X":"11188729249708916585755830932161601683233882821907761660802161281717029935487","Y":"13939739560611588903714149476107880688324941114388110476170326624791581451470"}},"OperatorId":[250,151,44,130,171,164,68,231,130,119,166,234,40,231,174,46,198,221,208,244,108,177,173,237,151,137,108,55,27,20,9,128]}, "operatorId": "fa972c82aba444e78277a6ea28e7ae2ec6ddd0f46cb1aded97896c371b140980"}
// avs-aggregator-1  | ProcessSignedTaskResponse - inputParameters:[{task (uint256,uint32,bytes,uint32) false} {taskResponse (uint32,bytes32,bytes32,bytes32) false} {nonSignerStakesAndSignature (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) false}]
// avs-aggregator-1  | ProcessSignedTaskResponse - args:[{taskResponse (uint32,bytes32,bytes32,bytes32) false}]
// avs-aggregator-1  | ProcessSignedTaskResponse - unpacked:[{2 [214 97 186 145 93 173 155 84 227 19 129 111 144 205 197 179 245 253 97 57 186 218 238 131 205 135 246 210 20 67 69 203] [11 124 21 58 182 18 33 61 50 249 120 236 20 49 31 130 16 242 71 131 141 166 205 174 121 233 209 73 115 82 172 104] [31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]}]
// avs-aggregator-1  | ProcessSignedTaskResponse - taskResponse:{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}
// avs-aggregator-1  | AbiEncodeTaskResponse - args:[{taskResponse (uint32,bytes32,bytes32,bytes32) false}]
// avs-aggregator-1  | AbiEncodeTaskResponse - packed:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]
// avs-aggregator-1  | AbiEncodeTaskResponse - unpacked_by_args:[{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}]
// avs-aggregator-1  | AbiEncodeTaskResponse - h:&{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}
// avs-aggregator-1  | AbiEncodeTaskResponse - bytes:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]
// avs-aggregator-1  | AbiEncodeTaskResponse - unpacked_by_arguments:[{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}]
// avs-aggregator-1  | 2024-06-20T08:46:02.998Z	INFO	avs-aggregator/rpc_server.go:94	Received signed task response	{"taskResponse": "0x0000000000000000000000000000000000000000000000000000000000000002d661ba915dad9b54e313816f90cdc5b3f5fd6139badaee83cd87f6d2144345cb0b7c153ab612213d32f978ec14311f8210f247838da6cdae79e9d1497352ac681fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919", "response": {"TaskResponse":"0x0000000000000000000000000000000000000000000000000000000000000002d661ba915dad9b54e313816f90cdc5b3f5fd6139badaee83cd87f6d2144345cb0b7c153ab612213d32f978ec14311f8210f247838da6cdae79e9d1497352ac681fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919","TaskResponseWire":{"ReferenceTaskIndex":0,"BlockHash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"StorageProofHash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"PendingStateHash":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]},"BlsSignature":{"g1_point":{"X":"11188729249708916585755830932161601683233882821907761660802161281717029935487","Y":"13939739560611588903714149476107880688324941114388110476170326624791581451470"}},"OperatorId":[250,151,44,130,171,164,68,231,130,119,166,234,40,231,174,46,198,221,208,244,108,177,173,237,151,137,108,55,27,20,9,128]}, "operatorId": "fa972c82aba444e78277a6ea28e7ae2ec6ddd0f46cb1aded97896c371b140980"}
// avs-aggregator-1  | ProcessSignedTaskResponse - taskResponseTrimmed:0000000000000000000000000000000000000000000000000000000000000002d661ba915dad9b54e313816f90cdc5b3f5fd6139badaee83cd87f6d2144345cb0b7c153ab612213d32f978ec14311f8210f247838da6cdae79e9d1497352ac681fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919
// avs-aggregator-1  | ProcessSignedTaskResponse - task_response_bytes:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 2 214 97 186 145 93 173 155 84 227 19 129 111 144 205 197 179 245 253 97 57 186 218 238 131 205 135 246 210 20 67 69 203 11 124 21 58 182 18 33 61 50 249 120 236 20 49 31 130 16 242 71 131 141 166 205 174 121 233 209 73 115 82 172 104 31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]
// avs-aggregator-1  | ProcessSignedTaskResponse - inputParameters:[{task (uint256,uint32,bytes,uint32) false} {taskResponse (uint32,bytes32,bytes32,bytes32) false} {nonSignerStakesAndSignature (uint32[],(uint256,uint256)[],(uint256,uint256)[],(uint256[2],uint256[2]),(uint256,uint256),uint32[],uint32[],uint32[][]) false}]
// avs-aggregator-1  | ProcessSignedTaskResponse - args:[{taskResponse (uint32,bytes32,bytes32,bytes32) false}]
// avs-aggregator-1  | ProcessSignedTaskResponse - unpacked:[{2 [214 97 186 145 93 173 155 84 227 19 129 111 144 205 197 179 245 253 97 57 186 218 238 131 205 135 246 210 20 67 69 203] [11 124 21 58 182 18 33 61 50 249 120 236 20 49 31 130 16 242 71 131 141 166 205 174 121 233 209 73 115 82 172 104] [31 188 19 31 78 175 205 220 101 13 225 81 155 55 247 31 107 154 134 69 35 200 63 22 57 47 71 152 204 46 185 25]}]
// avs-aggregator-1  | ProcessSignedTaskResponse - taskResponse:{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}
// avs-aggregator-1  | AbiEncodeTaskResponse - args:[{taskResponse (uint32,bytes32,bytes32,bytes32) false}]
// avs-aggregator-1  | AbiEncodeTaskResponse - packed:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]
// avs-aggregator-1  | AbiEncodeTaskResponse - unpacked_by_args:[{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}]
// avs-aggregator-1  | AbiEncodeTaskResponse - h:&{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}
// avs-aggregator-1  | AbiEncodeTaskResponse - bytes:[0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]
// avs-aggregator-1  | AbiEncodeTaskResponse - unpacked_by_arguments:[{0 [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0] [0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0]}]


// {"type":"function","name":"dummyForTaskResponseType","inputs":[{"name":"taskResponse","type":"tuple","internalType":"structIFinalizerTaskManager.TaskResponse","components":[{"name":"referenceTaskIndex","type":"uint32","internalType":"uint32"},{"name":"task","type":"tuple","internalType":"structIFinalizerTaskManager.Task","components":[{"name":"blockNumber","type":"uint256","internalType":"uint256"},{"name":"taskCreatedBlock","type":"uint32","internalType":"uint32"},{"name":"quorumNumbers","type":"bytes","internalType":"bytes"},{"name":"quorumThresholdPercentage","type":"uint32","internalType":"uint32"}]},{"name":"blockHash","type":"bytes32","internalType":"bytes32"},{"name":"storageProofHash","type":"bytes32","internalType":"bytes32"},{"name":"pendingStateHash","type":"bytes32","internalType":"bytes32"}]}],"outputs":[],"stateMutability":"view"}


// 0000000000000000000000000000000000000000000000000000000000000019000000000000000000000000000000000000000000000000000000000000001d0000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000004200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000


// {
// 	"name":"task",
// 	"type":"tuple",
// 	"internalType":"structIFinalizerTaskManager.Task",
// 	"components":[
// 		{
// 			"name":"blockNumber",
// 			"type":"uint256",
// 			"internalType":"uint256"
// 		},
// 		{
// 			"name":"taskCreatedBlock",
// 			"type":"uint32",
// 			"internalType":"uint32"
// 		},
// 		{
// 			"name":"quorumNumbers",
// 			"type":"bytes",
// 			"internalType":"bytes"
// 		},
// 		{
// 			"name":"quorumThresholdPercentage",
// 			"type":"uint32",
// 			"internalType":"uint32"
// 		}
// 	]
// },

// [
//     {
//         "constant": false,
//         "inputs":[

//     {
//         "name":"taskResponse",
//         "type":"tuple",
//         "components":[
//             {
//                 "name":"referenceTaskIndex",
//                 "type":"uint32",
//             },
//             {
//                 "name":"task",
//                 "type":"tuple",
//                 "components":[
//                     {
//                         "name":"blockNumber",
//                         "type":"uint256",
//                     },
//                     {
//                         "name":"taskCreatedBlock",
//                         "type":"uint32",
//                     },
//                     {
//                         "name":"quorumNumbers",
//                         "type":"bytes",
//                     },
//                     {
//                         "name":"quorumThresholdPercentage",
//                         "type":"uint32",
//                     }
//                 ]
//             },
//             {
//                 "name":"blockHash",
//                 "type":"bytes32",
//             },
//             {
//                 "name":"storageProofHash",
//                 "type":"bytes32",
//             },
//             {
//                 "name":"pendingStateHash",
//                 "type":"bytes32",
//             }
//         ]
//     }

// ],
//         "name": "bidOnSiringAuction",
//         "outputs": [],
//         "payable": true,
//         "stateMutability": "payable",
//         "type": "function"
//     }
// ]

// 000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000a01b8984ddc90fb24c893d70cb8324256300fc06d5a9f4d44182b25beb76b3a4f60ad0f927c04731ac4243d854d966fe662e57c2002e99ae249f3ed4d0e9e23c511fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb9190000000000000000000000000000000000000000000000000000000000000019000000000000000000000000000000000000000000000000000000000000001d0000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000004200000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000000

// 0xed60ade600000000000000000000000000000000000000000000000000000000000000021b8984ddc90fb24c893d70cb8324256300fc06d5a9f4d44182b25beb76b3a4f60ad0f927c04731ac4243d854d966fe662e57c2002e99ae249f3ed4d0e9e23c511fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb919

// 0xd2fd914c000000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000a01b8984ddc90fb24c893d70cb8324256300fc06d5a9f4d44182b25beb76b3a4f60ad0f927c04731ac4243d854d966fe662e57c2002e99ae249f3ed4d0e9e23c511fbc131f4eafcddc650de1519b37f71f6b9a864523c83f16392f4798cc2eb9190000000000000000000000000000000000000000000000000000000000000019

// 0000000000000000000000000000000000000000000000000000000000000019