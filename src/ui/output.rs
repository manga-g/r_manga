use std::io::{self, Stdout};
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, Paragraph, Text};
use tui::Terminal;

use crate::api::Manga;

pub fn output(
    term: &mut Terminal<CrosstermBackend<Stdout>>,
    manga_list: &[Manga],
) -> io::Result<()> {
    term.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(90),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(f.size());

        let title = Text::styled("Rust Manga Reader", Style::default().fg(Color::Cyan));
        let paragraph = Paragraph::new(title)
            .block(
                Block::default()
                    .title("Title")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Magenta)),
            )
            .alignment(tui::layout::Alignment::Center);

        f.render_widget(paragraph, chunks[0]);

        let items = manga_list
            .iter()
            .map(|manga| ListItem::new(manga.title.clone()))
            .collect::<Vec<_>>();
        let items = List::new(items)
            .block(Block::default().title("List").borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Magenta),
            );
        f.render_widget(items, chunks[1]);

        let help_text = Paragraph::new("Press 'q' to exit")
            .style(Style::default().fg(Color::Yellow))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Help")
                    .border_style(Style::default().fg(Color::Green)),
            )
            .alignment(tui::layout::Alignment::Center);

        f.render_widget(help_text, chunks[2]);
    })?;

    Ok(())
}

