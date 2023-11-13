use std::sync::mpsc;
use std::thread;
use crossterm::event;
use crossterm::event::{KeyEvent, MouseEvent, Event as CrosstermEvent};
use color_eyre::eyre::Result;

/// Terminal Events
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick
    Tick,
    /// Key press
    Key(KeyEvent),
    /// Mouse click/scroll
    Mouse(MouseEvent),
    /// Terminal Resize
    Resize(u16, u16),
}


/// Terminal Events Handler
#[derive(Debug, )]
pub struct EventHandler {
    /// Event sender channel
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    /// Event receiver channel
    receiver: mpsc::Receiver<Event>,
    /// Event Handler thread
    #[allow(dead_code)]
    thread: std::thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of the event handler [`EventHandler`]
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = std::time::Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = std::time::Instant::now();
                loop {
                    // poll for tick rate duration, if no events, sent tick event.
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or_else(|| std::time::Duration::from_secs(0));
                    // poll for events
                    if event::poll(timeout).expect("no events available") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    sender.send(Event::Key(e))
                                } else {
                                    Ok(()) // ignore KeyEventKind::Release on windows
                                }
                            }
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => unimplemented!(),
                        }
                            .expect("failed to send terminal event")
                    }
                    // send tick event
                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = std::time::Instant::now();
                    }
                }
            })
        };
         Self {
             sender,
             receiver,
             thread: handler,
         }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if there is no data available and it's possible for more data to be sent
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}