# plonky2-ecgfp5

> DISCLAIMER: this is a prototype. It hasn't been audited. It probably has bugs. DO NOT USE THIS IN PRODUCTION.

> DISCLAIMER: the curve is pretty new, and, being built upon an extension field, may be vulnerable to more kinds of attacks than other curves. Read Thomas Pornin's paper on eprint for more information about security of the curve itself.

> NOTE: this crate is not (yet) constant time. 

This crate provides plonky2 SNARK gadgets and an out-of-circuit implementation of `EcGFp5`, an elliptic curve whose base field is a degree-5 extension field of Goldilocks, the field whose modulus is `2^64 - 2^32 + 1`. 

Most of the out-of-circuit implementation is built atop Thomas Pornin's implementation, which can be found [here](https://github.com/pornin/ecgfp5). All credit for designing the curve and providing its first implementation belongs to him.


### Why does this exist?

One of the most useful things to have access to in a proof system is a curve that can be represented in its native field, as it allows one to efficiently verify public key cryptography (signatures, assymetric encryption, etc). For proof systems implemented atop BLS12-381, we have JubJub, and for proof systems implemented atop BN128, we have Baby JubJub. Plonky2 has something similar - EcGFp5. The hope is this will allow people to do in plonky2 the things people typically do with Baby JubJub in Circom/Groth16.
