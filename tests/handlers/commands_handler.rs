use anyhow::Result;
use std::fs::{self};
use std::path::PathBuf;
use std::time::Duration;

use resterrs::common::PowerState;
use resterrs::handlers::commands_power_state_change_handler::CommandsPowerStateChangeHandler;
use resterrs::traits::power_state_change_handler::PowerStateChangeHandler;
use tracing_test::traced_test;

struct TestMarkerFile {
    path: PathBuf,
}

impl TestMarkerFile {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("test_marker_{}", name));
        Self { path }
    }

    fn exists(&self) -> bool {
        self.path.exists()
    }

    fn delete(&self) -> Result<()> {
        if self.path.exists() {
            fs::remove_file(&self.path)?;
        }
        Ok(())
    }
}

impl Drop for TestMarkerFile {
    fn drop(&mut self) {
        let _ = self.delete();
    }
}

#[test]
#[traced_test]
fn test_commands_handler_unplugged_executes_commands() -> Result<()> {
    let marker = TestMarkerFile::new("unplugged");
    marker.delete()?; // Ensure clean state

    let create_marker = format!("touch {}", marker.path.display());
    let handler = CommandsPowerStateChangeHandler::new(Some(vec![create_marker]), None);

    handler.handle(&PowerState::Unplugged)?;

    // Give it a moment to execute
    std::thread::sleep(Duration::from_millis(100));

    assert!(
        marker.exists(),
        "Command should have created the marker file"
    );
    Ok(())
}

#[test]
fn test_commands_handler_plugged_executes_commands() -> Result<()> {
    let marker = TestMarkerFile::new("plugged");
    marker.delete()?; // Ensure clean state

    let create_marker = format!("touch {}", marker.path.display());
    let handler = CommandsPowerStateChangeHandler::new(None, Some(vec![create_marker]));

    handler.handle(&PowerState::Plugged)?;

    // Give it a moment to execute
    std::thread::sleep(Duration::from_millis(100));

    assert!(
        marker.exists(),
        "Command should have created the marker file"
    );
    Ok(())
}

#[test]
#[traced_test]
fn test_commands_handler_executes_nothing_when_no_commands() -> Result<()> {
    let marker_unplugged = TestMarkerFile::new("no_command_unplugged");
    let marker_plugged = TestMarkerFile::new("no_command_plugged");

    marker_unplugged.delete()?;
    marker_plugged.delete()?;

    let handler = CommandsPowerStateChangeHandler::new(None, None);

    // Should do nothing and not fail
    handler.handle(&PowerState::Unplugged)?;
    handler.handle(&PowerState::Plugged)?;

    std::thread::sleep(Duration::from_millis(100));

    assert!(
        !marker_unplugged.exists(),
        "No command should have run for unplugged state"
    );
    assert!(
        !marker_plugged.exists(),
        "No command should have run for plugged state"
    );

    Ok(())
}

#[test]
#[traced_test]
fn test_commands_handler_handles_multiple_commands() -> Result<()> {
    let marker1 = TestMarkerFile::new("multi_1");
    let marker2 = TestMarkerFile::new("multi_2");

    marker1.delete()?;
    marker2.delete()?;

    let create_marker1 = format!("touch {}", marker1.path.display());
    let create_marker2 = format!("touch {}", marker2.path.display());

    let handler =
        CommandsPowerStateChangeHandler::new(Some(vec![create_marker1, create_marker2]), None);

    handler.handle(&PowerState::Unplugged)?;

    std::thread::sleep(Duration::from_millis(100));

    assert!(
        marker1.exists(),
        "First command should have created its marker file"
    );
    assert!(
        marker2.exists(),
        "Second command should have created its marker file"
    );

    Ok(())
}

#[test]
#[traced_test]
fn test_commands_handler_continues_on_failed_command() -> Result<()> {
    let marker = TestMarkerFile::new("after_fail");
    marker.delete()?;

    let failing_command = "nonexistent_command_that_should_fail".to_string();
    let create_marker = format!("touch {}", marker.path.display());

    let handler =
        CommandsPowerStateChangeHandler::new(Some(vec![failing_command, create_marker]), None);

    handler.handle(&PowerState::Unplugged)?;

    std::thread::sleep(Duration::from_millis(100));

    assert!(
        marker.exists(),
        "Second command should execute even after first fails"
    );

    Ok(())
}
