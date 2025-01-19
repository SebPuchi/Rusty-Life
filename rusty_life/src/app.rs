use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        canvas::{Canvas, Map, MapResolution, Rectangle},
        Block, 
        Paragraph, 
        Widget
    },
    DefaultTerminal, Frame,
    style::Color,

};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {

    /// runs the application's main loop until the user quits
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

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

     fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('Q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

}
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::bordered()
            .title(" Basic Block ".bold())
            .border_set(border::THICK);

        block.render(area, buf);
    }
}

