package chainio

import (
	"github.com/ethereum/go-ethereum/accounts/abi/bind"
	gethcommon "github.com/ethereum/go-ethereum/common"
	"github.com/ethereum/go-ethereum/core/types"
	"github.com/ethereum/go-ethereum/event"

	"github.com/Layr-Labs/eigensdk-go/chainio/clients/eth"
	sdklogging "github.com/Layr-Labs/eigensdk-go/logging"

	taskmanager "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/FinalizerTaskManager"
	stakeRegistry "github.com/mangata-finance/eigen-layer-monorepo/avs-aggregator/bindings/StakeRegistry"
)


// Subscribers use a ws connection instead of http connection like Readers
// kind of stupid that the geth client doesn't have a unified interface for both...
// it takes a single url, so the bindings, even though they have watcher functions, those can't be used
// with the http connection... seems very very stupid. Am I missing something?
type AvsSubscriber struct {
	AvsContractBindings *AvsServiceBindings
	StreamSubscriber *StreamReader
	logger              sdklogging.Logger
}

func NewAvsSubscriber(registryAddr gethcommon.Address, ethclient eth.Client, logger sdklogging.Logger) (*AvsSubscriber, error) {
	avsContractBindings, err := NewAvsServiceBindings(registryAddr, ethclient, logger)
	if err != nil {
		logger.Errorf("Failed to create contract bindings", "err", err)
		return nil, err
	}
	streamSubscriber := StreamReader{
		Backend: ethclient,
	}
	return &AvsSubscriber{
		AvsContractBindings: avsContractBindings,
		StreamSubscriber:  &streamSubscriber,
		logger:              logger,
	}, nil
}

func (s *AvsSubscriber) SubscribeToNewRdTasks(newTaskCreatedChan chan *taskmanager.ContractFinalizerTaskManagerNewRdTaskCreated) event.Subscription {
	sub, err := s.AvsContractBindings.TaskManager.WatchNewRdTaskCreated(
		&bind.WatchOpts{}, newTaskCreatedChan, nil,
	)
	if err != nil {
		s.logger.Error("Failed to subscribe to new TaskManager tasks", "err", err)
	}
	s.logger.Infof("Subscribed to new TaskManager tasks")
	return sub
}

func (s *AvsSubscriber) SubscribeToRdTaskResponses(taskResponseLogs chan *taskmanager.ContractFinalizerTaskManagerRdTaskResponded) event.Subscription {
	sub, err := s.AvsContractBindings.TaskManager.WatchRdTaskResponded(
		&bind.WatchOpts{}, taskResponseLogs, []uint32{},
	)
	if err != nil {
		s.logger.Error("Failed to subscribe to TaskResponded events", "err", err)
	}
	s.logger.Infof("Subscribed to TaskResponded events")
	return sub
}

func (s *AvsSubscriber) SubscribeToOpTaskCompleted(opTaskCompletionLogs chan *taskmanager.ContractFinalizerTaskManagerOpTaskCompleted) (event.Subscription, error) {
	sub, err := s.AvsContractBindings.TaskManager.WatchOpTaskCompleted(
		&bind.WatchOpts{}, opTaskCompletionLogs, []uint32{},
	)
	if err != nil {
		s.logger.Error("Failed to subscribe to OpTaskCompleted events", "err", err)
	}
	s.logger.Infof("Subscribed to OpTaskCompleted events")
	return sub, err
}

func (s *AvsSubscriber) SubscribeToResumeTrackingOpState(resumeLogs chan *taskmanager.ContractFinalizerTaskManagerResumeTrackingOpState) (event.Subscription, error) {
	sub, err := s.AvsContractBindings.TaskManager.WatchResumeTrackingOpState(
		&bind.WatchOpts{}, resumeLogs,
	)
	if err != nil {
		s.logger.Error("Failed to subscribe to ResumeTrackingOpState events", "err", err)
	}
	s.logger.Infof("Subscribed to ResumeTrackingOpState events")
	return sub, err
}

func (s *AvsSubscriber) SubscribeToOperatorStakeUpdate(updateLogs chan *stakeRegistry.ContractStakeRegistryOperatorStakeUpdate) (event.Subscription, error) {
	sub, err := s.AvsContractBindings.StakeRegistry.WatchOperatorStakeUpdate(
		&bind.WatchOpts{}, updateLogs, [][32]byte{},
	)
	if err != nil {
		s.logger.Error("Failed to subscribe to OperatorStakeUpdate events", "err", err)
	}
	s.logger.Infof("Subscribed to OperatorStakeUpdate events")
	return sub, err
}

func (s *AvsSubscriber) ParseRdTaskResponded(rawLog types.Log) (*taskmanager.ContractFinalizerTaskManagerRdTaskResponded, error) {
	return s.AvsContractBindings.TaskManager.ContractFinalizerTaskManagerFilterer.ParseRdTaskResponded(rawLog)
}
