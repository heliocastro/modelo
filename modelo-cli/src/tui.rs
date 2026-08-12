// SPDX-FileCopyrightText: 2026 Helio Chissini de Castro <dev@heliocastro.info>
// SPDX-License-Identifier: MIT

//! A modest interactive TUI: pick one of the three model kinds, point it at a YAML
//! file, and see whether it validates. No multi-pane navigation, no editing — just
//! load a file and inspect the result.

use std::io;
use std::path::PathBuf;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::backend::{Backend, CrosstermBackend};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use serde::de::DeserializeOwned;

use modelo::models::Model;
use modelo::models::ort::license_classifications::LicenseClassifications;
use modelo::models::ort::ort_result::OrtResult;
use modelo::models::ort::repository_configuration::RepositoryConfiguration;

const KINDS: [&str; 3] = [
    "license-classifications",
    "repository-configuration",
    "ort-result",
];

enum Screen {
    SelectKind,
    InputPath,
    Result,
}

struct App {
    screen: Screen,
    kind_index: usize,
    path_input: String,
    output: String,
    is_error: bool,
    scroll: u16,
}

/// Runs the interactive TUI. `datafile`, if given, pre-fills the path prompt.
pub fn run(datafile: Option<PathBuf>) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = App {
        screen: Screen::SelectKind,
        kind_index: 0,
        path_input: datafile
            .map(|p| p.display().to_string())
            .unwrap_or_default(),
        output: String::new(),
        is_error: false,
        scroll: 0,
    };

    let result = event_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn event_loop<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> anyhow::Result<()> {
    loop {
        terminal.draw(|f| draw(f, app))?;

        if !event::poll(Duration::from_millis(250))? {
            continue;
        }
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        match app.screen {
            Screen::SelectKind => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Up | KeyCode::Char('k') => {
                    app.kind_index = app.kind_index.checked_sub(1).unwrap_or(KINDS.len() - 1);
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    app.kind_index = (app.kind_index + 1) % KINDS.len();
                }
                KeyCode::Enter => app.screen = Screen::InputPath,
                _ => {}
            },
            Screen::InputPath => match key.code {
                KeyCode::Esc => app.screen = Screen::SelectKind,
                KeyCode::Enter => {
                    load_and_validate(app);
                    app.screen = Screen::Result;
                }
                KeyCode::Backspace => {
                    app.path_input.pop();
                }
                KeyCode::Char(c) => app.path_input.push(c),
                _ => {}
            },
            Screen::Result => match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                KeyCode::Char('b') => {
                    app.screen = Screen::SelectKind;
                    app.scroll = 0;
                }
                KeyCode::Up | KeyCode::Char('k') => app.scroll = app.scroll.saturating_sub(1),
                KeyCode::Down | KeyCode::Char('j') => app.scroll = app.scroll.saturating_add(1),
                _ => {}
            },
        }
    }
}

fn load_and_validate(app: &mut App) {
    let contents = match std::fs::read_to_string(app.path_input.trim()) {
        Ok(c) => c,
        Err(e) => {
            app.output = format!("failed to read '{}': {e}", app.path_input.trim());
            app.is_error = true;
            return;
        }
    };

    let result = match app.kind_index {
        0 => validate_pretty::<LicenseClassifications>(&contents),
        1 => validate_pretty::<RepositoryConfiguration>(&contents),
        _ => validate_pretty::<OrtResult>(&contents),
    };

    match result {
        Ok(text) => {
            app.output = text;
            app.is_error = false;
        }
        Err(text) => {
            app.output = text;
            app.is_error = true;
        }
    }
}

fn validate_pretty<T>(contents: &str) -> Result<String, String>
where
    T: Model + DeserializeOwned,
{
    let value: T = serde_yaml::from_str(contents).map_err(|e| format!("parse error: {e}"))?;
    value
        .validate()
        .map_err(|e| format!("validation error: {e}"))?;
    Ok(format!("{value:#?}"))
}

fn draw(f: &mut Frame, app: &App) {
    match app.screen {
        Screen::SelectKind => draw_select_kind(f, app),
        Screen::InputPath => draw_input_path(f, app),
        Screen::Result => draw_result(f, app),
    }
}

fn draw_select_kind(f: &mut Frame, app: &App) {
    let items: Vec<ListItem> = KINDS
        .iter()
        .enumerate()
        .map(|(i, k)| {
            let style = if i == app.kind_index {
                Style::default().add_modifier(Modifier::REVERSED)
            } else {
                Style::default()
            };
            ListItem::new(*k).style(style)
        })
        .collect();
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("modelo — select model kind (up/down, Enter, q to quit)"),
    );
    f.render_widget(list, f.size());
}

fn draw_input_path(f: &mut Frame, app: &App) {
    let title = format!(
        "Path to {} YAML file (Enter to load, Esc to go back)",
        KINDS[app.kind_index]
    );
    let p = Paragraph::new(app.path_input.as_str())
        .block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(p, f.size());
}

fn draw_result(f: &mut Frame, app: &App) {
    let title = if app.is_error {
        "Validation FAILED (b: back, q: quit, up/down: scroll)"
    } else {
        "Validation OK (b: back, q: quit, up/down: scroll)"
    };
    let style = if app.is_error {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::Green)
    };
    let p = Paragraph::new(app.output.as_str())
        .style(style)
        .wrap(Wrap { trim: false })
        .scroll((app.scroll, 0))
        .block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(p, f.size());
}
