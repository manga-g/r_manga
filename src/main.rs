use std::io;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod api;
mod ui;

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let query = ui::input(&mut term)?;
    let manga_list = api::search_manga(&query.trim())?;

    ui::output(&mut term, &manga_list)?;

    Ok(())
}
