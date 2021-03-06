use std::io;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> io::Result<()> {
    let day = std::env::args().nth(1).expect("No day given");
    match day.parse::<i32>().unwrap() {
        1   =>  day1::run()?,
        2   =>  day2::run()?,
        3   =>  day3::run()?,
        4   =>  day4::run()?,
        5   =>  day5::run()?,
        
        _ => println!("Not implemented")
    }
    
    Ok(())
}
