use rand;
use rand::{Rng, rng};
use std::cmp::Ordering;

#[warn(unused_variables)]
use std::io;
fn main() {
    println!();
    println!();

    two();
}

fn one() {
    let mut str = String::new();
    let string = "ali";
    println!();

    let num = io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line");

    println!("number : {num}");
}

fn two () {
    let mynum = "50".parse::<i32>().unwrap();
    let num = rand::rng().random_range(0..100);

    println!("number : {num}");

    match mynum.cmp(&num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}
