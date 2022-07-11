use std::io;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;
fn main() {
    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret number is : {}", secret_number);
    loop {
        println!("please input your number");
        let mut guess = String::new();
       io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read");
    
       let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue
       };
    
       println!("You guessed: {}", guess);

       match guess.cmp(&secret_number) {
        Ordering:: Less => println!("{}","Too Less ".red()),
        Ordering:: Greater => println!("{}","Too Big".red()),
        Ordering:: Equal => {
            println!("{}","You Win".green());
            break;
        }
       
       }
    
    }
   
}
