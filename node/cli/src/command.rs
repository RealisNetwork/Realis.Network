// This file is part of Substrate.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::{Cli, Subcommand};
use futures::future::TryFutureExt;
use sc_cli::{Role, RuntimeVersion, SubstrateCli};
use sp_core::crypto::Ss58AddressFormatRegistry;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    PolkadotService(#[from] service::Error),

    #[error(transparent)]
    SubstrateCli(#[from] sc_cli::Error),

    #[error(transparent)]
    SubstrateService(#[from] sc_service::Error),

    #[error("Other: {0}")]
    Other(String),
}

impl std::convert::From<String> for Error {
    fn from(s: String) -> Self {
        Self::Other(s)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl SubstrateCli for Cli {
    fn impl_name() -> String {
        "Realis Node".into()
    }

    fn impl_version() -> String {
        env!("SUBSTRATE_CLI_IMPL_VERSION").into()
    }

    fn description() -> String {
        env!("CARGO_PKG_DESCRIPTION").into()
    }

    fn author() -> String {
        env!("CARGO_PKG_AUTHORS").into()
    }

    fn support_url() -> String {
        "https://github.com/paritytech/substrate/issues/new".into()
    }

    fn copyright_start_year() -> i32 {
        2021
    }

    fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
        let spec = match id {
            "" => {
                return Err(
                    "Please specify which chain you want to run, e.g. --dev or --chain=local"
                        .into(),
                )
            }
            "dev" => Box::new(service::chain_spec::polkadot_development_config()?),
            "local" => Box::new(service::chain_spec::polkadot_local_testnet_config()?),
            "realis" => Box::new(service::chain_spec::polkadot_config()?),
            // "realis2" => Box::new(service::chain_spec::po_testnet_config()),
            "fir" | "flaming-fir" => Box::new(service::chain_spec::flaming_fir_config()?),
            "staging" => Box::new(service::chain_spec::polkadot_staging_testnet_config()?),
            path => Box::new(service::RealisChainSpec::from_json_file(
                std::path::PathBuf::from(path),
            )?),
        };
        Ok(spec)
    }

    fn native_runtime_version(_: &Box<dyn service::ChainSpec>) -> &'static RuntimeVersion {
        &node_runtime::VERSION
    }
}

fn set_default_ss58_version(_spec: &Box<dyn service::ChainSpec>) {
    let ss58_version = Ss58AddressFormatRegistry::SubstrateAccount.into();

    sp_core::crypto::set_default_ss58_version(ss58_version);
}

/// Launch a node, accepting arguments just like a regular node,
/// accepts an alternative overseer generator, to adjust behavior
/// for integration tests as needed.
#[cfg(feature = "malus")]
pub fn run_node(cli: Cli, overseer_gen: impl service::OverseerGen) -> Result<()> {
    run_node_inner(cli, overseer_gen)
}

fn run_node_inner(cli: Cli, overseer_gen: impl service::OverseerGen) -> Result<()> {
    let runner = cli.create_runner(&cli.run.base).map_err(Error::from)?;
    let chain_spec = &runner.config().chain_spec;

    set_default_ss58_version(chain_spec);

    let grandpa_pause = if cli.run.grandpa_pause.is_empty() {
        None
    } else {
        Some((cli.run.grandpa_pause[0], cli.run.grandpa_pause[1]))
    };

    let jaeger_agent = cli.run.jaeger_agent;

    runner.run_node_until_exit(move |config| async move {
        let role = config.role.clone();

        match role {
            Role::Light => Err(Error::Other("Light client not enabled".into())),
            _ => service::build_full(
                config,
                service::IsCollator::No,
                grandpa_pause,
                cli.run.no_beefy,
                jaeger_agent,
                None,
                overseer_gen,
            )
                .map(|full| full.task_manager)
                .map_err(Into::into),
        }
    })
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
    let cli = Cli::from_args();

    match &cli.subcommand {
        None => {
            // let runner = cli.create_runner(&cli.run.base)?;
            // runner.run_node_until_exit(|config| async move {
            //     service::new_full(config).map_err(sc_cli::Error::Service)
            // })
            run_node_inner(cli, service::RealOverseerGen)
        }
        Some(Subcommand::Inspect(cmd)) => {
            let runner = cli.create_runner(cmd)?;

            runner.sync_run(|config| {
                cmd.run::<service::node_runtime::Block, node_runtime::RuntimeApi, service::PolkadotExecutorDispatch>(
                    config,
                )
                    .map_err(|e| Error::SubstrateCli(e))
            })
        }
        Some(Subcommand::Benchmark(cmd)) => {
            if cfg!(feature = "runtime-benchmarks") {
                let runner = cli.create_runner(cmd)?;

                runner.sync_run(|config| {
                    cmd.run::<service::node_runtime::Block, service::PolkadotExecutorDispatch>(
                        config,
                    )
                        .map_err(|e| Error::SubstrateCli(e))
                })
            } else {
                Err("Benchmarking wasn't enabled when building the node. \
				You can enable it with `--features runtime-benchmarks`."
                    .to_string()
                    .into())
            }
        }
        Some(Subcommand::Key(cmd)) => Ok(cmd.run(&cli)?),
        Some(Subcommand::Sign(cmd)) => Ok(cmd.run()?),
        Some(Subcommand::Verify(cmd)) => Ok(cmd.run()?),
        Some(Subcommand::Vanity(cmd)) => Ok(cmd.run()?),
        Some(Subcommand::BuildSpec(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            Ok(runner.sync_run(|config| cmd.run(config.chain_spec, config.network))?)
        }
        Some(Subcommand::CheckBlock(cmd)) => {
            let runner = cli.create_runner(cmd).map_err(Error::SubstrateCli)?;
            runner.async_run(|mut config| {
                let (client,
                    _,
                    import_queue,
                    task_manager,)
                    = service::new_chain_ops(&mut config, None)?;
                Ok((cmd.run(client, import_queue).map_err(Error::SubstrateCli), task_manager))
            })
        }
        Some(Subcommand::ExportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let (client, _, _, task_manager) =
                    service::new_chain_ops(&mut config, None).map_err(Error::PolkadotService)?;
                Ok((cmd.run(client, config.database).map_err(Error::SubstrateCli), task_manager))
            })
        }
        Some(Subcommand::ExportState(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let (client, _, _, task_manager) =
                    service::new_chain_ops(&mut config, None).map_err(Error::PolkadotService)?;
                Ok((cmd.run(client, config.chain_spec).map_err(Error::SubstrateCli), task_manager))
            })
        }
        Some(Subcommand::ImportBlocks(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let (client, _, import_queue, task_manager) =
                    service::new_chain_ops(&mut config, None)?;
                Ok((cmd.run(client, import_queue).map_err(Error::SubstrateCli), task_manager))
            })
        }
        Some(Subcommand::PurgeChain(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            Ok(runner.sync_run(|config| cmd.run(config.database))?)
        }
        Some(Subcommand::Revert(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|mut config| {
                let (client, backend, _, task_manager) = service::new_chain_ops(&mut config, None)?;
                Ok((cmd.run(client, backend).map_err(Error::SubstrateCli), task_manager))
            })
        }
        #[cfg(feature = "try-runtime")]
        Some(Subcommand::TryRuntime(cmd)) => {
            let runner = cli.create_runner(cmd)?;
            runner.async_run(|config| {
                // we don't need any of the components of new_partial, just a runtime, or a task
                // manager to do `async_run`.
                let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
                let task_manager =
                    sc_service::TaskManager::new(config.task_executor.clone(), registry)
                        .map_err(|e| sc_cli::Error::Service(sc_service::Error::Prometheus(e)))?;

                Ok((cmd.run::<Block, ExecutorDispatch>(config), task_manager))
            })
        }
        #[cfg(not(feature = "try-runtime"))]
        Some(Subcommand::TryRuntime) => Err("TryRuntime wasn't enabled when building the node. \
				You can enable it with `--features try-runtime`."
            .to_string()
            .into()),
    }
}
