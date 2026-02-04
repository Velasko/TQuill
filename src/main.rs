//! # [Ratatui] Tabs example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use std::{rc::Rc, cell::RefCell};
use color_eyre::Result;
use ratatui::{
    prelude::Text,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    symbols,
    text::Line,
    widgets::{Block, Padding, Paragraph, Tabs, Widget, Wrap},
    DefaultTerminal,
};

mod core;
use crate::core::file::{FileBuffer, FileBufferTrait};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    state: AppState,
    selected_tab: usize,
    tabs: Vec<Tab>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::default(),
            selected_tab: 0,
            tabs: vec![Tab::default()],
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

impl App {
    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                    KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                    KeyCode::Down => {let _ = self.tabs[self.selected_tab].content.borrow_mut().next_line();},
                    KeyCode::Up => {let _ = self.tabs[self.selected_tab].content.borrow_mut().previous_line(130u16);},
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.saturating_add(1) % self.tabs.len();
    }

    pub fn previous_tab(&mut self) {
        let sub = self.selected_tab.overflowing_sub(1);
        self.selected_tab = if sub.1 {
            self.tabs.len() - 1
        } else {
            sub.0
        };
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};
        let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        let horizontal = Layout::horizontal([Min(0), Length(20)]);
        let [tabs_area, title_area] = horizontal.areas(header_area);

        render_title(title_area, buf);
        self.render_tabs(tabs_area, buf);
        self.tabs[self.selected_tab].clone().render(inner_area, buf);
        render_footer(footer_area, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = self.tabs.iter().map(|tab| tab.get_title());
        let highlight_style = (Color::default(), tailwind::BLUE.c700);
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(self.selected_tab)
            .padding("", "")
            .divider(" ")
            .render(area, buf);
    }
}

fn render_title(area: Rect, buf: &mut Buffer) {
    let version = format!("v{}.{}", env!("CARGO_PKG_VERSION_MAJOR"), env!("CARGO_PKG_VERSION_MINOR"));
    format!("{} {}", env!("CARGO_PKG_NAME"), version).bold().render(area, buf);
}

fn render_footer(area: Rect, buf: &mut Buffer) {
    Line::raw("◄ ► to change tab | Press q to quit")
        .centered()
        .render(area, buf);
}

impl Widget for Tab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_tab(area, buf)
    }
}

#[derive(Clone)]
struct Tab {
    content: Rc<RefCell<FileBuffer>>
}

impl Tab {
    fn get_title(&self) -> String {
        self.content.borrow().get_filename().to_string()
    }

    fn render_tab(self, area: Rect, buf: &mut Buffer) {
        let content = self.content.borrow_mut().read_lines(area.height);
        Paragraph::new(Text::from_iter(content))
            .wrap(Wrap { trim: false })
            .block(
                Block::bordered()
                    .border_set(symbols::border::PROPORTIONAL_TALL)
                    .padding(Padding::horizontal(1))
                    .border_style(tailwind::BLUE.c700)
            )
            .render(area, buf);
    }
}

impl Default for Tab {
    fn default() -> Self {
        let filename = "Cargo.lock";
        Self {
            content: Rc::new(RefCell::new(FileBuffer::open(filename).expect("testing"))),
        }
    }
}
