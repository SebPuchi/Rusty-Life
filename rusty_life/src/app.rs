use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    symbols,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        canvas::{Canvas, Map, MapResolution, Rectangle, Line as CanvasLine},
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
        frame.render_widget(self.map_canvas(), frame.area());

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
fn map_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .marker(symbols::Marker::Block)
            .block(Block::bordered().title(" rusty-life "))
            .x_bounds([0.0, 10.0])
            .y_bounds([0.0, 10.0])
            .paint(|context| {
                // Draw vertical grid lines
                for x in -10..=10 {
                    let x = x as f64;
                    context.draw(&CanvasLine {
                        x1: x,
                        y1: 0.0,
                        x2: x,
                        y2: 10.0,
                        color: Color::DarkGray,
                    });
                }

            })
    }
    
}

