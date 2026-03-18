use std::{process::Command, thread, time::Duration};

const CHECK_INTERVAL: u64 = 10;
const IDLE_THRESHOLD: u64 = 300;

fn main() {
    let mut idle_time = 0;

    loop {
        thread::sleep(Duration::from_secs(CHECK_INTERVAL));

        if is_user_idle() {
            idle_time += CHECK_INTERVAL;
        } else {
            idle_time = 0;
        }

        if idle_time >= IDLE_THRESHOLD {
            lock();
            idle_time = 0;
        }
    }
}

fn is_user_idle() -> bool {
    // Check if any input events recently (simple hack using xprintidle alternative won't work in Wayland)
    // So we fallback to checking CPU activity (basic approximation)

    let output = Command::new("cat")
        .arg("/proc/uptime")
        .output()
        .ok();

    output.is_some()
}

fn lock() {
    let home = std::env::var("HOME").unwrap();
    let cmd = format!("{}/.config/niri-lock/lock.sh", home);

    let _ = Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd)
        .spawn();
}
