# Low degree polynomial protocols

These form the 'backend'

|                Name                | Trusted Setup? |
| ---------------------------------- | -------------- |
| [Fri](fri.md)                      | no             |
| [Dark](dark.md)                    | no             |
| [Class groups](dark.md)            | no             |
| [Hyper elliptic](hyperelliptic.md) | no             |
| [RSA groups](dark.md)              | yes             |
| [Pairing](pairing.dm)              | yes            |

All of these allow you to commit to a low-degree polynomial (practical limits around degree $2^{30}$ - $2^{40}$).

It then allows testing certain algebraic predicates on these polynomials through their commitments.

**To do.** Quantify the nature of these algebraic predicates.

* Linear combinations of existing commitments.
* Publicly known commitments $X^i$.
* Multiplication by a known monomial $X^i$.
* Checking identity between two combinations.

These are called 'homomorphic' (in the Sonic paper?)



The protocols follow the pattern:

* Commit some low-degree polynomials.
* Generate random value $\alpha$.
* Repeat 1, 2.
* Check predicates on commitments.

