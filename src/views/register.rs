use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};
use tui_input::Input;

use crate::AppState;

pub struct RegisterView {
    input: Input,
    input_mode: InputMode,
    pub should_go_to: Option<AppState>,
}

enum InputMode {
    Normal,
    Editing,
}

impl Default for RegisterView {
    fn default() -> Self {
        Self {
            input: Input::default(),
            input_mode: InputMode::Normal,
            should_go_to: None,
        }
    }
}

impl RegisterView {
    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('e') => self.input_mode = InputMode::Normal,
            KeyCode::Esc => self.should_go_to = Some(AppState::Landing),
            _ => {}
        }
    }
}

impl Widget for &mut RegisterView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let input = Paragraph::new(self.input.value())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::default().borders(Borders::ALL).title("Input"));
        Widget::render(input, area, buf);
    }
}
