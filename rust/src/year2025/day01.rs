///! --- Day 1: Secret Entrance ---

pub fn solve(input: &str) {
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

pub fn part1(input: &str) -> i32 {
    count_zeros(input).0
}

pub fn part2(input: &str) -> i32 {
    count_zeros(input).1
}

fn count_zeros(input: &str) -> (i32, i32) {
    let mut dial = 50;
    let mut landings = 0;
    let mut crossings = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (direction, value_str) = line.split_at(1);
        let value = value_str.parse::<i32>().expect("Invalid number");

        if direction == "R" {
            crossings += (dial + value) / 100;
            dial = (dial + value) % 100;
        } else if direction == "L" {
            // We can simply symmetrize the dial relative to 50 and count the number of
            // crossings as if the rotation was to the right.
            crossings += ((100 - dial) % 100 + value) / 100;
            dial = (dial - value).rem_euclid(100);
        } else {
            panic!("Invalid direction: {}", direction);
        }

        landings += (dial == 0) as i32
    }

    (landings, crossings)
}
