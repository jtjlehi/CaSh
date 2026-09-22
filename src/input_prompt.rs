use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget, WidgetRef},
};

/// The prompt box
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct InputPrompt {
    /// The current value of the string
    input: String,
    /// The current position of the cursor in the input
    character_index: usize,
    input_mode: InputMode,
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum InputMode {
    #[default]
    Normal,
    Editing,
}

impl WidgetRef for InputPrompt {
    fn render_ref(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::new()
            .borders(Borders::TOP)
            .title(Line::from(" Input Prompt ".bold()))
            .border_set(border::THICK);
        Paragraph::new("input text")
            .left_aligned()
            .block(block)
            .render(area, buf)
    }
}
