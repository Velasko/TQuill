use std::{rc::Rc, cell::RefCell};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{palette::tailwind, Color, Stylize},
    text::Line,
    widgets::{Tabs, Widget},
    DefaultTerminal,
};

use crate::tui::tab::*;

type RefTab = Rc<RefCell<Tab>>;

pub struct App {
    state: AppState,
    selected_tab: Option<RefTab>,
    tabs: Vec<RefTab>,
}

impl Default for App {
    fn default() -> Self {
        // todo! open with empty file.
        let tab = Rc::new(RefCell::new(Tab::default()));
        Self {
            state: AppState::default(),
            selected_tab: Some(Rc::clone(&tab)),
            tabs: vec![Rc::clone(&tab)],
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
    pub fn run(mut self, mut terminal: DefaultTerminal) -> std::io::Result<()> {
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

    fn with_selected_tab<S>(&self, func: impl Fn(&RefTab) -> S) -> Option<S> {
        self.selected_tab.as_ref().map(|tab| func(tab))
    }

    pub fn writer_move_right(&self) {
        self.with_selected_tab(|tab| tab.borrow_mut().writer_move_right(1));
    }

    pub fn writer_move_left(&self) {
        self.with_selected_tab(|tab| tab.borrow_mut().writer_move_left(1));
    }

    pub fn get_tab_index(&self, tab: &RefTab) -> Option<usize> {
        self.tabs.iter().position(|t| Rc::ptr_eq(t, tab))
    }

    pub fn get_selected_tab_index(&self) -> Option<usize> {
        self.with_selected_tab(|sel| self.get_tab_index(sel).expect("Selected tab must be in list"))
    }

    pub fn next_tab(&mut self) {
        let first_tab = self.tabs.first().map(|tab| Rc::clone(tab));
        self.selected_tab = match self.get_selected_tab_index() {
            Some(index) => match self.tabs.get(index+1) {
                Some(tab) => Some(Rc::clone(tab)),
                None => first_tab,
            },
            None => first_tab,
        }
    }

    pub fn previous_tab(&mut self) {
        let last_tab = self.tabs.last().map(|tab| Rc::clone(tab));
        self.selected_tab = match self.get_selected_tab_index() {
            Some(index) => match self.tabs.get(index+1) {
                Some(tab) => Some(Rc::clone(tab)),
                None => last_tab,
            },
            None => last_tab,
        }
    }

    pub fn line_up(&mut self) {
        // todo! parse internal tab width
        let _ = self.with_selected_tab(|tab| tab.borrow_mut().previous_line(130u16));
    }

    pub fn line_down(&mut self) {
        let _ = self.with_selected_tab(|tab| tab.borrow_mut().next_line());
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
        self.selected_tab.as_ref().map(|t| t.borrow_mut().clone().render(inner_area, buf));
        render_footer(footer_area, buf);
    }
}

impl App {
    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = self.tabs.iter().map(|tab| tab.borrow().get_title());
        let highlight_style = (Color::default(), tailwind::BLUE.c700);
        let selected_tab_index = self.get_selected_tab_index();
        Tabs::new(titles)
            .highlight_style(highlight_style)
            .select(selected_tab_index)
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

