fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess < secret {
        -1
    } else {
        1
    }
}

fn main() {
    let secret = 32;
    let mut guess;
    let mut attempts = 0;

    loop {
        attempts += 1;

        guess = 23 + attempts; 

        let result = check_guess(guess, secret);

      
        if result == 1 {
            println!("Your guess of {} is too high!", guess);
        } else if result == -1 {
            println!("Your guess of {} is too low!", guess);
        } else {
            println!("Your guess of {} is correct!", guess);
            break;
        }
    }

   
    println!("You guessed the number in {} attempts!", attempts);
}
