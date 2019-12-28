use std::fs;
use std::collections::HashMap;
use std::time::Instant;
use puzzle::Puzzle;

mod puzzle;

fn main() {
    let iters = std::env::args().nth(1).expect("no iters given").parse::<i32>().unwrap();
    let variation_map = words("/usr/share/dict/words/", 6);
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

fn words(path: &str, len: usize) -> HashMap<String, Vec<String>> {
    fs::read_to_string(path).expect("error reading word file")
        .split('\n')
        .filter(|s| s.len() == len || s.len() == len - 2)
        .map(|s| s.to_lowercase())
        .map(|e| (String::from(&e), Puzzle::make_variants(&e))).collect()
}