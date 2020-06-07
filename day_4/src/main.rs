mod password;
use rayon::prelude::*;

fn main() {
    println!("Hello, world!");

    println!(
        "There are {} valid passwords between 273025 and 767253",
        (273025u32..767253u32)
            .collect::<Vec<u32>>()
            .par_iter()
            .filter(|password| password::is_valid(password))
            .count()
    );
}
