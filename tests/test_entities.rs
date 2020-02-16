use dotsnboxes::entities::{MathOperations, Player, Line, Square, Board, DELTA};

#[test]
fn test_basic_distance() {
    let expected = 5.0;
    let actual = MathOperations::distance(3.0, 3.0, 8.0, 7.0);

    assert!(eq_float(expected, actual));
}


#[test]
fn test_basic_area() {
    let expected = 12.0;
    let actual = MathOperations::area(-3.0, 1.0, 1.0, 5.0, -2.0, 8.0);

    assert!(eq_float(expected, actual));
}

#[test]
fn test_is_inside_triangle() {
    let tr_x1 = 10.0;
    let tr_y1 = 10.0;

    let tr_x2 = 10.0;
    let tr_y2 = 30.0;

    let tr_x3 = 40.0;
    let tr_y3 = 20.0;
    let actual = MathOperations::is_inside_triangle(tr_x1, tr_y1, tr_x2, tr_y2, tr_x3, tr_y3, 20.0, 15.0);
    assert!(actual);
}

#[test]
fn test_is_outside_triangle() {
    let tr_x1 = 10.0;
    let tr_y1 = 10.0;

    let tr_x2 = 10.0;
    let tr_y2 = 30.0;

    let tr_x3 = 40.0;
    let tr_y3 = 20.0;
    let actual = MathOperations::is_inside_triangle(tr_x1, tr_y1, tr_x2, tr_y2, tr_x3, tr_y3, 5.0, 15.0);
    assert!(!actual);
}

#[test]
fn test_is_inside_rectangle() {
    let rec_x = 10.0;
    let rec_y = 20.0;
    let rec_width = 10.0;
    let rec_height = 40.0;


    let actual = MathOperations::is_inside_rectangle(15.0, 30.0, rec_x, rec_y, rec_width, rec_height);
    assert!(actual);
}

#[test]
fn test_is_outside_rectangle() {
    let rec_x = 10.0;
    let rec_y = 20.0;
    let rec_width = 10.0;
    let rec_height = 40.0;


    let actual = MathOperations::is_inside_rectangle(25.0, 30.0, rec_x, rec_y, rec_width, rec_height);
    assert!(!actual);
}

#[test]
fn test_are_on_the_same_line() {
    let x = 7.0;
    let y = 10.0;

    let points = vec![(1.0, 2.0), (7.0, 2.0), (7.0, 30.0)];

    assert!(MathOperations::are_on_same_line(x, y, &points));
}

#[test]
fn test_are_not_on_the_same_line() {
    let x = 7.0;
    let y = 11.0;

    let points = vec![(1.0, 11.0), (4.0, 2.0), (7.0, 30.0)];

    assert!(!MathOperations::are_on_same_line(x, y, &points));
}

#[test]
fn test_line_equallity() {
    let line1 = Line::new(6.0, 2.0, 3.0, 4.0, Player::Dummy);
    let line2 = Line::new(3.0, 4.0, 6.0, 2.0, Player::Dummy);


    assert!(line1 == line2);
}

#[test]
fn test_get_smallest() {
    let line1 = Line::new(6.0, 2.0, 3.0, 4.0, Player::Dummy);
    let line2 = Line::new(21.0, 11.0, 3.0, 9.0, Player::Dummy);
    let line3 = Line::new(2.0, 2.0, 4.0, 3.0, Player::Dummy);
    let line4 = Line::new(8.0, 18.0, 3.0, 13.0, Player::Dummy);

    let square = Square::new(line1, line2, line3, line4);
    let actual = square.get_smallest();

    assert!(eq_float(2.0, actual.0));
    assert!(eq_float(2.0, actual.1));
}

#[test]
fn test_square_equallity() {
    let line1 = Line::new(6.0, 2.0, 3.0, 4.0, Player::Dummy);
    let line2 = Line::new(21.0, 11.0, 3.0, 9.0, Player::Dummy);
    let line3 = Line::new(2.0, 2.0, 4.0, 3.0, Player::Dummy);
    let line4 = Line::new(8.0, 18.0, 3.0, 13.0, Player::Dummy);

    let square1 = Square::new(line1, line2, line3, line4);
    let square2 = Square::new(line3, line1, line2, line4);

    assert!(square1 == square2);
}

#[test]
fn test_board_add_line() {
    let mut board = Board::new(3.0, 3.0, 600.0, 600.0, 50.0, 50.0);
    let line = Line::new(6.0, 2.0, 3.0, 4.0, Player::Dummy);
    board.add_line(line);

    assert!(board.contains_line(&line));
}

#[test]
fn test_temp_line_positioning() {
    let expected = Line::new(300.0, 300.0, 300.0, 50.0, Player::Player1);

    let mut board = Board::new(3.0, 3.0, 600.0, 600.0, 50.0, 50.0);
    board.update_line(Player::Player1, 300.0, 200.0);
    let temp_line = board.get_temp_line();

    assert_eq!(expected, temp_line);
}

#[test]
fn test_update_squares() {
    let line1 = Line::new(50.0, 50.0, 50.0, 300.0, Player::Player1);
    let line2 = Line::new(300.0, 50.0, 50.0, 50.0, Player::Player1);
    let line3 = Line::new(50.0, 300.0, 300.0, 300.0, Player::Player1);
    let line4 = Line::new(300.0, 300.0, 300.0, 50.0, Player::Player1);

    let mut board = Board::new(3.0, 3.0, 600.0, 600.0, 50.0, 50.0);

    board.add_line(line1);
    board.add_line(line2);
    board.add_line(line3);
    board.add_line(line4);

    assert_eq!(0, board.get_marked_by_player_1().len());
    assert_eq!(0, board.get_marked_by_player_2().len());

    board.update_squares(Player::Player1);

    assert_eq!(1, board.get_marked_by_player_1().len());
    assert_eq!(0, board.get_marked_by_player_2().len());
}

fn eq_float(a: f32, b: f32) -> bool {
    (a - b) < DELTA
}