use std::io;
use rand::prelude::*; // Imports the necessary traits and Rng
fn main() {
    println!("Hello, world!");
    println!("Choose from the below list of fruits");
    let arr1 :[&str;4];
    arr1 = ["Apple" , "Grapes" , "Banana" , "Orange"];
    println!("{:?}" , arr1);
    println!("Your choice :-> ");
    let mut input = String::new(); // User has to put the user input in "input" variable
    io::stdin().
                read_line(&mut input).
                                      expect("Failed to read line");   
    println!("Evaluating your guess {} is write or wrong..." , input);

    let mut rng = rand::rng(); // Initialize the random number generator

    let choice = arr1.choose(&mut rng).unwrap();
    println!("Random number choosen by Computer is {}", choice);
    if input.trim() == *choice {
        println!("Your guess is correct!");
    } else {
        println!("Your guess is wrong! The correct answer was {}", choice);
    }                      
}
