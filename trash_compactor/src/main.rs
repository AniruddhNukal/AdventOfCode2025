use trash_compactor::*;

fn main() -> anyhow::Result<()> {
    let contents = read_file("trash_compactor/input.txt")?;
    let operations = parse_file_2(contents)?;

    let total = operations.iter().map(Operation::evaluate).sum::<u64>();

    println!("{total:?}");

    Ok(())
}
