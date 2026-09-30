//! The logic and state for running and storing the commands
//!
//! This will _not_ have any UI logic, though much of the design and structure
//! will be greatly influenced by how the UI will display the state here.

/// A Single command that can be run
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Command {}

/// The output of running a command
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Output {}

/// A store of all commands and their results
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Store {}
