use std::{io::Stdout, path::PathBuf};

use termion::{raw::RawTerminal, screen::AlternateScreen};
use tokio::sync::OnceCell;

pub static mut SCREEN: OnceCell<AlternateScreen<RawTerminal<Stdout>>> = OnceCell::const_new();
pub static ROOT: OnceCell<PathBuf> = OnceCell::const_new();
