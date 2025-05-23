const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // PART 1
    let (mut missiles, ready) : (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    
    // PART 2
    missiles = missiles - ready; // If I put this directly in the print statement
                                 // below, missiles doesn't need to be mutable!
    println!("{} missiles left", missiles);
}
