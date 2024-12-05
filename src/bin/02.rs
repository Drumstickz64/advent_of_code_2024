use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    fn is_safe_report(report: &str) -> bool {
        let mut prev_diff = i32::MAX;
        for (level1, level2) in report.split_whitespace().tuple_windows() {
            let level1: i32 = level1.parse().unwrap();
            let level2: i32 = level2.parse().unwrap();
            let diff = level2 - level1;
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
            .filter(|report| is_safe_report(report))
            .count()
            .try_into()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    fn is_safe_report(report: &str) -> bool {
        let mut prev_diff = i32::MAX;
        let mut has_encounter_unsafe = false;
        for (level1, level2) in report.split_whitespace().tuple_windows() {
            let level1: i32 = level1.parse().unwrap();
            let level2: i32 = level2.parse().unwrap();
            let diff = level2 - level1;

            match (!is_safe_level(diff, prev_diff), has_encounter_unsafe) {
                (true, true) => return false,
                (true, false) => has_encounter_unsafe = true,
                _ => (),
            }

            prev_diff = diff;
        }

        true
    }

    fn is_safe_level(diff: i32, prev_diff: i32) -> bool {
        if !(1..=3).contains(&diff.abs()) {
            return false;
        }

        if prev_diff == i32::MAX {
            return true;
        }

        if prev_diff.signum() != diff.signum() {
            return false;
        }

        true
    }

    Some(
        input
            .lines()
            .filter(|report| is_safe_report(report))
            .count()
            .try_into()
            .unwrap(),
    )
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
