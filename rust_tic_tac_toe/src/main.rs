mod game;
use game::{Letter, Player, PlayerType};


fn main() {
    

    
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_players() {
        let human_player = Player{player_type: PlayerType::human, letter: Letter::X, move_pick: 1};
        let cpu_player = Player{player_type: PlayerType::cpu, letter: Letter::O, move_pick: 2};
        assert!(human_player.get_move());
        assert!(cpu_player.get_move());
    }
}