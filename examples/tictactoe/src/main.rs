mod game;

use game::Game;
use std::io::{self, Write};

fn main() {
    println!("\n🎮 Welcome to Tic Tac Toe! 🎮\n");
    println!("Players take turns entering positions (1-9):");
    println!(" 1 | 2 | 3 ");
    println!("-----------");
    println!(" 4 | 5 | 6 ");
    println!("-----------");
    println!(" 7 | 8 | 9 ");
    println!();

    let mut game = Game::new();
    
    loop {
        game.display();
        
        if let Some(winner) = game.check_winner() {
            println!("🎉 Player {} wins! 🎉", winner);
            break;
        }
        
        if game.is_draw() {
            println!("🤝 It's a draw!");
            break;
        }
        
        print!("Player {}'s turn. Enter position (1-9): ", game.current_player());
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let position: usize = match input.trim().parse() {
            Ok(num) if (1..=9).contains(&num) => num,
            _ => {
                println!("❌ Invalid input! Please enter a number between 1 and 9.\n");
                continue;
            }
        };
        
        if let Err(e) = game.make_move(position) {
            println!("❌ {}\n", e);
            continue;
        }
        
        println!();
    }
    
    println!("\nThanks for playing! 👋\n");
}
