// src/ui.rs

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect}, 
    style::{Color, Style, Stylize},                
    text::Line,
    widgets::{Block, Borders, Paragraph}, 
    Frame,                                
};

use crate::model::{Model, RunningState};

/// Renders the entire application UI.
/// This function is called in the main loop and uses helper functions
/// to draw different parts of the UI.
pub fn render_app_ui(model: &Model, frame: &mut Frame) {
    // Create a main layout to split the screen horizontally into two panes
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left pane
            Constraint::Percentage(50), // Right pane
        ])
        .split(frame.area());

    render_left_pane(model, frame, main_layout[0]);
    render_right_pane(model, frame, main_layout[1]);
}

/// Renders the left pane of the UI.
fn render_left_pane(model: &Model, frame: &mut Frame, area: Rect) {
    let title = Line::from("Pomodoro TUI") 
        .bold()
        .fg(Color::Cyan) 
        .centered();

    
    let text_content = vec![
        Line::from("Hello, Ratatui!"),
        Line::from(""), 
        Line::from("This is your Pomodoro Timer."),
        Line::from(""),
        Line::from("Press 'q' or Esc to quit."),
        Line::from("Press 't' for a Test message (see console)."), 
        Line::from(""),
    ];

    let paragraph = Paragraph::new(text_content)
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .centered(); 

    frame.render_widget(paragraph, area);
}

/// Renders the right pane of the UI.
/// 
/// This pane is intented to have the todo list
fn render_right_pane(_model: &Model, frame: &mut Frame, area: Rect) {

    let text_content = "This is the right pane.\n\nIt can display:\n- Timer countdown\n- Cycle information\n- Logs or other details.";

    let paragraph = Paragraph::new(text_content)
        .block(
            Block::default()
                .title("Status / Details") 
                .borders(Borders::ALL),
        )
        .centered(); 

    frame.render_widget(paragraph, area);
}