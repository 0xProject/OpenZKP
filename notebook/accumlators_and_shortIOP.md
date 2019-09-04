# Notes on Accumulators and their potential in a STARK dex

## Intro
These notes are largely focused on the information and constructions presented in ["Batching Techniques for Accumulators with Applications to IOPs and Stateless Blockchains"](https://eprint.iacr.org/2018/1188.pdf) by Dan Boneh, Benedikt Bünz,
 and Ben Fisch out of the Stanford applied cryptography group. But we also reference some of the constructions of paring based elliptic curve cryptography in other works. 

These notes will give some definitions about accumulators and vector commitments and present the schemes which make them work in the BBF paper. I'll also include some projections on the potential impact on proof time to switch to accumulators for a STARK dex structure.

## Defintions
* **Accumulator** - An accumulator is a a protocol which allows commitment to a set of values with a constant sized commitment [think the root of a merkle tree] and a proof that an element is in a commitment [think of a merkle proof].
* **Vector Commitment** - A vector commitment is an accumulator which also binds the elements of the set to indices such that each index has a single provable element and each element has an index. For example a merkle tree is a vector commitment because each index in the tree contains one provable element.
* **Dynamic Accumulator** - A dynamic accumulator is one which supports addition and deletion of elements from the set of accumulated elements via manipulation of the root [many times assumed to be constant time].
* **Universal Accumulator** - A universal accumulator is on which supports non membership proofs for any element which is not in the set. For example in a merkle tree which is guaranteed to be on sorted data you can prove non inclusion of $y$ by providing a pair of decommitment to indexes $i$ and $i + 1$ where $m(i) < y < m(i+1)$.   
* **RSA Group Assumption** - The RSA group assumption for a group is that the the order of the group is computationally infeasible to compute. This applies to RSA groups and to class groups.

## A Batchable Universal Accumulator

Assume that we have some group $\mathbb{G}$ for which the RSA group assumption holds and there are no known or computable elements of low order. For example this hold for an RSA group for which the elements one and negative one are removed [or also for a class group]. Then we can build an accumulator for odd prime integers using exponentiation in the following way:
* **Setup** - Pick a generator of $\mathbb{G}$ and call it $A_0$ the first commitment.
* **Add** - Given an accumulator commitment $A_t$ for $S$ then an accumulator commitment $A_{t+1}$ for $S \cup \{ p \}$ is $A_t^p$.
* **Deletion** - Given an accumulator commitment $A_t$ for $S$ an accumulator commit $A_{t-1}$ for $S / \{ p \}$ is $g^{p_*}$ where $p_* = \prod_{ x \in S / \{ p \}} x$  [note since this requires recomputation this accumulator doesn't normally count as dynamic]
* **Inclusion Proof** - To prove that a prime $p$ has been accumulated we compute a witness which is the accumulator with that element deleted, then given $(A, w_p, p)$ we accept if $A = w_p^p$
* **Non Inclusion Proof** - Given an accumulator commitment of $S$, $A$ we know that $A$ is $g$ raised to the product of each element in $S$, $s_* = \prod_{ x \in S} x$. If we want to prove that $p$ is not in $S$ first we use the fact that $\text{gcd}(s_*, p) = 1$ to conclude that there must be $(x, y)$ with $s_* x + p y = 1$, our non membership witness is $\{x, g^y\}$. So given a proposed non membership proof $(A, \{x, g^y\}, p)$ we accept if $A^xB^p = g$.

These basic operations make a functional accumulator but if we want to do a set of adds, deletes, or membership verification/non-verification the complexity to do so scales linearly in the number of exponents. We can however transform the system to require a constant number of exponents [1-2 most of the time] and linear multiplications for any of the operations. Another nice property of this system is that the membership and non-membership witness can be aggregated together into single witnesses.

* **Batch Add** - To add a set of elements ${p_0, p_1, \dots, p_n}$ we just compute their product and add that to the accumulator.
* **Batch Delete** - To delete a set of elements we can construct an aggregated membership witness and that is the the status of the accumulator after deleting the elements. A simple zk proof can be used to improve the verification efficiency.

## A bit on Vector Commitments

The paper BBF demonstrates the construction of a vector commitment which allows batch-able operations like the accumulator. The simplest form of the vector commitment created is a commitment to a bit vector given by assigning each index a unique prime element $p_i$ the accumulating that prime if the index is 1 and not otherwise, since we can aggregate proofs of membership and non-membership we can construct an opening function by simply proving that $p_i$ was accumulated or not for a set of indexes. This construction can be extended to accumulate larger objects at each index, but because of the large number of primality checks, bit at checks, and hashes it is unrealistic that we would build this inside of the STARK. 

## A STARK dex

For a first design the aim is to replicate the functionality of the merkle stark dex constraint system built by Starkware. So this first design will not include replay protection [but it's easy to add], and we will limit ourselves further to say that given a set of orders and inputs [deposits and withdraws] that they are not interdependent [ie balance changes take affect at the end of the proof].

Moreover we make the follow changes to the Starkware system: vault's have a salt which must be known to prove where they are and that when hashed with this salt the vault's hash is prime. [To implement the replay protection system we make a similar change to the orders, but just use the order id as the salt].

Here's our control flow:

loop on Public inputs :
1. hash input vault with salt -> 2 hashes
2. hash output vault with salt -> 2 hashes
3. Primality test on output vault hash -> 1 primality test
4. Multiply the input vault by the 'input' result of the last loop, produce as result
5. Multiply the output vault by the 'input' result of the last loop, produce as result.

-> transition by imputing the resulting multiplication into (input, output) for the successive multiplication in the next loop.

loop on orders [private input]:
1. Hash maker and taker.
2. Hash the four input vaults with salts.
3. Calculate the resulting output vaults.
4. Hash the two output vaults with salts.
5. Primality test the four output vaults.
6. Multiply the input vaults into the successive input, multiply the output into the succesive output

Preform a proof of membership on the inputs successive membership, taking a witness in input.
-> Takes 2 rsa exponentiations and one rsa multiplication
-> Plus one hash and one primeality check

Use the witness from the membership proof [this is the same as deletion of elements], and then preform a proof of batch addition of the outputs onto the witness
-> Takes 2 rsa exponentiations and one rsa multiplication
-> Plus one hash and one primeality check

To extend this to dependent orders split up the orders into independent groups in the private inputs and run this as a loop.

To extend it to add replay protection and cancels we add a primeality check to the maker hash and a proof of non membership of the order hashes, and batch add of order hashes to a second accumulator.

## Analysis of the Prover Speed

Our main consideration for this analysis is the size of the trace table used to hold the calculation. While this is not a perfect predictor memory usage is at least 50% of the pedersen hash test and directly corresponds to the time of other large sub-components of the time taken [such as the FFT and constraint calculation on the low degree extension domain].

For each element of the public inputs which simply reset and element in the accumulator we will have 4 hashes and a primality test as the major components. Given as $\text{\#inputs}*(256*4*4 + \text{prime\_test})$ in terms of memory cells used (rows*cols). The executed orders will have 16 hashes, 4 prime tests, 2 ECDSA verifications and some trivial multiplications. Which gives $\text{\#orders}*(256*4*16 + 4*\text{prime\_test})$. The constant overhead for the accumlator stark is 4 4096 bit rsa multiplications and 2 4096 bit RSA multiples. We can break this down as $256*(\text{rsa\_mult} + \text{rsa\_squre}) + 2\text{rsa\_mult}$ The Merkle starkdex has roughly 65000 computation rows per executed order and 16000 per input.

The accumulator system is efficient if:

$$
\text{\#inputs}*(256*4*4 + \text{prime\_test}) + \text{\#orders}*(256*4*16 + 4*\text{prime\_test}) + 4*256*(\text{rsa\_mult} + \text{rsa\_square}) + 2\text{rsa\_mult}
$$
$$
< \newline \text{\#inputs}*16000*4 + \text{\#orders}*65000*4
$$

We can make some conclusions based on prime testing size: our system will be efficient when for $k = 64000 - \text{prime\_test}$ we have that:

$$
4*256*(\text{rsa\_mult} + \text{rsa\_square}) + 2\text{rsa\_mult} - (\text{\#inputs} + \text{\#orders})*k < 0
$$

What this expresses is that the more efficient the primal verification the more quickly the fixed cost of the constant sized proofs is offset by the savings per trade. For example if the primal test achievable in 10,000 memory cells, then we have on the order of 50,000 memory cells per order. Assuming an RSA multiplication takes 10 rows of 16 elements then the constant sized proofs are 327,680 cells in size which given 50,000 in savings per row means that the proof is more efficient after 7 orders.

Despite these optimistic numbers, given the huge scaling constants on the starkdex size even if it becomes more efficient after 700 or 1000 orders, the cost increases far more slowly for the accumulator stark dex since much of it is in the RSA operations, and so it would likely be able to handle far more than the 1024 produced by the current system.

## Discussion and open questions

The constant sized proofs can be equivalently switched out for class groups but they would likely require 3 600 bit integers per element and considerable higher precision arithmetic. So while they have the advantage of being trustless on setup they are going to be much harder to implement than RSA. 

Primally testing in the setup is a conversation which requires nuance, at least one of the primality tests which is deterministic is quite slow to search for and it's verification requires elliptic cure multiplication and properties over a field given by the $n$ being tested. The fast probabilistic tests however are not perfect and may not even be faster than checking the deterministic one inside of the STARK. Building this system to produce an optimal prover will require lots of research.

A major open question which has the potential to sink these efforts is if accumulating field elements instead of full precision numbers will have major negative impacts on the system, it appears that the mathematics still works in this case but it is not clear if the system is now open to attacks.