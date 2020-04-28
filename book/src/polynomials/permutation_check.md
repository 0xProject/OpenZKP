# Plonk's Permutation check

See the Plonk paper https://eprint.iacr.org/2019/953. Here follow some meditations on chapter 5 and appendix A.

Plonk uses the fact that polynomials can be uniquely represented as a vector of coefficients and as a multiset of irreducible factors to efficiently proof that one vector is a permutation of another. This is done using applications of the Schwartz-Zippel lemma.

IMHO, the two key innovations in a permutation check are 1) a protocol for multiset equality checking and 2) a way of encoding a permutation check as a multiset equality check over pairs.

---

**Lemma A.2.** *(Schwartz-Zippel). Let $P(\vec X) ∈ \F_p[X^n]$ be non-zero and $\vec α ∈ S ⊆ \F_p^n$ uniform random, then*

$$
\Pr\left[P(\vec α) = 0 \right] ≤ \frac{\deg P}{\norm S}
$$

**Corrolary.** *Given $P(X) ∈ \F_p[X]$ with $\deg P ≪ p$ and uniform random $α ∈ \F_p$, then $P(α) = 0$ with non-neglibile probability only when $P = 0$.*

**Corrolary.** *(Polynomial identity). Given $P(X), Q(X) ∈ \F_p[X]$ with $\deg P,\deg Q ≪ p$ and uniform random $α ∈ \F_p$, then $P(α) = Q(α)$ with non-neglibile probability only when $P = Q$.*

---

**Lemma A.3.** *(Multiset equality) Given two multisets $\set A, \set B$ over $\F_p$ with $\norm{\set A}, \norm{\set  B} ≪ p$ and $γ ∈ \F$ uniform random, then $\set A = \set B$ if the following holds with non-neglible probability:*

$$
\prod_{a ∈ \set A} \( a + γ \) =
\prod_{b ∈ \set B} \( b + γ \)
$$

*Proof.* Define $P(X) ≝ \prod_{a ∈ A} (a + X)$ and $Q(X) ≝ \prod_{b ∈ B}(b + X)$. If the above holds with non-neglible probability then by the polynomial identity corrolary we have $P = Q$. From the unique factorization of polynomials it follows that their irreducible factors are the same, which are given in their definition. From this follows $\set A = \set B$. □

**Note.** We do not require $\norm{\set A} = \norm{\set B}$ and the test will correctly fail when they are of unequal size.

**Remark.** Compared to lemma A.3 in the paper it is rephrased in terms of multisets and generalized to have multisets of unequal size. The protocol in chapter 5 is readily adjusted to become a multiset-equality protocol.

**To do.** Rephrased permutation checking protocol as an instance of multiset-equality checking.

**To do.** Is there utility in constructions where the irreducible factors are of higher degree?

---

**Lemma.** *(Vector equality).* Given two vectors $\vec a, \vec b ∈ \F_p^n$ with $n ≪ p$ and uniform random $\beta \in \F_p$, then $\vec a = \vec b$ if the following holds with non-neglible probability:

$$
\sum_{i ∈ [n]} a_i ⋅ β^i =
\sum_{i ∈ [n]} b_i ⋅ β^i
$$

*Proof.* Define $P(X) ≝ \sum_{i ∈ [n]} a_i ⋅ X^i$ and $Q(X) ≝ \sum_{i ∈ [n]} a_i ⋅ X^i$. If the above holds with non-neglible probability then by the polynomial identity corrolary we have $P = Q$. From the uniqueness of the coefficient representation of polynomials it follows that $\forall_{i \in [n]} a_i = b_i$ and therefore $\vec a = \vec b$. □

**Remark.** If $\vec a$ and $\vec b$ have unequal size, the test can incorrectly pass if the final entries of the longer vector are zero, otherwise it should work correctly. The stated form requires them to be equal length. (*to do*: we could add a $X^{n+1}$ term to bind the length of the vector.)

**Note.** In the permutation check we use the vector equality check with $n=2$, i.e. pairs.

---

**To do.** *(Efficient composability). Show that we can do a multiset check over vectors with only two random variables.*

---

### Permutation check

Given $\vec a, \vec b ∈ \F_p^n$ and a permutation $σ: [n] → [n]$. We want to show that $b_i = a_{σ(i)}$.

Take $\vec c$ with distinct values (i.e. $c_i = c_j ⇔ i = j$). Compose the multiset and vector equality protocols to prove the following equality:

$$
\begin{Bmatrix}
(a_0, c_0) \\\\
(a_1, c_1) \\\\
(a_2, c_2) \\\\
⋮
\end{Bmatrix} = 
\begin{Bmatrix}
(b_0, c_{σ(0)}) \\\\
(b_1, c_{σ(1)}) \\\\
(b_2, c_{σ(2)}) \\\\
⋮
\end{Bmatrix}
$$

*Proof.* Since the elements $c_i$ are unique, the pairs are unique and the multisets are just regular sets. Two sets are equal iff their elements can be brought in correspondance. Enumerate the elements as above, we are looking for correspondances between elements $(a_i, c_i) ≟ (b_j, c_{σ(j)})$. The second element in the pair is equal only if $i = σ(j)$, this means a correspondance exists iff $b_j = a_{σ(j)}$. □

---

**Claim A.1.** *If the following holds with non-negligible probability over random $β, γ \in \mathbb F$*

$$
∏_i \( a_i + β ⋅ i + γ \) =
∏_i \( b_i + β ⋅ σ\(i\) + γ \)
$$

*then $∀_{i \in [n]} b_i = a_{σ(i)}$*.

**To do.** Demonstrate this follows concretely from the above.
