use core::sudoku::Sudoku;
use std::num::NonZero;

use iced::{
    Border, Color,
    alignment::{Horizontal, Vertical},
    widget::{Column, button, column, container, row, text},
};

use iced::time::{self, Duration, Instant};

#[derive(Debug, Clone)]
pub enum Message {
    Next,
}

struct Game {
    sudoku: Sudoku,
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
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        let rows = self
            .sudoku
            .cells
            .chunks_exact(9)
            .map(|r| {
                let values = r
                    .iter()
                    .map(|s| s.map_or(" ".to_string(), |v| v.to_string()))
                    .map(|x| {
                        container(text(x))
                            .width(50)
                            .height(50)
                            .align_x(Horizontal::Center)
                            .align_y(Vertical::Center)
                            .style(|_theme| container::Style {
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

        column![rows, button("start")]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Next => {
                self.sudoku.next();
            }
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::time::every(Duration::from_millis(20)).map(|_| Message::Next)
    }
}

const fn non_zero(value: u8) -> NonZero<u8> {
    match NonZero::new(value) {
        Some(v) => v,
        None => unreachable!(),
    }
}
