use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

mod input_prompt;

#[derive(Debug, Default)]
struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// Updates the application's state based on events
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => {
                self.exit = true;
            }
            KeyCode::Char('i') => {
                todo!("enter insert mode")
            }
            _ => {}
        }
    }
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = AppLayout::new(area);
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

        Paragraph::new("empty text")
            .centered()
            .block(block)
            .render(layout.body_layout, buf);
    }
}

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━━━━━ Main Content ━━━━━━━━━━━━━━━━━┓",
            "┃                   empty text                   ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(18, 0, 14, 1), title_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }

    #[test]
    fn handle_key_event() {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);
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
