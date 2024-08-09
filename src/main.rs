use alphabeta::amazons::*;

fn main() {
    let mut b = starting_board();
    // print_board(&b);

    let mut n_moves = 1;

    while n_moves > 0 {
        print_board(&b);
        println!("Eval: {}", eval_board(&b));

        let m = decide_move(&b);
        apply_move(&mut b, &m);

        n_moves = moves(&b).len();
    }

    // let starting_moves = moves(&b);
    // println!("White has {} starting moves.", starting_moves.len());
    // println!("Eval: {}", eval_board(&b));

    // println!("Applying move...");

    // let m = decide_move(&b);
    // apply_move(&mut b, &m);

    // print_board(&b);

    // let next_moves = moves(&b);

    // println!("Black has {} moves.", next_moves.len());
    // println!("Eval: {}", eval_board(&b));
}
