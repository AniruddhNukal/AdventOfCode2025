use laboratories::*;

use std::io::{self, Write};

fn pause() {
    let mut stdout = io::stdout();
    print!("Press Enter to continue...");
    stdout.flush().unwrap();
    
    let mut _unused = String::new();
    io::stdin().read_line(&mut _unused).unwrap();
}

fn main() -> anyhow::Result<()> {
    let contents = read_file("laboratories/input.txt")?;
    let mut map = parse_file::<u64>(contents)?;

    // println!("{map}");

    while map.propagate() != map {
        // pause();
        map = map.propagate();
        // println!("{map}");
    }

    println!("Final splits: {}", map.get_splits());

    Ok(())
}
