// Instruction represents 
struct Instruction {
    dir: Direction,
    steps: u8,
}

// Direction is an enum
// represents the direction the instruction says to turn.
enum Direction {
    Left,
    Right,
}

fn main() {
    println!("Hello, world!");
}
