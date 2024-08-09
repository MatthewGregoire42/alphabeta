use alphabeta::amazons::*;

fn main() {
    let mut b = starting_board();
    print_board(&b);

    let starting_moves = moves(&b);
    println!("White has {} starting moves.", starting_moves.len());
    println!("Eval: {}", eval_board(&b));

    println!("Applying move...");

    let m = ((0, 6), (1, 6), (2, 6));
    apply_move(&mut b, &m);

    print_board(&b);

    let next_moves = moves(&b);

    println!("Black has {} moves.", next_moves.len());
    println!("Eval: {}", eval_board(&b));
}
