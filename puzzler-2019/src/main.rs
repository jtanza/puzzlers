use std::time::Instant;
use puzzle::Puzzle;

mod puzzle;

/// # Program Arguments
///
/// * [0] - path to a newline separated dictionary of words
/// * [1] - number of puzzles you'd like to generate
pub fn main() {
    let variation_map = Puzzle::build_var_map(std::env::args().nth(1).expect("no path given").as_str());
    let iters = std::env::args().nth(2).expect("no iters given").parse::<i32>().unwrap();
    let candidates = variation_map.keys().cloned().collect::<Vec<String>>();
    let mut total = 0;

    for _ in 0..iters {
        let mut puzzle = Puzzle::new(&candidates);
        let sw = Instant::now();

        while puzzle.not_complete() {
            let word = Puzzle::select_any(&candidates);
            if let Some(fit) = puzzle.find_fit(word, variation_map.get(word).unwrap()) {
                puzzle.update(word, fit.0, fit.1);
            }
        }

        total += sw.elapsed().as_millis();
        println!("\n{}\nPuzzle completed in: {}ms", puzzle, sw.elapsed().as_millis());
    }
    println!("\nCompleted {} iterations in {}ms. Average: {}ms.", iters, total, total / iters as u128);
}