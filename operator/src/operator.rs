use std::sync::Arc;

use crate::bindings::*;
use crate::config::NodeConfig;
// use bindings::incredible_squaring_task_manager::IncredibleSquaringTaskManagerEvents::NewTaskCreatedFilter;
use bindings::incredible_squaring_task_manager::NewTaskCreatedFilter;
use ethers::prelude::*;

type MW = Provider<Ws>;

pub struct Operator {
    service_manager: ServiceManager<MW>,
    task_manager: TaskManager<MW>,
    client: Arc<MW>,
}

impl Operator {
    pub async fn from_config(config: &NodeConfig) -> eyre::Result<Self> {
        let provider = MW::connect(&config.eth_ws_url).await?;
        let client = Arc::new(provider);
        let address: Address = config.avs_service_manager_address.parse()?;
        let service_manager = ServiceManager::new(address, client.clone());

        let address = service_manager
            .incredible_squaring_task_manager()
            .call()
            .await?;
        let task_manager = TaskManager::new(address, client.clone());

        Ok(Self {
            service_manager,
            task_manager,
            client,
        })
    }

    pub async fn watch_new_tasks(&self) -> eyre::Result<()> {
        let evs = self.task_manager.new_task_created_filter();
        let mut stream: stream::EventStream<'_, _, NewTaskCreatedFilter, _> = evs.subscribe().await?;
        while let Some(Ok(event)) = stream.next().await {
            println!("{:?}", event);
        }
        Ok(())
    }
}
