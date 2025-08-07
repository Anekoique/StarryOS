use core::{
    mem,
    sync::atomic::{AtomicBool, Ordering},
};

use axerrno::LinuxResult;
use axhal::context::TrapFrame;
use axtask::current;
use starry_core::task::{AsThread, Thread};
use starry_signal::{SignalOSAction, SignalSet, Signo};

use crate::task::do_exit;

pub fn check_signals(thr: &Thread, tf: &mut TrapFrame, restore_blocked: Option<SignalSet>) -> bool {
    let Some((sig, os_action)) = thr.signal.check_signals(tf, restore_blocked) else {
        return false;
    };

    let signo = sig.signo();
    match os_action {
        SignalOSAction::Terminate => {
            do_exit(signo as i32, true);
        }
        SignalOSAction::CoreDump => {
            // TODO: implement core dump
            do_exit(128 + signo as i32, true);
        }
        SignalOSAction::Stop => {
            // TODO: implement stop
            do_exit(1, true);
        }
        SignalOSAction::Continue => {
            // TODO: implement continue
        }
        SignalOSAction::Handler => {
            // do nothing
        }
    }
    true
}

static BLOCK_NEXT_SIGNAL_CHECK: AtomicBool = AtomicBool::new(false);

pub fn block_next_signal() {
    BLOCK_NEXT_SIGNAL_CHECK.store(true, Ordering::SeqCst);
}

pub fn unblock_next_signal() -> bool {
    BLOCK_NEXT_SIGNAL_CHECK.swap(false, Ordering::SeqCst)
}

pub fn with_replacen_blocked<R>(
    blocked: Option<SignalSet>,
    f: impl FnOnce() -> LinuxResult<R>,
) -> LinuxResult<R> {
    let curr = current();
    let old_blocked = blocked.map(|mut set| {
        set.remove(Signo::SIGKILL);
        set.remove(Signo::SIGSTOP);
        curr.as_thread()
            .signal
            .with_blocked_mut(|b| mem::replace(b, set))
    });
    f().inspect(|_| {
        if let Some(old) = old_blocked {
            current().as_thread().signal.with_blocked_mut(|b| *b = old);
        }
    })
}
