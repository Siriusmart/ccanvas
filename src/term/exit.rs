use nix::sys::signal::{self, SigHandler, Signal};

use crate::values::SCREEN;
use std::io::Write;

/// run when exiting
pub fn exit() {
    write!(
        unsafe { SCREEN.get_mut().unwrap() },
        "{}{}",
        termion::cursor::Show,
        termion::screen::ToMainScreen,
    )
    .unwrap();

    // changes the sig handler back to default
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
