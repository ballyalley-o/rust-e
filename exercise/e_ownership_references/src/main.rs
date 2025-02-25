// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
    // prints whether the contents of the String is plural or singular. Then uncomment and run this
    // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
    // String reference
    //
    inspect(&arg);

    fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("{} is plural", s)
    } else {
        println!("{} is singular", s)
    }
    }

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    //
    change(&mut arg);
    println!("I have many {}", arg);

    fn change(s: &mut String) {
        if s.ends_with("s") {
           return
        } else {
             s.push_str("s")
        }
    }

    fn eat(consumes: String) -> bool {
        if consumes.starts_with("b") && consumes.contains("a") {
             true
        } else {
             false
        }
    }


    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    // Hint 3: `return` is used to return a value from a function

    if eat(arg.to_string()) {
        println!("Yum!");
    } else {
        println!("Yuck!");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    //

    fn bedazzle(s: &mut String) {
        s.clear();
        s.push_str("sparkly");
    }

    let mut material = "mud".to_string();
    println!("This material is just {}", material);

    bedazzle(&mut material);
    println!("This material is now {}", material);

}