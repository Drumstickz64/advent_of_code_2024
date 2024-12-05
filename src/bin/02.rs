use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    fn is_valid_report(report: &str) -> bool {
        let mut prev_diff = i32::MAX;
        for (x, y) in report.split_whitespace().tuple_windows() {
            let x: i32 = x.parse().unwrap();
            let y: i32 = y.parse().unwrap();
            let diff = y - x;
            if !(1..=3).contains(&diff.abs()) {
                return false;
            }

            if prev_diff == i32::MAX {
                prev_diff = diff;
                continue;
            }

            if prev_diff.signum() != diff.signum() {
                return false;
            }

            prev_diff = diff;
        }

        true
    }

    Some(
        input
            .lines()
            .filter(|report| is_valid_report(report))
            .count()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
