use crate::utils;

fn get_total_from_muls(input: &str) -> u32 {
    let mut total = 0;
    let regex = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)").expect("invlaid regex");
    regex.captures_iter(&input).for_each(|captures| {
        captures.iter().for_each(|matched| {
            if let Some(matched) = matched {
                let matched = matched.as_str();
                let comma_idx = matched.find(",").expect("no comma in mul expr!");
                let a: u32 = matched[4..comma_idx].parse().unwrap();
                let b: u32 = matched[comma_idx + 1..matched.len() - 1].parse().unwrap();
                total += a * b;
            }
        });
    });
    total
}

pub fn day_03() -> (String, String) {
    let input = utils::fs::read_aoc_input(3);

    let mut enabled_total = 0;
    let split_instructions = input.split_terminator("do()");
    split_instructions.for_each(|instructions| {
        enabled_total += get_total_from_muls(
            &instructions[0..instructions.find("don't()").unwrap_or(instructions.len())],
        );
    });

    (
        get_total_from_muls(&input).to_string(),
        enabled_total.to_string(),
    )
}
