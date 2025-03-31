use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Circle, Map, MapResolution, Points, Rectangle},
        Block, 
        Borders, 
        BorderType,
    }, 
    text::Line as TextLine,
    symbols,
    DefaultTerminal, 
    Frame
};

use crate::grid::LifeGrid;


pub struct App {
    grid: LifeGrid,
    exit: bool,
    cell_size: u16
}

//App behaviour
impl App {
    pub fn new(grid: LifeGrid) -> Self {
        Self {
            exit: false,
            grid,
            cell_size: 10,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.ui(frame))?;
            self.handle_events()?;
        }
        Ok(())
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
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

}

//App rendering
impl App {
    fn ui(&self, frame: &mut Frame) {
        let frame_area = frame.area();

        let map = Canvas::default()
            .block(Block::bordered()
                .title_bottom(TextLine::from(" frame: 10 ").right_aligned())
                .title_bottom(TextLine::from(
                    format!(
                        " frame_width: {} | frame_height: {} ", 
                        frame_area.width, 
                        frame_area.height
                    )
                ).left_aligned())
            )
            .marker(symbols::Marker::HalfBlock)
            .paint(|ctx| {
                 ctx.draw(&Points {
                    coords: &[(10.0, 11.0),(10.0, 10.5), (10.0, 10.0)],
                    color: Color::Red,
                });
            }).x_bounds([0.0, (frame_area.width as f64)])  
            .y_bounds([0.0, (frame_area.height as f64)]);
            frame.render_widget(map, frame_area);
    }
}


