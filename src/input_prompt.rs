use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

/// The mode of the prompt (should be nested in a `crate::Mode` variant)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum PromptMode {
    /// Currently entering a shell command
    Shell,
    /// Currently entering a tui command
    Command,
    /// Currently searching
    Search,
}

/// The state (string and character position) of a prompt string
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct PromptString {
    input: String,
    character_index: usize,
}

impl PromptString {
    pub fn render(&self, mode: PromptMode, area: Rect, frame: &mut Frame) {
        let prefix = match mode {
            PromptMode::Shell => "!",
            PromptMode::Command => ":",
            PromptMode::Search => "/",
        };
        let text = Line::from(vec![prefix.bold(), " ".into(), self.input.as_str().into()]);

        let block = Block::new()
            .borders(Borders::TOP)
            .title(Line::from(" Input Prompt ".bold()))
            .border_set(border::THICK);

        frame.render_widget(Paragraph::new(text).left_aligned().block(block), area);
    }
}
