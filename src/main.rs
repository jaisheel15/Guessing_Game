use std::io;
use    rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main()
{
    println!("Guessing Game");
    let random = rand::thread_rng().gen_range(1..101);   
   
    loop{
    println!("Enter a number : ");

    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Error wrong input");
    let string : u32 = match string.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
    };
    match string.cmp(&random){
        Ordering::Equal => {println!("{}","You won".green()); break;},
        Ordering::Greater => println!("{}","Too big".red()),
        Ordering::Less => println!("{}","Too small".red()),
    }


    }



}