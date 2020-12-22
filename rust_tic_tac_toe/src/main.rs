mod game;
use game::{Board, Letter, Player, PlayerType,};
use rand::prelude::*;


fn main() {
    let mut game_board = Board::new();
    game_board.draw_board();

    let mut human = Player::new_human();
    let computer = Player::new_computer(&human);
    human.move_pick = human.get_move();
    game_board = game_board.update(&human);
    println!("Player is {:?}", human);
    println!("Player is {:?}", computer);
    game_board.draw_board();

}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
// //     fn test_players() {
// //         let human_player = Player{player_type: PlayerType::human, letter: Letter::X, move_pick: 1};
// //         let cpu_player = Player{player_type: PlayerType::cpu, letter: Letter::O, move_pick: 2};
// //         assert_ne!(human_player.get_move(), 0);
// //         assert_eq!(cpu_player.get_move(), 0);
// //     }
// // }