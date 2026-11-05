#[derive(Debug, Clone, PartialEq)]
pub struct CommandExecution {
    pub command: String,
    pub exit_code: i32,
    pub execution_time_ms: u64,
}

pub struct CommandCodeSession {
    pub history: Vec<CommandExecution>,
}

impl Default for CommandCodeSession {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandCodeSession {
    pub fn new() -> Self {
        Self { history: Vec::new() }
    }

    pub fn record(&mut self, command: impl Into<String>, exit_code: i32, time_ms: u64) {
        self.history.push(CommandExecution {
            command: command.into(),
            exit_code,
            execution_time_ms: time_ms,
        });
    }

    pub fn last_succeeded(&self) -> bool {
        self.history.last().map(|c| c.exit_code == 0).unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_session() {
        // Arrange
        let mut session = CommandCodeSession::new();
        // Act
        session.record("cargo check", 0, 150);
        // Assert
        assert!(session.last_succeeded());
    }
}
