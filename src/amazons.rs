use std::{collections::{HashSet, VecDeque}, isize};

const S: usize = 10;

type Square = (usize, usize);
type Move = (Square, Square, Square); // Start square, destination square, arrow square
#[derive(Clone)]
pub enum Player { Black, White }

#[derive(Clone)]
pub struct Board {
    pub black: HashSet<Square>,
    pub white: HashSet<Square>,
    pub arrows: HashSet<Square>,
    pub player: Player // Player whose turn it is to move
}

pub fn moves(board: &Board) -> Vec<Move> {
    let (pieces, enemy_pieces) = match board.player {
        Player::Black => (&board.black, &board.white),
        Player::White => (&board.white, &board.black)
    };

    let mut moves = VecDeque::<Move>::new();

    // For each piece, see how it can move
    for p in pieces {
        let dests = reachable(&board, &p);
        // For each destination, see where the arrow can go
        for d in dests {
            let trial = test_half_move(board, p, &d);

            let arrows = reachable(&trial, &d);

            // Add each combination to the moves list
            for a in arrows {
                let m = (p.clone(), d, a);

                // Heuristic: if the move shoots an arrow close to an enemy piece, it's likely good
                let mut good = false;
                for e in enemy_pieces {
                    if adjacent(&a, &e) {
                        good = true;
                        break;
                    }
                }
                if good {
                    moves.push_front(m);
                } else {
                    moves.push_back(m);
                }
            }
        }
    }

    moves.into_iter().collect()
}

pub fn adjacent(sq1: &Square, sq2: &Square) -> bool {
    let (x1, y1) = (isize::try_from(sq1.0).unwrap(), isize::try_from(sq1.1).unwrap());
    let (x2, y2) = (isize::try_from(sq2.0).unwrap(), isize::try_from(sq2.1).unwrap());

    (x2 - x1).abs() <= 1 && (y2 - y1).abs() <= 1
}

pub fn eval_board(b: &Board) -> isize {
    let mut b_copy = b.clone();

    b_copy.player = Player::White;
    let w_moves = moves(&b_copy).len();

    b_copy.player = Player::Black;
    let b_moves = moves(&b_copy).len();

    isize::try_from(w_moves).unwrap() - isize::try_from(b_moves).unwrap()
}

pub fn reachable(board: &Board, coord: &Square) -> Vec<Square> {
    let x_incr = [-1, 0, 1];
    let y_incr = [-1, 0, 1];

    let mut squares = Vec::<Square>::new();

    for x_dir in x_incr {
        for y_dir in y_incr {
            if x_dir == 0 && y_dir == 0 {
                continue;
            }

            // Go in a straight line in the specified direction until we can't anymore.
            let mut curr = [(coord.0 as isize) + x_dir, (coord.1 as isize) + y_dir];
            while 0 <= curr[0] && curr[0] < S as isize && 0 <= curr[1] && curr[1] < S as isize {
                let square = (curr[0] as usize, curr[1] as usize);
                // Square is filled, stop here
                if board.arrows.contains(&square)
                    || board.white.contains(&square)
                    || board.black.contains(&square) {
                        break;
                    }
                else {
                    squares.push(square);
                }

                curr = [curr[0] + x_dir, curr[1] + y_dir];
            }
        }
    }

    squares
}

pub fn apply_move(board: &mut Board, m: &Move) {
    let (src, dest, arrow) = m;

    if board.black.contains(&src) {
        board.black.remove(&src);
        board.black.insert(dest.clone());
        board.player = Player::White;
    } else if board.white.contains(&src) {
        board.white.remove(&src);
        board.white.insert(dest.clone());
        board.player = Player::Black;
    } else {
        panic!("Invalid move");
    }

    board.arrows.insert(arrow.clone());
}

pub fn test_move(board: &Board, m: &Move) -> Board {
    let mut new_board = board.clone();

    apply_move(&mut new_board, &m);

    new_board
}

pub fn test_half_move(board: &Board, src: &Square, dest: &Square) -> Board {
    let mut new_board = board.clone();

    if new_board.black.contains(&src) {
        new_board.black.remove(&src);
        new_board.black.insert(dest.clone());
    } else if new_board.white.contains(&src) {
        new_board.white.remove(&src);
        new_board.black.insert(dest.clone());
    } else {
        panic!("Invalid move");
    }

    new_board
}

pub fn minimax(pos: Board, depth: usize, mut alpha: isize, mut beta: isize) -> (isize, Move) {
    let mut best_move = ((0, 0), (0, 0), (0, 0));

    if depth == 0 {
        return (eval_board(&pos), best_move)
    }

    let white = match pos.player {
        Player::Black => false,
        Player::White => true
    };

    if white {
        let mut max_eval = isize::MIN;
        let moves = moves(&pos);

        // let mut i = 0;
        // let n_moves = moves.len();
        for m in moves {

            // if depth == 2 {
            //     // println!("Total moves: {}  Explored: {}", n_moves, i);
            // }

            let child = test_move(&pos, &m);
            let (eval, _) = minimax(child, depth-1, alpha, beta);

            // We've found a better move for white
            if eval > max_eval {
                max_eval = eval;
                best_move = m;
            }

            alpha = std::cmp::max(alpha, eval);

            if beta <= alpha {
                break;
            }

            // i += 1;
        }
        (max_eval, best_move)

    } else {
        let mut min_eval = isize::MAX;
        let moves = moves(&pos);

        for m in moves {
            let child = test_move(&pos, &m);
            let (eval, _) = minimax(child, depth-1, alpha, beta);

            // We've found a better move for black
            if eval < min_eval {
                min_eval = eval;
                best_move = m;
            }

            beta = std::cmp::min(beta, eval);

            if beta <= alpha {
                break
            }
        }
        (beta, best_move)
    }
}

pub fn decide_move(pos: &Board) -> Move {
    let (_eval, best_move) = minimax(pos.clone(), 2, isize::MIN, isize::MAX);
    best_move
}

pub fn print_board(board: &Board) {
    let mut out = [["  "; S]; S];
    for w in &board.white {
        out[w.1][w.0] = "W ";
    }
    for b in &board.black {
        out[b.1][b.0] = "B ";
    }
    for a in &board.arrows {
        out[a.1][a.0] = ". ";
    }

    for i in 0..S {
        let mut s: String = "".to_owned();
        for j in 0..S {
            s = s + out[i][j];
        }
        println!("{}", s);
    }
}

pub fn starting_board() -> Board {

    let mut b = Board {
        black: HashSet::new(),
        white: HashSet::new(),
        arrows: HashSet::new(),
        player: Player::White
    };

    b.black.insert((3, 0));
    b.black.insert((6, 0));
    b.black.insert((0, 3));
    b.black.insert((9, 3));

    b.white.insert((0, 6));
    b.white.insert((9, 6));
    b.white.insert((3, 9));
    b.white.insert((6, 9));

    b
}