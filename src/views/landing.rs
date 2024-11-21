use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Modifier, Style, Stylize},
    text::Text,
    widgets::{Block, HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget},
};

use crate::AppState;

pub struct LandingView {
    landing_list: LandingList,
    logo: String,
    pub should_go_to: Option<AppState>,
}

struct LandingList {
    items: [String; 2],
    state: ListState,
}

impl LandingView {
    pub fn new(logo: String) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            landing_list: LandingList {
                items: ["Login".to_string(), "Register".to_string()],
                state: list_state,
            },
            logo,
            should_go_to: None,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => self.landing_list.state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.landing_list.state.select_previous(),
            KeyCode::Char('l') | KeyCode::Enter => {
                if let Some(index) = self.landing_list.state.selected() {
                    if index == 1 {
                        self.should_go_to = Some(AppState::Register);
                    }
                }
            }
            _ => {}
        }
    }
}

impl Widget for &mut LandingView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [_, logo_row, _, list_row, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(11),
            Constraint::Length(2),
            Constraint::Length(4),
            Constraint::Fill(1),
        ])
        .areas(area);
        let [_, list_column, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Percentage(50),
            Constraint::Fill(1),
        ])
        .areas(list_row);

        let [_, logo_column, _] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(78),
            Constraint::Fill(1),
        ])
        .areas(logo_row);

        let items: Vec<ListItem> = self
            .landing_list
            .items
            .iter()
            .map(|item| ListItem::new(item.as_str()))
            .collect();

        let list = List::new(items)
            .block(Block::bordered())
            .style(Style::new().white())
            .highlight_style(Style::new().blue().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        StatefulWidget::render(list, list_column, buf, &mut self.landing_list.state);

        let logo = Text::from(self.logo.as_str()).blue();
        Widget::render(logo, logo_column, buf);
    }
}
