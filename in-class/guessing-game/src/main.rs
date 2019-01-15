use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret: i32 = rand::thread_rng().gen_range(1, 101);

    'mainloop:loop {
        println!("Guess the number!");
        println!("Please input your guess (between 1 and 100):");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        
        let guess: i32 = guess.trim().parse().expect("Failed to parse");    
        
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { println!("You win!"); break 'mainloop},
        }
    }
}
