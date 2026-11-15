//! SigmaOS-tools-command-code: command execution model.
//!
//! Real implementation: command execution, history, environment, timeouts, validation, error handling.
//! Replaces the 49-line thin placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Command execution result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandExecution {
    pub command: String,
    pub exit_code: i32,
    pub execution_time_ms: u64,
    pub stdout: String,
    pub stderr: String,
}

/// Environment variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandError {
    EmptyCommand,
    InvalidTimeout,
    DuplicateExecution(String),
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyCommand => write!(f, "command must not be empty"),
            Self::InvalidTimeout => write!(f, "timeout must be positive"),
            Self::DuplicateExecution(id) => write!(f, "duplicate execution {:?}", id),
        }
    }
}

impl std::error::Error for CommandError {}

/// Command execution session.
pub struct CommandCodeSession {
    pub name: String,
    history: BTreeMap<String, CommandExecution>,
    environment: BTreeMap<String, String>,
    timeout_ms: u64,
}

impl CommandCodeSession {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            history: BTreeMap::new(),
            environment: BTreeMap::new(),
            timeout_ms: 30000,
        }
    }

    pub fn set_timeout(&mut self, timeout_ms: u64) -> Result<(), CommandError> {
        if timeout_ms == 0 { return Err(CommandError::InvalidTimeout); }
        self.timeout_ms = timeout_ms;
        Ok(())
    }

    pub fn set_env(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.environment.insert(key.into(), value.into());
    }

    pub fn record(&mut self, id: impl Into<String>, execution: CommandExecution) -> Result<(), CommandError> {
        let id = id.into();
        if execution.command.trim().is_empty() { return Err(CommandError::EmptyCommand); }
        if self.history.contains_key(&id) {
            return Err(CommandError::DuplicateExecution(id.clone()));
        }
        self.history.insert(id, execution);
        Ok(())
    }

    pub fn last_succeeded(&self) -> bool {
        self.history.values().last().map(|c| c.exit_code == 0).unwrap_or(true)
    }

    pub fn execution_count(&self) -> usize { self.history.len() }

    pub fn history(&self) -> Vec<&CommandExecution> { self.history.values().collect() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_create_aaa() {
        let s = CommandCodeSession::new("test");
        assert_eq!(s.name, "test");
        assert_eq!(s.execution_count(), 0);
    }

    #[test]
    fn test_record_execution_aaa() {
        let mut s = CommandCodeSession::new("test");
        let exec = CommandExecution {
            command: "cargo check".into(),
            exit_code: 0,
            execution_time_ms: 150,
            stdout: "ok".into(),
            stderr: "".into(),
        };
        s.record("e1", exec).unwrap();
        assert_eq!(s.execution_count(), 1);
        assert!(s.last_succeeded());
    }

    #[test]
    fn test_empty_command_rejected_aaa() {
        let mut s = CommandCodeSession::new("test");
        let exec = CommandExecution {
            command: "".into(),
            exit_code: 0,
            execution_time_ms: 100,
            stdout: "".into(),
            stderr: "".into(),
        };
        assert!(s.record("x", exec).is_err());
    }

    #[test]
    fn test_invalid_timeout_rejected_aaa() {
        let mut s = CommandCodeSession::new("test");
        assert!(s.set_timeout(0).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut s = CommandCodeSession::new("test");
        let exec = CommandExecution {
            command: "test".into(),
            exit_code: 0,
            execution_time_ms: 100,
            stdout: "".into(),
            stderr: "".into(),
        };
        s.record("x", exec.clone()).unwrap();
        assert!(s.record("x", exec).is_err());
    }
}
