use color_eyre::Result;
use crate::{helpers::DefaultTerminal, ui::render_app_ui}; // Using the type alias

use ratatui::{
    Frame,
    layout::{Layout, Constraint, Direction, Rect}, // Added Rect
    style::Stylize, // For styling text
    text::Line,     // For creating lines of text
    widgets::{Block, Borders, Paragraph},
};
// Import functions from our other modules
//use crate::ui::render_app_ui;


/// The main application which holds the state and logic of the application. It is the model from ELM architecture
#[derive(Debug, Default)]
pub struct Model{
    /// Is the application running?
    running: bool,
}

#[derive(Debug, Default, PartialEq, Eq)]

/// Possible running states for the application. 
/// Running is for pomodoro's running
/// Done is when pomodoro timer is over
/// Pause is for user's pausing the timer
/// Exit is for taking actions for exiting the system.
pub enum RunningState{
    #[default]
    Running,
    Done,
    Pause,
    Exit
}


#[derive(PartialEq)]
/// Message struct for the ELM architecture. Updates are actions triggered by events (ex; user inputs)
/// and those events must be mapped to a message. 
/// 
/// This enum keep track of messages the determines the model's next state
enum Message{
    Test,
    Tmp,
    Quit,
}

impl Model{

    /// Creates a new instance of the app model.
    pub fn new() -> Self{
            Self::default()
    }


/// Run the application's main loop.
    pub fn run(mut self, terminal:&mut DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| render_app_ui(&self, frame))?;
        

            //FIXME: for now it will wait 5 seconds before leaving
            /// the next step is to implement a quit function 
            std::thread::sleep(std::time::Duration::from_secs(5));

            self.running = false;
        }
        Ok(())
    }

     /// Updates the application state based on a received message.
   // fn update_state(&mut self, message: Message) {
    //     match message {
    //         Message::Quit => self.quit(),
    //         // TODO: Handle other messages to change `current_run_state` or other fields
    //     }
    // }

    
    /// Sets the `running` flag to false to exit the application loop.
    fn quit(&mut self) {
        self.running = false;
    }
}