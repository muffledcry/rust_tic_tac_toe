// 4. AI picks random corner (done).
// 5. AI checks if corner is available using the get_available method(in progress).
// 6. Make game loop to check win condition AI (not started)

use rand::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Letter {
    X,
    O,
}

// impl fmt::Debug for Letter {

// }

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PlayerType {
    Human,
    Cpu,
}


#[derive(Debug, Clone, Copy)]
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
                player_type: PlayerType::Human,
                letter: Letter::X,
                move_pick: 0,
            };
                return human
        }else{
            let human = Player {
                player_type: PlayerType::Human,
                letter: Letter::O,
                move_pick: 0,
            };
                return human
        }
    }

    pub fn new_computer(human: &Player) -> Player {
        if human.letter == Letter::X {
            let computer = Player {
                player_type: PlayerType::Cpu,
                letter: Letter::O,
                move_pick: 0,
            };
            return computer
        }else{
            let computer = Player {
                player_type : PlayerType::Cpu,
                letter: Letter::X,
                move_pick: 0,
            };
            return computer
        }
    }

    pub fn get_move(self, board: &Board) -> u8 {
        match self.player_type {
            PlayerType::Human => loop {
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
            PlayerType::Cpu => {
                let player_copy = self.clone();
                let winning_move = &player_copy.check_win(&board);
                let blocking_move = &player_copy.block_win(&board);

                if winning_move != &0 {
                    return *winning_move
                }
                if blocking_move != &0 {
                    return *blocking_move
                }


                let available_moves = self.get_available(board);

                // Pick the center.
                if available_moves.contains(&5) {
                    return 5
                }

                //Pick a corner.
                if available_moves.contains(&1) || available_moves.contains(&3) || available_moves.contains(&7)
                || available_moves.contains(&9) {
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
                }else if available_moves.contains(&2) || available_moves.contains(&4) || available_moves.contains(&6)
                || available_moves.contains(&8) {
                    let side_rows = vec![1, 2, 3];
                    let mut rng = rand::thread_rng();
                    let side_pick = side_rows.choose(&mut rng).unwrap();
                    let side_move = match side_pick {
                        1 => 2,
                        2 => {
                            let choices: Vec<u8> = vec![4, 6];
                            let choice = choices.choose(&mut rng).unwrap();
                            *choice
                        },
                        3 => 8,
                        _ => {
                            println!("Tozzi's nuts.");
                            return 100
                        }
                    };
                    return side_move
                }else{
                    return 0
                }
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

    pub fn check_win(self, board: &Board) -> u8 {
        let available_moves = self.get_available(board);

        //Checks Across
        if board.computer_choices.contains(&1) && board.computer_choices.contains(&2) {
            if available_moves.contains(&3) {
                return 3
            }
        }
        if board.computer_choices.contains(&1) && board.computer_choices.contains(&3) {
            if available_moves.contains(&2) {
                return 2
            }
        }
        if board.computer_choices.contains(&2) && board.computer_choices.contains(&3) {
            if available_moves.contains(&1) {
                return 1
              }
        }
        if board.computer_choices.contains(&4) && board.computer_choices.contains(&5) {
            if available_moves.contains(&6) {
                return 6
              }
        }
        if board.computer_choices.contains(&4) && board.computer_choices.contains(&6) {
            if available_moves.contains(&5) {
                return 5
              }
        }
        if board.computer_choices.contains(&5) && board.computer_choices.contains(&6) {
            if available_moves.contains(&4) {
                return 4
              }
        }
        if board.computer_choices.contains(&7) && board.computer_choices.contains(&8) {
            if available_moves.contains(&9) {
                return 9
              }
        }
        if board.computer_choices.contains(&7) && board.computer_choices.contains(&9) {
            if available_moves.contains(&8) {
                return 8
              }
        }
        if board.computer_choices.contains(&8) && board.computer_choices.contains(&9) {
            if available_moves.contains(&7) {
                return 7
            }
        }
        
        //check diagonal
        if board.computer_choices.contains(&1) && board.computer_choices.contains(&5) {
            if available_moves.contains(&9) {
                return 9
            }
        }
        if board.computer_choices.contains(&1) && board.computer_choices.contains(&9) {
            if available_moves.contains(&5) {
                return 5
            }
        }
        if board.computer_choices.contains(&5) && board.computer_choices.contains(&9) {
            if available_moves.contains(&1) {
                return 1
            }
        }
        if board.computer_choices.contains(&3) && board.computer_choices.contains(&5) {
            if available_moves.contains(&7) {
                return 7
            }
        }
        if board.computer_choices.contains(&3) && board.computer_choices.contains(&7) {
            if available_moves.contains(&5) {
                return 5
            }
        }
        if board.computer_choices.contains(&5) && board.computer_choices.contains(&7) {
            if available_moves.contains(&3) {
                return 3
            }
        }

        // check down
        if board.computer_choices.contains(&1) && board.computer_choices.contains(&4) {
            if available_moves.contains(&7) {
                return 7
            }
        }
        if board.computer_choices.contains(&1) && board.computer_choices.contains(&7) {
            if available_moves.contains(&4) {
                return 4
            }
        }
        if board.computer_choices.contains(&4) && board.computer_choices.contains(&7) {
            if available_moves.contains(&1) {
                return 1
            }
        }
        if board.computer_choices.contains(&2) && board.computer_choices.contains(&5) {
            if available_moves.contains(&8) {
                return 8
            }
        }
        if board.computer_choices.contains(&2) && board.computer_choices.contains(&8) {
            if available_moves.contains(&5) {
                return 5
            }
        }
        if board.computer_choices.contains(&5) && board.computer_choices.contains(&8) {
            if available_moves.contains(&2) {
                return 2
            }
        }
        if board.computer_choices.contains(&3) && board.computer_choices.contains(&6) {
            if available_moves.contains(&9) {
                return 9
            }
        }
        if board.computer_choices.contains(&3) && board.computer_choices.contains(&9) {
            if available_moves.contains(&6) {
                return 6
            }
        }
        if board.computer_choices.contains(&6) && board.computer_choices.contains(&9) {
            if available_moves.contains(&3) {
                return 3
            }
        }
        return 0
    }

    pub fn block_win(self, board: &Board) -> u8 {
        let available_moves = self.get_available(board);

        //Checks Across
        if board.player_choices.contains(&1) && board.player_choices.contains(&2) {
            if available_moves.contains(&3) {
                return 3
            }
        }
        if board.player_choices.contains(&1) && board.player_choices.contains(&3) {
            if available_moves.contains(&2) {
                return 2
            }
        }
        if board.player_choices.contains(&2) && board.player_choices.contains(&3) {
            if available_moves.contains(&1) {
                return 1
            }
        }
        if board.player_choices.contains(&4) && board.player_choices.contains(&5) {
            if available_moves.contains(&6) {
                return 6
            }
        }
        if board.player_choices.contains(&4) && board.player_choices.contains(&6) {
            if available_moves.contains(&5) {
                return 5
            }
        }
        if board.player_choices.contains(&5) && board.player_choices.contains(&6) {
            if available_moves.contains(&4) {
                return 4
            }
        }
        if board.player_choices.contains(&7) && board.player_choices.contains(&8) {
            if available_moves.contains(&9) {
                return 9
            }
        }
        if board.player_choices.contains(&7) && board.player_choices.contains(&9) {
            if available_moves.contains(&8) {
                return 8
            }
        }
        if board.player_choices.contains(&8) && board.player_choices.contains(&9) {
            if available_moves.contains(&7) {
                return 7
            }
        }
        
        //check diagonal
        if board.player_choices.contains(&1) && board.player_choices.contains(&5) {
            if available_moves.contains(&9) {
                return 9
            }
        }
        if board.player_choices.contains(&1) && board.player_choices.contains(&9) {
            if available_moves.contains(&5) {
                return 5
            }
        }
        if board.player_choices.contains(&5) && board.player_choices.contains(&9) {
            if available_moves.contains(&1) {
                return 1
            }
        }
        if board.player_choices.contains(&3) && board.player_choices.contains(&5) {
            if available_moves.contains(&7) {
                return 7
            }
        }
        if board.player_choices.contains(&3) && board.player_choices.contains(&7) {
            if available_moves.contains(&5) {
                return 5
            }
        }
        if board.player_choices.contains(&5) && board.player_choices.contains(&7) {
            if available_moves.contains(&3) {
                return 3
            }
        }

        // check down
        if board.player_choices.contains(&1) && board.player_choices.contains(&4) {
            if available_moves.contains(&7) {
                return 7
            }
        }
        if board.player_choices.contains(&1) && board.player_choices.contains(&7) {
            if available_moves.contains(&4) {
                return 4
            }
        }
        if board.player_choices.contains(&4) && board.player_choices.contains(&7) {
            if available_moves.contains(&1) {
                return 1
            }
        }
        if board.player_choices.contains(&2) && board.player_choices.contains(&5) {
            if available_moves.contains(&8) {
                return 8
            }
        }
        if board.player_choices.contains(&2) && board.player_choices.contains(&8) {
            if available_moves.contains(&5) {
                return 5
            }
        }
        if board.player_choices.contains(&5) && board.player_choices.contains(&8) {
            if available_moves.contains(&2) {
                return 2
            }
        }
        if board.player_choices.contains(&3) && board.player_choices.contains(&6) {
            if available_moves.contains(&9) {
                return 9
            }
        }
        if board.player_choices.contains(&3) && board.player_choices.contains(&9) {
            if available_moves.contains(&6) {
                return 6
            }
        }
        if board.player_choices.contains(&6) && board.player_choices.contains(&9) {
            if available_moves.contains(&3) {
                return 3
            }
        }
        return 0
    }
}

#[derive(PartialEq, Debug)]
pub struct Board {
    row_1: Vec<char>,
    row_2: Vec<char>,
    row_3: Vec<char>,
    player_choices: Vec<u8>,
    computer_choices: Vec<u8>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            row_1: vec!['1', '2', '3'],
            row_2: vec!['4', '5', '6'],
            row_3: vec!['7', '8', '9'],
            player_choices: vec![],
            computer_choices: vec![],
        }
    }

    pub fn draw_board(&mut self) {
        println!("");
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

        if player.player_type == PlayerType::Human {
            match player.move_pick {
            1 => self.player_choices.push(1),
            2 => self.player_choices.push(2),
            3 => self.player_choices.push(3),
            4 => self.player_choices.push(4),
            5 => self.player_choices.push(5),
            6 => self.player_choices.push(6),
            7 => self.player_choices.push(7),
            8 => self.player_choices.push(8),
            9 => self.player_choices.push(9),
            _ => println!("Shit went bad populating player.choices.vec"),
            }
        }

        if player.player_type == PlayerType::Cpu {
            match player.move_pick {
            1 => self.computer_choices.push(1),
            2 => self.computer_choices.push(2),
            3 => self.computer_choices.push(3),
            4 => self.computer_choices.push(4),
            5 => self.computer_choices.push(5),
            6 => self.computer_choices.push(6),
            7 => self.computer_choices.push(7),
            8 => self.computer_choices.push(8),
            9 => self.computer_choices.push(9),
            _ => println!("Shit went bad populating computer.choices.vec"),
            }
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
