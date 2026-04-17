use std::sync::Arc;

use crate::{
    event::{AppEvent, Event, EventHandler},
    state::AppStates,
};
use ami_daemon::commands::Command;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use tokio::sync::{Mutex, mpsc::UnboundedSender};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,

    pub states: Arc<Mutex<AppStates>>,

    pub command_tx: UnboundedSender<Command>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(states: Arc<Mutex<AppStates>>, command_tx: UnboundedSender<Command>) -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            states,
            command_tx,
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_events(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            KeyCode::Enter => self.events.send(AppEvent::Enqueue),
            KeyCode::Char(' ') => self.events.send(AppEvent::TogglePlay),
            KeyCode::Char('s') => self.events.send(AppEvent::Next),
            KeyCode::Char('p') => self.events.send(AppEvent::Prev),
            KeyCode::Right => self.events.send(AppEvent::SeekForward),
            KeyCode::Left => self.events.send(AppEvent::SeekBackward),
            // Other handlers you could add here.
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn increment_counter(&mut self) {
        self.counter = self.counter.saturating_add(1);
    }

    pub fn decrement_counter(&mut self) {
        self.counter = self.counter.saturating_sub(1);
    }
}
