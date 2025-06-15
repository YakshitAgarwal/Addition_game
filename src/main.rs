use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Addition game");

    let number1 = rand::thread_rng().gen_range(1..=50);
    let number2 = rand::thread_rng().gen_range(1..=50);

    let sum = number1 + number2;

    println!("Add these number {number1} + {number2}");

    loop {
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Error");

        let answer: u32 = match answer.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match answer.cmp(&sum) {
            Ordering::Less => println!("Wrong answer"),
            Ordering::Greater => println!("Wrong answer"),
            Ordering::Equal => {
                println!("Correct answer, Hooray!");
                break;
            }
        };
    }
}
