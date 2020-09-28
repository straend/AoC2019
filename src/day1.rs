use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn fuel_for(mass: i32, for_fuel: bool) -> i32 {
    let fuel = mass / 3 - 2;
    match (mass, for_fuel) {
        (0..=9, _) => 0,
        (_, false) => fuel,
        (_, true)  => fuel + fuel_for(fuel, for_fuel),
    }
}

#[test]
fn t_fuel_for() {
    assert_eq!(2, fuel_for(12, false));
    assert_eq!(2, fuel_for(14, false));
    assert_eq!(654, fuel_for(1969, false));
    assert_eq!(33583, fuel_for(100756, false));
}
#[test]
fn t_with_fuel_for_fuel() {
    assert_eq!(2, fuel_for(14, true));
    assert_eq!(966, fuel_for(1969, true));
    assert_eq!(50346, fuel_for(100756, true));
}

pub fn run() -> io::Result<()> {
    let file = File::open("inputs/day1.txt")?;
    let reader = BufReader::new(file);

    let mut total_sum = 0;
    let mut total_sum_fuel_for_fuel = 0;
    for line in reader.lines() {
        // Let's panic if we can't parse input
        let mass = line.unwrap().parse::<i32>().unwrap();
        total_sum += fuel_for(mass, false);
        total_sum_fuel_for_fuel += fuel_for(mass, true);
    }
    
    println!("Part 1: {}", total_sum);
    println!("Part 2: {}", total_sum_fuel_for_fuel);
    Ok(())
}