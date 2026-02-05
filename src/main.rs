use std::{rc::Rc, cell::RefCell};
use color_eyre::Result;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    text::Line,
    widgets::{Tabs, Widget},
    DefaultTerminal,
};

mod core;
use crate::core::file::{FileBufferTrait};

mod tab;
use crate::tab::*;

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
    tabs: Vec<Rc<RefCell<Tab>>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: AppState::default(),
            selected_tab: 0,
            tabs: vec![Rc::new(RefCell::new(Tab::default()))],
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
                    KeyCode::Right => self.writer_move_right(),
                    KeyCode::Left => self.writer_move_left(),
                    KeyCode::PageDown => self.line_down(),
                    KeyCode::PageUp => self.line_up(),
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn writer_move_right(&mut self) {
        let _ = self.tabs[self.selected_tab].borrow_mut().content.borrow_mut().move_right(1);
    }

    pub fn writer_move_left(&mut self) {
        let _ = self.tabs[self.selected_tab].borrow_mut().content.borrow_mut().move_left(1);
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

    pub fn line_up(&mut self) {
        let _ = self.tabs[self.selected_tab].borrow_mut().content.borrow_mut().previous_line(130u16);
    }

    pub fn line_down(&mut self) {
        let _ = self.tabs[self.selected_tab].borrow_mut().content.borrow_mut().next_line();
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
        self.tabs[self.selected_tab].borrow_mut().clone().render(inner_area, buf);
        render_footer(footer_area, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = self.tabs.iter().map(|tab| tab.borrow().get_title());
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


