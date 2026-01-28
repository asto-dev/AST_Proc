use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    DefaultTerminal,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, ScrollbarState},
};
use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

use crate::process::Process;
use crate::read;

#[derive(Default)]
pub struct App {
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
}

impl App {
    pub fn run(mut self, mut terminal: DefaultTerminal) -> std::io::Result<()> {
        let mut procs = read::get_proc("/proc").unwrap();
        let mut tree = read::build_tree(&procs);

        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_secs(1);

        let mut prefix = String::new();
        let mut lines: Vec<Line<'static>> = Vec::new();

        build_tree_lines(0, &mut prefix, &tree, &procs, &mut lines);
        loop {
            if last_tick.elapsed() >= tick_rate {
                procs = read::get_proc("/proc")?;
                tree = read::build_tree(&procs);
                lines = Vec::new();
                build_tree_lines(0, &mut prefix, &tree, &procs, &mut lines);

                last_tick = Instant::now();
            }

            let max_scroll = lines.len().saturating_sub(1);

            terminal.draw(|frame| {
                let paragraph = Paragraph::new(Text::from(std::mem::take(&mut lines)))
                    .block(Block::default().title("ASTProc").borders(Borders::ALL))
                    .style(Style::default().fg(Color::LightMagenta))
                    .scroll((self.vertical_scroll as u16, self.horizontal_scroll as u16));

                frame.render_widget(paragraph, frame.area());
            })?;

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => break Ok(()),
                        KeyCode::Char('j') | KeyCode::Down => {
                            self.vertical_scroll =
                                self.vertical_scroll.saturating_add(2).min(max_scroll);
                            self.vertical_scroll_state =
                                self.vertical_scroll_state.position(self.vertical_scroll);
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            self.vertical_scroll =
                                self.vertical_scroll.saturating_sub(2).min(max_scroll);
                            self.vertical_scroll_state =
                                self.vertical_scroll_state.position(self.vertical_scroll);
                        }
                        KeyCode::Char('h') | KeyCode::Left => {
                            self.horizontal_scroll =
                                self.horizontal_scroll.saturating_sub(2).min(max_scroll);
                            self.horizontal_scroll_state = self
                                .horizontal_scroll_state
                                .position(self.horizontal_scroll);
                        }
                        KeyCode::Char('l') | KeyCode::Right => {
                            self.horizontal_scroll =
                                self.horizontal_scroll.saturating_add(2).min(max_scroll);
                            self.horizontal_scroll_state = self
                                .horizontal_scroll_state
                                .position(self.horizontal_scroll);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn build_tree_lines(
    pid: u32,
    prefix: &mut String,
    tree: &HashMap<u32, Vec<u32>>,
    procs: &HashMap<u32, Process>,
    // lines: &mut Vec<String>,
    lines: &mut Vec<Line<'static>>,
) {
    if let Some(children) = tree.get(&pid) {
        let mut sorted = children.clone();
        sorted.sort_unstable();

        for (i, child) in sorted.iter().enumerate() {
            let is_last = i == sorted.len() - 1;
            let mut new_prefix = if prefix.is_empty() && is_last {
                // If this is a root and it's the only/last root, no indentation needed for children
                "".to_string()
            } else {
                // Otherwise, append the vertical bar or spaces based on 'last'
                format!("{}{}", prefix, if is_last { "   " } else { "│  " })
            };
            let branch = if is_last { "└─ " } else { "├─ " };

            let p = &procs[&child];
            let content = format!("{}{}{} ({}) - [{}]", prefix, branch, p.name, p.pid, p.exe);
            lines.push(Line::from(content));
            build_tree_lines(*child, &mut new_prefix, tree, procs, lines);
        }
    }
}
