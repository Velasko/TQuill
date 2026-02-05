use std::{rc::Rc, cell::RefCell};
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
    pub content: Rc<RefCell<FileBuffer>>
}

impl Tab {
    pub fn get_title(&self) -> String {
        self.content.borrow().get_filename().to_string()
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
}

impl Default for Tab {
    fn default() -> Self {
        let filename = "Cargo.lock";
        Self {
            content: Rc::new(RefCell::new(FileBuffer::open(filename).expect("testing"))),
        }
    }
}
