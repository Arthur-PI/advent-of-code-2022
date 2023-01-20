use std::cmp::Reverse;

use color_eyre::eyre::Context;
use color_print::cprintln;
use itertools::Itertools;

#[allow(dead_code)]
fn part1(input: &str) {
    cprintln!("<bold>Part 1:</bold>");
    let lines = input
        .lines()
        .map(|line| line.parse::<u32>().ok())
        .collect::<Vec<_>>();
    let max_lead = lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<u32>())
        .max();
    cprintln!("Elven lead: <green>{:?}</green>", max_lead);
}

#[allow(dead_code)]
fn part1_itertools(input: &str) {
    cprintln!("<bold>Part 1:</bold>");
    let max_lead = input
        .lines()
        .map(|line| line.parse::<u32>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u32>())
        .max()
        .unwrap_or_default();
    cprintln!("Elven lead: <green>{}</green>", max_lead);
}

fn part2(input: &str) {
    cprintln!("\n<bold>Part 2:</bold>");
    let top3 = input
        .lines()
        .map(|line| line.parse::<u32>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u32>())
        .sorted_by_key(|&v| Reverse(v))
        .take(3)
        .sum::<u32>();
    cprintln!("Top 3 elven groups: <green>{}</green>", top3);
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let input = read_input()?;
    part1_itertools(&input);
    part2(&input);
    Ok(())
}

fn read_input() -> color_eyre::Result<String> {
    let filename = "input.txt";
    let input = std::fs::read_to_string(filename).wrap_err(format!("Reading '{filename}'"))?;
    Ok(input)
}
