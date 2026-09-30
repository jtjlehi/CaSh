//! The [`BodyContent`] widget and display logic
//!
//! Displaying and updating [`BodyContent`] are controlled through:
//! - [`render`]: How to display the widget
//! - [`update`]: How to update the widget based on the passed [`Message`]s
//!
//! [`render`]: BodyContent::render
//! [`update`]: BodyContent::update

use std::sync::Arc;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

/// The state/widget for the body of the tui
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct BodyContent {
    /// Shared pointer to the contents to display
    pub content: Arc<str>,
    /// The title of the content
    pub title: String,
}

impl Default for BodyContent {
    fn default() -> Self {
        Self {
            content: Arc::default(),
            title: "No Content".to_string(),
        }
    }
}

/// Rendering logic
impl Widget for &BodyContent {
    /// Display/render the body
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(Line::from(self.title.as_str().bold()).centered())
            .border_set(border::THICK);
        Paragraph::new(&self.content[..])
            .centered()
            .block(block)
            .render(area, buf);
    }
}

/// Messages/Events for the [`BodyContent`] widget
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Message {}

/// Update logic
impl BodyContent {
    /// Update the state with the given [`Message`]
    ///
    /// See [`Message`] docs for info on possible updates
    pub fn update(&mut self, _msg: Message) {}
}

#[cfg(test)]
mod tests {}
