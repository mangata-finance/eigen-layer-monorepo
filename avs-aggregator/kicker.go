package aggregator

import (
	"context"

	"github.com/Layr-Labs/eigensdk-go/logging"
	sdktypes "github.com/Layr-Labs/eigensdk-go/types"
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/core/chainio"
	"github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/types"
)

type Kicker struct {
	logger      logging.Logger
	kickPeriod  uint32
	blockPeriod uint32
	ethRpc      chainio.EthRpc
}

func NewKicker(logger logging.Logger, ethRpc chainio.EthRpc, kickPeriod uint32, blockPeriod uint32) (*Kicker, error) {
	return &Kicker{
		logger:      logger,
		kickPeriod:  kickPeriod,
		blockPeriod: blockPeriod,
		ethRpc:      ethRpc,
	}, nil
}

func (k *Kicker) TriggerNewTask(taskIndex uint32) {
	if !isTimeToKick(taskIndex, k.kickPeriod) {
		return
	}
	go k.CheckStateAndKick()
}

func (k *Kicker) CheckStateAndKick() error {
	k.logger.Info("Running Operator Active List check")
	// get N last TaskResponses
	// our block ~= eth block time, so this should roughly match; new task for every Nth block * evry Mth task to kick
	events, err := k.ethRpc.AvsReader.GetTaskRespondedEvents(context.Background(), k.kickPeriod*k.blockPeriod)
	if err != nil {
		k.logger.Error("Cannot get list of past TaskResponded events", "err", err)
		return err
	}

	k.logger.Debug("Got last events", "eventsCount", len(events))
	// get non signers present in every trx
	hash := make(map[sdktypes.OperatorId]*int)
	nonSigningOperatorIds := make([]sdktypes.OperatorId, 0)
	logIds := make([]string, 0)
	for _, ev := range events {
		keys, err := k.ethRpc.AvsReader.GetNonSigningOperatorPubKeys(ev)
		if err != nil {
			k.logger.Error("Cannot get list of NonSigningOperatorIds for TaskResponded event", "err", err)
			return err
		}
		for _, key := range keys {
			id := key.GetOperatorID()
			if counter := hash[id]; counter != nil {
				if *counter++; *counter >= len(events) {
					nonSigningOperatorIds = append(nonSigningOperatorIds, id)
					logIds = append(logIds, hexutil.Encode(id[:]))
				}
			} else {
				i := 1
				hash[id] = &i
			}
		}
	}

	k.logger.Info("OAL check found list of ids", "operatorIds", logIds)
	// fetch address and eject
	for i, key := range nonSigningOperatorIds {
		address, err := k.ethRpc.Clients.AvsRegistryChainReader.GetOperatorFromId(&bind.CallOpts{}, key)
		if err != nil {
			k.logger.Error("Cannot get operator address", "operatorId", logIds[i], "err", err)
			return err
		}

		k.logger.Info("Ejecting Operator", "address", address, "id", logIds[i])
		_, err = k.ethRpc.Clients.AvsRegistryChainWriter.EjectOperator(context.Background(), address, types.QUORUM_NUMBERS)
		if err != nil {
			k.logger.Error("Cannot eject operator", "operatorAddress", address, "err", err)
			return err
		}
		k.logger.Info("Operator ejected successfuly", "address", address)
	}

	return nil
}

func isTimeToKick(index uint32, period uint32) bool {
	return index%period == 0
}
