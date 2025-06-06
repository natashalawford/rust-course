// Silence some warnings so they don't distract from the exercise.
#![allow(unused_mut, unused_variables)]

fn main() {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    // inspect(&arg);
    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "bedazzle" 
    // Hint: You will need to dereference the mutable reference in order to assign it a
    // new value.
    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}

fn inspect(word : &str) -> bool{
    println!("The word {} is: ", word);
    if word.ends_with("s"){
        println!("plural!");
        true
    }
    else{
        println!("singular.");
        false
    }
}

fn change(word : &mut String){
    if inspect(word){
        println!("No need to change.");
    }
    else{
        word.push_str("s");
        println!("changing word to plural: {}", word);
    }
}

fn eat(consume : String) -> bool{
    if consume.starts_with("b") && consume.contains("a"){
        true
    }
    else{
        false
    }
}

fn bedazzle(word : &mut String){
    *word = "sparkly".to_string();
}
