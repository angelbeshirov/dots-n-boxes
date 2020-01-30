//! Example that just prints out all the input events.

use ggez;

use ggez::event::{self, Axis, Button, GamepadId, KeyCode, KeyMods, MouseButton};
use ggez::graphics::{self, DrawMode};
use ggez::input;
use ggez::{Context, GameResult};

struct Board {
    width: f32,
    height: f32,
}

impl Board {
    fn new() -> Board {
        Board {
            width: 3.0,
            height: 3.0,
        }
    }
}

struct MainState {
    pos_x: f32,
    pos_y: f32,
    mouse_down: bool,
}

impl MainState {
    fn new() -> MainState {
        MainState {
            pos_x: 100.0,
            pos_y: 100.0,
            mouse_down: false,
        }
    }
}

/**
50
175
300
425
**/

impl event::EventHandler for Board {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if input::keyboard::is_key_pressed(ctx, KeyCode::A) {
            println!("The A key is pressed");
            if input::keyboard::is_mod_active(ctx, input::keyboard::KeyMods::SHIFT) {
                println!("The shift key is held too.");
            }
            println!(
                "Full list of pressed keys: {:?}",
                input::keyboard::pressed_keys(ctx)
            );
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut points = Vec::<(f32, f32)>::new();
        graphics::clear(ctx, [0.4, 0.1, 1.0, 1.0].into());

        let start_x: f32 = 50.0;
        let step_x = 500.0 / (self.width - 1.0);
        let start_y: f32 = 50.0;
        let step_y = 500.0 / (self.height - 1.0);

        for i in 0..self.height as u32 {
            for j in 0..self.width as u32 {
                println!("{}", (start_x + (i as f32) * step_x));
                points.push((start_x + (j as f32) * step_x, start_y + (i as f32) * step_y));
            
                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    ggez::nalgebra::Point2::new(start_x + (j as f32) * step_x, start_y + (i as f32) * step_y),
                    10.0,
                    1.0,
                    graphics::BLACK,
                )?;
                graphics::draw(ctx, &circle, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
            }
        }
        let (origin, dest) = (ggez::nalgebra::Point2::new(points[0].0, points[0].1), ggez::nalgebra::Point2::new(points[3].0, points[3].1));
        let line = graphics::Mesh::new_line(ctx, &[origin, dest], 8.0, graphics::BLACK)?;
        graphics::draw(ctx, &line, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
        //graphics::line(ctx, &[ggez::nalgebra::Point2::new(100.0, 100.0), ggez::nalgebra::Point2::new(200.0, 200.0)])?;
        graphics::present(ctx)?;
        Ok(())
    }

    // fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
    //     self.mouse_down = true;
    //     println!("Mouse button pressed: {:?}, x: {}, y: {}", button, x, y);
    // }

    // fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
    //     self.mouse_down = false;
    //     println!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    // }

    // fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, xrel: f32, yrel: f32) {
    //     if self.mouse_down {
    //         self.pos_x = x;
    //         self.pos_y = y;
    //     }
    //     println!(
    //         "Mouse motion, x: {}, y: {}, relative x: {}, relative y: {}",
    //         x, y, xrel, yrel
    //     );
    // }

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
    event::run(ctx, event_loop, board)
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