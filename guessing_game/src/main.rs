extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    // We use '::' because `new` is an "associated function", i.e., it
    // is associated to `String` proper, not to an instance of `String`.
    // Based on the intro, it's analagous to static methods in most OO
    // languages.
    let mut guess = String::new();

    // Here we use '::' because `stdin` is an associated function on
    // `io`, but we use '.' for the others because `read_line`, `ok`,
    // and `expect` are all "methods".
    io::stdin().read_line(&mut guess)
        // `ok` is a method on `io::Result` instances, which is how Rust
        // communicates error information. It basically throws away the
        // error info, because there's not much we can do in the event
        // of an error here, anyway.
        .ok()
        // `ok` returns a `std::option::Option` instance, which has
        // `this expect` method to look at the value it's called on
        // `and `panic!` with the provided message if it isn't a
        // `"successful" value.
        .expect("Failed to read line!");

    // OK, before, `guess` was a `String`, because we were just reading
    // in user input. Now we want to turn it into an integer so we can
    // compare against it.
    let guess: u32 = guess.trim().parse()
        // This `ok` method is attached to a different type of `Result`,
        // but the idea is the same.
        .ok()
        .expect("Please type a number!");
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Just right!"),
    }
}
