use std::fs;

pub fn read_aoc_input(day: u8) -> String {
    let folder = "inputs";
    let path = format!("{folder}/day_{day:02}.txt");
    fs::read_to_string(&path).expect(format!("unable to read file: {path}").as_str())
}
