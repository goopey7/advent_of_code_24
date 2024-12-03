extern crate proc_macro;

use proc_macro2::{Punct, TokenStream};
use quote::{format_ident, quote, TokenStreamExt};

use std::{fs, io};

fn count_files_with_prefix(dir: &str, prefix: &str) -> io::Result<u8> {
    let mut count = 0;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if file_name_str.starts_with(prefix) {
            count += 1;
        }
    }

    Ok(count)
}

#[proc_macro]
pub fn days_completed(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let number_of_days = count_files_with_prefix("src/", "day_").unwrap();

    let mut mod_output: TokenStream = Default::default();
    let mut vec_output: TokenStream = Default::default();

    for day in 1..(number_of_days + 1) {
        let day_ident = format_ident!("day_{day:02}");

        mod_output.append(format_ident!("mod"));
        mod_output.append(day_ident.clone());
        mod_output.append(Punct::new(';', proc_macro2::Spacing::Alone));

        vec_output.append(day_ident.clone());
        vec_output.append(Punct::new(':', proc_macro2::Spacing::Joint));
        vec_output.append(Punct::new(':', proc_macro2::Spacing::Joint));
        vec_output.append(day_ident.clone());
        if day != number_of_days {
            vec_output.append(Punct::new(',', proc_macro2::Spacing::Alone));
        }
    }

    let output = quote![
        mod utils;
        #mod_output

        fn main() {
            let days = vec![ #vec_output ];
            for (mut day_idx, day_func) in days.iter().enumerate() {
                day_idx += 1;
                let (part_1, part_2) = day_func();
                println!("Day {day_idx}: (Part 1: {part_1}, Part 2: {part_2})");
            }
        }
    ];

    output.into()
}
