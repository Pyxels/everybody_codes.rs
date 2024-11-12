use everybody_codes::*;

macro_rules! run_day_parts {
    ($day_mod:ident: $($part_fn:ident),*) => {
        $(
            println!(
                "{}::{}: {}",
                stringify!($day_mod),
                stringify!($part_fn),
                $day_mod::$part_fn()
            );
        )*
        println!();
    };
}

fn main() {
    run_day_parts!(day01: run_part1, run_part2, run_part3);
}
