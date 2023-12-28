use std::io::stdin;

use nix::sys::signal::{self, SigHandler, Signal};
use termion::input::TermRead;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    OnceCell,
};

use super::Event;

/// a copy of the broadcast sender
///
/// make a clone of it and you can start broadcasting events
/// or you can subscribe to it and get a reciever to the broadcast
static EVENTS: OnceCell<UnboundedSender<Event>> = OnceCell::const_new();

impl Event {
    /// kick start the event broadcaster
    /// should only be called once for the entire duration of the program
    pub fn start() -> UnboundedReceiver<Event> {
        // can only be started once
        if EVENTS.get().is_some() {
            panic!("events broadcast has already been started");
        }

        let (tx, rx): (UnboundedSender<Event>, UnboundedReceiver<Event>) =
            mpsc::unbounded_channel();
        {
            let tx = tx.clone();
            tokio::task::spawn_blocking(move || {
                stdin()
                    .events()
                    // filter out events that cannot be converted into event
                    .filter_map(|event| -> Option<Event> {
                        let event = event;
                        if let Ok(event) = event {
                            if let Ok(event) = event.try_into() {
                                return Some(event);
                            }
                        }
                        None
                    })
                    .for_each(|event| {
                        // send events to master space
                        let _ = tx.send(event);
                    })
            });
        }

        extern "C" fn handle_resize(_: libc::c_int) {
            // send a screen resize event when it is resized
            let (x, y) = termion::terminal_size().unwrap();
            let _ = EVENTS
                .get()
                .unwrap()
                .send(Event::ScreenResize(x as u32, y as u32));
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

        // also let other codes send events
        EVENTS.set(tx).unwrap();

        rx
    }

    /// send an event to the main event stream
    pub fn send(event: Event) {
        EVENTS.get().unwrap().send(event).unwrap();
    }
}
