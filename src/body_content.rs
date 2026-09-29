//! The [`BodyContent`] widget and display logic
//!
//! Displaying and updating [`BodyContent`] are controlled through:
//! - [`render`]: How to display the widget
//! - [`update`]: How to update the widget based on the passed [`Message`]s
//!
//! [`render`]: BodyContent::render
//! [`update`]: BodyContent::update

use std::sync::Arc;

use ratatui::{Frame, layout::Rect};

/// The state/widget for the body of the tui
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct BodyContent {
    /// Shared pointer to the contents to display
    content: Arc<str>,
}

/// Rendering logic
impl BodyContent {
    /// Display/render the body
    pub fn render(&self, _area: Rect, _frame: &mut Frame<'_>) {
        todo!()
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
