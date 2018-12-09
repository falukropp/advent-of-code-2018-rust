extern crate regex;

fn high_score(players: usize, marbles: usize) -> u64 {
    // Maybe init it to point to self for all instead of 0?
    let mut marble_chain = vec![(0usize, 0usize); marbles + 1];
    // 1-indexed to match example.
    let mut scores = vec![0u64; players + 1];
    let mut current_pos = 0usize;

    for marble in 1..=marbles {
        let current_player = marble % players;

        if marble % 23 != 0 {
            let mut one_step_clockwise = marble_chain[current_pos].1;
            let mut two_steps_clockwise = marble_chain[one_step_clockwise].1;

            marble_chain[marble].0 = one_step_clockwise;
            marble_chain[marble].1 = two_steps_clockwise;
            marble_chain[one_step_clockwise].1 = marble as usize;
            marble_chain[two_steps_clockwise].0 = marble as usize;

            current_pos = marble;
        } else {
            scores[current_player] += marble as u64;

            let mut pos = current_pos;
            let mut link = marble_chain[pos];
            for _ in 0..7 {
                pos = link.0;
                link = marble_chain[pos];
            }
            scores[current_player] += pos as u64;
            current_pos = link.1;

            // Remove the marble at pos. (By setting the previous marbles next to the next of the marble being remove and vice versa)
            marble_chain[link.0].1 = marble_chain[pos].1;
            marble_chain[link.1].0 = marble_chain[pos].0;
        }
        // println!("[{}] {} {:?}", current_player, current_pos, marble_chain);
    }
    *scores.iter().max().unwrap()
}

fn main() {
    println!("Result {:?}", high_score(9, 25)); // 32
    println!("Result {:?}", high_score(10, 1618)); // 8317
    println!("Result {:?}", high_score(13, 7999)); // 146373
    println!("Result {:?}", high_score(17, 1104)); // 2764
    println!("Result {:?}", high_score(21, 6111)); // 54718
    println!("Result {:?}", high_score(30, 5807)); // 37305

    // 9_1
    println!("Result {:?}", high_score(462, 71938)); // 398371

    // 9_2
    println!("Result {:?}", high_score(462, 7193800)); // 3212830280
}
