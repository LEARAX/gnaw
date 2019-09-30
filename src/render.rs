use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::Modifier;
use tui::style::{Color, Style};
use tui::terminal::Frame;
use tui::widgets::{Block, Borders, Gauge, Paragraph, SelectableList, Text, Widget};

pub struct TabState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabState<'a> {
    pub fn goto(&mut self, num: usize) {
        self.index = num;
    }
}

pub fn draw_duration<B>(term: &mut Frame<B>, area: Rect, duration: Option<f64>, elapsed: Option<f64>)
where
    B: tui::backend::Backend,
{
    if let Some(elapsed) = elapsed {
        Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default()),
            )
            .style(Style::default().fg(Color::Green))
            .label("")
            .ratio(elapsed / duration.expect("failed to extract duration of current song, but elapsed time exists"))
            .render(term, area);
    } else {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default())
            .render(term, area);
    }
}

pub fn draw_queue<B>(term: &mut Frame<B>, area: Rect, queue: Option<&gnaw::Queue>)
where
    B: tui::backend::Backend,
{
    let mut queue_titles = Vec::new();
    if let Some(queue) = queue {
        for song in queue {
            queue_titles.push(song.title.as_ref().unwrap());
        }
    }
    SelectableList::default()
        .block(Block::default().borders(Borders::ALL))
        .items(&queue_titles)
        .select(Some(1))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().modifier(Modifier::ITALIC))
        .highlight_symbol(">>")
        .render(term, area);
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

fn draw_current<B>(term: &mut Frame<B>, section: Rect, current_song: Option<&gnaw::Song>)
    where
    B: tui::backend::Backend,
{
    let status_text = if let Some(song) = current_song {
        let title = match song.title {
            Some(_) => &song.title.as_ref().unwrap(),
            None => "",
        };
        let album = match song.album {
            Some(_) => &song.album.as_ref().unwrap(),
            None => "",
        };
        let artist = match song.artist {
            Some(_) => &song.artist.as_ref().unwrap(),
            None => "",
        };
        vec![
            Text::styled(title, Style::default().fg(Color::LightBlue)),
            Text::raw(" | "),
            Text::styled(album, Style::default().fg(Color::LightRed)),
            Text::raw(" | "),
            Text::styled(artist, Style::default().fg(Color::LightYellow)),
        ]
    } else {
        vec![Text::raw("")]
    };
    Paragraph::new(status_text.iter())
        .block(
            Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White)),
            )
        .alignment(Alignment::Center)
        .render(term, section);
}

pub fn draw_status<B>(
    term: &mut Frame<B>,
    area: Rect,
    tabs: &TabState,
    current_song: Option<&gnaw::Song>,
    ) where
B: tui::backend::Backend,
{
    let constraints = vec![Constraint::Length(36), Constraint::Percentage(50)];
    let chunks = Layout::default()
        .constraints(constraints)
        .direction(Direction::Horizontal)
        .split(area);
    draw_tabs(term, chunks[0], tabs);
    draw_current(term, chunks[1], current_song);
}
