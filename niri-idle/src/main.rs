use std::process::Command;

use clap::Parser;
use wayland_client::{
    globals::{registry_queue_init, GlobalListContents},
    protocol::{wl_registry, wl_seat::WlSeat},
    Connection, Dispatch, QueueHandle,
};

mod config;
use config::Config;

// =====================
// CLI
// =====================
#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    timeout: Option<u32>,

    #[arg(short, long)]
    cmd: Option<String>,
}

// =====================
// PROTOCOL
// =====================
mod idle {
    use wayland_client;
    use wayland_client::protocol::*;

    pub mod __interfaces {
        use wayland_client::protocol::__interfaces::*;
        wayland_scanner::generate_interfaces!("protocols/ext-idle-notify-v1.xml");
    }

    use self::__interfaces::*;

    wayland_scanner::generate_client_code!("protocols/ext-idle-notify-v1.xml");
}

use idle::ext_idle_notification_v1::ExtIdleNotificationV1;
use idle::ext_idle_notifier_v1::ExtIdleNotifierV1;

// =====================
// STATE
// =====================
struct State {
    locked: bool,
    cmd: String,
}

// =====================
// DISPATCHES
// =====================
impl Dispatch<wl_registry::WlRegistry, GlobalListContents> for State {
    fn event(
        _state: &mut Self,
        _proxy: &wl_registry::WlRegistry,
        _event: wl_registry::Event,
        _data: &GlobalListContents,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<WlSeat, ()> for State {
    fn event(
        _state: &mut Self,
        _proxy: &WlSeat,
        _event: wayland_client::protocol::wl_seat::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ExtIdleNotifierV1, ()> for State {
    fn event(
        _state: &mut Self,
        _proxy: &ExtIdleNotifierV1,
        _event: idle::ext_idle_notifier_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<ExtIdleNotificationV1, ()> for State {
    fn event(
        state: &mut Self,
        _proxy: &ExtIdleNotificationV1,
        event: idle::ext_idle_notification_v1::Event,
        _data: &(),
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
    ) {
        match event {
            idle::ext_idle_notification_v1::Event::Idled => {
                if !state.locked {
                    state.locked = true;

                    println!("Idle → locking");

                    let _ = Command::new("sh").arg("-c").arg(&state.cmd).spawn();
                }
            }

            idle::ext_idle_notification_v1::Event::Resumed => {
                println!("Resumed");
                state.locked = false;
            }
        }
    }
}

// =====================
// MAIN
// =====================
fn main() {
    let args = Args::parse();
    let config = Config::load();

    let timeout = args.timeout.or(config.timeout).unwrap_or(300_000);

    let cmd = args
        .cmd
        .or(config.command)
        .unwrap_or("~/.config/niri-lock/lock.sh".into());

    println!("Timeout: {} ms", timeout);
    println!("Command: {}", cmd);

    let conn = Connection::connect_to_env().unwrap();

    let (globals, mut event_queue) = registry_queue_init::<State>(&conn).unwrap();

    let qh = event_queue.handle();

    let mut state = State { locked: false, cmd };

    let seat: WlSeat = globals.bind(&qh, 1..=10, ()).unwrap();
    let notifier: ExtIdleNotifierV1 = globals.bind(&qh, 1..=2, ()).unwrap();

    notifier.get_idle_notification(timeout, &seat, &qh, ());
    println!("Idle timer started");

    loop {
        event_queue.blocking_dispatch(&mut state).unwrap();
    }
}
