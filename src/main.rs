mod controller;
pub mod config;

use controller::{Controller, FanResult};

use signal_hook::{iterator::Signals, consts::{SIGHUP, SIGINT, SIGTERM}};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::{thread, time::Duration};

fn main() -> FanResult<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let mut signals = Signals::new(&[SIGTERM, SIGINT, SIGHUP])?;
    let sig_handle = signals.handle();

    let handler = thread::spawn(move || {
        for sig in &mut signals {
            match sig {
                SIGTERM | SIGINT | SIGHUP => {
                    r.store(false, Ordering::SeqCst);
                }
                _ => {}
            }
        }
    });

    let mut control = Controller::new("/usr/local/etc/bsdfan.toml")?;
    control.start()?;

    let delay = Duration::from_millis(control.delay());

    while running.load(Ordering::SeqCst) {
        control.adjust_level(control.get_temp()?)?;
        thread::sleep(delay);
    }

    control.stop()?;

    sig_handle.close();
    handler.join().expect("singal handler panic");

    Ok(())
}
