use std::io;
use files::tests;

fn main(){
    println!{"Welcome to password strength tester"};
    println!{"Enter the password to test : "};
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read password");
    let password = password.trim();
    let mut score = 0;
    match tests::check_breach(&password){
        true => {
            println!("Password is breached please change it immidiately");
        }
        false => {
            score += tests::length(&password);
            score += tests::case(&password);
            score += tests::numbers(&password);
            score += tests::uppercase(&password);
            score += tests::special(&password);
            score += tests::pattern(&password);
            if score < 11{
                println!("Password is weak");
            }else{
                println!("Password is strong");
            }

        }
    }
}
