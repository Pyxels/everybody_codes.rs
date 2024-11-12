use brunch::{benches, Bench};

use everybody_codes::day01;

benches!(
    Bench::new("day01::part1")
        .with_samples(10_000)
        .run(|| day01::part1(day01::INPUT_1)),
    Bench::new("day01::part2")
        .with_samples(10_000)
        .run(|| day01::part2(day01::INPUT_2)),
    Bench::new("day01::part3")
        .with_samples(10_000)
        .run(|| day01::part3(day01::INPUT_2)),
    Bench::spacer(),
);
