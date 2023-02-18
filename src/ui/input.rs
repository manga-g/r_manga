use std::io::{self, Write};
use tui::layout::Alignment;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text};
use tui::Terminal;

pub fn input(term: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<String> {
    let mut buffer = String::new();

    term.draw(|f| {
        let prompt = Paragraph::new(Text::styled(
            "Enter a manga name to search:",
            Style::default().fg(Color::Green),
        ))
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::ALL));

        let input = Paragraph::new(Text::raw(&buffer))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(prompt, f.size());
        f.render_widget(input, f.size());
    })?;

    loop {
        if let Ok(event) = crossterm::event::read() {
            match event {
                crossterm::event::Event::Key(key_event) => match key_event {
                    crossterm::event::KeyEvent {
                        code: crossterm::event::KeyCode::Char(ch),
                        ..
                    } => {
                        buffer.push(ch);
                        term.draw(|f| {
                            let input = Paragraph::new(Text::raw(&buffer))
                                .style(Style::default().fg(Color::White))
                                .block(Block::default().borders(Borders::ALL));
                            f.render_widget(input, f.size());
                        })?;
                    }
                    crossterm::event::KeyEvent {
                        code: crossterm::event::KeyCode::Backspace,
                        ..
                    } => {
                        buffer.pop();
                        term.draw(|f| {
                            let input = Paragraph::new(Text::raw(&buffer))
                                .style(Style::default().fg(Color::White))
                                .block(Block::default().borders(Borders::ALL));
                            f.render_widget(input, f.size());
                        })?;
                    }
                    crossterm::event::KeyEvent {
                        code: crossterm::event::KeyCode::Enter,
                        ..
                    } => {
                        return Ok(buffer.trim().to_owned());
                    }
                    _ => {}
                },
                crossterm::event::Event::Resize(..) => {
                    term.clear()?;
                }
                _ => {}
            }
        }
    }
}

