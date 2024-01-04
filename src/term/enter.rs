use std::{
    fs,
    io::{stdout, Write},
    path::PathBuf,
    process,
};

use termion::{input::MouseTerminal, raw::IntoRawMode, screen::IntoAlternateScreen};

use crate::{
    structs::Storage,
    values::{ROOT, SCREEN},
};

/// run when entering
pub async fn enter() {
    let root = PathBuf::from("/tmp")
        .join("ccanvas")
        .join(process::id().to_string());

    Storage::remove_if_exist(&root).await.unwrap();

    fs::create_dir_all(&root).unwrap();
    ROOT.set(root).unwrap();

    let mut screen = MouseTerminal::from(
        stdout()
            .into_raw_mode()
            .unwrap()
            .into_alternate_screen()
            .unwrap(),
    );
    write!(screen, "{}", termion::clear::All).unwrap();
    screen.flush().unwrap();
    let _ = unsafe { SCREEN.set(screen) };

    #[cfg(feature = "log")]
    {
        let log_file = dirs::data_dir().unwrap().join("ccanvas.log");
        simplelog::WriteLogger::init(
            log::LevelFilter::Trace,
            simplelog::ConfigBuilder::new()
                .set_max_level(log::LevelFilter::Trace)
                .set_location_level(log::LevelFilter::Trace)
                .build(),
            std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(log_file)
                .unwrap(),
        )
        .unwrap();
    }
}
