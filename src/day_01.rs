use crate::utils;

pub fn day_01() -> (String, String) {
    let input = utils::fs::read_aoc_input(1);
    let input = input.split_terminator('\n');
    let number_of_lines = input.clone().count();

    let mut lhs_list: Vec<i32> = Vec::new();
    lhs_list.reserve(number_of_lines);

    let mut rhs_list: Vec<i32> = Vec::new();
    rhs_list.reserve(number_of_lines);

    for line in input {
        let (left, right) = line
            .split_once("   ") // three spaces
            .expect("unexpected spacing between list items!");

        lhs_list.push(left.parse().expect("could not parse left list to i32!"));
        rhs_list.push(right.parse().expect("could not parse right list to i32!"));
    }

    assert!(
        lhs_list.len() == rhs_list.len(),
        "lists are of different lengths!"
    );

    lhs_list.sort_unstable();
    rhs_list.sort_unstable();

    let mut total_distance = 0;
    let mut similarity_score = 0;

    for line_idx in 0..number_of_lines {
        total_distance += (lhs_list[line_idx] - rhs_list[line_idx]).abs();

        similarity_score += if lhs_list.contains(&rhs_list[line_idx]) {
            rhs_list[line_idx]
        } else {
            0
        };
    }

    (total_distance.to_string(), similarity_score.to_string())
}
