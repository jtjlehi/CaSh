//! A TUI for capturing shell outputs and exploring them

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::input_prompt::{Message as PromptMessage, ShellPrompt};

pub mod input_prompt;

/// The full state of the app
#[derive(Default, Debug, PartialEq, PartialOrd)]
pub struct State {
    mode: Mode,
    prompt_string: ShellPrompt,
}

/// The global mode of the ui
#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Mode {
    /// The default/normal mode
    #[default]
    Normal,
    /// Currently typing in a command in the shell
    Shell,
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
    pub fn view(&self, frame: &mut Frame<'_>) {
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

        // render the prompt if in edit mode
        if let Mode::Shell = self.mode {
            self.prompt_string.render(layout.prompt_layout, frame);
        }
    }
}

/// Message or event passed to the [`State::update`] function
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Message {
    /// Quit the application
    Quit,
    /// Transition to the given mode
    ToMode(Mode),
    /// A message that is passed to the [`PromptShell`]
    PromptMessage(PromptMessage),
    /// Enter the command in the Prompt
    ///
    /// Doing this will pass commands down to [`PromptShell::update`]
    EnterCmd,
}

/// The `update` logic
impl State {
    /// Update the state based on the message passsed
    ///
    /// If the app is quitting, it returns `None`
    ///
    /// See the docs for [`Message`] for details on the updates
    pub fn update(mut self, msg: Message) -> Option<State> {
        match msg {
            Message::Quit => return None,
            Message::ToMode(mode) => self.mode = mode,
            Message::PromptMessage(msg) => self.prompt_string.update(msg),
            Message::EnterCmd => {
                // TODO: actually enter the command

                self.prompt_string.update(PromptMessage::Reset);
            }
        }
        Some(self)
    }
}

/// Handle the next global event by converting it into a message
pub fn handle_event(state: &State) -> io::Result<Option<Message>> {
    match event::read()? {
        // it's important to check that the event is a key press event as
        // crossterm also emits key release and repeat events on Windows.
        Event::Key(key) if key.kind == KeyEventKind::Press => Ok(handle_key(state.mode, key)),
        _ => Ok(None),
    }
}

fn handle_key(mode: Mode, key: event::KeyEvent) -> Option<Message> {
    match mode {
        Mode::Normal => match key.code {
            KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Char('!') => Some(Message::ToMode(Mode::Shell)),
            _ => None,
        },
        Mode::Shell => match key.code {
            // Exit prompt mode back to normal mode
            KeyCode::Esc => Some(Message::ToMode(Mode::Normal)),
            KeyCode::Enter => Some(Message::EnterCmd),
            _ => input_prompt::handle_key(key).map(Message::PromptMessage),
        },
    }
}

fn main() -> io::Result<()> {
    ratatui::run(|terminal| -> io::Result<()> {
        let mut state_opt = Some(State::default());
        while let Some(state) = state_opt.take() {
            terminal.draw(|f| state.view(f))?;
            if let Some(msg) = handle_event(&state)? {
                state_opt = state.update(msg);
            } else {
                state_opt = Some(state);
            }
        }
        Ok(())
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// test that the given key event produces the given message and the state
    /// is correctly updated
    fn test_normal_key_evt(key: event::KeyEvent, expected_msg: Message) {
        assert_eq!(
            handle_key(Mode::Normal, key).expect("The key event to produce a message"),
            expected_msg
        );
    }

    #[test]
    fn normal_q_key_quits() {
        test_normal_key_evt(KeyCode::Char('q').into(), Message::Quit);
    }

    #[test]
    fn quit_message_quits() {
        assert_eq!(State::default().update(Message::Quit), None);
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
