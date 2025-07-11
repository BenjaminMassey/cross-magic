mod puzzle;

fn main() {
    let puzzles = puzzle::load();
    let square = puzzle::new(&puzzles);
    for word in square.across {
        println!("{word}");
    }
    println!("===");
    for word in square.down {
        println!("{word}");
    }
}
