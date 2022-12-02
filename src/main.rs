fn main() {
    part1();
    part2();
}

fn part1() {
    let input = include_str!("../input.txt");
    let mut max = 0;
    let mut current = 0;
    for line in input.lines() {
        if let Ok(val) = line.parse::<u64>() {
            current += val;
            if current > max {
                max = current;
            }
        } else {
            current = 0;
        }
    }
    println!("Current: {}, Max: {}", current, max)
}

fn part2() {
    let input = include_str!("../input.txt");
    let mut totals: Vec<u64> = Vec::new();
    let mut current = 0;

    for line in input.lines() {
        if let Ok(val) = line.parse::<u64>() {
            current += val;
        } else {
            totals.push(current);
            current = 0;
        }
    }
    totals.sort_unstable();
    totals.reverse();
    totals.truncate(3);
    let total = totals.iter().sum::<u64>();
    dbg!(total);
}
