use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut list1, mut list2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| line.split("   ").collect_tuple::<(_, _)>().unwrap())
        .map(|(string1, string2)| {
            (
                string1.parse::<i32>().unwrap(),
                string2.parse::<i32>().unwrap(),
            )
        })
        .unzip();

    log::debug!("List 1 = {list1:?}");
    log::debug!("List 2 = {list2:?}");

    list1.sort();
    list2.sort();

    log::debug!("List 1 sorted = {list1:?}");
    log::debug!("List 2 sorted = {list2:?}");

    Some(
        list1
            .into_iter()
            .zip(list2)
            .map(|(num1, num2)| (num2 - num1).abs())
            .sum::<i32>()
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
