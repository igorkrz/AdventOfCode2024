advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let mut sum = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        if is_safe(nums.clone()) { sum += 1 }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut sum = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        if is_safe(nums.clone()) { sum += 1 }
        else { sum += brute_force(nums.clone()) }
    }

    Some(sum)
}

fn is_safe(nums: Vec<i32>) -> bool {
    let mut inc = false;
    let mut dec = false;

    for i in 0..nums.len() - 1 {
        let mut current = nums[i];
        let mut next = nums[i + 1];

        if current == next { return false }
        else if current > next { dec = true }
        else if next > current { inc = true }

        if (inc && dec) || (next - current).abs() > 3 {
            return false
        }
    }

    true
}

fn brute_force(nums: Vec<i32>) -> i32 {
    let len = nums.len();
    let mut nums_copy = nums.clone();

    for l in 0..len {
        nums_copy = nums.clone();
        nums_copy.remove(l);
        if is_safe(nums_copy) { return 1 }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
