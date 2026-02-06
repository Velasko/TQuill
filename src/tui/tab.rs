use std::{rc::Rc, cell::{Ref, RefCell}};
use ratatui::{
    prelude::Text,
    buffer::Buffer,
    layout::Rect,
    style::palette::tailwind,
    symbols,
    widgets::{Block, Padding, Paragraph, Widget, Wrap},
};

use crate::core::file::{FileBuffer, FileBufferTrait};


impl Widget for Tab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.render_tab(area, buf)
    }
}

#[derive(Clone)]
pub struct Tab {
    content: Rc<RefCell<FileBuffer>>
}

impl Tab {
    pub fn new(filename: &str) -> Self {
        Self {
            content: Rc::new(RefCell::new(FileBuffer::open(filename).expect("testing"))),
        }
    }

    pub fn get_content(&self) -> Ref<'_, FileBuffer> {
        self.content.borrow()
    }

    pub fn get_title(&self) -> String {
        self.get_content().get_filename().to_string()
    }

    pub fn render_tab(self, area: Rect, buf: &mut Buffer) {
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

    pub fn writer_move_right(&mut self, ammount: usize) {
        self.content.borrow_mut().move_right(ammount)
    }

    pub fn writer_move_left(&mut self, ammount: usize) {
        self.content.borrow_mut().move_left(ammount)
    }

    pub fn previous_line<S>(&mut self, max_line_size: S) where S: Into<usize> {
        self.content.borrow_mut().previous_line(max_line_size)
    }

    pub fn next_line(&mut self) -> String {
        self.content.borrow_mut().next_line()
    }

}
