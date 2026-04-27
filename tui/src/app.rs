use std::sync::Arc;

use crate::{
    event::{AppEvent, Event, EventHandler},
    handler,
    state::DaemonStates,
};
use ami_daemon::commands::Command;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::DefaultTerminal;
use ratatui_image::picker::Picker;
use tokio::sync::{Mutex, mpsc::UnboundedSender};

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    /// Event handler.
    pub events: EventHandler,

    pub daemon_states: Arc<Mutex<DaemonStates>>,

    pub command_tx: UnboundedSender<Command>,

    pub image_picker: Arc<Picker>,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(states: Arc<Mutex<DaemonStates>>, command_tx: UnboundedSender<Command>) -> Self {
        Self {
            running: true,
            events: EventHandler::new(),
            daemon_states: states,
            command_tx,
            image_picker: Arc::new(
                Picker::from_query_stdio().unwrap_or_else(|_| Picker::halfblocks()),
            ),
        }
    }

    /// Run the application's main loop.
    pub async fn run(mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => {
                    self.tick();
                    terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
                }
                Event::Crossterm(event) => match event {
                    crossterm::event::Event::Key(key_event)
                        if key_event.kind == crossterm::event::KeyEventKind::Press =>
                    {
                        self.handle_key_events(key_event)?
                    }
                    _ => {}
                },
                Event::App(app_event) => match app_event {
                    AppEvent::CursorDown => {
                        handler::library::select_next_track(self.daemon_states.clone()).await
                    }
                    AppEvent::CursorUp => {
                        handler::library::select_prev_track(self.daemon_states.clone()).await
                    }
                    AppEvent::Enqueue => {
                        handler::queue::enqueue(
                            self.command_tx.clone(),
                            self.daemon_states.clone(),
                        )
                        .await;
                    }
                    AppEvent::Next => {
                        handler::queue::next(self.command_tx.clone());
                    }
                    AppEvent::Prev => {
                        handler::queue::prev(self.command_tx.clone());
                    }
                    AppEvent::TogglePlay => {
                        handler::playback::toggle_play(self.command_tx.clone());
                    }
                    AppEvent::SeekForward => {
                        handler::playback::seek_forward(self.command_tx.clone());
                    }
                    AppEvent::SeekBackward => {
                        handler::playback::seek_backward(self.command_tx.clone());
                    }

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
            KeyCode::Up => self.events.send(AppEvent::CursorUp),
            KeyCode::Down => self.events.send(AppEvent::CursorDown),
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
}
