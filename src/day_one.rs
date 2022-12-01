use itertools::Itertools;

/// Find the Elf carrying the most Calories.
pub fn get_best_meal(input: &'static str) -> Option<u32> {
    iter_calories(input).max()
}

/// Find the top three Elves carrying the most Calories.
pub fn get_top_three_meals(input: &'static str) -> u32 {
    iter_calories(input).sorted().rev().take(3).sum()
}

fn iter_calories(input: &'static str) -> impl Iterator<Item = u32> {
    input.split("\n\n").map(|meal| {
        meal.lines()
            .map(|calories| calories.parse::<u32>())
            .filter_map(Result::ok)
            .sum()
    })
}

#[cfg(test)]
mod test {
    const DATA: &str = include_str!("../assets/day_one");

    const TEST_DATA: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    use crate::day_one::{get_best_meal, get_top_three_meals};

    #[test]
    fn exercise_one_test_data() {
        let solution = get_best_meal(TEST_DATA);
        assert_eq!(solution, Some(24000));
    }

    #[test]
    fn exercise_one() {
        let solution = get_best_meal(DATA);
        assert_eq!(solution, Some(70509));
    }

    #[test]
    fn exercise_two_test_data() {
        let solution = get_top_three_meals(TEST_DATA);
        assert_eq!(solution, 45000);
    }

    #[test]
    fn exercise_two() {
        let solution = get_best_meal(DATA);
        assert_eq!(solution, Some(70509));
    }
}
