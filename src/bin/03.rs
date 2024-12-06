use advent_of_code::{match_pat, parse_number};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    fn solve(input: &str, acc: u32) -> u32 {
        if input.is_empty() {
            return acc;
        }

        if let Ok(input) = match_pat(input, "mul") {
            match parse_mul(input) {
                Ok((input, num1, num2)) => return solve(input, acc + num1 * num2),
                Err(input) => return solve(input, acc),
            }
        }

        solve(&input[1..], acc)
    }

    Some(solve(input, 0))
}

pub fn part_two(input: &str) -> Option<u32> {
    fn solve(input: &str, acc: u32) -> u32 {
        if input.is_empty() {
            return acc;
        }

        if let Ok(input) = match_pat(input, "don't()") {
            match input.find("do()") {
                Some(pos) => return solve(&input[pos + 4..], acc),
                None => return acc,
            }
        }

        if let Ok(input) = match_pat(input, "mul") {
            match parse_mul(input) {
                Ok((input, num1, num2)) => return solve(input, acc + num1 * num2),
                Err(input) => return solve(input, acc),
            }
        }

        solve(&input[1..], acc)
    }

    Some(solve(input, 0))
}

pub fn parse_mul(input: &str) -> Result<(&str, u32, u32), &str> {
    let input = match_pat(input, "(")?;
    let (input, num1) = parse_number::<u32>(input)?;
    let input = match_pat(input, ",")?;
    let (input, num2) = parse_number::<u32>(input)?;
    let input = match_pat(input, ")")?;

    Ok((input, num1, num2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(48));
    }
}
