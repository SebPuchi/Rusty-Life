use std::io;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize}, 
    symbols::{self, border::{self, THICK}}, 
    text::{Line, Text}, 
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Rectangle},
        Block, 
        Widget,
        BorderType,
    }, DefaultTerminal, Frame
};

use crate::grid::LifeGrid;



pub struct App {
    grid: LifeGrid,
    exit: bool,
}

impl App {
    pub fn new(grid: LifeGrid) -> Self {
        Self {
            exit: false,
            grid,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        let aspect_ratio = self.grid.length as f64 / self.grid.width as f64;
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),  // Top margin
                Constraint::Min(0),          // Grid area
                Constraint::Percentage(10),  // Bottom margin
            ])
            .split(area);

        // Create horizontal layout for the middle section
        let middle_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),  // Left margin
                Constraint::Min(0),          // Grid area
                Constraint::Percentage(10),  // Right margin
            ])
            .split(layout[1]);

        frame.render_widget(self.map_canvas(), middle_layout[1]);
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
            .marker(symbols::Marker::HalfBlock)
            .block(Block::bordered()
                .title(" rusty-life ")
                .border_type(BorderType::Thick)
            )
            .x_bounds([0.0, self.grid.length as f64])
            .y_bounds([0.0, self.grid.width as f64])
            .paint(|context| {
            })
    }
    
}

