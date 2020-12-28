// 4. AI picks random corner (done).
// 5. AI checks if corner is available using the get_available method(in progress).


use std::io::*;
use rand::prelude::*;
use std::fmt;

#[derive(Debug, PartialEq)]
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

    pub fn new_human() -> Player {
        let mut flipper = rand::thread_rng();
        let coin_flip = flipper.gen_range(0,2);

        if coin_flip == 0 {
            let human = Player {
                player_type: PlayerType::human,
                letter: Letter::X,
                move_pick: 0,
            };
                return human
        }else{
            let human = Player {
                player_type: PlayerType::human,
                letter: Letter::O,
                move_pick: 0,
            };
                return human
        }
    }

    pub fn new_computer(human: &Player) -> Player {
        if human.letter == Letter::X {
            let computer = Player {
                player_type: PlayerType::cpu,
                letter: Letter::O,
                move_pick: 0,
            };
            return computer
        }else{
            let computer = Player {
                player_type : PlayerType::cpu,
                letter: Letter::X,
                move_pick: 0,
            };
            return computer
        }
    }

    pub fn get_move(&self, board: &Board) -> u8 {
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
            PlayerType::cpu => {

                //Pick a corner.
                let corner_rows = vec![1, 3];
                let mut rng = rand::thread_rng();
                let row_pick = corner_rows.choose(&mut rng).unwrap();
                let corner_move = match row_pick {
                    1 => {
                        let choices: Vec<u8> = vec![1, 3];
                        let choice = choices.choose(&mut rng).unwrap();
                        *choice
                    },
                    3 => {
                        let choices: Vec<u8> = vec![7, 9];
                        let choice = choices.choose(&mut rng).unwrap();
                        *choice
                    },
                    _ => {
                        println!("What the fuck happened (line 103)?");
                        return 0
                    }
                };



                return corner_move
            }
        } 
    }

    pub fn get_available(&self, board: &Board) -> Vec<u8> {
        let mut available_vec: Vec<u8> = Vec::new();
        if board.row_1[0] == '1' {
            available_vec.push(1);
        }
        if board.row_1[1] == '2' {
            available_vec.push(2);
        }
        if board.row_1[2] == '3' {
            available_vec.push(3);
        }
        if board.row_2[0] == '4' {
            available_vec.push(4);
        }
        if board.row_2[1] == '5' {
            available_vec.push(5);
        }
        if board.row_2[2] == '6' {
            available_vec.push(6);
        }
        if board.row_3[0] == '7' {
            available_vec.push(7);
        }
        if board.row_3[1] == '8' {
            available_vec.push(8);
        }
        if board.row_3[2] == '9' {
            available_vec.push(9);
        }
        return available_vec
}

}


#[derive(PartialEq, Debug)]
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

    pub fn update(mut self, player: &Player) -> Board {
        let letter: char = match player.letter {
            Letter::X => 'X',
            Letter::O => 'O',
        };

        match player.move_pick {
            1 => self.row_1[((player.move_pick -1) as usize)]= letter,
            2 => self.row_1[((player.move_pick -1) as usize)]= letter,
            3 => self.row_1[((player.move_pick-1) as usize)]= letter,
            4 => self.row_2[((player.move_pick -4) as usize)]= letter,
            5 => self.row_2[((player.move_pick-4) as usize)]= letter,
            6 => self.row_2[((player.move_pick -4) as usize)]= letter,
            7 => self.row_3[((player.move_pick -7) as usize)]= letter,
            8 => self.row_3[((player.move_pick -7) as usize)]= letter,
            9 => self.row_3[((player.move_pick -7) as usize)]= letter,
            _ => println!("Some shit went bad."),
        }

        return self
    }

}


// pub fn player_constructor() -> Vec<Player> {
//     let mut flipper = rand::thread_rng();
//     let coin_flip = flipper.gen_range(0,1);

//     if coin_flip == 0 {
//         let human = Player {
//             player_type: PlayerType::human,
//             letter: Letter::X,
//             move_pick: 0,
//         };

//         let computer = Player {
//             player_type: PlayerType::cpu,
//             letter: Letter::O,
//             move_pick: 0,
//         };

//         println!("You are letter X. You will go first.");
//         println!("");
//         let player_vec = vec![human, computer];
//         return player_vec
//     }else{
//         let human = Player {
//             player_type: PlayerType::human,
//             letter: Letter::O,
//             move_pick: 0,
//         };

//         let computer = Player {
//             player_type: PlayerType::cpu,
//             letter: Letter::X,
//             move_pick: 0,
//         };
//         println!("You are letter O. You will go second.");
//         println!("");

//         let player_vec = vec![human, computer];
//         return player_vec
//     }
// }
    
//Player and computer X, O
//Draw a board
// To get player input
// To get computer input
// Get the best for the computer
// win condition
// draw condition
// Play again
//Scoreboard
