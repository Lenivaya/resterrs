use anyhow::Result;
use tracing_test::traced_test;

use resterrs::handlers::app_power_state_change_handler::AppPowerStateChangeHandler;
use resterrs::power_state::PowerState;
use resterrs::traits::power_state_change_handler::PowerStateChangeHandler;

use crate::common::test_process::TestProcess;
use std::time::Duration;

const TEST_TIMEOUT: Duration = Duration::from_secs(5);

#[test]
#[traced_test]
fn test_app_handler_unplugged_kills_matching_process() -> Result<()> {
    let mut test_process = TestProcess::new();
    let process_name = test_process.name.clone();

    assert!(
        test_process.is_running(),
        "Test process should be running initially"
    );

    let handler = AppPowerStateChangeHandler::new(vec![process_name]); // Match the actual process name
    handler.handle(&PowerState::Unplugged)?;

    assert!(
        test_process.wait_for_exit(TEST_TIMEOUT),
        "Test process should have been killed"
    );

    Ok(())
}

#[test]
#[traced_test]
fn test_app_handler_unplugged_ignores_non_matching_process() -> Result<()> {
    let mut test_process = TestProcess::new();
    let process_name = test_process.name.clone();

    let another_name = "another one definitely not_matching".to_string();

    assert_ne!(process_name, another_name);

    assert!(
        test_process.is_running(),
        "Test process should be running initially"
    );

    let handler = AppPowerStateChangeHandler::new(vec![another_name]);
    handler.handle(&PowerState::Unplugged)?;

    assert!(
        test_process.is_running(),
        "Process should not be killed when unplugged"
    );

    Ok(())
}

#[test]
#[traced_test]
fn test_app_handler_plugged_preserves_processes() -> Result<()> {
    let mut test_process = TestProcess::new();
    let process_name = test_process.name.clone();

    assert!(
        test_process.is_running(),
        "Test process should be running initially"
    );

    let handler = AppPowerStateChangeHandler::new(vec![process_name]);
    handler.handle(&PowerState::Plugged)?;

    assert!(
        test_process.is_running(),
        "Process should not be killed when plugged"
    );

    Ok(())
}
