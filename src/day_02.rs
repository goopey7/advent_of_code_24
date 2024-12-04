use crate::utils;

fn are_levels_safe(levels: &Vec<i8>) -> bool {
    let mut has_increasing = false;
    let mut has_decreasing = false;
    let mut has_valid_spacing = true;
    for (level_idx, level) in levels.iter().enumerate() {
        if let Some(next_level) = levels.get(level_idx + 1) {
            has_increasing = has_increasing || level < next_level;
            has_decreasing = has_decreasing || level > next_level;
            let distance = (level - next_level).abs();
            has_valid_spacing = has_valid_spacing && distance >= 1 && distance <= 3;
        }
    }
    (has_increasing ^ has_decreasing) && has_valid_spacing
}

pub fn day_02() -> (String, String) {
    let input = utils::fs::read_aoc_input(2);
    let input = input.split_terminator('\n');
    let mut total_safe = 0;
    for line in input {
        let level_strings = line.split_terminator(' ');
        let levels = level_strings
            .map(|level: &str| level.parse().expect("unable to parse integer!"))
            .collect::<Vec<i8>>();

        if are_levels_safe(&levels) {
            total_safe += 1;
        }
    }

    (total_safe.to_string(), "".to_string())
}
