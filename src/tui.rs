use std::panic;
use ratatui::Terminal;
use crate::event::EventHandler;
use color_eyre::eyre::Result;
use crossterm::event::EnableMouseCapture;
use crate::app::App;
use crate::ui;

pub type CrosstermTerminal = Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

/// Representation of a terminal user interface
///
/// It is responsible for setting up the terminal, initializing the application and handling the drawing of the user interface
pub struct Tui{
    /// Interface to the Terminal
    pub terminal: CrosstermTerminal,
    /// Terminal events handler
    pub events: EventHandler
}

impl Tui{
    /// Constructs a new instance of the terminal user interface [`Tui`]
    pub fn new (terminal: CrosstermTerminal, events: EventHandler) -> Self{
        Self{
            terminal,
            events
        }
    }

    /// Initialize the terminal user interface
    ///
    /// It enables raw mode, and sets terminal properties
    pub fn enter(&mut self) -> Result<()>{
        // Enable raw mode
        crossterm::terminal::enable_raw_mode()?;
        // To alternate screen and enable mouse capture
        crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen, EnableMouseCapture)?;

        // Define a custom panic hook to reset the terminal state in case of panic
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic   | {
            Self::reset().expect("Failed to reset terminal state");
            panic_hook(panic);
        }));

        // Hide the cursor
        self.terminal.hide_cursor()?;
        // Clear the terminal
        self.terminal.clear()?;
        Ok(())
    }
    /// Reset the terminal user interface
    ///
    /// This function is also used for the panic hook to reset the terminal state in case of panic
    fn reset() -> Result<()>{
        // Disable raw mode
        crossterm::terminal::disable_raw_mode()?;
        // To main screen and disable mouse capture
        crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen, crossterm::event::DisableMouseCapture)?;
        Ok(())
    }

    /// Exit the terminal user interface
    ///
    /// It disables the raw mode and reverts back the terminal properties
    pub fn exit(&mut self) -> Result<()>{
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    /// [`Draw`] the terminal user interface by [`rendering`] the widgets
    ///
    /// [`Draw`]: crate::tui::Tui::draw
    /// [`rendering`]: crate::tui::Tui::render
    pub fn draw(&mut self, app: &mut App) -> Result<()>{
        self.terminal.draw(|frame|ui::render(app,frame))?;
        Ok(())
    }
}