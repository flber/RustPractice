use std::io::{self, Write};
use tui::{
    *,
    layout::*,
    backend::TermionBackend,
    widgets::*,
};
use termion::{
    raw::IntoRawMode,
    clear,
};

fn main() -> Result<(), io::Error> {
    let mut stdout = io::stdout().into_raw_mode()?;
    write!(stdout, "{}", clear::All)?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(f.size());
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(30),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(chunks[0]);
        let upper_left_pane =
            Block::default().title("BlockUL").borders(Borders::ALL);
        let lower_left_pane =
            Block::default().title("BlockLL").borders(Borders::ALL);
        let right_pane =
            Block::default().title("BlockR").borders(Borders::ALL);
        f.render_widget(upper_left_pane, left_chunks[0]);
        f.render_widget(lower_left_pane, left_chunks[1]);
        f.render_widget(right_pane, chunks[1]);
    })?;
    print!("\r\n");
    Ok(())
}
