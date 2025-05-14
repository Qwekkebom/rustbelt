//! This module defines the `ExampleCommand` which is an example implementation of a command
//! using the `Command` trait. It demonstrates how to register a command and implement its
//! execution logic.

use clap::Command as ClapCommand;
use windows::{core::*, Win32::System::EventLog};
use crate::{
    commands::base::registry::CommandRegistration,
    commands::base::{
        Command, CommandDTO, CommandData,
        CommandResult::{self, Simple},
    },
    runtime::Runtime,
};

pub struct ProcessCreationEventCommand {
    data: CommandData,
}

inventory::submit! {
    CommandRegistration {
        name: "processcreationevents",
        factory: || Box::new(ProcessCreationEventCommand::default()),
        clap_command: || ClapCommand
            ::new("processcreationevents")
            .version("1.0")
            .about("An command to check the process creation events (id: 4688) for sensitive data.")
    }
}

impl Command for ProcessCreationEventCommand {
    fn execute(&self, runtime: &Runtime, _: &[String]) -> Result<CommandResult> {
        Ok(Simple(CommandDTO {
            source: "Example".to_string(),
            data: vec![],
        }))
    }
}

impl Default for ProcessCreationEventCommand {
    fn default() -> Self {
        ProcessCreationEventCommand {
            data: CommandData {
                support_remote: false,
            },
        }
    }
}
