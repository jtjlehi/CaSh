use crossterm::event::{self, KeyCode};
use ratatui::{
    Frame,
    layout::{Position, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use unicode_segmentation::{Graphemes, UnicodeSegmentation};
use unicode_width::UnicodeWidthStr;

/// The state (string and character position) of a prompt string
#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ShellPrompt {
    input: String,
    grapheme_idx: u16,
}

/// Helper Functions
impl ShellPrompt {
    /// The `char_idx` field as a usize
    fn char_idx(&self) -> usize {
        self.grapheme_idx.into()
    }

    /// The index into `input` that is the byte that corresponds to `char_idx`
    fn byte_idx(&self) -> usize {
        self.input
            .grapheme_indices(true)
            .map(|(i, _)| i)
            .nth(self.char_idx())
            .unwrap_or(self.input.len())
    }

    fn graphemes(&self) -> Graphemes<'_> {
        self.input.graphemes(true)
    }
}

/// Rendering logic
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
            // space and to the correct `char_idx`
            area.x + PREFIX.chars().count() as u16 + self.cursor_pos() as u16,
            area.y + 1,
        ));
    }

    /// The position of the cursor (ignoring the prefix)
    fn cursor_pos(&self) -> usize {
        self.input[..self.byte_idx()].width_cjk()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Message {
    MoveCursor(Dir),
    Insert(char),
    Delete,
    Reset,
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
        _ => return None,
    })
}

/// Update Logic
impl ShellPrompt {
    pub fn update(&mut self, msg: Message) {
        match msg {
            Message::MoveCursor(dir) => self.move_cursor(dir),
            Message::Insert(to_insert) => self.insert_char(to_insert),
            Message::Delete => self.delete_char(),
            Message::Reset => self.reset(),
        }
    }

    fn move_cursor(&mut self, dir: Dir) {
        let cursor_moved = match dir {
            Dir::Left => self.grapheme_idx.saturating_sub(1),
            Dir::Right => self.grapheme_idx.saturating_add(1),
        };
        self.grapheme_idx = cursor_moved.clamp(0, self.graphemes().count() as u16);
    }

    /// insert `new_char` at the current `char_idx`
    fn insert_char(&mut self, new_char: char) {
        // Since each character in a string can contain multiple bytes,
        // it's necessary to calculate the byte index based on the index of
        // the character.
        self.input.insert(self.byte_idx(), new_char);
        self.move_cursor(Dir::Right);
    }

    /// Delete the character at the current `char_idx`
    // FIXME: I don't like how this is done. seems like it's needlessly allocating
    fn delete_char(&mut self) {
        // Method "remove" is not used on the saved text for deleting the selected char.
        // Reason: Using remove on String works on bytes instead of the chars.
        // Using remove would require special care because of char boundaries.

        // Getting all characters before the selected character.
        let before_char_to_delete = self.graphemes().take(self.char_idx().saturating_sub(1));
        // Getting all characters after selected character.
        let after_char_to_delete = self.graphemes().skip(self.char_idx());

        // Put all characters together except the selected one.
        // By leaving the selected one out, it is forgotten and therefore deleted.
        self.input = before_char_to_delete.chain(after_char_to_delete).collect();
        self.move_cursor(Dir::Left);
    }

    fn reset(&mut self) {
        self.input.clear();
        self.grapheme_idx = 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn new_shell_prompt(input: &str, grapheme_idx: u16) -> ShellPrompt {
        ShellPrompt {
            input: input.to_string(),
            grapheme_idx,
        }
    }

    // Now with a `new_char` !!!!
    fn test_insert(input: &str, grapheme_idx: u16, new_char: char, output: &str) {
        let mut prompt = new_shell_prompt(input, grapheme_idx);
        prompt.insert_char(new_char);
        assert_eq!(
            prompt.input, output,
            "input('{input}', {grapheme_idx}), produced '{}' instead of '{output}'",
            prompt.input
        );

        assert_eq!(
            prompt.grapheme_idx,
            grapheme_idx + 1,
            "delete('{input}', {grapheme_idx}), moved char idx to `{}` instead of `{}`",
            prompt.grapheme_idx,
            grapheme_idx + 1
        );
    }

    #[test]
    fn insert_works() {
        test_insert("", 0, 'a', "a");
        test_insert("abc", 3, 'd', "abcd");
        test_insert("abc", 0, 'd', "dabc");
        test_insert("abc", 2, 'd', "abdc");
        test_insert("abc", 3, 'Ｈ', "abcＨ");
        test_insert("abc", 0, 'Ｈ', "Ｈabc");
        test_insert("abc", 2, 'Ｈ', "abＨc");
    }

    fn test_delete(input: &str, grapheme_idx: u16, output: &str) {
        assert!(
            usize::from(grapheme_idx) < input.chars().count() + 1 && grapheme_idx > 0,
            "delete_char test configured with invalid `char_idx`"
        );
        let mut prompt = new_shell_prompt(input, grapheme_idx);
        prompt.delete_char();
        assert_eq!(
            prompt.input, output,
            "delete('{input}', {grapheme_idx}), produced '{}' instead of '{output}'",
            prompt.input
        );
        assert_eq!(
            prompt.grapheme_idx,
            grapheme_idx - 1,
            "delete('{input}', {grapheme_idx}), moved char idx to `{}` instead of `{}`",
            prompt.grapheme_idx,
            grapheme_idx - 1
        );
    }

    #[test]
    fn delete_at_zero() {
        let mut prompt = ShellPrompt::default();
        let expected = prompt.clone();
        prompt.delete_char();
        assert_eq!(prompt, expected);

        prompt.input = "abc".to_string();
        let expected = prompt.clone();
        prompt.delete_char();
        assert_eq!(prompt, expected);
    }

    #[test]
    fn delete_char_ascii() {
        test_delete("a", 1, "");
        test_delete("abc", 3, "ab");
        test_delete("abc", 2, "ac");
    }

    #[test]
    fn delete_unicode_codepoint() {
        // latin small letter e with acute
        // single code point
        test_delete("é", 1, "");
        test_delete("éabc", 1, "abc");
        test_delete("éabc", 2, "ébc");
        test_delete("aéabc", 2, "aabc");
    }

    #[test]
    fn delete_unicode_wide_codepoint() {
        test_delete("Ｈｅｌｌｏ, ｗｏｒｌｄ!", 2, "Ｈｌｌｏ, ｗｏｒｌｄ!");
        test_delete("Ｈｅｌｌｏ, ｗｏｒｌｄ!", 5, "Ｈｅｌｌ, ｗｏｒｌｄ!");
        test_delete("Ｈｅｌｌｏ, ｗｏｒｌｄ!", 7, "Ｈｅｌｌｏ,ｗｏｒｌｄ!");
        test_delete("Ｈｅｌｌｏ, ｗｏｒｌｄ!", 9, "Ｈｅｌｌｏ, ｗｒｌｄ!");
    }

    #[test]
    fn delete_unicode_grapheme() {
        // latin small letter e + combining acute accent
        // 2 code points
        test_delete("é", 1, "");
        test_delete("éabc", 1, "abc");
        test_delete("é", 1, "");
        test_delete("éabc", 1, "abc");
        test_delete("éabc", 2, "ébc");
        test_delete("aéabc", 2, "aabc");
    }

    #[test]
    fn correct_cursor_pos() {
        assert_eq!(new_shell_prompt("abcd", 3).cursor_pos(), 3);
        // The wide characters should be accounted for in the cursor pos
        assert_eq!(
            new_shell_prompt("Ｈｅｌｌｏ, ｗｏｒｌｄ!", 3).cursor_pos(),
            6
        );
        assert_eq!(
            new_shell_prompt("Ｈｅｌｌｏ, ｗｏｒｌｄ!", 5).cursor_pos(),
            10
        );
        assert_eq!(
            new_shell_prompt("Ｈｅｌｌｏ, ｗｏｒｌｄ!", 8).cursor_pos(),
            14
        );
    }
}
