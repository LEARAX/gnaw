use std::io;
use termion::event::{Event, Key, MouseEvent};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, Tabs, Widget};
use tui::Terminal;

struct TabState<'a> {
    titles: Vec<&'a str>,
    index: usize
}

impl<'a> TabState<'a> {
    fn goto(&mut self, num: usize) {
        self.index = num;
    }
}

fn main() -> Result<(), io::Error> {
    if let Ok(mpd) = gnaw::Mpd::new("127.0.0.1:6600".parse().unwrap()) {
        let mut tabs = TabState {
            titles: vec!("Currently Playing", "Queue", "Albums", "Artists"),
            index: 0
        };
        println!("{:#?}", mpd);
        let stdin = io::stdin();
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.hide_cursor().unwrap();
        terminal.clear()?;
        for c in stdin.events() {
                let evt = c.unwrap();
                match evt {
                    Event::Key(Key::Char('1')) => tabs.goto(0),
                    Event::Key(Key::Char('2')) => tabs.goto(1),
                    Event::Key(Key::Char('3')) => tabs.goto(2),
                    Event::Key(Key::Char('4')) => tabs.goto(3),
                    Event::Key(Key::Char('q')) => break,
                    Event::Mouse(me) => {
                        match me {
                            /*
                             * MouseEvent::Press(_, x, y) => {
                             *     write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
                             * },
                             */
                            _ => (),
                        }
                    }
                    _ => {}
                }
            terminal
                .draw(|mut term| {
                    let size = term.size();
                    Tabs::default()
                        .block(Block::default().title("Tabs").borders(Borders::ALL))
                        .titles(&tabs.titles)
                        .select(tabs.index)
                        .style(Style::default().fg(Color::White))
                        .highlight_style(Style::default().fg(Color::Yellow))
                        .divider(tui::symbols::DOT)
                        .render(&mut term, size);
                })
                .unwrap();
        }
        terminal.show_cursor().unwrap();
        terminal.clear()?;
        mpd.connection.shutdown(std::net::Shutdown::Both).unwrap(); // TODO: Move to library
        return Ok(());
    } else {
        panic!("Failed to connect to mpd!")
    }
}
