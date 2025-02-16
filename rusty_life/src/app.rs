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
        Padding,
        Borders, 
        Paragraph
    }, 
    DefaultTerminal, 
    Frame
};

use crate::grid::LifeGrid;


pub struct App {
    grid: LifeGrid,
    exit: bool,
}

//App behaviour
impl App {
    pub fn new(grid: LifeGrid) -> Self {
        Self {
            exit: false,
            grid,
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

    // ANCHOR: handle_key_event fn
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
    fn ui(&self, frame: &mut ratatui::Frame) {
        // Define the layout: Two vertical sections (one for content, one for status)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(80), // Main content (80% of terminal height)
                Constraint::Percentage(20), // Footer/status (20%)
            ])
            .split(frame.area()); // Applies the layout to the frame area

        // Render a block in the first section
        let block = Block::default()
            .title("Main Grid")
            .borders(Borders::ALL);
        frame.render_widget(block, chunks[0]);

        // Render a status line in the second section
        let status = Paragraph::new("Press 'Q' to exit")
            .block(Block::default().borders(Borders::ALL));
        frame.render_widget(status, chunks[1]);
    }
}


