use std::sync::mpsc;
use std::time::Duration;
use std::{io, thread};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Tabs, Widget};
use tui::Terminal;

struct TabState<'a> {
    titles: Vec<&'a str>,
    index: usize,
}

impl<'a> TabState<'a> {
    fn goto(&mut self, num: usize) {
        self.index = num;
    }
}

fn main() -> Result<(), io::Error> {
    if let Ok(mpd) = gnaw::Mpd::new("127.0.0.1:6600".parse().unwrap()) {
        let mut tabs = TabState {
            titles: vec!["Currently Playing", "Queue", "Albums", "Artists"],
            index: 0,
        };
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor().unwrap();
        terminal.clear()?;
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let stdin = std::io::stdin();
            for c in stdin.lock().events() {
                let evt = c.unwrap();
                tx.send(evt).unwrap();
            }
        });
        loop {
            terminal
                .draw(|mut term| {
                    let chunks = Layout::default()
                        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                        .split(term.size());
                    Tabs::default()
                        .block(Block::default().borders(Borders::ALL))
                        .titles(&tabs.titles)
                        .select(tabs.index)
                        .style(Style::default().fg(Color::White))
                        .highlight_style(Style::default().fg(Color::Green))
                        .divider(tui::symbols::line::VERTICAL)
                        .render(&mut term, chunks[1]);
                })
                .unwrap();
            match rx.recv_timeout(Duration::from_millis(1000)) {
                Ok(Event::Key(Key::Char('1'))) => tabs.goto(0),
                Ok(Event::Key(Key::Char('2'))) => tabs.goto(1),
                Ok(Event::Key(Key::Char('3'))) => tabs.goto(2),
                Ok(Event::Key(Key::Char('4'))) => tabs.goto(3),
                Ok(Event::Key(Key::Char('q'))) => break,
                Ok(_) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => panic!("isolated from input thread"),
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
        }
        terminal.show_cursor().unwrap();
        terminal.clear()?;
        mpd.connection.shutdown(std::net::Shutdown::Both).unwrap(); // TODO: Move to library
        return Ok(());
    } else {
        panic!("failed to connect to mpd")
    }
}
