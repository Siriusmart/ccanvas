use std::io::{stdout, Write};

use termion::{raw::IntoRawMode, screen::IntoAlternateScreen};

use crate::{structs::Event, values::SCREEN};

/// run when entering
pub fn enter() {
    let mut screen = stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    write!(screen, "{}", termion::clear::All).unwrap();
    screen.flush().unwrap();
    let _ = unsafe { SCREEN.set(screen) };
    Event::start();
}
