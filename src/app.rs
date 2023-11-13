/// Application
#[derive(Debug,Default)]
pub struct App{
    /// Should the application exit
    pub should_exit: bool,
    /// counter
    pub counter: u8
}

impl App{
    /// Create a new application [`App`]
    pub fn new() -> Self{
        Self::default()
    }
    /// Handles the tick event of the terminal
    pub fn on_tick(&mut self){
    }
    /// Increment the counter
    pub fn increment_counter(&mut self){
        self.counter += 1;
    }
    /// Decrement the counter
    pub fn decrement_counter(&mut self){
        self.counter -= 1;
    }
    /// Quit the application
    pub fn quit(&mut self){
        self.should_exit = true;
    }
}
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_app_increment_counter() {
    let mut app = App::default();
    app.increment_counter();
    assert_eq!(app.counter, 1);
  }

  #[test]
  fn test_app_decrement_counter() {
    let mut app = App::default();
    app.decrement_counter();
    assert_eq!(app.counter, 0);
  }
}