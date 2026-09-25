use crossterm::event::{self, KeyCode};
use ratatui::{
    Frame,
    layout::{Position, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

/// The state (string and character position) of a prompt string
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ShellPrompt {
    input: String,
    character_index: u16,
}

impl ShellPrompt {
    pub fn render(&self, area: Rect, frame: &mut Frame) {
        const PREFIX: &str = "! ";
        let text = Line::from(vec![PREFIX.bold(), self.input.as_str().into()]);

        let block = Block::new()
            .borders(Borders::TOP)
            .title(Line::from(" Input Prompt ".bold()))
            .border_set(border::THICK);

        frame.render_widget(Paragraph::new(text).left_aligned().block(block), area);

        frame.set_cursor_position(Position::new(
            // Start at the beginining of the area, move passed the prefix and
            // space (the `+ 1`) and to the correct `character_index`
            area.x + PREFIX.chars().count() as u16 + self.character_index,
            area.y + 1,
        ));
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Message {
    MoveCursor(Dir),
    Insert(char),
    Delete,
    Enter,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Dir {
    Left,
    Right,
}

pub fn handle_key(key: event::KeyEvent) -> Option<Message> {
    Some(match key.code {
        KeyCode::Char(to_insert) => Message::Insert(to_insert),
        KeyCode::Left => Message::MoveCursor(Dir::Left),
        KeyCode::Right => Message::MoveCursor(Dir::Right),
        KeyCode::Backspace => Message::Delete,
        KeyCode::Enter => Message::Enter,
        _ => return None,
    })
}

impl ShellPrompt {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::MoveCursor(dir) => self.move_cursor(dir),
            Message::Insert(to_insert) => self.enter_char(to_insert),
            Message::Delete => todo!(),
            Message::Enter => todo!(),
        }
    }

    fn move_cursor(&mut self, dir: Dir) {
        let cursor_moved = match dir {
            Dir::Left => self.character_index.saturating_sub(1),
            Dir::Right => self.character_index.saturating_add(1),
        };
        self.character_index = cursor_moved.clamp(0, self.input.chars().count() as u16);
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can contain multiple bytes,
    /// it's necessary to calculate the byte index based on the index of
    /// the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index.into())
            .unwrap_or(self.input.len())
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor(Dir::Right);
    }
}
