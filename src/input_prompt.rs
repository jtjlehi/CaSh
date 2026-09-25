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
            // space and to the correct `character_index`
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
            Message::Insert(to_insert) => self.insert_char(to_insert),
            Message::Delete => self.delete_char(),
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

    /// insert `new_char` at the current `character_index`
    fn insert_char(&mut self, new_char: char) {
        // Since each character in a string can contain multiple bytes,
        // it's necessary to calculate the byte index based on the index of
        // the character.
        let index = self
            .input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index())
            .unwrap_or(self.input.len());
        self.input.insert(index, new_char);
        self.move_cursor(Dir::Right);
    }

    /// Delete the character at the current `character_index`
    // FIXME: I don't like how this is done. seems like it's needlessly allocating
    fn delete_char(&mut self) {
        // Method "remove" is not used on the saved text for deleting the selected char.
        // Reason: Using remove on String works on bytes instead of the chars.
        // Using remove would require special care because of char boundaries.

        // Getting all characters before the selected character.
        let before_char_to_delete = self
            .input
            .chars()
            .take(self.character_index().saturating_sub(1));
        // Getting all characters after selected character.
        let after_char_to_delete = self.input.chars().skip(self.character_index());

        // Put all characters together except the selected one.
        // By leaving the selected one out, it is forgotten and therefore deleted.
        self.input = before_char_to_delete.chain(after_char_to_delete).collect();
        self.move_cursor(Dir::Left);
    }

    fn character_index(&self) -> usize {
        self.character_index.into()
    }
}
