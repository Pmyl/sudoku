use core::sudoku::Sudoku;
use std::{num::NonZero, str::FromStr};

use iced::{
    Border, Color, Subscription,
    alignment::{Horizontal, Vertical},
    widget::{Column, button, column, container, row, text},
};

use iced::time::Duration;

#[derive(Debug, Clone)]
pub enum Message {
    Next,
    TogglePause,
    Solve,
}

struct Game {
    sudoku: Sudoku,
    is_paused: bool,
    is_complete: bool,
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
            is_paused: false,
            is_complete: false,
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        let index = self.sudoku.index().unwrap_or(usize::MAX);
        let current_background_color = if self.sudoku.is_valid {
            iced::Background::Color(Color {
                r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0,
            })
        } else {
            iced::Background::Color(Color {
                r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0,
            })
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
                        container(text(c))
                            .width(50)
                            .height(50)
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                            .style(move |_theme| container::Style {
                                background: if y * 9 + x == index {
                                    Some(current_background_color)
                                } else {
                                    None
                                },
                                border: Border {
                                    color: Color::BLACK,
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
            button("solve").on_press_maybe(if self.is_complete {
                None
            } else {
                Some(Message::Solve)
            }),
            button("pause/start").on_press(Message::TogglePause),
            button("next").on_press_maybe(if self.is_paused && !self.is_complete {
                Some(Message::Next)
            } else {
                None
            }),
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
                self.is_complete = true;
                self.sudoku.solve();
            }
            Message::TogglePause => self.is_paused = !self.is_paused,
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        if self.is_paused || self.is_complete {
            Subscription::none()
        } else {
            iced::time::every(Duration::from_millis(100)).map(|_| Message::Next)
        }
    }
}

const fn non_zero(value: u8) -> NonZero<u8> {
    match NonZero::new(value) {
        Some(v) => v,
        None => unreachable!(),
    }
}
