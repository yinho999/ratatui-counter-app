use color_eyre::eyre::Result;
use counter_app_ratatui::app::App;
use counter_app_ratatui::event::Event;
use counter_app_ratatui::update::update;

fn main()->Result<()>{
    // Create an application
    let mut app = App::new();

    // Initialize the terminal user interface
    let backend = ratatui::backend::CrosstermBackend::new(std::io::stderr());
    let terminal = ratatui::Terminal::new(backend)?;
    let events = counter_app_ratatui::event::EventHandler::new(250);
    let mut tui = counter_app_ratatui::tui::Tui::new(terminal, events);
    tui.enter()?;

    // Start the application
    while !app.should_exit{
        // Render the user interface
        tui.draw(&mut app)?;
        // Handle events
        match tui.events.next()?{
            Event::Tick=>{},
            Event::Key(key_event)=>update(&mut app, key_event),
            Event::Mouse(_mouse_event)=>{},
            Event::Resize(_width, _height)=>{}
        }
    };
    // Reset the terminal state
    tui.exit()?;
    Ok(())
}