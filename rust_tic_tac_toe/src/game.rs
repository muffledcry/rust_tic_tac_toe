// 3. Change the board by player input and redraw.
// 4. AI and win conditions.

use std::io::*;
use rand::prelude::*;
use std::fmt;

#[derive(Debug)]
pub enum Letter {
    X,
    O,
}

// impl fmt::Debug for Letter {

// }

#[derive(Debug)]
pub enum PlayerType {
    human,
    cpu,
}


#[derive(Debug)]
pub struct Player {
    pub player_type: PlayerType,
    pub letter: Letter,
    pub move_pick: u8,
}

impl Player {

    pub fn get_move(&self) -> u8 {
        match self.player_type {
            PlayerType::human => loop {
                println!("Please choose a move.");
                let mut move_pick = String::new();
                std::io::stdin()
                    .read_line(&mut move_pick)
                    .expect("Failed to get player pick.");
                let move_pick = move_pick.trim();
                if move_pick.len() == 1 {
                    let move_pick = move_pick.parse::<u8>().unwrap();
                    break move_pick
                }else {
                    continue;
                } 
            },
            PlayerType::cpu => 0,
        }
    }
}



pub struct Board {
    row_1: Vec<char>,
    row_2: Vec<char>,
    row_3: Vec<char>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            row_1: vec!['1', '2', '3'],
            row_2: vec!['4', '5', '6'],
            row_3: vec!['7', '8', '9'],
        }
    }

    pub fn draw_board(&mut self) {
        println!("{:?}", self.row_3);
        println!("{:?}", self.row_2);
        println!("{:?}", self.row_1);
    }

    // pub fn update(&mut self) -> Board {
        
    // }

}


pub fn player_constructor() -> Vec<Player> {
    let mut flipper = rand::thread_rng();
    let coin_flip = flipper.gen_range(0,1);

    if coin_flip == 0 {
        let human = Player {
            player_type: PlayerType::human,
            letter: Letter::X,
            move_pick: 0,
        };

        let computer = Player {
            player_type: PlayerType::cpu,
            letter: Letter::O,
            move_pick: 0,
        };

        println!("You are letter X. You will go first.");
        println!("");
        let player_vec = vec![human, computer];
        return player_vec
    }else{
        let human = Player {
            player_type: PlayerType::human,
            letter: Letter::O,
            move_pick: 0,
        };

        let computer = Player {
            player_type: PlayerType::cpu,
            letter: Letter::X,
            move_pick: 0,
        };
        println!("You are letter O. You will go second.");
        println!("");

        let player_vec = vec![human, computer];
        return player_vec
    }
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
