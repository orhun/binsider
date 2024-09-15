use crate::error::Result;
use crate::TraceData;
use ratatui::crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc,
};
use std::thread;
use std::time::{Duration, Instant};

/// Terminal events.
#[derive(Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
    /// Result of `strings` call.
    FileStrings(Result<Vec<(String, u64)>>),
    /// Trace system calls.
    Trace,
    /// Results of tracer.
    TraceResult(Result<TraceData>),
    /// Restart TUI with a new path.
    Restart(Option<PathBuf>),
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    pub sender: mpsc::Sender<Event>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread.
    handler: thread::JoinHandle<()>,
    /// Is the key input disabled?
    pub key_input_disabled: Arc<AtomicBool>,
    /// Is it listening for events?
    running: Arc<AtomicBool>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let key_input_disabled = Arc::new(AtomicBool::new(false));
        let running = Arc::new(AtomicBool::new(true));
        let handler = {
            let sender = sender.clone();
            let key_input_disabled = key_input_disabled.clone();
            let running = running.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                while running.load(Ordering::Relaxed) {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);
                    if key_input_disabled.load(Ordering::Relaxed) {
                        thread::sleep(timeout);
                        continue;
                    } else if event::poll(timeout).expect("no events available") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                            CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                            _ => unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
            key_input_disabled,
            running,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }

    /// Stops the event listener.
    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
