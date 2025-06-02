// src/main.rs

mod model;
mod ui;
mod helpers;
use color_eyre::Result;
use model::Model;


fn main() -> Result<()> {
    // Initialize error handling with color-eyre
    color_eyre::install()?;

    // Initialize the terminal using helper functions created by Google Gemini Pro
    let mut terminal = helpers::init_terminal()?;

    let app_model = Model::new();
    let app_result = app_model.run(&mut terminal); 

    // Restore the terminal before exiting
    helpers::restore_terminal(&mut terminal)?;

    app_result 
}