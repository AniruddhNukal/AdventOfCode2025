use cafeteria::*;

fn main() -> anyhow::Result<()> {
    let contents = read_file("cafeteria/input.txt")?;
    let (ranges, values) = parse_file(contents)?;
    // println!("{:?}", ranges);
    // println!("{:?}", values);

    let mut tree = func_tree::RangeTree::new();
    // println!("{:?}", tree);
    for i in ranges {
        tree = tree.add(i);
        // println!("{:?}", tree);
    }

    let mut count = 0;
    for i in values {
        // println!("Found value {i}: {}", tree.search(i))
        count += tree.contains(i) as i32;
    }

    println!("{count}");

    let span = tree.span();

    println!("{span}");

    Ok(())
}
