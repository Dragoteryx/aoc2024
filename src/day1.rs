use std::collections::HashMap;
use super::*;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
	input
		.lines()
		.fold((Vec::new(), Vec::new()), |(mut left, mut right), line| {
			let mut parts = line.split_whitespace();
			let a = parts.next().unwrap().parse().unwrap();
			let b = parts.next().unwrap().parse().unwrap();
			left.push(a);
			right.push(b);
			(left, right)
		})
}

#[aoc(day1, part1)]
pub fn part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
	let (mut left, mut right) = input.clone();

	left.sort();
	right.sort();

	left.into_iter()
		.zip(right)
		.map(|(a, b)| a.abs_diff(b))
		.sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
	let occurences = input.1.iter().fold(HashMap::new(), |mut map, &x| {
		*map.entry(x).or_insert(0) += 1;
		map
	});

	input.0.iter()
		.map(|x| x * occurences.get(x).unwrap_or(&0))
		.sum()
}
