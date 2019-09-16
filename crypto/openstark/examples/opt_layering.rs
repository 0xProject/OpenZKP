use openstark::{decommitment_size_upper_bound};
fn main() {
    //This file computes a series of proof size bounds for million element trace table systems
    // We work under the rough assumption that the final decommitment should be len 64.
    // This tells us that the sum of what is in the fri layout should be 20 - 6 = 14
    // We don't do an exhaustive search but test some simple changes to test general slopes.

    println!("Tests which seem to indicate that 3 is an optimal size");
    // Note we use 5 queries and know that this is roughly linear in the number of queries
    // As many 3 as possible to get to 64
    println!(
        "[3, 3, 3, 3, 2]: {}",
        decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 3, 2], 20)
    );
    // As many twos as possible
    println!(
        "[2, 2, 2, 2, 2, 2, 2] : {}",
        decommitment_size_upper_bound(20, 2, vec![2, 2, 2, 2, 2, 2, 2], 20)
    );
    // As many ones as possible
    println!(
        "[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1] : {}",
        decommitment_size_upper_bound(20, 2, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 20)
    );
    // As many fours as possible
    println!(
        "[4, 4, 4, 2]: {}",
        decommitment_size_upper_bound(20, 2, vec![4, 4, 4, 2], 20)
    );
    // One less each step
    println!(
        "[5, 4, 3, 2]: {}",
        decommitment_size_upper_bound(20, 2, vec![5, 4, 3, 2], 20)
    );
    // One less each step
    println!(
        "[4, 4, 3, 3]: {}",
        decommitment_size_upper_bound(20, 2, vec![4, 4, 3, 3], 20)
    );
    println!("A test showing higher numbers up front are worse");
    // Threes with the final one front loaded
    println!(
        "[2, 3, 3, 3, 3]: {}",
        decommitment_size_upper_bound(20, 2, vec![2, 3, 3, 3, 3], 20)
    );
    // Threes with 2 in 2nd
    println!(
        "[3, 2, 3, 3, 3]: {}",
        decommitment_size_upper_bound(20, 2, vec![3, 2, 3, 3, 3], 20)
    );
    // Threes with 2 in 3nd
    println!(
        "[3, 3, 2, 3, 3]: {}",
        decommitment_size_upper_bound(20, 2, vec![3, 3, 2, 3, 3], 20)
    );
    // Threes with 2 in 4nd
    println!(
        "[3, 3, 3, 2, 3]: {}",
        decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 2, 3], 20)
    );
    
    // Testing to find opt on final entry length
    println!("Testing on boundary for final entry");
    // Another reduction to 32 on final step
    println!(
        "[3, 3, 3, 3, 3]: {}",
        decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 3, 3], 20)
    );
    println!(
        "[3, 3, 3, 3, 2]: {}",
        decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 3, 2], 20)
    );
    println!(
        "[3, 3, 3, 3, 1]: {}",
        decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 3, 1], 20)
    );
    println!(
        "[3, 3, 3, 3]: {}",
        decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 3], 20)
    );
    println!(
        "[3, 3, 3, 2]: {}",
        decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 2], 20)
    );
    println!(
        "[3, 3, 3, 1]: {}",
        decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 1], 20)
    );
    println!(
        "[3, 3, 3]: {}",
        decommitment_size_upper_bound(20, 2, vec![3, 3, 3], 20)
    );
    println!(
        "[3, 3, 2]: {}",
        decommitment_size_upper_bound(20, 2, vec![3, 3, 2], 20)
    );
    println!(
        "[3, 3, 1]: {}",
        decommitment_size_upper_bound(20, 2, vec![3, 3, 1], 20)
    );

    //Large front loading with larger decommitment
    println!("Larger front loading test");
    println!(
        "[4, 3, 3]: {}",
        decommitment_size_upper_bound(20, 2, vec![4, 3, 3], 20)
    );
    println!(
        "[6, 4]: {}",
        decommitment_size_upper_bound(20, 2, vec![6, 4], 20)
    );
    println!(
        "[5, 4, 1]: {}",
        decommitment_size_upper_bound(20, 2, vec![5, 4, 3], 20)
    );
    println!(
        "[5, 3, 2]: {}",
        decommitment_size_upper_bound(20, 2, vec![5, 3, 2], 20)
    );
    println!(
        "[4, 3, 2]: {}",
        decommitment_size_upper_bound(20, 2, vec![5, 4, 3], 20)
    );
}