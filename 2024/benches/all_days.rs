use brunch::{benches, Bench};

use everybody_codes::*;

macro_rules! bench_day_parts {
    ($($day_mod:ident: $samples:expr => $($part_fn:ident),*);* $(;)?) => {
        benches!(
            $(
                $(
                    Bench::new(concat!(stringify!($day_mod), "::", stringify!($part_fn)))
                        .with_samples($samples)
                        .run(|| $day_mod::$part_fn()),
                )*
                Bench::spacer(),
            )*
        );
    };
}

bench_day_parts! {
    day01: 10_000 => run_part1, run_part2, run_part3;
}
