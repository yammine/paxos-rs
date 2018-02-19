#![feature(test, option_filter)]
#![allow(unknown_lints)]
//! Rust implementation of the Paxos algorithm for replicated state machines.
//!
//! The implementation of multi-decree paxos uses multiple instances of the Paxos consus algorithm
//! to chain together commands against the replicated state machine.
//!
//! # Examples
//!
//! ```rust,no_run
//! # use paxos::{MultiPaxosBuilder, Configuration, UdpServer};
//! let config = Configuration::new(
//!     (0u32, "127.0.0.1:4000".parse().unwrap()),
//!     vec![(1, "127.0.0.1:4001".parse().unwrap()),
//!          (2, "127.0.0.1:4002".parse().unwrap())].into_iter());
//!
//! let (proposal_sink, multipaxos) = MultiPaxosBuilder::new(config.clone()).build();
//!
//! let server = UdpServer::new(config).unwrap();
//! server.run(multipaxos).unwrap();
//! ```
#[cfg(test)]
#[macro_use]
extern crate assert_matches;
extern crate either;
#[macro_use]
extern crate futures;
extern crate futures_timer;
#[macro_use]
extern crate log;
extern crate rand;
extern crate serde;
extern crate serde_cbor;
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
extern crate test;
extern crate tokio_core;

mod algo;
mod state;
mod statemachine;
mod master;
pub mod messages;
mod multipaxos;
mod net;
mod register;
mod config;
pub mod timer;
mod proposals;

pub use multipaxos::{Instance, MultiPaxos};
pub use statemachine::ReplicatedState;
pub use net::UdpServer;
pub use register::Register;
pub use config::{Configuration, PeerIntoIter, PeerIter};
pub use algo::{BytesValue, NodeId, Value};
pub use proposals::ProposalSender;
use timer::{FuturesScheduler, Scheduler};
use master::{DistinguishedProposer, MasterStrategy, Masterless};

/// Builder for the MultiPaxos node
pub struct MultiPaxosBuilder<R: ReplicatedState, M: MasterStrategy, S: Scheduler> {
    state_machine: R,
    config: Configuration,
    master_strategy: M,
    scheduler: S,
}

impl MultiPaxosBuilder<Register, DistinguishedProposer<FuturesScheduler>, FuturesScheduler> {
    /// Creates a default implementation of MultiPaxos that uses a `Register` as the state machine,
    /// `DistinguishedProposer` master strategy, and default scheduler.
    pub fn new(
        config: Configuration,
    ) -> MultiPaxosBuilder<Register, DistinguishedProposer<FuturesScheduler>, FuturesScheduler>
    {
        let master_strategy = DistinguishedProposer::new(config.clone(), FuturesScheduler);
        MultiPaxosBuilder {
            state_machine: Register::default(),
            config,
            master_strategy,
            scheduler: FuturesScheduler,
        }
    }
}

impl<R: ReplicatedState, M: MasterStrategy, S: Scheduler> MultiPaxosBuilder<R, M, S> {
    /// Sets the state machine
    pub fn with_state_machine<SM: ReplicatedState>(
        self,
        state_machine: SM,
    ) -> MultiPaxosBuilder<SM, M, S> {
        MultiPaxosBuilder {
            state_machine,
            config: self.config,
            master_strategy: self.master_strategy,
            scheduler: self.scheduler,
        }
    }

    /// Sets the master strategy to utilize masterless
    pub fn with_masterless_strategy(self) -> MultiPaxosBuilder<R, Masterless<S>, S> {
        let master_strategy = Masterless::new(self.config.clone(), self.scheduler.clone());
        MultiPaxosBuilder {
            state_machine: self.state_machine,
            config: self.config,
            master_strategy,
            scheduler: self.scheduler,
        }
    }

    /// Sets the master strategy to utilize a distinguished proposer
    pub fn with_distinguished_proposer(self) -> MultiPaxosBuilder<R, DistinguishedProposer<S>, S> {
        let master_strategy =
            DistinguishedProposer::new(self.config.clone(), self.scheduler.clone());
        MultiPaxosBuilder {
            state_machine: self.state_machine,
            config: self.config,
            master_strategy,
            scheduler: self.scheduler,
        }
    }

    /// Sets the scheduler used by MultiPaxos
    pub fn with_scheduler<T: Scheduler>(self, scheduler: T) -> MultiPaxosBuilder<R, M, T> {
        MultiPaxosBuilder {
            state_machine: self.state_machine,
            config: self.config,
            master_strategy: self.master_strategy,
            scheduler,
        }
    }

    /// Builds the multi-paxos instance
    pub fn build(self) -> (ProposalSender<R::Command>, MultiPaxos<R, M, S>) {
        let (sink, stream) = proposals::proposal_channel::<R::Command>();
        let multi_paxos = MultiPaxos::new(
            self.scheduler,
            stream,
            self.state_machine,
            self.config,
            self.master_strategy,
        );
        (sink, multi_paxos)
    }
}
