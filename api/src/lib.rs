#![no_std]
#![feature(likely_unlikely)]
#![feature(bstr)]
#![allow(missing_docs)]

#[macro_use]
extern crate axlog;

extern crate alloc;

pub mod file;
pub mod io;
pub mod mm;
pub mod signal;
pub mod socket;
pub mod syscall;
pub mod task;
pub mod terminal;
pub mod time;
pub mod vfs;

/// Initialize.
pub fn init() {
    info!("Initialize VFS...");
    vfs::mount_all().expect("Failed to mount vfs");

    info!("Initialize /proc/interrupts...");
    axtask::register_timer_callback(|_| {
        time::inc_irq_cnt();
    });

    info!("Initialize alarm...");
    starry_core::time::spawn_alarm_task();
}
