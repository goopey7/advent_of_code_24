use crate::utils;

fn are_levels_safe(levels: &Vec<i8>) -> bool {
    let mut is_increasing = false;
    let mut is_decreasing = false;
    for (level_idx, level) in levels.iter().enumerate() {
        if let Some(next_level) = levels.get(level_idx + 1) {
            is_increasing = is_increasing || level < next_level;
            is_decreasing = is_decreasing || level > next_level;
            let distance = (level - next_level).abs();
            let has_valid_spacing = distance >= 1 && distance <= 3;
            if (is_increasing && is_decreasing) || !has_valid_spacing {
                return false;
            }
        }
    }
    true
}

pub fn day_02() -> (String, String) {
    let input = utils::fs::read_aoc_input(2);
    let input = input.split_terminator('\n');
    let mut total_safe = 0;
    let mut total_dampened = 0;
    for line in input {
        let levels = line.split_terminator(' ');
        let levels = levels
            .map(|level: &str| level.parse().expect("unable to parse integer!"))
            .collect::<Vec<i8>>();

        if are_levels_safe(&levels) {
            total_safe += 1
        } else {
            for index_to_remove in 0..levels.len() {
                let mut level_clone = levels.clone();
                level_clone.remove(index_to_remove);
                if are_levels_safe(&level_clone) {
                    total_dampened += 1;
                    break;
                }
            }
        }
    }

    (
        total_safe.to_string(),
        (total_safe + total_dampened).to_string(),
    )
}
