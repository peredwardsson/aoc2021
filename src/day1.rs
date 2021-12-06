#[aoc_generator(day1)]
fn ints(input: &str) -> Vec<u32> {
    input.lines().filter_map(|dep| dep.parse().ok()).collect()
}

#[aoc(day1, part1)]
pub fn pt1(input: &[u32]) -> usize {
    input.windows(2).filter(|w| w[0] < w[1]).count() 
}

#[aoc(day1, part2)]
pub fn pt2(input: &[u32]) -> usize {
    input
        .windows(3)
        .map(|w| w.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(
            |w| w[0] < w[1]
        )
        .count()
}