use openstark::decommitment_size_upper_bound;

fn main() {
    // //Uncomment for non-wide ranging tests
    // //This file computes a series of proof size bounds for million element trace
    // table systems // We work under the rough assumption that the final
    // decommitment should be len 64. // This tells us that the sum of what is
    // in the fri layout should be 40 - 6 = 14 // We don't do an exhaustive
    // search but test some simple changes to test general slopes.

    // println!("Tests which seem to indicate that 3 is an optimal size");
    // // Note we use 5 queries and know that this is roughly linear in the number
    // of queries // As many 3 as possible to get to 64
    // println!(
    //     "[3, 3, 3, 3, 2]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 3, 2], 40)
    // );
    // // As many twos as possible
    // println!(
    //     "[2, 2, 2, 2, 2, 2, 2] : {}",
    //     decommitment_size_upper_bound(20, 2, vec![2, 2, 2, 2, 2, 2, 2], 40)
    // );
    // // As many ones as possible
    // println!(
    //     "[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1] : {}",
    //     decommitment_size_upper_bound(20, 2, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    // 1, 1, 1, 1], 40) );
    // // As many fours as possible
    // println!(
    //     "[4, 4, 4, 2]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![4, 4, 4, 2], 40)
    // );
    // // One less each step
    // println!(
    //     "[5, 4, 3, 2]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![5, 4, 3, 2], 40)
    // );
    // // One less each step
    // println!(
    //     "[4, 4, 3, 3]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![4, 4, 3, 3], 40)
    // );
    // println!("A test showing higher numbers up front are worse");
    // // Threes with the final one front loaded
    // println!(
    //     "[2, 3, 3, 3, 3]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![2, 3, 3, 3, 3], 40)
    // );
    // // Threes with 2 in 2nd
    // println!(
    //     "[3, 2, 3, 3, 3]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![3, 2, 3, 3, 3], 40)
    // );
    // // Threes with 2 in 3nd
    // println!(
    //     "[3, 3, 2, 3, 3]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![3, 3, 2, 3, 3], 40)
    // );
    // // Threes with 2 in 4nd
    // println!(
    //     "[3, 3, 3, 2, 3]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 2, 3], 40)
    // );

    // // Testing to find opt on final entry length
    // println!("Testing on boundary for final entry");
    // // Another reduction to 32 on final step
    // println!(
    //     "[3, 3, 3, 3, 3]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 3, 3], 40)
    // );
    // println!(
    //     "[3, 3, 3, 3, 2]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 3, 2], 40)
    // );
    // println!(
    //     "[3, 3, 3, 3, 1]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 3, 1], 40)
    // );
    // println!(
    //     "[3, 3, 3, 3]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 3], 40)
    // );
    // println!(
    //     "[3, 3, 3, 2]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 2], 40)
    // );
    // println!(
    //     "[3, 3, 3, 1]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![3, 3, 3, 1], 40)
    // );
    // println!(
    //     "[3, 3, 3]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![3, 3, 3], 40)
    // );
    // println!(
    //     "[3, 3, 2]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![3, 3, 2], 40)
    // );
    // println!(
    //     "[3, 3, 1]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![3, 3, 1], 40)
    // );

    // //Large front loading with larger decommitment
    // println!("Larger front loading test");
    // println!(
    //     "[4, 3, 3]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![4, 3, 3], 40)
    // );
    // println!(
    //     "[6, 4]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![6, 4], 40)
    // );
    // println!(
    //     "[5, 4, 1]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![5, 4, 3], 40)
    // );
    // println!(
    //     "[5, 3, 2]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![5, 3, 2], 40)
    // );
    // println!(
    //     "[4, 3, 2]: {}",
    //     decommitment_size_upper_bound(20, 2, vec![5, 4, 3], 40)
    // );

    println!("{:?}", total_search(20, 2, 20));
}

fn total_search(domain_size: usize, num_cols: usize, queries: usize) -> (Vec<usize>, usize) {
    let mut current_min = std::usize::MAX;
    let mut current_partition = Vec::new();

    // Searches each possible total number of reductions
    for i in 1..(domain_size - 1) {
        let (min_vec, min_cost) = partion_search(i, domain_size, num_cols, queries);

        if min_cost < current_min {
            current_min = min_cost;
            current_partition = min_vec;
        }
    }
    (current_partition, current_min)
}

// Searches over all permutations of all partitions of the n provided to find
// min upper bound cost.
fn partion_search(
    n: usize,
    domain_size: usize,
    num_cols: usize,
    queries: usize,
) -> (Vec<usize>, usize) {
    let mut partion = vec![0; n]; // We know the max size is n ones
    partion[0] = n;
    let mut k: i32 = 0;

    let mut current_min = std::usize::MAX;
    let mut current_partition = Vec::new();

    loop {
        let trimmed: Vec<usize> = partion.clone().into_iter().filter(|x| *x != 0).collect();
        let upper_bound =
            decommitment_size_upper_bound(domain_size, num_cols, trimmed.clone(), queries);
        let (permuted, permuted_upper) = permutation_search(
            trimmed.as_slice(),
            upper_bound,
            domain_size,
            num_cols,
            queries,
        );

        if permuted_upper < current_min && !permuted.is_empty() {
            current_min = permuted_upper;
            current_partition = permuted;
        }

        let mut rem_value = 0;
        while k >= 0 && partion[k as usize] == 1 {
            rem_value += partion[k as usize];
            partion[k as usize] = 0;
            k -= 1;
        }

        if k < 0 {
            break;
        }

        partion[k as usize] -= 1;
        rem_value += 1;

        while rem_value > partion[k as usize] {
            partion[k as usize + 1] = partion[k as usize];
            rem_value -= partion[k as usize];
            k += 1;
        }

        partion[k as usize + 1] = rem_value;
        k += 1;
    }

    (current_partition, current_min)
}

fn permutation_search(
    partion: &[usize],
    cost: usize,
    domain_size: usize,
    num_cols: usize,
    queries: usize,
) -> (Vec<usize>, usize) {
    let mut mut_part = partion.to_vec();
    let n = mut_part.len();

    let mut current_min = cost;
    let mut current_partition = partion.to_vec();

    // This inefficient brute force search doubles up since swap(1, 2) = swap(2, 1)
    // Moreover since partitions have repeated elements often this transformation is
    // trivial We should monitor running time and if it becomes a problem fix
    // this.
    for i in 0..n {
        for j in 0..n {
            if i != j {
                mut_part.swap(i, j);
                let upper_bound =
                    decommitment_size_upper_bound(domain_size, num_cols, mut_part.clone(), queries);
                if upper_bound < current_min {
                    current_min = upper_bound;
                    current_partition = mut_part.clone();
                }
                mut_part.swap(i, j);
            }
        }
    }
    (current_partition, current_min)
}
