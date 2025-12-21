//! Tests for Tic Tac Toe game logic

use tictactoe::game::{Cell, Game};

#[test]
fn test_new_game_has_empty_board() {
    let game = Game::new();
    for cell in game.board() {
        assert_eq!(*cell, Cell::Empty);
    }
}

#[test]
fn test_new_game_starts_with_x() {
    let game = Game::new();
    assert_eq!(game.current_player(), 'X');
}

#[test]
fn test_make_valid_move() {
    let mut game = Game::new();
    assert!(game.make_move(5).is_ok());
    assert_eq!(game.board()[4], Cell::X);
}

#[test]
fn test_players_alternate() {
    let mut game = Game::new();
    assert_eq!(game.current_player(), 'X');
    game.make_move(1).unwrap();
    assert_eq!(game.current_player(), 'O');
    game.make_move(2).unwrap();
    assert_eq!(game.current_player(), 'X');
}

#[test]
fn test_cannot_move_to_occupied_cell() {
    let mut game = Game::new();
    game.make_move(5).unwrap();
    assert!(game.make_move(5).is_err());
}

#[test]
fn test_invalid_position() {
    let mut game = Game::new();
    assert!(game.make_move(0).is_err());
    assert!(game.make_move(10).is_err());
}

#[test]
fn test_horizontal_win() {
    let mut game = Game::new();
    // X moves: 1, 2, 3 (top row)
    // O moves: 4, 5
    game.make_move(1).unwrap(); // X
    game.make_move(4).unwrap(); // O
    game.make_move(2).unwrap(); // X
    game.make_move(5).unwrap(); // O
    game.make_move(3).unwrap(); // X wins
    
    assert_eq!(game.check_winner(), Some('X'));
}

#[test]
fn test_vertical_win() {
    let mut game = Game::new();
    // X moves: 1, 4, 7 (left column)
    // O moves: 2, 5
    game.make_move(1).unwrap(); // X
    game.make_move(2).unwrap(); // O
    game.make_move(4).unwrap(); // X
    game.make_move(5).unwrap(); // O
    game.make_move(7).unwrap(); // X wins
    
    assert_eq!(game.check_winner(), Some('X'));
}

#[test]
fn test_diagonal_win() {
    let mut game = Game::new();
    // X moves: 1, 5, 9 (diagonal)
    // O moves: 2, 3
    game.make_move(1).unwrap(); // X
    game.make_move(2).unwrap(); // O
    game.make_move(5).unwrap(); // X
    game.make_move(3).unwrap(); // O
    game.make_move(9).unwrap(); // X wins
    
    assert_eq!(game.check_winner(), Some('X'));
}

#[test]
fn test_draw() {
    let mut game = Game::new();
    // Fill board with no winner:
    // X | O | X
    // X | O | O
    // O | X | X
    game.make_move(1).unwrap(); // X
    game.make_move(2).unwrap(); // O
    game.make_move(3).unwrap(); // X
    game.make_move(5).unwrap(); // O
    game.make_move(4).unwrap(); // X
    game.make_move(6).unwrap(); // O
    game.make_move(8).unwrap(); // X
    game.make_move(7).unwrap(); // O
    game.make_move(9).unwrap(); // X
    
    assert!(game.check_winner().is_none());
    assert!(game.is_draw());
}

#[test]
fn test_no_winner_mid_game() {
    let mut game = Game::new();
    game.make_move(1).unwrap();
    game.make_move(5).unwrap();
    
    assert!(game.check_winner().is_none());
    assert!(!game.is_draw());
}
