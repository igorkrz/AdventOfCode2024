advent_of_code::solution!(1);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i32> {
    let mut first_row = Vec::new();
    let mut second_row = Vec::new();

    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok());
        if let (Some(a), Some(b)) = (nums.next(), nums.next()) {
            first_row.push(a);
            second_row.push(b);
        }
    }

    first_row.sort();
    second_row.sort();

    let result: i32 = first_row
        .iter()
        .zip(second_row.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut first_row = Vec::new();
    let mut second_row = Vec::new();

    for line in input.lines() {
        let mut nums = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok());
        if let (Some(a), Some(b)) = (nums.next(), nums.next()) {
            first_row.push(a);
            second_row.push(b);
        } else {
            return None;
        }
    }

    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    for num in second_row {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = first_row
        .iter()
        .map(|&num| num * frequency_map.get(&num).unwrap_or(&0)) // Multiply num by its frequency
        .sum();

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
