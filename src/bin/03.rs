use std::{fmt::Debug, str::FromStr};

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

fn parse_mul(input: &str) -> Result<(&str, u32, u32), &str> {
    let input = match_pat(input, "(")?;
    let (input, num1) = parse_number::<u32>(input)?;
    let input = match_pat(input, ",")?;
    let (input, num2) = parse_number::<u32>(input)?;
    let input = match_pat(input, ")")?;

    Ok((input, num1, num2))
}

fn parse_number<T>(input: &str) -> Result<(&str, T), &str>
where
    T: FromStr,
    T::Err: Debug,
{
    let mut curr = 0;
    while input.as_bytes()[curr].is_ascii_digit() {
        curr += 1;
    }

    if curr == 0 {
        return Err(input);
    }

    Ok((&input[curr..], input[..curr].parse().unwrap()))
}

fn match_pat<'input>(input: &'input str, pat: &str) -> Result<&'input str, &'input str> {
    if !input.starts_with(pat) {
        return Err(input);
    }

    Ok(&input[pat.len()..])
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
