use anyhow::Result;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::prelude::*;
use ratatui::widgets::*;

use crate::diagram::{Diagram, Direction, HookFamily};
use crate::tui::Tui;

#[derive(Debug, Default)]
pub struct App {
    hook_family: HookFamily,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AppMessage {
    #[default]
    Running,
    Quitting,
}

impl App {
    pub fn run(&mut self, terminal: &mut Tui) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render_frame(frame))?;
            if AppMessage::Quitting == self.handle_events()? {
                break;
            }
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        let area = frame.size();
        let layouts = Layout::vertical([Constraint::Length(12), Constraint::Fill(1)]).split(area);
        let layouts = Layout::horizontal([Constraint::Length(25), Constraint::Fill(1)]).split(layouts[0]);
        
        let block = Block::bordered().border_type(BorderType::Double);
        let diagram_area = block.inner(layouts[0]);
        frame.render_widget(block, layouts[0]);
        
        let diagram = Diagram::new(self.hook_family);
        frame.render_widget(diagram, diagram_area);

        let hook_family = Line::from(self.hook_family.to_string());
        frame.render_widget(hook_family, layouts[1]);
    }

    fn handle_events(&mut self) -> Result<AppMessage> {
        let mut msg = AppMessage::Running;
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => msg = AppMessage::Quitting,
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        msg = AppMessage::Quitting
                    }
                    direction @ (KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right) => {
                        self.hook_family = self.hook_family.move_by_direction(direction.into());
                    }
                    _ => (),
                }
            }
            _ => (),
        };
        Ok(msg)
    }
}

impl From<KeyCode> for Direction {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Up => Direction::Up,
            KeyCode::Down => Direction::Down,
            KeyCode::Left => Direction::Left,
            KeyCode::Right => Direction::Right,
            _ => unreachable!(),
        }
    }
}
