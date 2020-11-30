use rand::Rng;

#[derive(Debug, Copy, Clone, Eq, PartialEq)] //Removed "Display" for now because error.
pub enum Player {
    X,
    O
}

impl Player {
    pub fn opponent(&self) -> Player {
        match *self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
#[derive(Copy, Clone)]
pub struct Board {
    pub fields: [[Option<Player>; 3]; 3],
    pub next_player: Player,
}

impl Board {
    pub fn new(&self) -> Board {
        let mut rng = rand::thread_rng();
        let first_player = rng.gen_range(1, 2);
        let first_player = match first_player {
            1 => Player::O,
            2 => Player::X,
            _ => Player::O,
        };
        Board {
            fields: {[
                [None, None, None],
                [None, None, None],
                [None, None, None],
            ]},
            next_player: first_player,
        }
    }



    // update

    // show

    // check_board_free

    // check_board_win

    // check_board_draw

}