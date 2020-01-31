//! Example that just prints out all the input events.

use ggez;

use ggez::event::{self, Axis, Button, GamepadId, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, DrawMode};
use ggez::input;
use ggez::{Context, GameResult};
use std::time::{Duration, Instant};

// Here we're defining how many quickly we want our game to update. This will be
// important later so that we don't have our snake fly across the screen because
// it's moving a full tile every frame.
const UPDATES_PER_SECOND: f32 = 8.0;
// And we get the milliseconds of delay that this update rate corresponds to.
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

struct MyPoint {
    x: f32,
    y: f32,
}

struct Board {
    width: f32,
    height: f32,
    points: Vec::<(f32, f32)>,
    lines: Vec::<(f32, f32, f32, f32)>,
}

impl Board {
    fn new() -> Board {
        Board {
            width: 4.0,
            height: 4.0,
            points: Vec::<(f32, f32)>::new(),
            lines: Vec::<(f32, f32, f32, f32)>::new(),
        }
    }

    fn distance(&self, x: f32, y: f32, x1: f32, y1: f32) -> f32 {
        ((x - x1) * (x - x1) + (y - y1) * (y - y1)).sqrt()
    }

    fn set_up(&mut self) {
        let start_x: f32 = 50.0;
        let step_x = 500.0 / (self.width - 1.0);
        let start_y: f32 = 50.0;
        let step_y = 500.0 / (self.height - 1.0);

        for i in 0..self.height as u32 {
            for j in 0..self.width as u32 {
                self.points.push((start_x + (j as f32) * step_x, start_y + (i as f32) * step_y));
            }
        }
    }
}

struct GameState {
    /// First we need a Snake
    board: Board,
    /// Whether the game is over or not
    gameover: bool,
    /// And we track the last time we updated so that we can limit
    /// our update rate.
    last_update: Instant,
}

impl GameState {
    /// Our new function will set up the initial state of our game.
    pub fn new() -> Self {
        let mut board = Board::new();
        board.set_up();

        GameState {
            board: board,
            gameover: false,
            last_update: Instant::now(),
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            // Then we check to see if the game is over. If not, we'll update. If so, we'll just do nothing.
            println!("Update");
            if !self.gameover {
                // board should be updated here
            }
            // If we updated, we set our last_update to be now
            self.last_update = Instant::now();
        }
        // Finally we return `Ok` to indicate we didn't run into any errors
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("Lines length: {}", self.board.lines.len());
        graphics::clear(ctx, [0.4, 0.1, 1.0, 1.0].into());
        // Then we tell the snake and the food to draw themselves
        for point in &self.board.points {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                ggez::nalgebra::Point2::new(point.0, point.1),
                10.0,
                1.0,
                graphics::BLACK,
            )?;
            graphics::draw(ctx, &circle, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
        }

        for line in &self.board.lines {
            let (origin, dest) = (ggez::nalgebra::Point2::new(line.0, line.1), ggez::nalgebra::Point2::new(line.2, line.3));
            let line = graphics::Mesh::new_line(ctx, &[origin, dest], 5.0, graphics::BLACK)?;
            graphics::draw(ctx, &line, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
        }
        graphics::present(ctx)?;
        // We yield the current thread until the next update
        ggez::timer::yield_now();
        // And return success.
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, xrel: f32, yrel: f32) {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            ggez::input::mouse::set_cursor_type(_ctx, ggez::input::mouse::MouseCursor::Default);
            println!("Points length: {}", &self.board.points.len());
            let mut diffX = 100.0;
            let mut diffY = 100.0;
            let mut isHorizontal = false;
            let mut isVertical = false;

            for point in &self.board.points {
                diffX = (point.0 - x).abs();
                diffY = (point.1 - y).abs();
                
                if(diffX < 3.0) {
                    isVertical = true;
                }

                if(diffY < 3.0) {
                    isHorizontal = true;
                }
            }
            
            if isVertical || isHorizontal {
                ggez::input::mouse::set_cursor_type(_ctx, ggez::input::mouse::MouseCursor::Hand);
                let mut firstClosest: (f32, f32) = (10000.0, 10000.0);
                let mut secondClosest: (f32, f32) = (10000.0, 10000.0);

                for point in &self.board.points {
                    let distance = self.board.distance(x, y, point.0, point.1);

                    if(distance < self.board.distance(x, y, firstClosest.0, firstClosest.1)) {
                        firstClosest = (point.0, point.1);
                    } else if distance < self.board.distance(x, y, secondClosest.0, secondClosest.1) {
                        secondClosest = (point.0, point.1);
                    }
                }

                println!("Points found are {} {} and {} {}", firstClosest.0, firstClosest.1, secondClosest.0, secondClosest.1);
                if !self.board.lines.contains(&(firstClosest.0, firstClosest.1, secondClosest.0, secondClosest.1)) {
                    self.board.lines.push((firstClosest.0, firstClosest.1, secondClosest.0, secondClosest.1));
                }
            }

            if isVertical {
                println!("vertical detected");
            } else if isHorizontal {
                println!("horizontal detected");
            }
        }
    }

    // fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: f32, y: f32) {
    //     println!("Mousewheel event, x: {}, y: {}", x, y);
    // }

    // fn key_down_event(
    //     &mut self,
    //     _ctx: &mut Context,
    //     keycode: KeyCode,
    //     keymod: KeyMods,
    //     repeat: bool,
    // ) {
    //     println!(
    //         "Key pressed: {:?}, modifier {:?}, repeat: {}",
    //         keycode, keymod, repeat
    //     );
    // }

    // fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, keymod: KeyMods) {
    //     println!("Key released: {:?}, modifier {:?}", keycode, keymod);
    // }

    // fn text_input_event(&mut self, _ctx: &mut Context, ch: char) {
    //     println!("Text input: {}", ch);
    // }

    // fn gamepad_button_down_event(&mut self, _ctx: &mut Context, btn: Button, id: GamepadId) {
    //     println!("Gamepad button pressed: {:?} Gamepad_Id: {:?}", btn, id);
    // }

    // fn gamepad_button_up_event(&mut self, _ctx: &mut Context, btn: Button, id: GamepadId) {
    //     println!("Gamepad button released: {:?} Gamepad_Id: {:?}", btn, id);
    // }

    // fn gamepad_axis_event(&mut self, _ctx: &mut Context, axis: Axis, value: f32, id: GamepadId) {
    //     println!(
    //         "Axis Event: {:?} Value: {} Gamepad_Id: {:?}",
    //         axis, value, id
    //     );
    // }

    // fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
    //     if gained {
    //         println!("Focus gained");
    //     } else {
    //         println!("Focus lost");
    //     }
    // }
}

pub fn main() -> GameResult {
    // let setup = ggez::conf::WindowSetup {
    //     title: "Dots and boxes",
    //     icon: "".to_owned(),
    //     resizable: false,
    //     samples: NumSamples::One,
    // };

    let cb = ggez::ContextBuilder::new("dots and boxes", "dasd")
    .window_setup(
        ggez::conf::WindowSetup::default().title("Dots and boxes")
    )
    .window_mode(
        ggez::conf::WindowMode {
            width: 600.0,
            height: 600.0,
            resizable: true,
            ..Default::default()
        }
    );
    let (ctx, event_loop) = &mut cb.build()?;

    let board = &mut Board::new();
    let gameState = &mut GameState::new();
    event::run(ctx, event_loop, gameState)
}

fn resize_event(ctx: &mut Context, width: f32, height: f32) {
    let window_mode = ggez::conf::WindowMode {
        width,
        height,
        resizable: true,
        ..Default::default()
    };
    graphics::set_mode(ctx, window_mode);
    graphics::set_screen_coordinates(ctx, graphics::Rect{x: 0.0, y: 0.0, w: width, h: height}).unwrap();
}