use resterrs::power_state::PowerState;
use resterrs::power_state_tracker::PowerStateTracker;
use tracing_test::traced_test;

#[test]
#[traced_test]
fn test_should_handle_first_event() {
    let mut tracker = PowerStateTracker::new();
    assert!(tracker.should_handle(&PowerState::Plugged));
}

#[test]
#[traced_test]
fn test_should_handle_state_change() {
    let mut tracker = PowerStateTracker::new();

    // First event
    assert!(tracker.should_handle(&PowerState::Plugged));

    // Same state - should not handle
    assert!(!tracker.should_handle(&PowerState::Plugged));

    // Different state - should handle
    assert!(tracker.should_handle(&PowerState::Unplugged));
}

#[test]
#[traced_test]
fn test_should_not_handle_same_state() {
    let mut tracker = PowerStateTracker::new();

    tracker.should_handle(&PowerState::Plugged);
    assert!(!tracker.should_handle(&PowerState::Plugged));
}
