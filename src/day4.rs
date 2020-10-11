use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn six_digits(num: i32) -> bool {
    num > 99999 && num < 1000000
}

fn atleast_two_same(num: &[i32; 6]) -> bool {
    let mut iter = num.iter();
    let mut last_num = *iter.next().unwrap();
    
    for x in iter {
        let cur = *x;
        if cur == last_num { return true;}
        last_num = cur;
    }
    false
}

fn two_same(num: &[i32; 6]) -> bool {
    let mut iter = num.iter();
    let mut last_num = *iter.next().unwrap();
    let mut count = 1;
    for x in iter {
        let cur = *x;
        if cur == last_num { 
            count+=1; 
        } else {
            if count == 2 {return true;}
            count = 1;
        }
        last_num = cur;
    }
    // If last 2 is same
    count == 2
}

fn only_incrementing(num: &[i32; 6]) -> bool {
    let mut iter = num.iter();
    let mut last_num = *iter.next().unwrap();
    
    for x in iter {
        let cur = *x;
        if cur > last_num { return false;}
        last_num = cur;
    }
    true
}

fn num_to_array(num: i32) ->  [i32; 6] {
    let mut array: [i32; 6] = [0; 6];
    let base = 10i32;

    let mut my_num = num;
    for x in array.iter_mut() {
        *x = my_num % base;
        my_num /= base;
    }
    array
}

fn check(num: i32, exactlytwo: bool) -> bool {
    if !six_digits(num) {return false; }
    let arr = num_to_array(num);
    if exactlytwo {
        if !two_same(&arr)  {return false; }
    } else {
        if !atleast_two_same(&arr)  {return false; }
    }
    if !only_incrementing(&arr)  {return false; }

    true
}

pub fn run() -> io::Result<()> {
    let file = File::open("inputs/day4.txt")?;
    let reader = BufReader::new(file);
    for line2 in reader.lines() {
        let line = line2.unwrap();  
        let v: Vec<&str> = line.splitn(2, '-').collect();
        let x:Vec<i32> = v.iter().map(|x| x.parse::<i32>().unwrap()).collect();
        
        let mut total_pass:i32 = 0;
        let mut total_pass2:i32 = 0;
        for num in x[0]..x[1] {
            if check(num, false) {
                total_pass +=1;
            }
            if check(num, true) {
                total_pass2 +=1;
            }
        }
        println!("Total passes: {}", total_pass);
        println!("           2: {}", total_pass2);
    
    }
    
    Ok(())
}

#[test]
fn test_six_digits(){
    assert_eq!(true,  six_digits(123456));
    assert_eq!(true,  six_digits(999999));
    assert_eq!(true,  six_digits(100000));
    assert_eq!(false, six_digits(12345));
    assert_eq!(false, six_digits(99999));
    assert_eq!(false, six_digits(1234567));
}
#[test]
fn test_atleast_two_same(){
    assert_eq!(true,  atleast_two_same(&num_to_array(123455)));
    assert_eq!(true,  atleast_two_same(&num_to_array(123345)));
    assert_eq!(true,  atleast_two_same(&num_to_array(111111)));
    assert_eq!(false, atleast_two_same(&num_to_array(123456)));
}
#[test]
fn test_twosame(){
    assert_eq!(true,  two_same(&num_to_array(112233)));
    assert_eq!(true,  two_same(&num_to_array(111122)));
    assert_eq!(false, two_same(&num_to_array(123444)));
    assert_eq!(false, two_same(&num_to_array(123334)));
    assert_eq!(true, two_same(&num_to_array(000022)));
    
    assert_eq!(true, two_same(&num_to_array(112233)));
    assert_eq!(true, two_same(&num_to_array(111122)));
    assert_eq!(true, two_same(&num_to_array(223333)));
    
    assert_eq!(false, two_same(&num_to_array(123444)));
}

#[test]
fn test_only_incrementing(){
    assert_eq!(true,  only_incrementing(&num_to_array(123456)));
    assert_eq!(true,  only_incrementing(&num_to_array(123456)));
    assert_eq!(true,  only_incrementing(&num_to_array(111111)));
    assert_eq!(false, only_incrementing(&num_to_array(123245)));
}

#[test]
fn test_day4(){
    assert_eq!(true,  check(111111,false));
    assert_eq!(false, check(223450,false));
    assert_eq!(false, check(123789,false));

    assert_eq!(false, check(111111,true));
    assert_eq!(false, check(223450,true));
    assert_eq!(false, check(123789,true));
    assert_eq!(true,  check(122789,true));
    assert_eq!(true,  check(222788,true));
}
