use crate::{executor, network, storage, telemetry};
use futures::{executor::ThreadPool, prelude::*};
use primitive_types::H256;

pub use builder::{builder, ServiceBuilder};

mod builder;

pub struct Service {
    /// Collection of all the Wasm VMs that are currently running.
    wasm_vms: executor::WasmVirtualMachines<()>,
    /// Database of the state of all the blocks.
    storage: storage::Storage,
    /// Management of the network. Contains all the active connections and their state.
    network: network::Network,
    /// Connections to zero or more telemetry servers.
    telemetry: telemetry::Telemetry,

    /// Optional threads pool that is used to dispatch tasks and that we keep alive.
    _threads_pool: Option<ThreadPool>,
}

pub enum Event {
    /// Head of the chain has been updated.
    NewChainHead(u64),

    /// The finalized block has been updated to a different one.
    NewFinalized {
        /// Number of the finalized block.
        number: u64,
        /// Hash of the finalized block.
        hash: H256,
    },
}

impl Service {
    /// Returns the next event that happens in the service.
    pub async fn next_event(&mut self) -> Event {
        /*let block0 = "0000000000000000000000000000000000000000000000000000000000000000".parse().unwrap();
        let wasm_runtime = executor::WasmBlob::from_bytes(self.storage.block(&block0).storage().unwrap().code_key().unwrap()).unwrap();
        self.wasm_vms
            .execute((), &wasm_runtime, "Core_version", &[]);*/

        loop {
            let event = {
                let network_next = self.network.next_event();
                /*let telemetry_next = async move {
                    self.telemetry.next_event().await
                };*/
                futures::pin_mut!(network_next);
                network_next.await
            };

            match event {
                network::Event::BlockAnnounce(header) => {
                    self.network.start_block_request(header.number).await;
                    return Event::NewChainHead(header.number); // TODO: not necessarily the head
                }
                network::Event::BlocksRequestFinished { result } => {
                    println!("{:?}", result);
                }
            }
        }
    }
}
