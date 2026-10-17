use core::sudoku::Sudoku;
use std::{fmt::Display, num::NonZero, str::FromStr, time::Instant};

use data::{SUDOKU_EXAMPLE_1, SUDOKU_EXAMPLE_2, ToNonZero, norwig1, norwig2};
use iced::{
    Border, Color, Length, Subscription,
    alignment::{Horizontal, Vertical},
    widget::{Column, button, column, combo_box, container, row, text, text_input},
};

use iced::time::Duration;

#[derive(Clone)]
enum Message {
    Next,
    TogglePause,
    Solve,
    Reset,
    UpdateSpeed(String),
    UpdateHeatmap(Heatmap),
    UpdateSudoku(SudokuExample),
    ClearHeatmap,
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

impl Display for SudokuExample {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone)]
struct SudokuExample {
    pub name: String,
    pub value: [Option<NonZero<u8>>; 81],
}

struct Game {
    sudoku: Sudoku,
    is_paused: bool,
    is_complete: bool,
    time_to_solve: Option<Duration>,
    update_nanoseconds: u128,
    selected_heatmap: Option<Heatmap>,
    heatmaps: combo_box::State<Heatmap>,
    selected_sudoku: SudokuExample,
    sudoku_options: combo_box::State<SudokuExample>,
}

fn main() -> iced::Result {
    iced::application(Game::new, Game::update, Game::view)
        .subscription(Game::subscription)
        .run()
}

impl Game {
    pub fn new() -> Game {
        let mut sudoku_options: Vec<SudokuExample> = vec![
            SudokuExample {
                value: SUDOKU_EXAMPLE_1.to_non_zero(),
                name: "example 1".to_string(),
            },
            SudokuExample {
                value: SUDOKU_EXAMPLE_2.to_non_zero(),
                name: "example 2".to_string(),
            },
        ];

        for (i, v) in norwig1().iter().enumerate() {
            sudoku_options.push(SudokuExample {
                value: *v,
                name: format!("norwig 1 - {}", i),
            });
        }

        for (i, v) in norwig2().iter().enumerate() {
            sudoku_options.push(SudokuExample {
                value: *v,
                name: format!("norwig 2 - {}", i),
            });
        }

        let selected_sudoku = sudoku_options.first().unwrap().clone();

        Game {
            sudoku: Sudoku::new(selected_sudoku.value),
            is_paused: true,
            is_complete: false,
            time_to_solve: None,
            update_nanoseconds: 1_000_000_000,
            selected_heatmap: None,
            heatmaps: combo_box::State::new(vec![
                Heatmap::Backtracked,
                Heatmap::Visited,
                Heatmap::InvalidAttempts,
            ]),
            sudoku_options: combo_box::State::new(sudoku_options),
            selected_sudoku,
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        let sudoku = self.sudoku_view();
        let controls = self.controls_view();
        let results = self.resuts_view();

        column![sudoku, controls, results].spacing(10).padding(10)
    }

    pub fn sudoku_view(&self) -> Column<'_, Message> {
        let index = self.sudoku.index().unwrap_or(usize::MAX);

        let heatmap = self.heatmap();
        let current_border_color = self.sudoku_border_color();

        self.sudoku
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
                                background: match heatmap {
                                    Some(v) => Some(iced::Background::Color(Color::from_rgba(
                                        1.0,
                                        0.0,
                                        0.0,
                                        v.1[cell_index] as f32 / v.0 as f32,
                                    ))),
                                    None => None,
                                },
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
            .fold(Column::new(), |column, r| column.push(r))
    }

    fn sudoku_border_color(&self) -> Color {
        if self.sudoku.is_valid {
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
        }
    }

    fn heatmap(&self) -> Option<(usize, &[usize; 81])> {
        match self.selected_heatmap {
            None => None,
            Some(Heatmap::Backtracked) => Some((
                *self
                    .sudoku
                    .backtracked_heatmap
                    .iter()
                    .max()
                    .unwrap()
                    .max(&1),
                &self.sudoku.backtracked_heatmap,
            )),
            Some(Heatmap::Visited) => Some((
                *self
                    .sudoku
                    .visited_cells_heatmap
                    .iter()
                    .max()
                    .unwrap()
                    .max(&1),
                &self.sudoku.visited_cells_heatmap,
            )),
            Some(Heatmap::InvalidAttempts) => Some((
                *self
                    .sudoku
                    .invalid_attempts_heatmap
                    .iter()
                    .max()
                    .unwrap()
                    .max(&1),
                &self.sudoku.invalid_attempts_heatmap,
            )),
        }
    }

    pub fn controls_view(&self) -> Column<'_, Message> {
        let toggle_icon = if self.is_paused { "▶" } else { "Ⅱ" };
        column![
            row![
                button("solve").on_press(Message::Solve),
                button(
                    container(text(toggle_icon))
                        .width(Length::Fill)
                        .align_x(Horizontal::Center)
                )
                .width(40)
                .on_press(Message::TogglePause),
                button("reset").on_press(Message::Reset),
                button("next").on_press_maybe(if self.is_paused && !self.is_complete {
                    Some(Message::Next)
                } else {
                    None
                }),
            ]
            .spacing(10),
            row![
                column![
                    text("Heatmap").size(12),
                    row![
                        combo_box(
                            &self.heatmaps,
                            "Select",
                            self.selected_heatmap.as_ref(),
                            Message::UpdateHeatmap
                        ),
                        button(
                            container(text("×"))
                                .width(Length::Fill)
                                .align_x(Horizontal::Center)
                        )
                        .width(40)
                        .on_press(Message::ClearHeatmap)
                        .style(|_theme, _status| {
                            button::Style {
                                background: Some(iced::Background::Color(iced::Color::from_rgb8(
                                    83, 49, 56,
                                ))),
                                text_color: iced::Color::from_rgb8(255, 190, 185),
                                ..button::Style::default()
                            }
                        })
                    ]
                    .spacing(4)
                ],
                column![
                    text("Sudoku").size(12),
                    combo_box(
                        &self.sudoku_options,
                        "Select",
                        Some(&self.selected_sudoku),
                        Message::UpdateSudoku
                    )
                ],
                column![
                    text("Step every (nanoseconds)").size(12),
                    text_input("Update every: ", &self.update_nanoseconds.to_string())
                        .on_input(Message::UpdateSpeed),
                    text(format!(
                        "seconds {}",
                        (self.update_nanoseconds as f64) / 1_000_000_000.0
                    ))
                    .size(12),
                ]
            ]
            .spacing(8)
        ]
        .spacing(10)
    }

    pub fn resuts_view(&self) -> Column<'_, Message> {
        column![
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
            Message::UpdateHeatmap(heatmap) => self.selected_heatmap = Some(heatmap),
            Message::ClearHeatmap => self.selected_heatmap = None,
            Message::UpdateSudoku(sudoku_example) => {
                self.is_paused = true;
                self.selected_sudoku = sudoku_example;
                self.is_complete = false;
                self.time_to_solve = None;
                self.sudoku = Sudoku::new(self.selected_sudoku.value);
            }
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
