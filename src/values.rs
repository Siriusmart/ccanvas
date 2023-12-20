use std::io::Stdout;

use termion::{raw::RawTerminal, screen::AlternateScreen};
use tokio::sync::OnceCell;

static mut DISCRIM: OnceCell<u32> = OnceCell::const_new_with(0);
pub fn discrim() -> u32 {
    let discrim = unsafe { DISCRIM.get_mut().unwrap() };
    *discrim += 1;
    *discrim
}

pub static mut SCREEN: OnceCell<AlternateScreen<RawTerminal<Stdout>>> = OnceCell::const_new();
