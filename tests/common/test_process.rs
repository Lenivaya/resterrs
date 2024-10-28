use rand::Rng;
use std::process::{Child, Command};
use std::thread;
use std::time::Duration;

pub struct TestProcess {
    child: Child,
    pub name: String,
}

impl Default for TestProcess {
    fn default() -> Self {
        Self::new()
    }
}

impl TestProcess {
    pub fn new() -> Self {
        let time = Self::get_random_sleep_time();
        let sleep_command = format!("sleep {}", time);
        let child = Command::new("sh")
            .arg("-c")
            .arg(&sleep_command)
            .spawn()
            .expect("Failed to spawn test process");

        Self {
            child,
            name: sleep_command,
        }
    }

    fn get_random_sleep_time() -> u64 {
        rand::thread_rng().gen_range(30..50)
    }

    pub fn is_running(&mut self) -> bool {
        match self.child.try_wait() {
            Ok(None) => true,
            Ok(Some(_)) => false,
            Err(_) => false,
        }
    }

    pub fn wait_for_exit(&mut self, timeout: Duration) -> bool {
        let start = std::time::Instant::now();
        while self.is_running() {
            if start.elapsed() > timeout {
                return false;
            }
            thread::sleep(Duration::from_millis(100));
        }
        true
    }

    pub fn pid(&self) -> u32 {
        self.child.id()
    }
}

impl Drop for TestProcess {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
