pub mod controller;
pub mod level;
pub mod parser;

use controller::{Controller, FanError, FanResult};
use libc::__error;
use signal_hook::{iterator::Signals, consts::{SIGHUP, SIGINT, SIGTERM}};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::{thread, time::Duration};

fn main() -> FanResult {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    let mut signals = Signals::new(&[SIGTERM, SIGINT, SIGHUP])?;
    let sig_handle = signals.handle();

    let handler = thread::spawn(move || {
        for sig in &mut signals {
            println!("Received signal {:?}", sig);
            match sig {
                SIGTERM | SIGINT | SIGHUP => {
                    r.store(false, Ordering::SeqCst);
                }
                _ => {}
            }
        }
    });

    let mut c = Controller::new();
    c.start_fan("/usr/local/etc/bsdfan.conf")?;

    let delay = Duration::from_millis(c.delay() as u64);

    while running.load(Ordering::SeqCst) {
        match c.get_temp() {
            Some(t) => c.adjust_level(t),
            None => Err(FanError::SysctlError(unsafe { *__error() })),
        }?;
        thread::sleep(delay);
    }

    println!("Done");

    sig_handle.close();
    handler.join().expect("singal handler panic");

    Ok(())
}
