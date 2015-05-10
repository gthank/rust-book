use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

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

    println!("You guessed: {}", guess);
}
