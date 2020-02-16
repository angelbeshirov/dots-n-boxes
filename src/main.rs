use ggez;

use ggez::event::{KeyCode, KeyMods};
use ggez::event::{self, MouseButton};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::graphics::{self};
use ggez::{Context, GameResult, ContextBuilder, timer};
use std::time::{Duration, Instant};

use dotsnboxes::entities::{State, Player, Board, MinMax, MainMenu, 
    EndMenu, WINDOW_WIDTH, WINDOW_HEIGHT, WIDTH, HEIGHT, X_INITIAL_OFFSET, Y_INITIAL_OFFSET};

const UPDATES_PER_SECOND: f32 = 10.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

struct GameState {
    board: Board,
    last_update: Instant,
    main_menu: MainMenu,
    end_menu: EndMenu,
    mode: State,
    next: Player,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        GameState {
            board: Board::new(WIDTH, HEIGHT, WINDOW_WIDTH, WINDOW_HEIGHT, X_INITIAL_OFFSET, Y_INITIAL_OFFSET),
            last_update: Instant::now(),
            main_menu: MainMenu::new(ctx).unwrap(),
            end_menu: EndMenu::new(ctx).unwrap(),
            mode: State::None,
            next: Player::Player1,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.4, 0.1, 1.0, 1.0].into());

        if self.mode == State::None {
            self.main_menu.draw(ctx)?;
        } else if self.mode == State::OnePlayer || self.mode == State::TwoPlayers {
            self.board.draw(ctx, self.next)?;
        } else if self.mode == State::GameOver {
            self.end_menu.draw(ctx, self.board.get_marked_by_player_1().len() as u8, self.board.get_marked_by_player_2().len() as u8)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            ggez::input::mouse::set_cursor_type(_ctx, ggez::input::mouse::MouseCursor::Default);
            if self.mode == State::OnePlayer || self.mode == State::TwoPlayers || 
                    (self.mode == State::None && (self.main_menu.is_on_one_player_entry(x, y) || self.main_menu.is_on_two_player_entry(x, y))) ||
                    (self.mode == State::GameOver && self.end_menu.is_on_restart(x, y)) {
                ggez::input::mouse::set_cursor_type(_ctx, ggez::input::mouse::MouseCursor::Hand);
            }
            
            self.board.update_line(self.next, x, y);
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        if self.mode == State::None {
            if self.main_menu.is_on_one_player_entry(_x, _y) {
                self.mode = State::OnePlayer;
            } else if self.main_menu.is_on_two_player_entry(_x, _y) {
                self.mode = State::TwoPlayers;
            }
        } else if self.mode == State::OnePlayer {
            if !self.board.contains_line(&self.board.get_temp_line()) {
                let previous_score = self.board.get_marked_by_player_1().len();

                self.board.add_line(self.board.get_temp_line());
                self.board.update_squares(Player::Player1);
    
                if self.board.get_marked_by_player_1().len() == previous_score {
                    let mut previous_score = self.board.get_marked_by_player_2().len();

                    let computer_move = MinMax::alphabeta(&self.board, 6, std::i32::MIN, std::i32::MAX, true);
                    self.board = computer_move.0;
                    let mut current_score = self.board.get_marked_by_player_2().len();
    
                    while previous_score != current_score {
                        previous_score = current_score;
                        let computer_move = MinMax::alphabeta(&self.board, 6, std::i32::MIN, std::i32::MAX, true);
                        self.board = computer_move.0;
                        current_score = self.board.get_marked_by_player_2().len();
                    }
                }

                if self.board.is_complete() {
                    self.mode = State::GameOver;
                }
            }
        } else if self.mode == State::TwoPlayers {
            if !self.board.contains_line(&self.board.get_temp_line()) {
                if self.next == Player::Player1 {
                    let previous = self.board.get_marked_by_player_1().len();
                    self.board.add_line(self.board.get_temp_line());
                    self.board.update_squares(Player::Player1);

                    if self.board.get_marked_by_player_1().len() == previous {
                        self.next = Player::Player2;
                    }
                } else if self.next == Player::Player2 {
                    let previous = self.board.get_marked_by_player_2().len();
                    self.board.add_line(self.board.get_temp_line());
                    self.board.update_squares(Player::Player2);

                    if self.board.get_marked_by_player_2().len() == previous {
                        self.next = Player::Player1;
                    }
                }
            }

            if self.board.is_complete() {
                self.mode = State::GameOver;
            }
        } else if self.mode == State::GameOver {

            if self.end_menu.is_on_restart(_x, _y) {
                self.mode = State::None;
                self.board = Board::new(WIDTH, HEIGHT, WINDOW_WIDTH, WINDOW_HEIGHT, X_INITIAL_OFFSET, Y_INITIAL_OFFSET);
                self.next = Player::Player1;
            }
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            event::KeyCode::Escape => event::quit(_ctx),
            _ => (),
        }
    }
}

pub fn main() {
    let cb = ContextBuilder::new("dots-n-boxes", "Angel Beshirov")
    .window_setup(
        WindowSetup::default().title("Dots and boxes")
    )
    .window_mode(
        WindowMode {
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            resizable: false,
            ..Default::default()
        }
    );
    let (ctx, event_loop) = &mut cb.build().unwrap();
    let game_state = &mut GameState::new(ctx);

    if let Err(e) = event::run(ctx, event_loop, game_state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly!");
    }
}