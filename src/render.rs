use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};

pub struct TabState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabState<'a> {
    pub fn goto(&mut self, num: usize) {
        self.index = num;
    }
}

pub fn draw_queue<B>(term: &mut Frame<B>, area: Rect)
where
    B: tui::backend::Backend,
{
    // TODO
}

fn draw_tabs<B>(term: &mut Frame<B>, section: Rect, tabs: &TabState)
where
    B: tui::backend::Backend,
{
    let tab_text = [
        Text::styled(
            tabs.titles[0],
            Style::default().fg(match tabs.index {
                0 => Color::Green,
                _ => Color::White,
            }),
        ),
        Text::raw(" | "),
        Text::styled(
            tabs.titles[1],
            Style::default().fg(match tabs.index {
                1 => Color::Green,
                _ => Color::White,
            }),
        ),
        Text::raw(" | "),
        Text::styled(
            tabs.titles[2],
            Style::default().fg(match tabs.index {
                2 => Color::Green,
                _ => Color::White,
            }),
        ),
        Text::raw(" | "),
        Text::styled(
            tabs.titles[3],
            Style::default().fg(match tabs.index {
                3 => Color::Green,
                _ => Color::White,
            }),
        ),
    ];
    Paragraph::new(tab_text.iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .alignment(Alignment::Center)
        .render(term, section);
}

fn draw_current<B>(term: &mut Frame<B>, section: Rect, current_song: gnaw::Song)
where
    B: tui::backend::Backend,
{
    let status_text = [
        Text::styled(current_song.title, Style::default().fg(Color::LightBlue)),
        Text::raw(" | "),
        Text::styled(current_song.album, Style::default().fg(Color::LightRed)),
        Text::raw(" | "),
        Text::styled(current_song.artist, Style::default().fg(Color::LightYellow)),
    ];
    Paragraph::new(status_text.iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White)),
        )
        .alignment(Alignment::Center)
        .render(term, section);
}

pub fn draw_status<B>(term: &mut Frame<B>, area: Rect, tabs: &TabState, current_song: gnaw::Song)
where
    B: tui::backend::Backend,
{
    let constraints = vec![Constraint::Percentage(50), Constraint::Percentage(50)];
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);
    draw_tabs(term, chunks[0], tabs);
    draw_current(term, chunks[1], current_song);
}
