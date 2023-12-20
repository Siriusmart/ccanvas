use nix::sys::signal::{self, SigHandler, Signal};

use crate::values::SCREEN;
use std::io::Write;

pub fn exit() {
    write!(
        unsafe { SCREEN.get_mut().unwrap() },
        "{}{}",
        termion::cursor::Show,
        termion::screen::ToMainScreen,
    )
    .unwrap();

    unsafe {
        signal::sigaction(
            Signal::SIGWINCH,
            &signal::SigAction::new(
                SigHandler::SigDfl,
                signal::SaFlags::empty(),
                signal::SigSet::empty(),
            ),
        )
        .unwrap();
    }
}
