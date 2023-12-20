use std::io::stdin;

use nix::sys::signal::{self, SigHandler, Signal};
use termion::input::TermReadEventsAndRaw;
use tokio::sync::{
        broadcast::{self, Receiver, Sender},
        OnceCell,
};

use super::Event;

/// a copy of the broadcast sender
///
/// make a clone of it and you can start broadcasting events
/// or you can subscribe to it and get a reciever to the broadcast
static EVENTS: OnceCell<Sender<Event>> = OnceCell::const_new();

impl Event {
    /// kick start the event broadcaster
    /// should only be called once for the entire duration of the program
    pub fn start() {
        if EVENTS.get().is_some() {
            panic!("events broadcast has already been started");
        }

        let (tx, _rx): (Sender<Event>, Receiver<Event>) = broadcast::channel(100);
        {
            let tx = tx.clone();
            tokio::task::spawn_blocking(move || {
                stdin()
                    .events_and_raw()
                    .into_iter()
                    .filter_map(|event| -> Option<Event> {
                        let event = event;
                        if let Ok(event) = event {
                            if let Ok(event) = event.0.try_into() {
                                return Some(event);
                            }
                        }
                        return None;
                    })
                    .for_each(|event| {
                        let _ = tx.send(event);
                    })
            });
        }

        extern "C" fn handle_resize(_: libc::c_int) {
            let (y, x) = termion::terminal_size().unwrap();
            let _ = EVENTS.get().unwrap().send(Event::ScreenResize(x as u32, y as u32));
        }

        // listen for SIGWINCH, as it is the only way to listen for window resize event
        // without pulling in huge dependencies
        let sig_action = signal::SigAction::new(
            SigHandler::Handler(handle_resize),
            signal::SaFlags::empty(),
            signal::SigSet::empty(),
        );
        unsafe {
            signal::sigaction(Signal::SIGWINCH, &sig_action).unwrap();
        }

        EVENTS.set(tx).unwrap();
    }

    pub fn listen() -> Receiver<Event> {
        EVENTS.get().unwrap().subscribe()
    }
}
