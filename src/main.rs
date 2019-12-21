#![warn(clippy::all)]

use std::collections::HashSet;
use std::fmt;

#[allow(non_snake_case)]
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Board {
    //       TL---TM---TR
    //      / \        / \
    //     /   \      /   \
    //    /     \    /     \
    //   /       \  /       \
    // L/         MID        \R
    //  \         /\         /
    //   \       /  \       /
    //    \     /    \     /
    //     \   /      \   /
    //      \DL---DM---DR/
    TL: Kuhle,
    TM: Kuhle,
    TR: Kuhle,

    DL: Kuhle,
    DM: Kuhle,
    DR: Kuhle,

    L: Kuhle,
    R: Kuhle,

    MID: Kuhle,

    game_state: GameState,
}

impl Board {
    fn new() -> Self {
        Board {
            TL: Kuhle::METALL(Position::TL),
            TM: Kuhle::EMPTY(Position::TM),
            TR: Kuhle::GLAS(Position::TR),

            DL: Kuhle::METALL(Position::DL),
            DM: Kuhle::EMPTY(Position::DM),
            DR: Kuhle::GLAS(Position::DR),

            L: Kuhle::METALL(Position::L),
            R: Kuhle::GLAS(Position::R),

            MID: Kuhle::GROSS(Position::MID),

            game_state: GameState::KleinMuss, // am Anfang kann die grosse nicht, ein kleiner muss gezogen werden
        }
    }

    fn iter(&self) -> BoardIter {
        BoardIter {
            curr: 0,
            board: self,
        }
    }
}

fn move_pos_from_to(board: Board, old_pos: Position, empty: Kuhle) -> Board {
    let mut ret = board.clone();

    let new_pos = match empty {
        Kuhle::EMPTY(pos) => pos,
        _ => unimplemented!(),
    };

    let old_kuhle_type_with_old_pos = match old_pos {
        Position::TL => board.TL,
        Position::TM => board.TM,
        Position::TR => board.TR,
        Position::DL => board.DL,
        Position::DM => board.DM,
        Position::DR => board.DR,
        Position::L => board.L,
        Position::R => board.R,
        Position::MID => board.MID,
    };

    let new_kuhle = match old_kuhle_type_with_old_pos {
        Kuhle::METALL(_) => Kuhle::METALL(new_pos),
        Kuhle::GLAS(_) => Kuhle::GLAS(new_pos),
        Kuhle::GROSS(_) => Kuhle::GROSS(new_pos),
        _ => unimplemented!(),
    };

    // set kuhle at new pos
    use Position::*;
    match new_pos {
        TL => ret.TL = new_kuhle,
        TM => ret.TM = new_kuhle,
        TR => ret.TR = new_kuhle,

        DL => ret.DL = new_kuhle,
        DM => ret.DM = new_kuhle,
        DR => ret.DR = new_kuhle,

        L => ret.L = new_kuhle,
        R => ret.R = new_kuhle,

        MID => ret.MID = new_kuhle,
    }

    // reset kuhle at old pos
    match old_pos {
        TL => ret.TL = Kuhle::EMPTY(TL),
        TM => ret.TM = Kuhle::EMPTY(TM),
        TR => ret.TR = Kuhle::EMPTY(TR),

        DL => ret.DL = Kuhle::EMPTY(DL),
        DM => ret.DM = Kuhle::EMPTY(DM),
        DR => ret.DR = Kuhle::EMPTY(DR),

        L => ret.L = Kuhle::EMPTY(L),
        R => ret.R = Kuhle::EMPTY(R),

        MID => ret.MID = Kuhle::EMPTY(MID),
    }

    // update game_state
    match ret.game_state {
        GameState::GrossMuss => {
            ret.game_state = GameState::KleinMuss;
        }
        GameState::KleinMuss => {
            ret.game_state = GameState::GrossMuss;
        }
    }

    ret
}

struct BoardIter<'a> {
    curr: i8,
    board: &'a Board,
}

impl<'a> Iterator for BoardIter<'a> {
    type Item = Kuhle;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<Kuhle> {
        match self.curr {
            0 => {
                self.curr = 1;
                Some(self.board.TL)
            }
            1 => {
                self.curr = 2;
                Some(self.board.TM)
            }
            2 => {
                self.curr = 3;
                Some(self.board.TR)
            }
            3 => {
                self.curr = 4;
                Some(self.board.DL)
            }
            4 => {
                self.curr = 5;
                Some(self.board.DM)
            }
            5 => {
                self.curr = 6;
                Some(self.board.DR)
            }
            6 => {
                self.curr = 7;
                Some(self.board.L)
            }
            7 => {
                self.curr = 8;
                Some(self.board.R)
            }
            8 => {
                self.curr = 9;
                Some(self.board.MID)
            }
            9 => None,
            _ => unimplemented!(),
        }
    }
}

impl fmt::Display for Kuhle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Kuhle::EMPTY(_) => write!(f, "E "),
            Kuhle::GLAS(_) => write!(f, "G "),
            Kuhle::METALL(_) => write!(f, "M "),
            Kuhle::GROSS(_) => write!(f, "GR"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Kuhle {
    EMPTY(Position),
    GLAS(Position),
    METALL(Position),
    GROSS(Position),
}

#[allow(non_snake_case)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Position {
    TL,
    TM,
    TR,

    DL,
    DM,
    DR,

    L,
    R,

    MID,
}

fn print_board(board: &Board) {
    println!(
        r#"           {}---{}---{}
          / \        / \
         /   \      /   \
        /     \    /     \
       /       \  /       \
    {}/         {}         \{}
      \         /\         /
       \       /  \       /
        \     /    \     /
         \   /      \   /
          \{}---{}---{}/"#,
        board.TL, board.TM, board.TR, board.L, board.MID, board.R, board.DL, board.DM, board.DR
    );

    println!("{:?}", board.game_state)
}

fn possible_next_moves(board: &Board) -> Vec<Board> {
    let mut ret = Vec::<Board>::new();
    let empty = board.iter().filter(|&k| match k {
        Kuhle::EMPTY(_) => true,
        _ => false,
    });

    for e in empty {
        match e {
            Kuhle::EMPTY(pos) => {
                for neighbour in neighbors(pos, &board) {
                    let mut new_board = board.clone();
                    // TODO modify board
                    new_board = move_pos_from_to(new_board, neighbour, e);

                    ret.push(new_board);
                }
            }
            _ => unimplemented!(),
        }
    }

    ret
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    GrossMuss,
    KleinMuss,
}

//       TL---TM---TR
//      / \        / \
//     /   \      /   \
//    /     \    /     \
//   /       \  /       \
// L/         MID        \R
//  \         /\         /
//   \       /  \       /
//    \     /    \     /
//     \   /      \   /
//      \DL---DM---DR/
fn neighbors(position: Position, board: &Board) -> Vec<Position> {
    let ret: Vec<Position>;

    use Position::*;
    match position {
        TL => ret = vec![TM, L, MID],
        TM => ret = vec![TL, TR],
        TR => ret = vec![TM, MID, R],

        DL => ret = vec![L, MID, DM],
        DM => ret = vec![DL, DR],
        DR => ret = vec![DM, MID, R],

        L => ret = vec![TL, DL],
        R => ret = vec![TR, DR],

        MID => ret = vec![TL, TR, DL, DR],
    }

    // empty neighbors are discarded because an empty thing can not be moved
    let ret = ret.into_iter().filter(|&pos| !pos_empty(pos, board));

    match board.game_state {
        GameState::GrossMuss => ret.filter(|&pos| pos_has_gross(pos, board)).collect(),
        GameState::KleinMuss => ret.filter(|&pos| !pos_has_gross(pos, board)).collect(),
    }
}

fn pos_empty(pos: Position, board: &Board) -> bool {
    let old_kuhle_type_with_old_pos = match pos {
        Position::TL => board.TL,
        Position::TM => board.TM,
        Position::TR => board.TR,
        Position::DL => board.DL,
        Position::DM => board.DM,
        Position::DR => board.DR,
        Position::L => board.L,
        Position::R => board.R,
        Position::MID => board.MID,
    };

    match old_kuhle_type_with_old_pos {
        Kuhle::METALL(_) => false,
        Kuhle::GLAS(_) => false,
        Kuhle::GROSS(_) => false,
        Kuhle::EMPTY(_) => true,
    }
}

fn pos_has_gross(pos: Position, board: &Board) -> bool {
    let old_kuhle_type_with_old_pos = match pos {
        Position::TL => board.TL,
        Position::TM => board.TM,
        Position::TR => board.TR,
        Position::DL => board.DL,
        Position::DM => board.DM,
        Position::DR => board.DR,
        Position::L => board.L,
        Position::R => board.R,
        Position::MID => board.MID,
    };

    match old_kuhle_type_with_old_pos {
        Kuhle::METALL(_) => false,
        Kuhle::GLAS(_) => false,
        Kuhle::GROSS(_) => true,
        _ => unimplemented!(),
    }
}

//       TL---TM---TR
//      / \        / \
//     /   \      /   \
//    /     \    /     \
//   /       \  /       \
// L/         MID        \R
//  \         /\         /
//   \       /  \       /
//    \     /    \     /
//     \   /      \   /
//      \DL---DM---DR/
fn done(board: &Board) -> bool {
    board.TL == Kuhle::GLAS(Position::TL)
        && board.TM == Kuhle::EMPTY(Position::TM)
        && board.TR == Kuhle::METALL(Position::TR)
        && board.DL == Kuhle::GLAS(Position::DL)
        && board.DM == Kuhle::EMPTY(Position::DM)
        && board.DR == Kuhle::METALL(Position::DR)
        && board.L == Kuhle::GLAS(Position::L)
        && board.R == Kuhle::METALL(Position::R)
        && board.MID == Kuhle::GROSS(Position::MID)
}

fn main() {
    let mut set: HashSet<Board> = HashSet::<Board>::new();
    set.insert(Board::new());

    let mut is_done = false;
    while !is_done {
        let mut new_set: HashSet<Board> = HashSet::<Board>::new();
        for val in set.iter() {
            if done(&val) {
                is_done = true;
                println!("YEAH DONE",);
                print_board(&val);
                break;
            }
            for b in possible_next_moves(&val).iter() {
                new_set.insert(b.clone());
            }
        }
        set = set.union(&new_set).cloned().collect();

        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>\n\n\n");
        let gross_mid = set.clone().into_iter().filter(|e| {
            e.MID == Kuhle::GROSS(Position::MID)
                && e.L == Kuhle::GLAS(Position::L)
                && e.TL == Kuhle::GLAS(Position::TL)
                && e.DL == Kuhle::GLAS(Position::DL)
                && e.R == Kuhle::METALL(Position::R)
                && e.TR == Kuhle::METALL(Position::TR)
                && e.DR == Kuhle::METALL(Position::DR)
        });
        for val in gross_mid {
            print_board(&val);
        }
        println!("at some point size does not increase anymore {}", set.len());
    }
}
