use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use input_prompt::InputPrompt;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph},
};

mod input_prompt;

#[derive(Default, Debug, PartialEq, PartialOrd)]
/// The full state of the app
pub struct State {
    mode: Mode,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Mode {
    #[default]
    Normal,
    EditPrompt,
    /// It is time to exit/close the app
    Exit,
}

struct AppLayout {
    body_layout: Rect,
    prompt_layout: Rect,
}

impl AppLayout {
    fn new(area: Rect) -> Self {
        let [body_layout, prompt_layout] = area.layout(&Layout::vertical([
            // main body should be at least 30 tall
            Constraint::Min(30),
            Constraint::Length(10),
        ]));
        AppLayout {
            body_layout,
            prompt_layout,
        }
    }
}

/// The `view` logic
impl State {
    /// How to render the app based on the state
    pub fn view(&self, frame: &mut Frame) {
        let layout = AppLayout::new(frame.area());

        // Draw the main content (for now it isn't a seperate thing)
        let title = Line::from(" Main Content ".bold());
        let instructions = Line::from(vec![
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        frame.render_widget(
            Paragraph::new("empty text").centered().block(block),
            layout.body_layout,
        );
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Message {
    /// Quit the application
    Quit,
}

/// The `update` logic
impl State {
    /// Update the state based on the message passsed
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::Quit => {
                self.mode = Mode::Exit;
            }
        }
    }
}

pub fn handle_event(_: &State) -> io::Result<Option<Message>> {
    match event::read()? {
        // it's important to check that the event is a key press event as
        // crossterm also emits key release and repeat events on Windows.
        Event::Key(key) if key.kind == KeyEventKind::Press => Ok(handle_key(key)),
        _ => Ok(None),
    }
}

fn handle_key(key: event::KeyEvent) -> Option<Message> {
    match key.code {
        // KeyCode::Char('j') => Some(Message::Increment),
        // KeyCode::Char('k') => Some(Message::Decrement),
        KeyCode::Char('q') => Some(Message::Quit),
        _ => None,
    }
}

fn main() -> io::Result<()> {
    ratatui::run(|terminal| -> io::Result<()> {
        let mut state = State::default();
        while state.mode != Mode::Exit {
            terminal.draw(|f| state.view(f))?;
            if let Some(msg) = handle_event(&state)? {
                state.update(msg);
            }
        }
        Ok(())
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the given message updates the state correctly (from the default state)
    fn test_update(msg: Message, expected_state: State) {
        let mut state = State::default();
        state.update(msg);
        assert_eq!(state, expected_state);
    }

    /// test that the given key event produces the given message and the state
    /// is correctly updated
    fn test_key_evt(key: event::KeyEvent, expected_msg: Message, expected_state: State) {
        let msg = handle_key(key).expect("The key event to produce a message");
        assert_eq!(msg, expected_msg);

        test_update(msg, expected_state);
    }

    #[test]
    fn handle_q_key() {
        let msg = handle_key(KeyCode::Char('q').into()).unwrap();
        assert_eq!(msg, Message::Quit);

        let mut state = State::default();

        state.update(msg);

        assert_eq!(state.mode, Mode::Exit);

        test_key_evt(
            KeyCode::Char('q').into(),
            Message::Quit,
            State { mode: Mode::Exit },
        );
    }

    #[test]
    fn builds_app_layout() {
        let AppLayout {
            body_layout,
            prompt_layout,
        } = AppLayout::new(Rect::new(0, 0, 10, 100));
        assert_eq!(body_layout, Rect::new(0, 0, 10, 90));
        assert_eq!(prompt_layout, Rect::new(0, 90, 10, 10));
    }
}
