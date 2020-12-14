// 1. Get player input.
// 2. Get the unchanged board to display.
// 3. Change the board by player input and redraw.
// 4. AI and win conditions.

use std::io::*;


pub enum Letter {
    X,
    O,
}

pub enum PlayerType {
    human,
    cpu,
}


pub struct Player {
    pub player_type: PlayerType,
    pub letter: Letter,
    pub move_pick: u8,
}

impl Player {
    pub fn get_move(&self) -> bool {
        match self.player_type {
            PlayerType::human => true,
            PlayerType::cpu => true,
        }
        
        
        // {
    //         println!("Please choose a move.");
    //         let mut move_pick = String::new();
    //         std::io::stdin().read_line(&mut move_pick).expect("Failed to get player pick.");
    // }
    }
}


pub struct Board {
    board: Vec<char>,
}


//Player and computer X, O
//Draw a board
// To get player input
// To get computer input
// Get the best for the computer
// win condition
// draw condition
// Play again
//Scoreboard