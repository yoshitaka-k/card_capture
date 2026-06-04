use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use color_eyre::Result;
use ratatui::crossterm::event::{
    self,
    Event as CrosstermEvent,
    KeyEvent,
    MouseButton,
    MouseEvent,
    MouseEventKind,
};

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
}

#[derive(Debug)]
pub struct EventHandler {
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,

    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();

            thread::spawn(move || {
                let mut last_tick = Instant::now();

                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    // イベントを取得
                    if event::poll(timeout).expect("unable to poll event") {
                        if let Some(ev) = Self::from_crossterm(event::read().expect("unable to read event")) {
                            Self::dispatch(&sender, ev);
                        }

                        // イベントを継続的に処理
                        while event::poll(Duration::ZERO).expect("unable to poll event") {
                            if let Some(ev) = Self::from_crossterm(event::read().expect("unable to read event")) {
                                Self::dispatch(&sender, ev);
                            }
                        }
                    }

                    // 1 tick 経過したときの処理
                    if last_tick.elapsed() >= tick_rate {
                        Self::dispatch(&sender, Event::Tick);
                        last_tick = Instant::now();
                    }
                }
            })
        };

        Self {
            sender,
            receiver,
            handler,
        }
    }

    fn from_crossterm(event: CrosstermEvent) -> Option<Event> {
        match event {
            CrosstermEvent::Key(e) => Some(Event::Key(e)),
            CrosstermEvent::Mouse(e) => Some(Event::Mouse(e)),
            _ => None,
        }
    }

    fn dispatch(sender: &mpsc::Sender<Event>, event: Event) {
        match event {
            Event::Key(key_event) => {
                if key_event.kind == event::KeyEventKind::Press {
                    sender.send(Event::Key(key_event)).expect("failed to send terminal event");
                }
            }
            Event::Mouse(mouse_event) => {
                if matches!(
                    mouse_event.kind,
                    MouseEventKind::Up(MouseButton::Left) | MouseEventKind::Up(MouseButton::Right)
                ) {
                    sender.send(Event::Mouse(mouse_event)).expect("failed to send terminal event");
                }
            }
            Event::Tick => {
                sender.send(Event::Tick).expect("failed to send terminal event");
            }
        }
    }

    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
