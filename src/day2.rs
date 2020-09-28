use std::fs::File;
use std::io::{self, prelude::*};

fn gravity_assist(input: &Vec<i32>) -> Vec<i32> {
    
    let mut program = input.clone();
    for i in (0..program.len()).step_by(4) {
        let cmd = *program.get(i).unwrap();
        if cmd == 99 {
            break;
        }
        let p1 = program[*(program.get(i+1).unwrap()) as usize];
        let p2 = program[*(program.get(i+2).unwrap()) as usize];
        let p3 = *(program.get(i+3).unwrap()) as usize;
        
        // Check Opcode and do it
        program[p3 as usize] = match cmd {
            1 => p1 + p2,
            2 => p1 * p2,
            _ => panic!("Invalid opcode: {}", cmd)
        };
        
    }
    program
}

#[test]
fn opcode_promgram() {
    let program = vec!(1,0,0,0,99);
    assert_eq!(vec!(2,0,0,0,99), gravity_assist(&program));

    let program = vec!(2,3,0,3,99);
    assert_eq!(vec!(2,3,0,6,99), gravity_assist(&program));

    let program = vec!(2,4,4,5,99,0);
    assert_eq!(vec!(2,4,4,5,99,9801), gravity_assist(&program));

    let program = vec!(1,1,1,4,99,5,6,0,99);
    assert_eq!(vec!(30,1,1,4,2,5,6,0,99), gravity_assist(&program));
}

pub fn run() -> io::Result<()> {
    let mut file = match File::open("inputs/day2.txt"){
        Err(reason) => panic!("Could not open file {}", reason),
        Ok(file) => file,
    };
    
    let mut program_string = String::new();
    file.read_to_string(&mut program_string).unwrap();
    let splits = program_string.trim_end().split(",");
    
    let mut program: Vec<i32> = splits.collect::<Vec<&str>>().iter()
                    .map( |x| x.parse::<i32>().unwrap() )
                    .collect::<Vec<i32>>();
    /*
        Once you have a working computer, the first step is to restore the gravity assist program 
        (your puzzle input) to the "1202 program alarm" state it had just before the last computer 
        caught fire. To do this, before running the program, replace position 1 with the value 12 
        and replace position 2 with the value 2. What value is left at position 0 after the program halts?
    */
    program[1] = 12;
    program[2] = 2;
    let res = gravity_assist(&program);
    println!("Part 1: {}", res[0]);
    
    // Brute force part 2, fin noun and verb for result of "19690720"

    for noun in 0..99 {
        for verb in 0..99 {
            program[1] = noun;
            program[2] = verb;
            let res = gravity_assist(&program)[0];
            if res == 19690720 {
                println!("Noun: {}", noun);
                println!("Verb: {}", verb);
                println!("Part 2: {}", 100*noun + verb);
                break;
            }
        }
    }

    Ok(())
}