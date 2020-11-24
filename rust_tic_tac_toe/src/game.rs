#[derive(Display, Debug, Copy, Clone, Eq, PartialEq)]
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
        Board {
            fields: {[
                [None, None, None],
                [None, None, None],
                [None, None, None],
            ]},
            next_player: first_player,
        }
    }

    update

    show

    check_board_free

    check_board_win

    check_board_draw

    pick_first_player



}