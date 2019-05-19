use std::io;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;

type TerminalUI = Terminal<TermionBackend<RawTerminal<io::Stdout>>>;

pub fn create_terminal() -> Result<TerminalUI, io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    return Terminal::new(backend);
}

pub fn draw_frame(
    terminal: &mut TerminalUI,
    process_strings: Vec<String>,
    application_strings: Vec<String>,
) -> Result<(), io::Error> {
    let process_text: Vec<Text> = process_strings
        .into_iter()
        .map(|s| Text::raw(s + "\n"))
        .rev()
        .collect();
    let application_text: Vec<Text> = application_strings
        .into_iter()
        .map(|s| Text::raw(s + "\n"))
        .rev()
        .collect();

    let mut process_paragraph = Paragraph::new(process_text.iter())
        .block(Block::default().title("Processes").borders(Borders::ALL))
        .wrap(true);
    let mut application_paragraph = Paragraph::new(application_text.iter())
        .block(Block::default().title("Applications").borders(Borders::ALL))
        .wrap(true);

    return terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(f.size());
        process_paragraph.render(&mut f, chunks[0]);
        application_paragraph.render(&mut f, chunks[1]);
    });
}
