// 1. Define a trait named `Bite`
//
// Define a single required method, `fn bite(self: &mut Self)`.  We will call this method when we
// want to bite something.  Once this trait is defined, you should be able to run the program with
// `cargo run` without any errors.
//
trait Bite {
    fn bite(self: &mut Self);
}



fn main() {
    // Once you finish #1 above, this part should work.
    let mut carrot = Carrot { percent_left: 100.0 };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { amount_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    // - takes a mutable reference to any type that implements Bite
    // - calls `.bite()` several times
    // Hint: Define the generic type between the function name and open paren:
    //       fn function_name<T: Bite>(...)
    //
    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
    bunny_nibbles(&mut grapes);
    println!("Bunny nibbles for awhile, but with grapes! : {:?}", grapes);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

#[derive(Debug)]
struct Grapes {
    amount_left: u32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

impl Bite for Grapes{
    fn bite(self: &mut Self) {
        // Eat 1 grape
        self.amount_left -= 1;
    }
}

fn bunny_nibbles<T: Bite>(item: &mut T) {
    // Call the bite method several times
    for _ in 0..5 {
        item.bite();
    }
}