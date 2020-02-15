use ggez;

use ggez::event::{self, KeyCode, KeyMods, MouseButton};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::graphics::{self, Color, DrawParam};
use ggez::{Context, GameResult, ContextBuilder};
use std::time::{Duration, Instant};
use graphics::{Font, Text};
use ggez::nalgebra::Point2;

const UPDATES_PER_SECOND: f32 = 8.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

const PLAYER_1: &'static str = "P1";
const PLAYER_2: &'static str = "P2";
const PLAYER: &'static str = "P";
const COMPUTER: &'static str = "C";

const WIDTH: f32 = 3.0;
const HEIGHT: f32 = 3.0;

const WINDOW_WIDTH: f32 = 600.0;
const WINDOW_HEIGHT: f32 = 600.0;

const X_INITIAL_OFFSET: f32 = 60.0;
const Y_INITIAL_OFFSET: f32 = 60.0;

const DELTA: f32 = 0.00001;
const RED: Color = Color::new(255.0, 0.0, 0.0, 255.0);

struct MathOperations {}

impl MathOperations {
    fn distance(x: f32, y: f32, x1: f32, y1: f32) -> f32 {
        ((x - x1) * (x - x1) + (y - y1) * (y - y1)).sqrt()
    }

    fn area(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> f32 { 
        ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2.0).abs()
    }

    fn is_inside_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x: f32, y: f32) -> bool { 
        let a = MathOperations::distance(x1, y1, x2, y2);
        let b = MathOperations::distance(x2, y2, x3, y3);
        let c = MathOperations::distance(x3, y3, x1, y1);

        if a + b <= c || b + c <= a || a + c <= b {
            return false;
        }

        let a = MathOperations::area(x1, y1, x2, y2, x3, y3); 
        let a1 = MathOperations::area(x, y, x2, y2, x3, y3); 
        let a2 = MathOperations::area(x1, y1, x, y, x3, y3); 
        let a3 = MathOperations::area(x1, y1, x2, y2, x, y); 
        (a == (a1 + a2 + a3))
    }

    fn is_inside_rectangle(x: f32, y: f32, x_rectangle: f32, y_rectangle: f32, width: f32, height: f32) -> bool {
        x_rectangle <= x && x_rectangle + width >= x && y_rectangle <= y && y_rectangle + height >= y
    }

    fn are_on_same_line(x: f32, y: f32, points: &[(f32, f32)]) -> bool {
        let mut on_x: u8 = 0;
        let mut on_y: u8 = 0;
        for point in points {
            if (point.0 - x).abs() <= 0.1 {
                on_x = on_x + 1;
            }

            if (point.1 - y).abs() <= 0.1 {
                on_y = on_y + 1;
            }
        }

        on_x >= 2 || on_y >= 2
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SelectedMode {
    OnePlayer, TwoPlayers, None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    Player_1,
    Player_2,
    Dummy,
}

#[derive(Debug, Copy, Clone)]
struct Line {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    marked_by: Player,
}

impl Line {
    fn new(_x1: f32, _y1: f32, _x2: f32, _y2: f32, _marked_by: Player) -> Line {
        Line {
            x1: _x1,
            y1: _y1,
            x2: _x2,
            y2: _y2,
            marked_by: _marked_by,
        }
    }

    fn get_x1(&self) -> f32 {
        self.x1
    }
    
    fn get_y1(&self) -> f32 {
        self.y1
    }

    fn get_x2(&self) -> f32 {
        self.x2
    }

    fn get_y2(&self) -> f32 {
        self.y2
    }

    fn get_marked_by(&self) -> Player {
        self.marked_by
    }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
        ((self.x1 - other.x1).abs() <= DELTA &&
        (self.y1 - other.y1).abs() <= DELTA &&
        (self.x2 - other.x2).abs() <= DELTA &&
        (self.y2 - other.y2).abs() <= DELTA) || 
        ((self.x1 - other.x2).abs() <= DELTA &&
        (self.y1 - other.y2).abs() <= DELTA &&
        (self.x2 - other.x1).abs() <= DELTA &&
        (self.y2 - other.y1).abs() <= DELTA)
    }
}

#[derive(Debug, Copy, Clone)]
struct Square {
    line1: Line,
    line2: Line,
    line3: Line,
    line4: Line,
}

impl Square {
    fn new(_line1: Line, _line2: Line, _line3: Line, _line4: Line) -> Square {
        Square {
            line1: _line1,
            line2: _line2,
            line3: _line3,
            line4: _line4,
        }
    }

    fn get_line1(&self) -> Line {
        self.line1
    }

    fn get_smallest(&self) -> (f32, f32) {
        let mut smallest_x = 10000.0; // TODO fix
        let mut smallest_y = 10000.0;
        let container = vec![self.line1, self.line2, self.line3, self.line4];

        for x in container {
            if x.get_x1() <= smallest_x {
                smallest_x = x.get_x1();
            }

            if x.get_x2() <= smallest_x {
                smallest_x = x.get_x2();
            }

            if x.get_y1() <= smallest_y {
                smallest_y = x.get_y1();
            }

            if x.get_y2() <= smallest_y {
                smallest_y = x.get_y2();
            }
        }

        (smallest_x, smallest_y)
    }
}


impl PartialEq  for Square {
    fn eq(&self, other: &Self) -> bool {
        (self.line1 == other.line1 || self.line1 == other.line2 || self.line1 == other.line3 || self.line1 == other.line4) &&
        (self.line2 == other.line1 || self.line2 == other.line2 || self.line2 == other.line3 || self.line2 == other.line4) &&
        (self.line3 == other.line1 || self.line3 == other.line2 || self.line3 == other.line3 || self.line3 == other.line4) &&
        (self.line4 == other.line1 || self.line4 == other.line2 || self.line4 == other.line3 || self.line4 == other.line4)
    }
}

impl Eq for Square {}

#[derive(Debug, Clone)]
struct Board {
    width: f32,
    height: f32,
    points: Vec::<(f32, f32)>,
    lines: Vec::<Line>,
    temp_line: Line,
    start_x: f32,
    step_x: f32,
    start_y: f32, 
    step_y: f32,
    squares: Vec::<Square>,
    marked_squares_by_player_1: Vec::<Square>,
    marked_squares_by_player_2: Vec::<Square>,
}

impl Board {
    fn new() -> Board {
        Board {
            width: WIDTH,
            height: HEIGHT,
            points: Vec::<(f32, f32)>::new(),
            lines: Vec::<Line>::new(),
            temp_line: Line::new(0.0, 0.0, 0.0, 0.0, Player::Dummy),
            start_x: X_INITIAL_OFFSET,
            step_x: (WINDOW_WIDTH - 2.0 * X_INITIAL_OFFSET) / (WIDTH - 1.0),
            start_y: Y_INITIAL_OFFSET,
            step_y: (WINDOW_HEIGHT - 2.0 * Y_INITIAL_OFFSET) / (HEIGHT - 1.0),
            squares: Vec::new(),
            marked_squares_by_player_1: Vec::new(), // player_1 will always be you
            marked_squares_by_player_2: Vec::new(), // player_2 will be either player_2 or the computer
        }
    }

    fn get_marked_by_player_1(&self) -> &[Square] {
        &self.marked_squares_by_player_1
    }

    fn get_marked_by_player_2(&self) -> &[Square] {
        &self.marked_squares_by_player_2
    }

    fn get_step_x(&self) -> f32 {
        self.step_x
    }

    fn get_step_y(&self) -> f32 {
        self.step_y
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn get_lines(&self) -> Vec<Line> {
        self.lines.clone()
    }

    fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    fn set_up(&mut self) {
        for i in 0..self.height as u32 {
            for j in 0..self.width as u32 {
                self.points.push((self.start_x + (j as f32) * self.step_x, self.start_y + (i as f32) * self.step_y));
            }
        }

        for i in 0..(self.height - 1.0) as u32 {
            for j in 0..(self.width - 1.0) as u32 {
                let point = self.points[(i * self.height as u32 + j) as usize];
                let line1 = Line::new(point.0, point.1,point.0 + self.step_x, point.1, Player::Dummy);
                let line2 = Line::new(point.0 + self.step_x, point.1,point.0 + self.step_x, point.1 + self.step_y, Player::Dummy);
                let line3 = Line::new(point.0 + self.step_x, point.1 + self.step_y,point.0, point.1 + self.step_y, Player::Dummy);
                let line4 = Line::new(point.0, point.1 + self.step_y,point.0, point.1, Player::Dummy);

                self.squares.push(Square::new(line1, line2, line3, line4));
            }
        }
    }

    fn is_complete(&self) -> bool {
        self.get_marked_by_player_1().len() + self.get_marked_by_player_2().len() == (self.width - 1.0) as usize * (self.height - 1.0) as usize
    }

    fn update_squares(&mut self, player: Player) {
        let lines_len = self.lines.len();
        for i in 0..lines_len {
            for j in (i + 1)..lines_len {
                for t in (j + 1)..lines_len {
                    for k in (t + 1)..lines_len { // TODO how to improve
                        let potential_square = Square::new(self.lines[i], self.lines[j], self.lines[t], self.lines[k]);

                        if player == Player::Player_2 {
                            if self.squares.contains(&potential_square) && !self.marked_squares_by_player_1.contains(&potential_square) && 
                                !self.marked_squares_by_player_2.contains(&potential_square) {
                                self.marked_squares_by_player_2.push(potential_square);
                            }
                        } else if player == Player::Player_1 {
                            if self.squares.contains(&potential_square) && !self.marked_squares_by_player_1.contains(&potential_square) && 
                                !self.marked_squares_by_player_2.contains(&potential_square) {
                                self.marked_squares_by_player_1.push(potential_square);
                            }
                        }
                    }
                }
            }
        }
    }
}

struct MinMax {

}

impl MinMax {

    fn new() -> MinMax {
        MinMax {

        }
    }

    fn get_children(&self, board: &Board, player: Player) -> Vec<Board> {
        let mut children: Vec<Board> = Vec::new();
        let start = (X_INITIAL_OFFSET, Y_INITIAL_OFFSET);

        for j in 0..board.get_height() as usize {
            for i in 0..(board.get_width() - 1.0) as usize {
                let line = Line::new(
                start.0 + i as f32 * board.get_step_x(), 
                start.1 + j as f32 * board.get_step_y(), 
                start.0 + board.get_step_x() + i as f32 * board.get_step_x(), 
                start.1 + j as f32 * board.get_step_y(), player);
                if !board.get_lines().contains(&line) {
                    let mut cloned = board.clone();
                    cloned.add_line(line);
                    cloned.update_squares(player.clone());
                    // TODO score should be updated here
                    children.push(cloned);
                }
            }
        }

        for i in 0..(board.get_height() - 1.0) as usize {
            for j in 0..(board.get_width()) as usize {
                let line = Line::new(
                start.0 + j as f32 * board.get_step_x(), 
                start.1 + i as f32 * board.get_step_y(), 
                start.0 + j as f32 * board.get_step_x(), 
                start.1 + board.get_step_y() + i as f32 * board.get_step_y(), player);
                if !board.get_lines().contains(&line) {
                    let mut cloned = board.clone();
                    cloned.add_line(line);
                    cloned.update_squares(player.clone());
                    // TODO score should be updated here
                    children.push(cloned);
                }
            }
        }

        children
    }

    fn alphabeta(&mut self, board: &Board, max_depth: u8, alpha: i32, beta: i32, is_max: bool) -> (Board, i32) {
        if board.is_complete() || max_depth <= 0 {
            //println!("Hit botom for {} {}", board.get_marked_by_computer().len(), is_max);
            //let newBoard = board.clone();
            return (board.clone(), if is_max {
                    board.get_marked_by_player_2().len() as i32
                } else {
                    -(board.get_marked_by_player_1().len() as i32)
                });
        }

        let mut value: i32;
        let mut result: (Board, i32) = (board.clone(), 0); // TODO empty init here
        let parent_score = (board.get_marked_by_player_1().len(), board.get_marked_by_player_2().len());

        if is_max {
            value = std::i32::MIN;
            let children = self.get_children(board, Player::Player_2);
            let mut all_different: bool = true;
            for child in children.clone() {
                if child.get_marked_by_player_2().len() != parent_score.1 {
                    all_different = false;
                }
            }

            for child in children {
                //println!("Child for max is is {}", child.get_marked_by_computer().len());
                let new_value: (Board, i32);
                if child.get_marked_by_player_2().len() != parent_score.1 {
                    new_value = self.alphabeta(&child, max_depth - 1, alpha, beta, true);
                } else {
                    new_value = self.alphabeta(&child, max_depth - 1, alpha, beta, false);
                }

                //println!("Found computer score {}", new_value.1);

                if value < new_value.1 {
                    //println!("In for that score");
                    result = (child, new_value.1);
                    value = new_value.1;
                }

                let alpha = if alpha < value {
                    value 
                } else {
                    alpha
                };

                if alpha >= beta && all_different {
                    break;
                }
            }

            //println!("Result for max is {}", result.1);
        } else {
            value = std::i32::MAX;
            let children = self.get_children(board, Player::Player_1);
            let mut all_different: bool = true;

            for child in children.clone() {
                if child.get_marked_by_player_1().len() != parent_score.0 {
                    all_different = false;
                }
            }

            for child in children {
                let new_value: (Board, i32);
                if child.get_marked_by_player_1().len() != parent_score.0 {
                    new_value = self.alphabeta(&child, max_depth - 1, alpha, beta, true);
                } else {
                    new_value = self.alphabeta(&child, max_depth - 1, alpha, beta, false);
                }

                if value > new_value.1 {
                    result = (child, new_value.1);
                    value = new_value.1;
                }

                let beta = if beta > value {
                    value 
                } else {
                    beta
                };
                
                if alpha >= beta && all_different {
                    break;
                }

                //println!("Child for min is {}", new_value.1);
            }

            //println!("Result for min is {}", result.1);
        }

        //println!("Returning {:?}", result.1);

        result
    }
}

struct MainMenuEntry {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: String,
}

impl MainMenuEntry {
    fn new(_x: f32, _y: f32, _width: f32, _height: f32, _text: String) -> MainMenuEntry {
        MainMenuEntry {
            x: _x,
            y: _y,
            width: _width,
            height: _height,
            text: _text,
        }
    }

    fn get_x(&self) -> f32 {
        self.x
    }
    
    fn get_y(&self) -> f32 {
        self.y
    }

    fn get_width(&self) -> f32 {
        self.width
    }

    fn get_height(&self) -> f32 {
        self.height
    }

    fn get_text(&self) -> &str {
        &self.text
    }
}

struct MainMenu {
    one_player_entry: MainMenuEntry,
    two_player_entry: MainMenuEntry,
}

impl MainMenu {
    fn new(ctx: &mut Context) -> GameResult<MainMenu> {
        let font = Font::new(ctx, "/DejaVuSansMono.ttf")?;
        let mut menu_entries = Vec::<MainMenuEntry>::new();
        let font_size = 40.0;
        let start_y = 600.0 / 3.0;
        let step = 600.0 / 5.0;

        let text_one_player = Text::new(("1 Player", font, font_size));
        let text_two_player = Text::new(("2 Players", font, font_size));

        let text_one_player_width = text_one_player.width(ctx) as f32;
        let text_one_player_height = text_one_player.height(ctx) as f32;

        let text_two_player_width = text_two_player.width(ctx) as f32;
        let text_two_player_height = text_two_player.height(ctx) as f32;

        let x = (600.0 - text_one_player_width) / 2.0;

        let k = MainMenu {
            one_player_entry: MainMenuEntry::new(x, start_y, text_one_player_width, text_one_player_height, String::from("1 Player")),
            two_player_entry: MainMenuEntry::new(x, start_y + step, text_two_player_width, text_two_player_height, String::from("2 Players")),
        };

        Ok(k)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let font = Font::new(ctx, "/DejaVuSansMono.ttf")?;
        let font_size = 40.0;

        // main
        let text = Text::new(("Dots and boxes", font, font_size));
        let text_width = text.width(ctx) as f32;
        graphics::draw(ctx, &text, DrawParam::default()
        .dest(Point2::new((600.0 - text_width) / 2.0, 30.0)))?;

        let text = Text::new((self.one_player_entry.get_text(), font, font_size));
        graphics::draw(ctx, &text, DrawParam::default()
        .dest(Point2::new(self.one_player_entry.get_x(), self.one_player_entry.get_y())))?;

        let text = Text::new((self.two_player_entry.get_text(), font, font_size));
        graphics::draw(ctx, &text, DrawParam::default()
        .dest(Point2::new(self.two_player_entry.get_x(), self.two_player_entry.get_y())))?;
        
        Ok(())
    }

    fn get_one_player_entry(&self) -> &MainMenuEntry {
        &self.one_player_entry
    }

    fn get_two_player_entry(&self) -> &MainMenuEntry {
        &self.two_player_entry
    }
}

struct GameState {
    board: Board,
    min_max: MinMax,
    gameover: bool,
    last_update: Instant,
    main_menu: MainMenu,
    mode: SelectedMode,
    next: Player,
}

impl GameState {
    /// Our new function will set up the initial state of our game.
    pub fn new(ctx: &mut Context) -> Self {
        let mut board = Board::new();
        board.set_up();

        GameState {
            board: board,
            min_max: MinMax::new(),
            gameover: false,
            last_update: Instant::now(),
            main_menu: MainMenu::new(ctx).unwrap(),
            mode: SelectedMode::None,
            next: Player::Player_1,
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            // Then we check to see if the game is over. If not, we'll update. If so, we'll just do nothing.
            //println!("Update");
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
        //println!("Lines length: {}", self.board.lines.len());
        graphics::clear(ctx, [0.4, 0.1, 1.0, 1.0].into());
        // Then we tell the snake and the food to draw themselves


        if self.mode == SelectedMode::None {
            //self.sub_draw(ctx);
            self.main_menu.draw(ctx);
        } else if self.mode == SelectedMode::OnePlayer {
            for point in &self.board.points {
                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Point2::new(point.0, point.1),
                    10.0,
                    1.0,
                    graphics::BLACK,
                )?;
                graphics::draw(ctx, &circle, (Point2::new(0.0, 0.0),))?;
            }
    
            for line in &self.board.lines {
                let (origin, dest) = (Point2::new(line.get_x1(), line.get_y1()), Point2::new(line.get_x2(), line.get_y2()));
                let line = graphics::Mesh::new_line(ctx, &[origin, dest], 5.0, graphics::BLACK)?;
                graphics::draw(ctx, &line, (Point2::new(0.0, 0.0),))?;
            }
    
            if self.board.temp_line.get_x1() != 0.0 {
                let (origin, dest) = (Point2::new(self.board.temp_line.get_x1(), self.board.temp_line.get_y1()), 
                Point2::new(self.board.temp_line.get_x2(), self.board.temp_line.get_y2()));
        
                let line = graphics::Mesh::new_line(ctx, &[origin, dest], 5.0, graphics::BLACK)?;
                graphics::draw(ctx, &line, (Point2::new(0.0, 0.0),))?;
            }
    
            
            for square in self.board.get_marked_by_player_1() {
                let x = square.get_smallest();
                let font = graphics::Font::new(ctx, "/DejaVuSansMono.ttf")?;
                let font_size = MathOperations::distance(square.get_line1().get_x1(), square.get_line1().get_y1(), square.get_line1().get_x2(), square.get_line1().get_y2());
                let text = graphics::Text::new((PLAYER, font, font_size));
                graphics::draw(ctx, &text, DrawParam::default().dest(Point2::new(x.0 + (1.0 / 5.0) * font_size, x.1 + (1.0 / 20.0) * font_size)))?;
            }
    
            for square in self.board.get_marked_by_player_2() {
                let x = square.get_smallest();
                let font = graphics::Font::new(ctx, "/DejaVuSansMono.ttf")?;
                let font_size = MathOperations::distance(square.get_line1().get_x1(), square.get_line1().get_y1(), square.get_line1().get_x2(), square.get_line1().get_y2());
                let text = graphics::Text::new((COMPUTER, font, font_size));
                graphics::draw(ctx, &text, DrawParam::default().dest(Point2::new(x.0 + (1.0 / 5.0) * font_size, x.1 + (1.0 / 20.0) * font_size)))?;
            }
        } else if self.mode == SelectedMode::TwoPlayers {
            for point in &self.board.points {
                let circle = graphics::Mesh::new_circle(
                    ctx,
                    graphics::DrawMode::fill(),
                    Point2::new(point.0, point.1),
                    10.0,
                    1.0,
                    graphics::BLACK,
                )?;
                graphics::draw(ctx, &circle, (Point2::new(0.0, 0.0),))?;
            }
    
            for line in &self.board.lines {
                let (origin, dest) = (Point2::new(line.get_x1(), line.get_y1()), Point2::new(line.get_x2(), line.get_y2()));
                let color = if line.get_marked_by() == Player::Player_1 {
                    graphics::BLACK
                } else {
                    RED
                };
                let line = graphics::Mesh::new_line(ctx, &[origin, dest], 5.0, color)?;
                graphics::draw(ctx, &line, (Point2::new(0.0, 0.0),))?;
            }
    
            if self.board.temp_line.get_x1() != 0.0 {
                let (origin, dest) = (Point2::new(self.board.temp_line.get_x1(), self.board.temp_line.get_y1()), 
                Point2::new(self.board.temp_line.get_x2(), self.board.temp_line.get_y2()));
        
                let color = if self.next == Player::Player_1 {
                    graphics::BLACK
                } else {
                    RED
                };

                let line = graphics::Mesh::new_line(ctx, &[origin, dest], 5.0, color)?;
                graphics::draw(ctx, &line, (Point2::new(0.0, 0.0),))?;
            }
    
            
            for square in self.board.get_marked_by_player_1() {
                let x = square.get_smallest();
                let font = graphics::Font::new(ctx, "/DejaVuSansMono.ttf")?;
                let distance =  MathOperations::distance(square.get_line1().get_x1(), square.get_line1().get_y1(), square.get_line1().get_x2(), square.get_line1().get_y2());
                let text = graphics::Text::new((PLAYER_1, font, (7.0 / 10.0) * distance));
                let w = text.width(ctx);
                let h = text.height(ctx);                
                graphics::draw(ctx, &text, DrawParam::default().dest(
                    Point2::new(x.0 + (distance - w as f32) / 2.0, x.1 + (distance - h as f32) / 2.0)))?;
            }
    
            for square in self.board.get_marked_by_player_2() {
                let x = square.get_smallest();
                let font = graphics::Font::new(ctx, "/DejaVuSansMono.ttf")?;
                let distance =  MathOperations::distance(square.get_line1().get_x1(), square.get_line1().get_y1(), square.get_line1().get_x2(), square.get_line1().get_y2());
                let text = graphics::Text::new((PLAYER_2, font, (7.0 / 10.0) * distance));
                let w = text.width(ctx);
                let h = text.height(ctx);
                graphics::draw(ctx, &text, DrawParam::default().dest(
                    Point2::new(x.0 + (distance - w as f32) / 2.0, x.1 + (distance - h as f32) / 2.0)))?;
            }
        }

        graphics::present(ctx)?;
        // We yield the current thread until the next update
        ggez::timer::yield_now();
        // And return success.
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _xrel: f32, _yrel: f32) {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            ggez::input::mouse::set_cursor_type(_ctx, ggez::input::mouse::MouseCursor::Default);

            let mut closest_points = Vec::<(f32, f32)>::new();

            for _i in 0..4 {
                let mut closest_point: (f32, f32) = (std::f32::MAX, std::f32::MAX);
                for point in &self.board.points {
                    let distance = MathOperations::distance(x, y, point.0, point.1);
    
                    if distance < MathOperations::distance(x, y, closest_point.0, closest_point.1) 
                    && !closest_points.contains(&point)
                    && !MathOperations::are_on_same_line(point.0, point.1, &closest_points)  {
                        closest_point = (point.0, point.1);
                    }
                }
                
                if !closest_points.contains(&closest_point) {
                    closest_points.push(closest_point);
                }
            }

            let mut middle_x = 0.0;
            let mut middle_y = 0.0;
            for i in 0..3 {
                if closest_points[i].0 != closest_points[i + 1].0 {
                    let offset = if closest_points[i].0 < closest_points[i + 1].0 {
                        closest_points[i].0 
                    } else {
                        closest_points[i + 1].0 
                    };
                    middle_x = offset + ((closest_points[i].0 - closest_points[i + 1].0) / 2.0).abs();
                }

                if closest_points[i].1 != closest_points[i + 1].1 {
                    let offset = if closest_points[i].1 < closest_points[i + 1].1 {
                        closest_points[i].1 
                    } else {
                        closest_points[i + 1].1 
                    };
                    middle_y = offset + ((closest_points[i].1 - closest_points[i + 1].1) / 2.0).abs();
                }
            }

            if MathOperations::is_inside_triangle(closest_points[0].0, closest_points[0].1, closest_points[3].0, closest_points[3].1, middle_x, middle_y, x, y) {
                // let (origin, dest) = (Point2::new(closest_points[0].0, closest_points[0].1), 
                // Point2::new(closest_points[3].0, closest_points[3].1));
                self.board.temp_line = Line::new(closest_points[0].0, closest_points[0].1, closest_points[3].0, closest_points[3].1, self.next);  
            } else {
                for i in 0..3 {
                    if MathOperations::is_inside_triangle(closest_points[i].0, closest_points[i].1, closest_points[i + 1].0, closest_points[i + 1].1, middle_x, middle_y, x, y) {
                        // let (origin, dest) = (Point2::new(closest_points[i].0, closest_points[i].1), 
                        // Point2::new(closest_points[i + 1].0, closest_points[i + 1].1));
                        self.board.temp_line = Line::new(closest_points[i].0, closest_points[i].1, closest_points[i + 1].0, closest_points[i + 1].1, self.next);
                        break;
                    }
                }
            }
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        if self.mode == SelectedMode::None {
            let one_player = self.main_menu.get_one_player_entry();
            let two_players = self.main_menu.get_two_player_entry();

            if MathOperations::is_inside_rectangle(_x, _y, one_player.get_x(), one_player.get_y(), 
                    one_player.get_width(), one_player.get_height()) {
                self.mode = SelectedMode::OnePlayer;
            } else if MathOperations::is_inside_rectangle(_x, _y, two_players.get_x(), two_players.get_y(), 
                    two_players.get_width(), two_players.get_height()) { // TODO put in function
                self.mode = SelectedMode::TwoPlayers;
            }
        } else if self.mode == SelectedMode::OnePlayer {
            if !self.board.get_lines().contains(&self.board.temp_line) {
                let previous = self.board.get_marked_by_player_1().len();
                self.board.add_line(self.board.temp_line);
                self.board.update_squares(Player::Player_1);
    
                if self.board.get_marked_by_player_1().len() == previous {
                    let mut previous = self.board.get_marked_by_player_2().len();
                    let k = self.min_max.alphabeta(&self.board, 6, std::i32::MIN, std::i32::MAX, true);
                    self.board = k.0;
                    println!("Got score {}", k.1);
                    let mut current = self.board.get_marked_by_player_2().len();
    
                    while previous != current {
                        previous = current;
                        let k = self.min_max.alphabeta(&self.board, 6, std::i32::MIN, std::i32::MAX, true);
                        self.board = k.0;
                        current = self.board.get_marked_by_player_2().len();
                    }
                }
            }
        } else if self.mode == SelectedMode::TwoPlayers {
            if !self.board.get_lines().contains(&self.board.temp_line) {
                if self.next == Player::Player_1 {
                    let previous = self.board.get_marked_by_player_1().len();
                    self.board.add_line(self.board.temp_line);
                    self.board.update_squares(Player::Player_1);

                    if self.board.get_marked_by_player_1().len() == previous {
                        self.next = Player::Player_2;
                    }
                } else if self.next == Player::Player_2 {
                    let previous = self.board.get_marked_by_player_2().len();
                    self.board.add_line(self.board.temp_line);
                    self.board.update_squares(Player::Player_2);

                    if self.board.get_marked_by_player_2().len() == previous {
                        self.next = Player::Player_1;
                    }
                }
            }
        }
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        keymod: KeyMods,
        repeat: bool,
) {
        println!(
            "Key pressed: {:?}, modifier {:?}, repeat: {}",
            keycode, keymod, repeat
        );
    }
}

pub fn main() -> GameResult {
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
    let (ctx, event_loop) = &mut cb.build()?;
    let game_state = &mut GameState::new(ctx);
    event::run(ctx, event_loop, game_state)
}