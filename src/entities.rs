use ggez::{Context, GameResult};
use ggez::nalgebra::Point2;
use ggez::graphics::{self, DrawParam};
use graphics::{Font, Text, Color};
use graphics::DrawMode;

pub const DELTA: f32 = 0.00001;

pub const WINDOW_WIDTH: f32 = 600.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

pub const X_INITIAL_OFFSET: f32 = 60.0;
pub const Y_INITIAL_OFFSET: f32 = 60.0;

pub const PLAYER_1: &'static str = "P1";
pub const PLAYER_2: &'static str = "P2";

pub const PLAYER_1_COLOR: Color = graphics::BLACK;
pub const PLAYER_2_COLOR: Color = Color::new(255.0, 0.0, 0.0, 255.0);

pub const WIDTH: f32 = 3.0;
pub const HEIGHT: f32 = 3.0;

pub struct MathOperations {}

impl MathOperations {
    pub fn distance(x: f32, y: f32, x1: f32, y1: f32) -> f32 {
        ((x - x1) * (x - x1) + (y - y1) * (y - y1)).sqrt()
    }

    pub fn area(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32) -> f32 { 
        ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2.0).abs()
    }

    pub fn is_inside_triangle(x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x: f32, y: f32) -> bool { 
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

    pub fn is_inside_rectangle(x: f32, y: f32, x_rectangle: f32, y_rectangle: f32, width: f32, height: f32) -> bool {
        x_rectangle <= x && x_rectangle + width >= x && y_rectangle <= y && y_rectangle + height >= y
    }

    pub fn are_on_same_line(x: f32, y: f32, points: &[(f32, f32)]) -> bool {
        let mut on_x: u8 = 0;
        let mut on_y: u8 = 0;
        for point in points {
            if (point.0 - x).abs() <= DELTA {
                on_x = on_x + 1;
            }

            if (point.1 - y).abs() <= DELTA {
                on_y = on_y + 1;
            }
        }

        on_x >= 2 || on_y >= 2
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    OnePlayer, TwoPlayers, None, GameOver
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Player {
    Player1,
    Player2,
    Dummy,
}

#[derive(Debug, Copy, Clone)]
pub struct Line {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    marked_by: Player,
}

impl Line {
    pub fn new(_x1: f32, _y1: f32, _x2: f32, _y2: f32, _marked_by: Player) -> Line {
        Line {
            x1: _x1,
            y1: _y1,
            x2: _x2,
            y2: _y2,
            marked_by: _marked_by,
        }
    }

    pub fn get_x1(&self) -> f32 {
        self.x1
    }
    
    pub fn get_y1(&self) -> f32 {
        self.y1
    }

    pub fn get_x2(&self) -> f32 {
        self.x2
    }

    pub fn get_y2(&self) -> f32 {
        self.y2
    }

    pub fn get_marked_by(&self) -> Player {
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
pub struct Square {
    line1: Line,
    line2: Line,
    line3: Line,
    line4: Line,
}

impl Square {
    pub fn new(_line1: Line, _line2: Line, _line3: Line, _line4: Line) -> Square {
        Square {
            line1: _line1,
            line2: _line2,
            line3: _line3,
            line4: _line4,
        }
    }

    pub fn get_line1(&self) -> Line {
        self.line1
    }

    pub fn get_smallest(&self) -> (f32, f32) {
        let mut smallest_x = std::f32::MAX;
        let mut smallest_y = std::f32::MAX;

        for x in vec![self.line1, self.line2, self.line3, self.line4] {
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

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        (self.line1 == other.line1 || self.line1 == other.line2 || self.line1 == other.line3 || self.line1 == other.line4) &&
        (self.line2 == other.line1 || self.line2 == other.line2 || self.line2 == other.line3 || self.line2 == other.line4) &&
        (self.line3 == other.line1 || self.line3 == other.line2 || self.line3 == other.line3 || self.line3 == other.line4) &&
        (self.line4 == other.line1 || self.line4 == other.line2 || self.line4 == other.line3 || self.line4 == other.line4)
    }
}

#[derive(Debug, Clone)]
pub struct Board {
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
    pub fn new(_width: f32, _height: f32, window_width: f32, window_height: f32, _start_x: f32, _start_y: f32) -> Board {
        let _step_x = (window_width - 2.0 * _start_x) / (_width - 1.0);
        let _step_y = (window_height - 2.0 * _start_y) / (_height - 1.0);
        let mut _points = Vec::<(f32, f32)>::new();
        let mut _squares = Vec::<Square>::new();

        for i in 0.._height as u32 {
            for j in 0.._width as u32 {
                _points.push((_start_x + (j as f32) * _step_x, _start_y + (i as f32) * _step_y));
            }
        }

        for i in 0..(_height - 1.0) as u32 {
            for j in 0..(_width - 1.0) as u32 {
                let point = _points[(i * _height as u32 + j) as usize];
                let line1 = Line::new(point.0, point.1,point.0 + _step_x, point.1, Player::Dummy);
                let line2 = Line::new(point.0 + _step_x, point.1,point.0 + _step_x, point.1 + _step_y, Player::Dummy);
                let line3 = Line::new(point.0 + _step_x, point.1 + _step_y,point.0, point.1 + _step_y, Player::Dummy);
                let line4 = Line::new(point.0, point.1 + _step_y,point.0, point.1, Player::Dummy);

                _squares.push(Square::new(line1, line2, line3, line4));
            }
        }

        Board {
            width: _width,
            height: _height,
            points: _points,
            lines: Vec::new(),
            temp_line: Line::new(0.0, 0.0, 0.0, 0.0, Player::Dummy),
            start_x: _start_x,
            step_x: _step_x,
            start_y: _start_y,
            step_y: _step_y,
            squares: _squares,
            marked_squares_by_player_1: Vec::new(), // player_1 is always you
            marked_squares_by_player_2: Vec::new(), // player_2 is either player_2 or the computer
        }
    }

    pub fn get_marked_by_player_1(&self) -> &[Square] {
        &self.marked_squares_by_player_1
    }

    pub fn get_marked_by_player_2(&self) -> &[Square] {
        &self.marked_squares_by_player_2
    }

    pub fn get_step_x(&self) -> f32 {
        self.step_x
    }

    pub fn get_step_y(&self) -> f32 {
        self.step_y
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_temp_line(&self) -> Line {
        self.temp_line
    }

    pub fn get_lines(&self) -> Vec<Line> {
        self.lines.clone()
    }

    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn is_complete(&self) -> bool {
        self.get_marked_by_player_1().len() + self.get_marked_by_player_2().len() == (self.width - 1.0) as usize * (self.height - 1.0) as usize
    }

    pub fn update_squares(&mut self, player: Player) {
        let lines_len = self.lines.len();
        for i in 0..lines_len {
            for j in (i + 1)..lines_len {
                for t in (j + 1)..lines_len {
                    for k in (t + 1)..lines_len {
                        let potential_square = Square::new(self.lines[i], self.lines[j], self.lines[t], self.lines[k]);

                        if player == Player::Player2 {
                            if self.squares.contains(&potential_square) && !self.marked_squares_by_player_1.contains(&potential_square) && 
                                !self.marked_squares_by_player_2.contains(&potential_square) {
                                self.marked_squares_by_player_2.push(potential_square);
                            }
                        } else if player == Player::Player1 {
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

    pub fn draw(&mut self, ctx: &mut Context, next: Player) -> GameResult {
        for point in &self.points {
            let circle = graphics::Mesh::new_circle(
                ctx,
                DrawMode::fill(),
                Point2::new(point.0, point.1),
                10.0,
                1.0,
                graphics::BLACK,
            )?;
            graphics::draw(ctx, &circle, (Point2::new(0.0, 0.0),))?;
        }

        for line in &self.lines {
            let origin = Point2::new(line.get_x1(), line.get_y1());
            let dest = Point2::new(line.get_x2(), line.get_y2());

            let color = if line.get_marked_by() == Player::Player1 {
                PLAYER_1_COLOR
            } else {
                PLAYER_2_COLOR
            };
            let line = graphics::Mesh::new_line(ctx, &[origin, dest], 5.0, color)?;
            graphics::draw(ctx, &line, (Point2::new(0.0, 0.0),))?;
        }

        if self.temp_line.get_x1() != 0.0 {
            let origin = Point2::new(self.temp_line.get_x1(), self.temp_line.get_y1());
            let dest = Point2::new(self.temp_line.get_x2(), self.temp_line.get_y2());

            let color = if next == Player::Player1 {
                PLAYER_1_COLOR
            } else {
                PLAYER_2_COLOR
            };

            let line = graphics::Mesh::new_line(ctx, &[origin, dest], 5.0, color)?;
            graphics::draw(ctx, &line, (Point2::new(0.0, 0.0),))?;
        }

        let font = graphics::Font::new(ctx, "/DejaVuSansMono.ttf")?;
        
        for square in &self.marked_squares_by_player_1 {
            let x = square.get_smallest();
            let distance =  MathOperations::distance(square.get_line1().get_x1(), square.get_line1().get_y1(), square.get_line1().get_x2(), square.get_line1().get_y2());
            let text = graphics::Text::new((PLAYER_1, font, (7.0 / 10.0) * distance));
            let w = text.width(ctx);
            let h = text.height(ctx);                
            graphics::draw(ctx, &text, DrawParam::default().dest(
                Point2::new(x.0 + (distance - w as f32) / 2.0, x.1 + (distance - h as f32) / 2.0)))?;
        }

        for square in &self.marked_squares_by_player_2 {
            let x = square.get_smallest();
            let distance =  MathOperations::distance(square.get_line1().get_x1(), square.get_line1().get_y1(), square.get_line1().get_x2(), square.get_line1().get_y2());
            let text = graphics::Text::new((PLAYER_2, font, (7.0 / 10.0) * distance));
            let w = text.width(ctx);
            let h = text.height(ctx);
            graphics::draw(ctx, &text, DrawParam::default().dest(
                Point2::new(x.0 + (distance - w as f32) / 2.0, x.1 + (distance - h as f32) / 2.0)))?;
        }

        Ok(())
    }

    pub fn update_line(&mut self, next: Player, x: f32, y: f32) {
        let mut closest_points = Vec::<(f32, f32)>::new();

        for _i in 0..4 {
            let mut closest_point: (f32, f32) = (std::f32::MAX, std::f32::MAX);
            for point in &self.points {
                let distance = MathOperations::distance(x, y, point.0, point.1);

                if distance < MathOperations::distance(x, y, closest_point.0, closest_point.1) 
                && !closest_points.contains(&point)
                && !MathOperations::are_on_same_line(point.0, point.1, &closest_points) {
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
            self.temp_line = Line::new(closest_points[0].0, closest_points[0].1, closest_points[3].0, closest_points[3].1, next);  
        } else {
            for i in 0..3 {
                if MathOperations::is_inside_triangle(closest_points[i].0, closest_points[i].1, closest_points[i + 1].0, closest_points[i + 1].1, middle_x, middle_y, x, y) {
                    self.temp_line = Line::new(closest_points[i].0, closest_points[i].1, closest_points[i + 1].0, closest_points[i + 1].1, next);
                    break;
                }
            }
        }
    }
}

pub struct MinMax {

}

impl MinMax {

    pub fn new() -> MinMax {
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
                start.1 + board.get_step_y() + i as f32 * board.get_step_y(), 
                player);
                if !board.get_lines().contains(&line) {
                    let mut cloned = board.clone();
                    cloned.add_line(line);
                    cloned.update_squares(player.clone());
                    children.push(cloned);
                }
            }
        }

        children
    }

    pub fn alphabeta(&mut self, board: &Board, max_depth: u8, alpha: i32, beta: i32, is_max: bool) -> (Board, i32) {
        if board.is_complete() || max_depth <= 0 {
            return (board.clone(), if is_max {
                    board.get_marked_by_player_2().len() as i32
                } else {
                    -(board.get_marked_by_player_1().len() as i32)
                });
        }

        let mut value: i32;
        let mut result: (Board, i32) = (board.clone(), 0);
        let parent_score = (board.get_marked_by_player_1().len(), board.get_marked_by_player_2().len());

        if is_max {
            value = std::i32::MIN;
            let children = self.get_children(board, Player::Player2);
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
            let children = self.get_children(board, Player::Player1);
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

pub struct LabelButton {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: String,
}

impl LabelButton {
    pub fn new(_x: f32, _y: f32, _width: f32, _height: f32, _text: String) -> LabelButton {
        LabelButton {
            x: _x,
            y: _y,
            width: _width,
            height: _height,
            text: _text,
        }
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }
    
    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}

pub struct MainMenu {
    one_player_entry: LabelButton,
    two_player_entry: LabelButton,
}

impl MainMenu {
    pub fn new(ctx: &mut Context) -> GameResult<MainMenu> {
        let font = Font::new(ctx, "/DejaVuSansMono.ttf")?;
        let font_size = 40.0;
        let start_y = WINDOW_HEIGHT / 3.0;
        let step = WINDOW_HEIGHT / 5.0;

        let text_one_player = Text::new(("1 Player", font, font_size));
        let text_two_player = Text::new(("2 Players", font, font_size));

        let text_one_player_width = text_one_player.width(ctx) as f32;
        let text_one_player_height = text_one_player.height(ctx) as f32;

        let text_two_player_width = text_two_player.width(ctx) as f32;
        let text_two_player_height = text_two_player.height(ctx) as f32;

        let x = (600.0 - text_one_player_width) / 2.0;

        let k = MainMenu {
            one_player_entry: LabelButton::new(x, start_y, text_one_player_width, text_one_player_height, String::from("1 Player")),
            two_player_entry: LabelButton::new(x, start_y + step, text_two_player_width, text_two_player_height, String::from("2 Players")),
        };

        Ok(k)
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let font = Font::new(ctx, "/DejaVuSansMono.ttf")?;
        let font_size = 40.0;

        let text = Text::new(("Dots and boxes", font, font_size));
        let text_width = text.width(ctx) as f32;
        graphics::draw(ctx, &text, DrawParam::default()
        .dest(Point2::new((WINDOW_WIDTH - text_width) / 2.0, 30.0)))?;

        let text = Text::new((self.one_player_entry.get_text(), font, font_size));
        graphics::draw(ctx, &text, DrawParam::default()
        .dest(Point2::new(self.one_player_entry.get_x(), self.one_player_entry.get_y())))?;

        let text = Text::new((self.two_player_entry.get_text(), font, font_size));
        graphics::draw(ctx, &text, DrawParam::default()
        .dest(Point2::new(self.two_player_entry.get_x(), self.two_player_entry.get_y())))?;
        
        Ok(())
    }

    pub fn is_one_player_entry_clicked(&self, x: f32, y: f32) -> bool {
        MathOperations::is_inside_rectangle(x, y, self.one_player_entry.get_x(), self.one_player_entry.get_y(), 
            self.one_player_entry.get_width(), self.one_player_entry.get_height())
    }

    pub fn is_two_player_entry_clicked(&self, x: f32, y: f32) -> bool {
        MathOperations::is_inside_rectangle(x, y, self.two_player_entry.get_x(), self.two_player_entry.get_y(), 
            self.two_player_entry.get_width(), self.two_player_entry.get_height())
    }
}

pub struct EndMenu {
    restart: LabelButton,
}

impl EndMenu {
    pub fn new(ctx: &mut Context) -> GameResult<EndMenu> {
        let font = Font::new(ctx, "/DejaVuSansMono.ttf")?;
        let font_size = 40.0;

        let text_restart = Text::new(("Play again!", font, font_size));
        let text_restart_width = text_restart.width(ctx) as f32;
        let text_restart_height = text_restart.height(ctx) as f32;

        let x = (WINDOW_WIDTH - text_restart_width) / 2.0;
        let y = WINDOW_HEIGHT - text_restart_height - 50.0;

        let k = EndMenu {
            restart: LabelButton::new(x, y, text_restart_width, text_restart_height, String::from("Play again!")),
        };

        Ok(k)
    }

    pub fn draw(&mut self, ctx: &mut Context, player1_score: u8, player2_score: u8) -> GameResult {
        let font = Font::new(ctx, "/DejaVuSansMono.ttf")?;
        let font_size = 40.0;

        let text = Text::new(("Game over", font, font_size));
        let text_width = text.width(ctx) as f32;

        graphics::draw(ctx, &text, DrawParam::default()
        .dest(Point2::new((WINDOW_WIDTH - text_width) / 2.0, 30.0)))?;

        let result_string = if player1_score < player2_score {
            String::from("Winner is player 2")
        } else if player1_score > player2_score {
            String::from("Winner is Player 1")
        } else {
            String::from("It is a draw")
        };

        let result = Text::new((result_string.clone(), font, font_size));
        let text_width = result.width(ctx) as f32;
        let text_height = result.height(ctx) as f32;

        let x = (WINDOW_WIDTH - text_width) / 2.0;
        let y = (WINDOW_HEIGHT - text_height) / 2.0;

        graphics::draw(ctx, &result, DrawParam::default()
        .dest(Point2::new(x, y)))?;

        let text = Text::new((self.restart.get_text(), font, font_size));
        graphics::draw(ctx, &text, DrawParam::default()
        .dest(Point2::new(self.restart.get_x(), self.restart.get_y())))?;
        
        Ok(())
    }

    pub fn get_restart(&self) -> &LabelButton {
        &self.restart
    }
}

