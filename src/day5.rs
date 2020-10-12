use std::fs::File;
use std::io::{self, prelude::*};

fn get_at(input: &Vec<i32>, index2: usize, mode: i32) -> i32 {
    
    let p1 = *input.get(index2).unwrap();
    let ret = match mode {
        0 =>  *input.get(p1 as usize).unwrap(),
        1 => p1,
        _ => panic!("Parameter mode {} not implemented", mode)
    };
    ret
}

fn thermal_environment_supervision_terminal(input: &Vec<i32>, inp: i32) -> Vec<i32> {
    
    let mut program = input.clone();
    
    let mut index:usize = 0;
    let mut cmd = *program.get(index).unwrap();
    let mut last_cmd = cmd;
    while cmd != 99 {
        let opcode = cmd % 100;
        let mode1 = (cmd / 100) % 10;
        let mode2 = (cmd / 1000) % 10;
        // third parameter (where to write is never immediate mode)
        
        let p1:i32;
        let p2:i32;
        let p3:i32;
        match opcode {
            1|2|7|8 => {
                p1 = get_at(&program, index+1, mode1);
                p2 = get_at(&program, index+2, mode2);
                // target
                p3 = get_at(&program, index+3, 1);
                if p3 as usize != index { index+=4; }
            }
            3 => {
                // Target mode
                p1 = get_at(&program, index+1, 1);
                p2 = 0;
                p3 = 0;
                if p1 as usize != index { index+=2; }
                
            }
            4 => {
                p1 = get_at(&program, index+1, mode1);
                p2 = 0;
                p3 = 0;
                index += 2;
            }
            5|6 => {
                p1 = get_at(&program, index+1, mode1);
                p2 = get_at(&program, index+2, mode2);
                p3 = 0;
                index += 3;
            }
            
            _ => panic!("Invalid opcode: at {}: {}\n\t{:?}", index, cmd,program)
        }
        match opcode {
            // Add
            1 => program[p3 as usize] = p1 + p2,
            // Multiply
            2 => program[p3 as usize] = p1 * p2,
            // Input
            3 => program[p1 as usize] = inp,
            // Output
            4 =>  {println!("Test result: {}", p1); if p1 != 0 {println!("\tLast cmd: {}", last_cmd);}}
            // Jump if true
            5 => {if p1 != 0 {index = p2 as usize; }}
            // Jump if false
            6 => {if p1 == 0 {index = p2 as usize; }}
            // less than
            7 => { program[p3 as usize] = if p1 < p2 {1} else {0} }
            // equals
            8 => { program[p3 as usize] = if p1 == p2 {1} else {0} }
            _ => panic!("Invalid opcode: at {}: {}", index, cmd)
        };
        last_cmd = cmd;
        cmd = *program.get(index).unwrap();
    }
    program
}

#[test]
fn test_thermal_environment_supervision_terminal() {
    let program = vec!(3,9,8,9,10,9,4,9,99,-1,8);
    assert_eq!(vec!(3,9,8,9,10,9,4,9,99,1,8), thermal_environment_supervision_terminal(&program, 8));
    let program = vec!(3,9,8,9,10,9,4,9,99,-1,8);
    assert_eq!(vec!(3,9,8,9,10,9,4,9,99,0,8), thermal_environment_supervision_terminal(&program, 7));
    
    let program = vec!(3,9,7,9,10,9,4,9,99,-1,8);
    assert_eq!(vec!(3,9,7,9,10,9,4,9,99,1,8), thermal_environment_supervision_terminal(&program, 7));
    let program = vec!(3,9,7,9,10,9,4,9,99,-1,8);
    assert_eq!(vec!(3,9,7,9,10,9,4,9,99,0,8), thermal_environment_supervision_terminal(&program, 8));
    let program = vec!(3,9,7,9,10,9,4,9,99,-1,8);
    assert_eq!(vec!(3,9,7,9,10,9,4,9,99,0,8), thermal_environment_supervision_terminal(&program, 9));
    
    let program = vec!(3,3,1108,-1,8,3,4,3,99);
    assert_eq!(vec!(3,3,1108,1,8,3,4,3,99), thermal_environment_supervision_terminal(&program, 8));
    let program = vec!(3,3,1108,-1,8,3,4,3,99);
    assert_eq!(vec!(3,3,1108,0,8,3,4,3,99), thermal_environment_supervision_terminal(&program, 9));
    
    let program = vec!(3,3,1107,-1,8,3,4,3,99);
    assert_eq!(vec!(3,3,1107,1,8,3,4,3,99), thermal_environment_supervision_terminal(&program, 7));
    let program = vec!(3,3,1107,-1,8,3,4,3,99);
    assert_eq!(vec!(3,3,1107,0,8,3,4,3,99), thermal_environment_supervision_terminal(&program, 9));
    
    // output 0 if input is 0 else 1
    let program = vec!(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9);
    assert_eq!(vec!(3,12,6,12,15,1,13,14,13,4,13,99,0,0,1,9), thermal_environment_supervision_terminal(&program, 0));
    let program = vec!(3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9);
    assert_eq!(vec!(3,12,6,12,15,1,13,14,13,4,13,99,1,1,1,9), thermal_environment_supervision_terminal(&program, 1));
    
    let program = vec!(3,3,1105,-1,9,1101,0,0,12,4,12,99,1);
    assert_eq!(0, thermal_environment_supervision_terminal(&program, 0)[12]);
    
    let program = vec!(3,3,1105,-1,9,1101,0,0,12,4,12,99,1);
    assert_eq!(1, thermal_environment_supervision_terminal(&program, 1)[12]);
    
    let program = vec!(3,3,1105,-1,9,1101,0,0,12,4,12,99,1);
    assert_eq!(1, thermal_environment_supervision_terminal(&program, 2)[12]);
    
    
    let program = vec!(3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99);
    
    //assert_eq!(999, thermal_environment_supervision_terminal(&program, 7)[20]);
    assert_eq!(1000, thermal_environment_supervision_terminal(&program, 8)[20]);
    assert_eq!(1001, thermal_environment_supervision_terminal(&program, 9)[20]);

}

pub fn run() -> io::Result<()> {
    let mut file = match File::open("inputs/day5.txt"){
        Err(reason) => panic!("Could not open file {}", reason),
        Ok(file) => file,
    };
    
    let mut program_string = String::new();
    file.read_to_string(&mut program_string).unwrap();
    let splits = program_string.trim_end().split(",");
    
    let program: Vec<i32> = splits.collect::<Vec<&str>>().iter()
                    .map( |x| x.parse::<i32>().unwrap() )
                    .collect::<Vec<i32>>();
    
    println!("Running part1");
    thermal_environment_supervision_terminal(&program, 1);
    println!("Running part2");
    thermal_environment_supervision_terminal(&program, 5);
    
   

    Ok(())
}