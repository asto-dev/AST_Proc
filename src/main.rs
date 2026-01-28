mod process;
mod read;
mod app;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let terminal = ratatui::init();
    app::App::default().run(terminal)?;
    ratatui::restore();
    Ok(())
}

