//! Substrate Parachain Node Template CLI

#![warn(missing_docs)]

pub mod chain_spec;
#[macro_use]
mod service;
pub mod cli;
mod command;
pub mod rpc;

fn main() -> sc_cli::Result<()> {
	command::run()
}
