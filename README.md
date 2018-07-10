simd-prngs
====

A crate researching various SIMD PRNGs.
You need nightly Rust and SIMD capable hardware to use this crate.
To use it, run:
```console
$ RUSTFLAGS='-C target-cpu=native' cargo bench
```

Also provided is a utility for printing a PRNG's output to stdout for use with testing utilities like [PractRand](http://pracrand.sourceforge.net/): `src/bin/cat_rng.rs`.

Note: not all implementations of PRNGs are verified to be correct.

## Currently implemented PRNGs
- `Ars5`, `Ars7`: An AES implementation optimized for non-cryptographic designed by D. E. Shaw Research
- `rand_sse`: An LCG designed for SSE2 hardware by Intel
- `Jsf32`, `Jsf64`: A small chaotic PRNG designed by Bob Jenkins.
- `Sfc32`, `Sfc64`: A small chaotic PRNG combined with a counter, designed by Chris Doty-Humphrey.
- `Xorshift32`, `Xorshift128`: A Xorshift PRNG (32/32-bit and 128/32-bit variants).
- `Xorshift128Plus`: The Xorshift128+ PRNG.
- `Xoroshiro128StarStar`: The Xoroshiro128** PRNG.
- `Xoshiro256StarStar`: The Xoshiro256** PRNG
- `Pcg32`: A PCG PRNG (XSH 64/32 RR (LCG) variant).
- `Xsm32`, `Xsm64`: A small random-access PRNG designed by Chris Doty-Humphrey
- `ChaCha4`: A stream cipher designed by Daniel J. Bernstein. We reduce the rounds to 4 for a faster non-cryptographic version.

Most of the PRNGs are parallelized scalar PRNGs. For most of those, variants with all vector lanes available with [`stdsimd`](https://github.com/rust-lang-nursery/stdsimd) are provided.

## Currently implemented stream features
- `Xoroshiro`: equally-spaced blocks via Xoroshiro's jumping features, `blocks_from_rng`
- `Xoshiro`: equally-spaced blocks via Xoshiro's jumping features, `blocks_from_rng`
- `Pcg`: random LCG increments
- `Xsm`: equally-spaced blocks via XSM's `seek_forward`, `blocks_from_rng`

Otherwise, parallel PRNGs are given a random seed for each stream with `SeedableRng`. The probabilities of stream correlation for such a method are listed in the source code for each PRNG.
