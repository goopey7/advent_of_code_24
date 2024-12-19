use crate::utils;

pub fn day_03() -> (String, String) {
    let input = utils::fs::read_aoc_input(3);
    let regex = regex::Regex::new(r"mul\(\d{1,3},\d{1,3}\)").expect("invlaid regex");

    let mut total = 0;

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

    (total.to_string(), String::new())
}
