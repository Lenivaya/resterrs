use anyhow::Result;
use tracing_test::traced_test;

use resterrs::common::PowerState;
use resterrs::handlers::systemd_power_state_change_handler::SystemdPowerStateChangeHandler;
use resterrs::traits::power_state_change_handler::PowerStateChangeHandler;

use crate::common::mocks::MockSystemdServiceManager;

#[test]
#[traced_test]
fn test_handle_plugged_state() -> Result<()> {
    let mut mock_service_manager = MockSystemdServiceManager::new();

    mock_service_manager
        .expect_start()
        .times(2)
        .returning(|_| Ok(()));

    let services = vec!["service1".to_string(), "service2".to_string()];
    let handler = SystemdPowerStateChangeHandler::new(services, Box::new(mock_service_manager));

    handler.handle(&PowerState::Plugged)?;
    Ok(())
}

#[test]
#[traced_test]
fn test_handle_unplugged_state() -> Result<()> {
    let mut mock_service_manager = MockSystemdServiceManager::new();

    mock_service_manager
        .expect_stop()
        .times(2)
        .returning(|_| Ok(()));

    let services = vec!["service1".to_string(), "service2".to_string()];
    let handler = SystemdPowerStateChangeHandler::new(services, Box::new(mock_service_manager));

    handler.handle(&PowerState::Unplugged)?;
    Ok(())
}
