use core::sudoku::Sudoku;
use std::{fmt::Display, num::NonZero, str::FromStr, time::Instant};

use iced::{
    Border, Color, Subscription,
    alignment::{Horizontal, Vertical},
    widget::{Column, ComboBox, button, column, combo_box, container, row, text, text_input},
};

use iced::time::Duration;

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    TogglePause,
    Solve,
    Reset,
    UpdateSpeed(String),
    UpdateHeatmap(Heatmap),
}

#[derive(Debug, Clone)]
enum Heatmap {
    Backtracked,
    Visited,
    InvalidAttempts,
}

impl Display for Heatmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heatmap::Backtracked => write!(f, "Backtracked"),
            Heatmap::Visited => write!(f, "Visited"),
            Heatmap::InvalidAttempts => write!(f, "InvalidAttempts"),
        }
    }
}

struct Game {
    sudoku: Sudoku,
    is_paused: bool,
    is_complete: bool,
    time_to_solve: Option<Duration>,
    update_nanoseconds: u128,
    selected_heatmap: Heatmap,
    heatmaps: combo_box::State<Heatmap>,
}

fn main() -> iced::Result {
    iced::application(Game::new, Game::update, Game::view)
        .subscription(Game::subscription)
        .run()
}

impl Game {
    pub fn new() -> Game {
        let example: [Option<NonZero<u8>>; 81] = [
            0, 0, 0, 0, 0, 0, 0, 1, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 5, 0, 4, 0, 7, 0, 0, 8, 0, 0, 0, 3, 0, 0, 0, 0, 1, 0, 9, 0, 0, 0, 0, 3, 0, 0, 4,
            0, 0, 2, 0, 0, 0, 5, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 6, 0, 0, 0,
        ]
        .map(|v| match v {
            0 => None,
            a => Some(non_zero(a)),
        });
        Game {
            sudoku: Sudoku::new(example),
            is_paused: true,
            is_complete: false,
            time_to_solve: None,
            update_nanoseconds: 100_000_000,
            selected_heatmap: Heatmap::InvalidAttempts,
            heatmaps: combo_box::State::new(vec![
                Heatmap::Backtracked,
                Heatmap::Visited,
                Heatmap::InvalidAttempts,
            ]),
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        let index = self.sudoku.index().unwrap_or(usize::MAX);

        let (max, heatmap) = match self.selected_heatmap {
            Heatmap::Backtracked => (
                *self
                    .sudoku
                    .backtracked_heatmap
                    .iter()
                    .max()
                    .unwrap()
                    .max(&1),
                &self.sudoku.backtracked_heatmap,
            ),
            Heatmap::Visited => (
                *self
                    .sudoku
                    .visited_cells_heatmap
                    .iter()
                    .max()
                    .unwrap()
                    .max(&1),
                &self.sudoku.visited_cells_heatmap,
            ),
            Heatmap::InvalidAttempts => (
                *self
                    .sudoku
                    .invalid_attempts_heatmap
                    .iter()
                    .max()
                    .unwrap()
                    .max(&1),
                &self.sudoku.invalid_attempts_heatmap,
            ),
        };

        let current_border_color = if self.sudoku.is_valid {
            Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            }
        } else {
            Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            }
        };

        let rows = self
            .sudoku
            .cells
            .chunks_exact(9)
            .enumerate()
            .map(|(y, r)| {
                let values = r
                    .iter()
                    .map(|s| s.map_or(" ".to_string(), |v| v.to_string()))
                    .enumerate()
                    .map(|(x, c)| {
                        let cell_index = y * 9 + x;
                        container(text(c))
                            .width(50)
                            .height(50)
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                            .style(move |_theme| container::Style {
                                background: Some(iced::Background::Color(Color::from_rgba(
                                    1.0,
                                    0.0,
                                    0.0,
                                    heatmap[cell_index] as f32 / max as f32,
                                ))),
                                border: Border {
                                    color: if cell_index == index {
                                        current_border_color
                                    } else {
                                        Color::BLACK
                                    },
                                    width: 1.0,
                                    radius: 4.0.into(),
                                },
                                ..Default::default()
                            })
                            .into()
                    })
                    .collect::<Vec<_>>();

                return row(values);
            })
            .fold(Column::new(), |column, r| column.push(r));

        let buttons = row![
            button("solve").on_press(Message::Solve),
            button("pause/start").on_press(Message::TogglePause),
            button("reset").on_press(Message::Reset),
            button("next").on_press_maybe(if self.is_paused && !self.is_complete {
                Some(Message::Next)
            } else {
                None
            }),
            combo_box(
                &self.heatmaps,
                "Select",
                Some(&self.selected_heatmap),
                Message::UpdateHeatmap
            ),
            text_input("Update every: ", &self.update_nanoseconds.to_string())
                .on_input(Message::UpdateSpeed)
        ];
        column![
            rows,
            buttons,
            text(format!("Steps: {}", self.sudoku.step_count))
                .color(Color::from_str("#50C6B2").unwrap()),
            text(format!(
                "Invalid attempts: {}",
                self.sudoku.invalid_attempts
            ))
            .color(Color::from_str("#17CCEF").unwrap()),
            text(format!("Backtracked: {}", self.sudoku.backtracking_count))
                .color(Color::from_str("#C2C461").unwrap()),
            text(format!(
                "Finished in: {} seconds",
                self.time_to_solve
                    .map(|t| t.as_secs_f32().to_string())
                    .unwrap_or("-".to_string())
            ))
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Next => {
                if !self.is_complete {
                    match self.sudoku.next() {
                        Some(_) => self.is_complete = true,
                        None => (),
                    }
                }
            }
            Message::Solve => {
                self.sudoku.reset();
                let timer = Instant::now();
                self.sudoku.solve();
                self.time_to_solve = Some(timer.elapsed());
                self.is_complete = true;
            }
            Message::Reset => {
                self.sudoku.reset();
                self.is_complete = false;
                self.time_to_solve = None;
                self.is_paused = true;
            }
            Message::TogglePause => self.is_paused = !self.is_paused,
            Message::UpdateSpeed(v) => {
                if let Ok(m) = u128::from_str(&v) {
                    self.update_nanoseconds = m;
                }
            }
            Message::UpdateHeatmap(heatmap) => self.selected_heatmap = heatmap,
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        if self.is_paused || self.is_complete {
            Subscription::none()
        } else {
            iced::time::every(Duration::from_nanos_u128(self.update_nanoseconds))
                .map(|_| Message::Next)
        }
    }
}

const fn non_zero(value: u8) -> NonZero<u8> {
    match NonZero::new(value) {
        Some(v) => v,
        None => unreachable!(),
    }
}
