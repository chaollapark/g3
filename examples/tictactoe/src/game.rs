//! Tic Tac Toe game logic

/// Represents a cell on the board
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Empty,
    X,
    O,
}

impl Cell {
    fn to_char(self, index: usize) -> char {
        match self {
            Cell::Empty => char::from_digit((index + 1) as u32, 10).unwrap(),
            Cell::X => 'X',
            Cell::O => 'O',
        }
    }
}

/// The main game state
pub struct Game {
    board: [Cell; 9],
    current_player: Cell,
    moves_count: u8,
}

impl Game {
    /// Create a new game with an empty board
    pub fn new() -> Self {
        Self {
            board: [Cell::Empty; 9],
            current_player: Cell::X,
            moves_count: 0,
        }
    }

    /// Get the current player symbol
    pub fn current_player(&self) -> char {
        match self.current_player {
            Cell::X => 'X',
            Cell::O => 'O',
            Cell::Empty => ' ',
        }
    }

    /// Display the current board state
    pub fn display(&self) {
        println!();
        for row in 0..3 {
            let base = row * 3;
            println!(
                " {} | {} | {} ",
                self.board[base].to_char(base),
                self.board[base + 1].to_char(base + 1),
                self.board[base + 2].to_char(base + 2)
            );
            if row < 2 {
                println!("-----------");
            }
        }
        println!();
    }

    /// Make a move at the given position (1-9)
    pub fn make_move(&mut self, position: usize) -> Result<(), &'static str> {
        if position < 1 || position > 9 {
            return Err("Position must be between 1 and 9");
        }

        let index = position - 1;
        
        if self.board[index] != Cell::Empty {
            return Err("That cell is already taken!");
        }
        
        self.board[index] = self.current_player;
        self.moves_count += 1;
        self.switch_player();
        
        Ok(())
    }

    /// Switch to the other player
    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            Cell::Empty => Cell::X,
        };
    }

    /// Check if there's a winner, returns the winning player's symbol
    pub fn check_winner(&self) -> Option<char> {
        const WIN_PATTERNS: [[usize; 3]; 8] = [
            [0, 1, 2], // Top row
            [3, 4, 5], // Middle row
            [6, 7, 8], // Bottom row
            [0, 3, 6], // Left column
            [1, 4, 7], // Middle column
            [2, 5, 8], // Right column
            [0, 4, 8], // Diagonal top-left to bottom-right
            [2, 4, 6], // Diagonal top-right to bottom-left
        ];

        for pattern in WIN_PATTERNS {
            let [a, b, c] = pattern;
            if self.board[a] != Cell::Empty
                && self.board[a] == self.board[b]
                && self.board[b] == self.board[c]
            {
                return Some(match self.board[a] {
                    Cell::X => 'X',
                    Cell::O => 'O',
                    Cell::Empty => unreachable!(),
                });
            }
        }
        
        None
    }

    /// Check if the game is a draw (board full with no winner)
    pub fn is_draw(&self) -> bool {
        self.moves_count == 9 && self.check_winner().is_none()
    }

    /// Get the board state
    #[allow(dead_code)]
    pub fn board(&self) -> &[Cell; 9] {
        &self.board
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
