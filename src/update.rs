use crossterm::event::{KeyCode, KeyEvent};
use crate::app::App;

pub fn update(app:&mut App, key_event:KeyEvent){
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C')=>{
            if key_event.modifiers == crossterm::event::KeyModifiers::CONTROL{
                app.quit();
            }
        },
        KeyCode::Char('j') | KeyCode::Right=> app.increment_counter(),
        KeyCode::Char('k') | KeyCode::Left=> app.decrement_counter(),
        _=>{}
    }
}