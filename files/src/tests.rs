use regex::Regex;
use rayon::prelude::*;
use std::fs;
pub fn check_breach(password : &str) -> bool{
    let passwords = match fs::read_to_string("passwords.txt"){
        Ok(file) => file,
        Err(_) => {
            println!("Failed to read file");
            fs::File::create("passwords.txt").expect("Failed to create file");
            panic!("File created please add passwords to it");
        }
    };
    let passwords = passwords.split("\n").collect::<Vec<&str>>();
    if passwords.par_iter().any(|x| x == &password){
        true
    }
    else {
        false
    }
}
pub fn length(password : &str) -> i32{
    match password.len(){
        0..=7 => {
            println!("Increase password length");
            -5
        },
        8..=15 => 2,
        16..=23 => 3,
        24..=31 => 4,
        _ => 5
    }
}
pub fn matchr(re : regex::Regex , password : &str) -> i32{
    if re.is_match(password){
        2
    }else{
        println!("Add special characters");
        -5
    }
}

pub fn case(password : &str) -> i32{
    let re = Regex::new(r"[A-Z]").unwrap();
    matchr(re , password)
}

pub fn numbers(password : &str) -> i32{
    let re = Regex::new(r"[0-9]").unwrap();
    matchr(re , password)
}
pub fn uppercase(password : &str) -> i32{
    let re = Regex::new(r"[a-z]").unwrap();
    matchr(re , password)
}
pub fn special(password : &str) -> i32{
    let re = Regex::new(r"[!@#$%^&*()_+]").unwrap();
    matchr(re , password)
}
pub fn pattern(password : &str) -> i32 {
    let re = Regex::new(r"([a-z]{3,}[A-Z]{1,}[0-9]{1,}[!@#$%^&*()_+]{1,})").unwrap();
    matchr(re , password)
}
