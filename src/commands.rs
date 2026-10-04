//! The logic and state for running and storing the commands
//!
//! This will _not_ have any UI logic, though much of the design and structure
//! will be greatly influenced by how the UI will display the state here.
//!
//! The basic building block is the [`Cmd`] which is just a wrapper around the
//! command being run with some assoicated meta data. Each [`Cmd`] is associated
//! with a unique [`CmdId`] in the [`Store`].
//! The [`Store`] contains:
//! - Set of all commands
//! - Set of [`Child`] processes that have been started
//! - The [`ChildStatus`] of all started processes
//!
//! The [`Store`] is the main structure you interface with to start and parse
//! commands.
//!

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::process::{Child, Command, ExitStatus, Stdio};

use snafu::prelude::*;

/// Encapsulate the unique id generator
mod cmd_id {

    /// A unique identifier for a given command
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
    #[must_use]
    pub struct CmdId(usize);

    impl CmdId {
        pub(super) fn next() -> Self {
            use std::sync::atomic::{AtomicUsize, Ordering};
            static CMD_COUNTER: AtomicUsize = AtomicUsize::new(1);

            Self(CMD_COUNTER.fetch_add(1, Ordering::Relaxed))
        }
    }
}
pub use cmd_id::CmdId;

/// A single command that can be run and is verified to be correct
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Cmd {
    /// The program to run
    program: String,
    /// The rest of the arguments (may be empty)
    args: String,
}

/// Error for when parsing [`Cmd`] from a string fails
#[derive(Debug, Snafu, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[snafu(module, display("Failed to parse '{s}' as `Cmd`"))]
pub struct CmdParseError {
    s: String,
}

impl std::str::FromStr for Cmd {
    type Err = CmdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim_start();

        if let Some((program, args)) = trimmed.split_once(char::is_whitespace) {
            Ok(Self {
                program: program.to_string(),
                args: args.to_string(),
            })
        } else if !trimmed.is_empty() {
            Ok(Self {
                program: trimmed.to_string(),
                args: String::new(),
            })
        } else {
            Err(CmdParseError { s: s.to_string() })
        }
    }
}

/// Errors for the [`Store::run_cmd`]
#[derive(Debug, Snafu, Clone, Copy)]
#[snafu(module, context(suffix(false)))]
#[expect(missing_docs)]
pub enum RunError {
    #[snafu(display("Could not find {cmd_id:?}"))]
    CmdNotFound { cmd_id: CmdId },
}

/// The status of a child
#[derive(Debug)]
pub enum ChildStatus {
    /// Spawning the child failed
    SpawnFailed(std::io::Error),
    /// The child is (potentially) still running
    Running,
    /// The child exited with status code of 0
    Success,
    /// The child exited with status code other than 0
    Failed(ExitStatus),
    /// The child ran but returned an error later
    Errored(std::io::Error),
}

/// A store of all commands and their results
#[derive(Default, Debug)]
pub struct Store {
    cmds: HashMap<CmdId, Cmd>,
    /// Set of all spawned children, indexed by their ID
    children: HashMap<CmdId, Child>,
    /// The results or status of the children that have been run
    statuses: HashMap<CmdId, ChildStatus>,
}

impl Store {
    /// Add's the command to the store (without running it) and returns it's [`CmdId`]
    ///
    /// Returns an error if the cmd fails to parse
    pub fn add_cmd(&mut self, cmd_str: &str) -> Result<(CmdId, &Cmd), CmdParseError> {
        let cmd = cmd_str.parse()?;
        let cmd_id = CmdId::next();
        match self.cmds.entry(cmd_id) {
            Entry::Occupied(_) => {
                panic!("Failed to insert '{cmd_str}', store already contains cmd at {cmd_id:?}");
            }
            Entry::Vacant(vacant_entry) => Ok((cmd_id, vacant_entry.insert(cmd))),
        }
    }

    /// Checks if the given command has started running
    #[inline]
    pub fn has_started(&self, cmd_id: CmdId) -> bool {
        self.children.contains_key(&cmd_id)
    }

    /// Try to run the given command, adding the [`Child`] to the store if it starts
    ///
    /// If the command has already been run, it does nothing
    ///
    /// Returns an error if the command isn't found
    pub fn run_cmd(&mut self, cmd_str: &str) -> Result<CmdId, CmdParseError> {
        let (cmd_id, cmd) = self.add_cmd(cmd_str)?;

        match Command::new(&cmd.program)
            .args(cmd.args.split_whitespace())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
        {
            Ok(child) => {
                self.children.insert(cmd_id, child);
                self.statuses.insert(cmd_id, ChildStatus::Running);
            }
            Err(err) => {
                self.statuses.insert(cmd_id, ChildStatus::SpawnFailed(err));
            }
        }

        Ok(cmd_id)
    }

    /// Wait for all [`ChildStatus::Running`] processes to finish
    ///
    /// (probably should be called before terminating the parent process)
    pub fn wait_all(&mut self) {
        for (cmd_id, child) in &mut self.children {
            if !matches!(self.statuses.get(cmd_id), Some(ChildStatus::Running)) {
                continue;
            }
            self.statuses.insert(
                *cmd_id,
                match child.wait() {
                    Ok(exit_status) if exit_status.success() => ChildStatus::Success,
                    Ok(exit_status) => ChildStatus::Failed(exit_status),
                    Err(err) => ChildStatus::Errored(err),
                },
            );
        }
    }
}
