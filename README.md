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
- `Ars5`, `Ars7`: An AES implementation optimized for non-cryptographic use designed by D. E. Shaw Research
- `IntelLcg`: An LCG designed for SSE2 hardware by Intel
- `Jsf32`, `Jsf64`: A small chaotic PRNG designed by Bob Jenkins.
- `Sfc32`, `Sfc64`: A small chaotic PRNG combined with a counter, designed by Chris Doty-Humphrey.
- `Xorshift32`, `Xorshift128`: A Xorshift PRNG (32/32-bit and 128/32-bit variants).
- `Xorshift128Plus`: The Xorshift128+ PRNG.
- `Xoroshiro128StarStar`: The Xoroshiro128** PRNG.
- `Xoshiro256StarStar`: The Xoshiro256** PRNG
- `Pcg32`: A PCG PRNG (XSH 64/32 RR (LCG) variant).
- `Xsm32`, `Xsm64`: A small random-access PRNG designed by Chris Doty-Humphrey
- `ChaCha4`: A stream cipher designed by Daniel J. Bernstein. We reduce the rounds to 4 for a faster non-cryptographic version.

**To be added:**
- [`AESRand`](https://github.com/dragontamer/AESRand): A counter-based invertible PRNG using AES-NI instructions by @dragontamer. VERY fast, \~0.12 cycles per byte.

Most of the PRNGs are parallelized scalar PRNGs. For most of those, variants with all vector lanes available with [`stdsimd`](https://github.com/rust-lang-nursery/stdsimd) are provided.


## Currently implemented stream features
- `Xoroshiro`: equally-spaced blocks via Xoroshiro's jumping features, `blocks_from_rng`
- `Xoshiro`: equally-spaced blocks via Xoshiro's jumping features, `blocks_from_rng`
- `Pcg`: random LCG increments
- `Xsm`: equally-spaced blocks via XSM's `seek_forward`, `blocks_from_rng`

Otherwise, parallel PRNGs are given a random seed for each stream with `SeedableRng`. The probabilities of stream correlation for such a method are listed in the source code for each PRNG.

## Possible future work
- Other counter-based PRNGs inspired by [Random123](http://www.deshawresearch.com/resources_random123.html). They offer Threefry and Philox but both are too slow to be worthwhile. A faster vectorizable pseudo-random permutation/bijection might be viable (see below). AVX-512 offers instructions which would allow 8 64-bit widening multiplications at once which is roughly equivalent to 8 rounds of Philox2×64.
- Block ciphers/hashes. Any fast, statistically strong, vectorizable block cipher or hash would be viable. Weakening cryptographic algorithms could be fruitful. Most wouldn't need multiple streams as they tend to generate blocks of data. If multiple streams were implemented though, avoiding correlation would be easy in counter mode.
- [Mrg32k3a](https://www.informs-sim.org/wsc00papers/090.PDF) is a popular choice for its large period and convenient streaming features, although it is a little slow. If it could be sped up with newer instructions it might be viable. (Perhaps [*MRG8: Random Number Generation for the Exascale Era*](https://dl.acm.org/citation.cfm?id=3218230)?)

## Benchmarks

Sorted by throughput. The full benchmarks are available in the `benches` directory.

NOTE: even with flags like `target-feature=mmx` the benchmarks will still likely differ on other hardware. Latencies and throughputs may still be the same even with the feature flag.

`RUSTFLAGS='-C target-feature=mmx -C codegen-units=1 -C lto=thin' cargo bench` (the oldest SIMD instruction set):
```rust
test gen_ars7                           ... bench:      21,680 ns/iter (+/- 5,803) = 755 MB/s
test gen_sfc16_x2                       ... bench:       4,471 ns/iter (+/- 1,751) = 916 MB/s
test gen_ars5                           ... bench:      17,139 ns/iter (+/- 20,142) = 955 MB/s
test gen_xsm32_x2                       ... bench:       6,747 ns/iter (+/- 4,571) = 1214 MB/s
test gen_pcg32x2                        ... bench:       6,462 ns/iter (+/- 4,870) = 1267 MB/s
test gen_lfsr113_x2                     ... bench:       6,415 ns/iter (+/- 6,084) = 1277 MB/s
test gen_pcg32x8                        ... bench:      25,539 ns/iter (+/- 14,090) = 1283 MB/s
test gen_pcg32x4                        ... bench:      11,403 ns/iter (+/- 196) = 1436 MB/s
test gen_pcg_fixed_xsl32x2              ... bench:       5,375 ns/iter (+/- 6,246) = 1524 MB/s
test gen_lcg32x2                        ... bench:       4,494 ns/iter (+/- 2,315) = 1822 MB/s
test gen_sfc16_x4                       ... bench:       4,442 ns/iter (+/- 2,145) = 1844 MB/s
test gen_pcg_fixed_xsh32x2              ... bench:       4,344 ns/iter (+/- 47) = 1885 MB/s
test gen_xsm64_x8                       ... bench:      34,405 ns/iter (+/- 13,985) = 1904 MB/s
test gen_lfsr258_x8                     ... bench:      33,860 ns/iter (+/- 24,947) = 1935 MB/s
test gen_xorshift32_x2                  ... bench:       4,200 ns/iter (+/- 2,458) = 1950 MB/s
test gen_xoshiro512starstar_x8          ... bench:      32,357 ns/iter (+/- 20,595) = 2025 MB/s
test gen_xsm64_x2                       ... bench:       7,722 ns/iter (+/- 6,336) = 2121 MB/s
test gen_sfc32_x2                       ... bench:       3,846 ns/iter (+/- 2,942) = 2130 MB/s
test gen_xoshiro256starstar_x4          ... bench:      14,847 ns/iter (+/- 7,420) = 2207 MB/s
test gen_xoroshiro128starstar_x2        ... bench:       7,359 ns/iter (+/- 2,912) = 2226 MB/s
test gen_xsm64_x4                       ... bench:      14,577 ns/iter (+/- 5,742) = 2247 MB/s
test gen_xoroshiro128starstar_x4        ... bench:      14,414 ns/iter (+/- 10,534) = 2273 MB/s
test gen_xoroshiro128starstar_x8        ... bench:      27,764 ns/iter (+/- 17,722) = 2360 MB/s
test gen_lfsr258_x4                     ... bench:      13,812 ns/iter (+/- 7,646) = 2372 MB/s
test gen_pcg_fixed_xsh32x8              ... bench:      12,992 ns/iter (+/- 3,401) = 2522 MB/s
test gen_xoshiro512starstar_x4          ... bench:      12,883 ns/iter (+/- 9,844) = 2543 MB/s
test gen_lfsr113_x4                     ... bench:       6,434 ns/iter (+/- 5,574) = 2546 MB/s
test gen_xoshiro256starstar_x8          ... bench:      25,723 ns/iter (+/- 17,530) = 2547 MB/s
test gen_xoshiro256starstar_x2          ... bench:       6,261 ns/iter (+/- 3,661) = 2616 MB/s
test gen_chacha4                        ... bench:      24,536 ns/iter (+/- 13,310) = 2671 MB/s
test gen_pcg_fixed_xsl32x4              ... bench:       6,124 ns/iter (+/- 1,438) = 2675 MB/s
test gen_pcg_fixed_xsh32x4              ... bench:       6,082 ns/iter (+/- 13,547) = 2693 MB/s
test gen_lcg32x4                        ... bench:       6,076 ns/iter (+/- 3,772) = 2696 MB/s
test gen_lfsr113_x16                    ... bench:      24,220 ns/iter (+/- 20,261) = 2705 MB/s
test gen_lfsr113_x8                     ... bench:      12,108 ns/iter (+/- 7,811) = 2706 MB/s
test gen_xoshiro512starstar_x2          ... bench:       6,025 ns/iter (+/- 1,002) = 2719 MB/s
test gen_jsf32_x2                       ... bench:       2,979 ns/iter (+/- 1,679) = 2749 MB/s
test gen_lcg32x8                        ... bench:      11,466 ns/iter (+/- 4,920) = 2857 MB/s
test gen_xsm32_x4                       ... bench:       5,630 ns/iter (+/- 3,807) = 2910 MB/s
test gen_lfsr258_x2                     ... bench:       5,543 ns/iter (+/- 5,043) = 2955 MB/s
test gen_xsm32_x16                      ... bench:      21,466 ns/iter (+/- 17,873) = 3053 MB/s
test gen_xsm32_x8                       ... bench:      10,687 ns/iter (+/- 4,087) = 3066 MB/s
test gen_pcg_fixed_xsl32x8              ... bench:      10,267 ns/iter (+/- 4,993) = 3191 MB/s
test gen_chacha4_a                      ... bench:      17,330 ns/iter (+/- 6,992) = 3781 MB/s
test gen_intel_lcg                      ... bench:       4,076 ns/iter (+/- 633) = 4019 MB/s
test gen_mwc8                           ... bench:       3,871 ns/iter (+/- 1,126) = 4232 MB/s
test gen_xorshift128_x2                 ... bench:       1,907 ns/iter (+/- 17) = 4295 MB/s
test gen_sfc64_x2                       ... bench:       3,562 ns/iter (+/- 2,341) = 4599 MB/s
test gen_sfc32_x4                       ... bench:       3,541 ns/iter (+/- 2,689) = 4626 MB/s
test gen_xorshift32_x4                  ... bench:       3,425 ns/iter (+/- 1,746) = 4783 MB/s
test gen_mwc4                           ... bench:       3,309 ns/iter (+/- 1,820) = 4951 MB/s
test gen_sfc64_x8                       ... bench:      13,222 ns/iter (+/- 7,875) = 4956 MB/s
test gen_sfc64_x4                       ... bench:       6,344 ns/iter (+/- 4,123) = 5165 MB/s
test gen_mwc2                           ... bench:       3,077 ns/iter (+/- 1,123) = 5324 MB/s
test gen_jsf32_x4                       ... bench:       2,963 ns/iter (+/- 2,251) = 5529 MB/s
test gen_sfc32_x8                       ... bench:       5,909 ns/iter (+/- 4,696) = 5545 MB/s
test gen_jsf64_x2                       ... bench:       2,942 ns/iter (+/- 1,789) = 5569 MB/s
test gen_sfc32_x16                      ... bench:      11,744 ns/iter (+/- 3,283) = 5580 MB/s
test gen_jsf64_x4                       ... bench:       5,861 ns/iter (+/- 5,589) = 5590 MB/s
test gen_jsf32_x16                      ... bench:      11,347 ns/iter (+/- 4,840) = 5775 MB/s
test gen_sfc16_x16                      ... bench:       5,575 ns/iter (+/- 2,210) = 5877 MB/s
test gen_sfc16_x32                      ... bench:      10,997 ns/iter (+/- 6,275) = 5959 MB/s
test gen_jsf64_x8                       ... bench:      10,970 ns/iter (+/- 8,222) = 5974 MB/s
test gen_jsf32_x8                       ... bench:       5,300 ns/iter (+/- 1,443) = 6182 MB/s
test gen_sfc16_x8                       ... bench:       2,626 ns/iter (+/- 1,096) = 6239 MB/s
test gen_xorshift128_x4                 ... bench:       2,050 ns/iter (+/- 647) = 7992 MB/s
test gen_xorshift32_x8                  ... bench:       4,045 ns/iter (+/- 3,108) = 8100 MB/s
test gen_xorshift128plus_x2             ... bench:       1,832 ns/iter (+/- 1,243) = 8943 MB/s
test gen_xorshift128plus_x8             ... bench:       6,854 ns/iter (+/- 3,117) = 9561 MB/s
test gen_xorshift32_x16                 ... bench:       6,492 ns/iter (+/- 10,851) = 10094 MB/s
test gen_xorshift128plus_x4             ... bench:       3,150 ns/iter (+/- 2,549) = 10402 MB/s
test gen_xorshift128_x8                 ... bench:       3,115 ns/iter (+/- 871) = 10519 MB/s
test gen_xorshift128_x16                ... bench:       5,496 ns/iter (+/- 2,958) = 11924 MB/s
```

`RUSTFLAGS='-C target-cpu=nehalem -C codegen-units=1 -C lto=thin' cargo bench` (the latest Intel microarchitecture without AES-NI):
```rust
test gen_ars7                           ... bench:      18,797 ns/iter (+/- 9,595) = 871 MB/s
test gen_sfc16_x2                       ... bench:       3,447 ns/iter (+/- 2,506) = 1188 MB/s
test gen_xsm32_x2                       ... bench:       6,781 ns/iter (+/- 1,865) = 1208 MB/s
test gen_ars5                           ... bench:      13,009 ns/iter (+/- 6,332) = 1259 MB/s
test gen_pcg32x2                        ... bench:       6,397 ns/iter (+/- 1,588) = 1280 MB/s
test gen_pcg32x8                        ... bench:      22,102 ns/iter (+/- 8,273) = 1482 MB/s
test gen_lfsr113_x2                     ... bench:       5,132 ns/iter (+/- 1,599) = 1596 MB/s
test gen_pcg32x4                        ... bench:       9,607 ns/iter (+/- 2,484) = 1705 MB/s
test gen_pcg_fixed_xsl32x2              ... bench:       4,502 ns/iter (+/- 159) = 1819 MB/s
test gen_pcg_fixed_xsh32x2              ... bench:       4,344 ns/iter (+/- 2,618) = 1885 MB/s
test gen_lcg32x2                        ... bench:       4,248 ns/iter (+/- 1,665) = 1928 MB/s
test gen_sfc32_x2                       ... bench:       3,824 ns/iter (+/- 1,152) = 2142 MB/s
test gen_xoshiro512starstar_x8          ... bench:      30,254 ns/iter (+/- 594) = 2166 MB/s
test gen_xoroshiro128starstar_x2        ... bench:       7,355 ns/iter (+/- 3,242) = 2227 MB/s
test gen_xorshift32_x2                  ... bench:       3,676 ns/iter (+/- 1,069) = 2228 MB/s
test gen_xoroshiro128starstar_x4        ... bench:      14,413 ns/iter (+/- 7,420) = 2273 MB/s
test gen_xoroshiro128starstar_x8        ... bench:      28,045 ns/iter (+/- 16,710) = 2336 MB/s
test gen_xsm64_x2                       ... bench:       6,764 ns/iter (+/- 1,429) = 2422 MB/s
test gen_xsm64_x4                       ... bench:      13,472 ns/iter (+/- 2,879) = 2432 MB/s
test gen_lfsr258_x8                     ... bench:      26,713 ns/iter (+/- 18,211) = 2453 MB/s
test gen_xsm64_x8                       ... bench:      26,274 ns/iter (+/- 3,678) = 2494 MB/s
test gen_xoshiro256starstar_x4          ... bench:      12,822 ns/iter (+/- 9,596) = 2555 MB/s
test gen_xoshiro512starstar_x4          ... bench:      12,821 ns/iter (+/- 8,409) = 2555 MB/s
test gen_xoshiro256starstar_x8          ... bench:      24,840 ns/iter (+/- 267) = 2638 MB/s
test gen_pcg_fixed_xsh32x4              ... bench:       6,054 ns/iter (+/- 4,514) = 2706 MB/s
test gen_xoshiro512starstar_x2          ... bench:       6,021 ns/iter (+/- 1,585) = 2721 MB/s
test gen_xoshiro256starstar_x2          ... bench:       5,998 ns/iter (+/- 2,208) = 2731 MB/s
test gen_jsf32_x2                       ... bench:       2,925 ns/iter (+/- 1,150) = 2800 MB/s
test gen_sfc16_x4                       ... bench:       2,910 ns/iter (+/- 1,408) = 2815 MB/s
test gen_lfsr258_x4                     ... bench:      11,078 ns/iter (+/- 4,768) = 2957 MB/s
test gen_lfsr258_x2                     ... bench:       5,480 ns/iter (+/- 2,203) = 2989 MB/s
test gen_pcg_fixed_xsl32x4              ... bench:       5,421 ns/iter (+/- 3,209) = 3022 MB/s
test gen_chacha4                        ... bench:      21,526 ns/iter (+/- 4,536) = 3044 MB/s
test gen_pcg_fixed_xsh32x8              ... bench:      10,622 ns/iter (+/- 2,354) = 3084 MB/s
test gen_lcg32x4                        ... bench:       4,995 ns/iter (+/- 3,801) = 3280 MB/s
test gen_lfsr113_x16                    ... bench:      19,235 ns/iter (+/- 2,247) = 3407 MB/s
test gen_lfsr113_x4                     ... bench:       4,610 ns/iter (+/- 3,121) = 3554 MB/s
test gen_lfsr113_x8                     ... bench:       8,912 ns/iter (+/- 2,317) = 3676 MB/s
test gen_pcg_fixed_xsl32x8              ... bench:       8,511 ns/iter (+/- 796) = 3850 MB/s
test gen_lcg32x8                        ... bench:       8,338 ns/iter (+/- 43) = 3929 MB/s
test gen_xorshift128_x2                 ... bench:       2,038 ns/iter (+/- 2,443) = 4019 MB/s
test gen_sfc64_x8                       ... bench:      15,807 ns/iter (+/- 15,548) = 4146 MB/s
test gen_xsm32_x16                      ... bench:      15,271 ns/iter (+/- 1,073) = 4291 MB/s
test gen_intel_lcg                      ... bench:       3,630 ns/iter (+/- 2,950) = 4513 MB/s
test gen_sfc64_x2                       ... bench:       3,581 ns/iter (+/- 1,836) = 4575 MB/s
test gen_mwc8                           ... bench:       3,520 ns/iter (+/- 1,264) = 4654 MB/s
test gen_chacha4_a                      ... bench:      13,984 ns/iter (+/- 4,827) = 4686 MB/s
test gen_sfc64_x4                       ... bench:       6,815 ns/iter (+/- 6,897) = 4808 MB/s
test gen_xsm32_x4                       ... bench:       3,366 ns/iter (+/- 180) = 4867 MB/s
test gen_xsm32_x8                       ... bench:       6,684 ns/iter (+/- 550) = 4902 MB/s
test gen_xorshift32_x4                  ... bench:       3,341 ns/iter (+/- 1,817) = 4903 MB/s
test gen_mwc4                           ... bench:       3,310 ns/iter (+/- 751) = 4949 MB/s
test gen_mwc2                           ... bench:       3,076 ns/iter (+/- 1,091) = 5326 MB/s
test gen_jsf32_x4                       ... bench:       2,756 ns/iter (+/- 865) = 5944 MB/s
test gen_jsf64_x2                       ... bench:       2,699 ns/iter (+/- 952) = 6070 MB/s
test gen_sfc16_x8                       ... bench:       2,626 ns/iter (+/- 25) = 6239 MB/s
test gen_sfc32_x4                       ... bench:       2,626 ns/iter (+/- 1,222) = 6239 MB/s
test gen_jsf64_x8                       ... bench:       8,874 ns/iter (+/- 6,448) = 7385 MB/s
test gen_jsf32_x16                      ... bench:       8,870 ns/iter (+/- 3,705) = 7388 MB/s
test gen_sfc32_x8                       ... bench:       4,407 ns/iter (+/- 3,648) = 7435 MB/s
test gen_sfc16_x16                      ... bench:       4,381 ns/iter (+/- 1,839) = 7479 MB/s
test gen_jsf32_x8                       ... bench:       4,355 ns/iter (+/- 3,030) = 7524 MB/s
test gen_jsf64_x4                       ... bench:       4,347 ns/iter (+/- 645) = 7538 MB/s
test gen_sfc32_x16                      ... bench:       8,652 ns/iter (+/- 7,062) = 7574 MB/s
test gen_sfc16_x32                      ... bench:       8,644 ns/iter (+/- 426) = 7581 MB/s
test gen_xorshift128_x4                 ... bench:       2,046 ns/iter (+/- 622) = 8007 MB/s
test gen_xorshift32_x8                  ... bench:       4,041 ns/iter (+/- 62) = 8108 MB/s
test gen_xorshift128plus_x2             ... bench:       1,808 ns/iter (+/- 235) = 9061 MB/s
test gen_xorshift128plus_x8             ... bench:       6,477 ns/iter (+/- 439) = 10118 MB/s
test gen_xorshift128_x8                 ... bench:       3,119 ns/iter (+/- 32) = 10505 MB/s
test gen_xorshift128plus_x4             ... bench:       3,089 ns/iter (+/- 433) = 10607 MB/s
test gen_xorshift32_x16                 ... bench:       5,657 ns/iter (+/- 171) = 11584 MB/s
test gen_xorshift128_x16                ... bench:       5,379 ns/iter (+/- 1,882) = 12183 MB/s
```

`RUSTFLAGS='-C target-cpu=native -C codegen-units=1 -C lto=thin' cargo bench` (2.3GHz dual-core Intel Core i5 - Sandy Bridge):
```rust
test gen_pcg32x2                        ... bench:       6,932 ns/iter (+/- 406) = 1181 MB/s
test gen_sfc16_x2                       ... bench:       3,221 ns/iter (+/- 540) = 1271 MB/s
test gen_xsm32_x2                       ... bench:       5,779 ns/iter (+/- 853) = 1417 MB/s
test gen_pcg32x4                        ... bench:       9,839 ns/iter (+/- 2,100) = 1665 MB/s
test gen_pcg32x8                        ... bench:      18,893 ns/iter (+/- 4,447) = 1734 MB/s
test gen_lfsr113_x2                     ... bench:       4,208 ns/iter (+/- 1,958) = 1946 MB/s
test gen_lcg32x2                        ... bench:       3,900 ns/iter (+/- 1,784) = 2100 MB/s
test gen_pcg_fixed_xsl32x2              ... bench:       3,898 ns/iter (+/- 2,289) = 2101 MB/s
test gen_pcg_fixed_xsh32x2              ... bench:       3,894 ns/iter (+/- 587) = 2103 MB/s
test gen_xoroshiro128starstar_x4        ... bench:      14,889 ns/iter (+/- 430) = 2200 MB/s
test gen_xoroshiro128starstar_x2        ... bench:       7,251 ns/iter (+/- 47) = 2259 MB/s
test gen_xsm64_x2                       ... bench:       6,602 ns/iter (+/- 179) = 2481 MB/s
test gen_xoshiro512starstar_x2          ... bench:       6,530 ns/iter (+/- 38) = 2509 MB/s
test gen_xoroshiro128starstar_x8        ... bench:      25,780 ns/iter (+/- 13,992) = 2542 MB/s
test gen_sfc32_x2                       ... bench:       3,221 ns/iter (+/- 135) = 2543 MB/s
test gen_xsm64_x4                       ... bench:      12,764 ns/iter (+/- 74) = 2567 MB/s
test gen_xsm64_x8                       ... bench:      25,527 ns/iter (+/- 9,878) = 2567 MB/s
test gen_xoshiro256starstar_x4          ... bench:      12,723 ns/iter (+/- 2,463) = 2575 MB/s
test gen_xoshiro256starstar_x2          ... bench:       6,346 ns/iter (+/- 669) = 2581 MB/s
test gen_xoshiro512starstar_x4          ... bench:      12,323 ns/iter (+/- 9,976) = 2659 MB/s
test gen_pcg_fixed_xsh32x4              ... bench:       6,051 ns/iter (+/- 943) = 2707 MB/s
test gen_xoshiro512starstar_x8          ... bench:      23,841 ns/iter (+/- 5,938) = 2748 MB/s
test gen_lfsr258_x4                     ... bench:      11,687 ns/iter (+/- 255) = 2803 MB/s
test gen_xoshiro256starstar_x8          ... bench:      23,231 ns/iter (+/- 8,848) = 2821 MB/s
test gen_lfsr258_x8                     ... bench:      23,021 ns/iter (+/- 111) = 2846 MB/s
test gen_pcg_fixed_xsh32x8              ... bench:      11,038 ns/iter (+/- 2,054) = 2968 MB/s
test gen_pcg_fixed_xsl32x4              ... bench:       5,317 ns/iter (+/- 52) = 3081 MB/s
test gen_lfsr258_x2                     ... bench:       4,983 ns/iter (+/- 100) = 3287 MB/s
test gen_xorshift32_x2                  ... bench:       2,478 ns/iter (+/- 92) = 3305 MB/s
test gen_lcg32x4                        ... bench:       4,897 ns/iter (+/- 2,762) = 3345 MB/s
test gen_lfsr113_x8                     ... bench:       9,565 ns/iter (+/- 5,668) = 3425 MB/s
test gen_lfsr113_x16                    ... bench:      19,124 ns/iter (+/- 11,686) = 3426 MB/s
test gen_chacha4                        ... bench:      18,856 ns/iter (+/- 265) = 3475 MB/s
test gen_sfc16_x4                       ... bench:       2,255 ns/iter (+/- 722) = 3632 MB/s
test gen_pcg_fixed_xsl32x8              ... bench:       8,805 ns/iter (+/- 2,169) = 3721 MB/s
test gen_lcg32x8                        ... bench:       8,328 ns/iter (+/- 259) = 3934 MB/s
test gen_lfsr113_x4                     ... bench:       3,895 ns/iter (+/- 45) = 4206 MB/s
test gen_jsf32_x2                       ... bench:       1,911 ns/iter (+/- 8) = 4286 MB/s
test gen_intel_lcg                      ... bench:       3,617 ns/iter (+/- 1,985) = 4529 MB/s
test gen_chacha4_a                      ... bench:      14,272 ns/iter (+/- 4,059) = 4591 MB/s
test gen_xsm32_x16                      ... bench:      14,235 ns/iter (+/- 6,606) = 4603 MB/s
test gen_ars7                           ... bench:       3,531 ns/iter (+/- 832) = 4640 MB/s
test gen_xsm32_x8                       ... bench:       6,963 ns/iter (+/- 45) = 4706 MB/s
test gen_xorshift128_x2                 ... bench:       1,690 ns/iter (+/- 963) = 4847 MB/s
test gen_sfc64_x4                       ... bench:       6,665 ns/iter (+/- 114) = 4916 MB/s
test gen_sfc64_x2                       ... bench:       3,186 ns/iter (+/- 58) = 5142 MB/s
test gen_mwc8                           ... bench:       3,168 ns/iter (+/- 192) = 5171 MB/s
test gen_mwc4                           ... bench:       2,991 ns/iter (+/- 659) = 5477 MB/s
test gen_sfc64_x8                       ... bench:      11,957 ns/iter (+/- 79) = 5480 MB/s
test gen_xsm32_x4                       ... bench:       2,808 ns/iter (+/- 89) = 5834 MB/s
test gen_mwc2                           ... bench:       2,759 ns/iter (+/- 52) = 5938 MB/s
test gen_ars5                           ... bench:       2,436 ns/iter (+/- 1,421) = 6725 MB/s
test gen_jsf32_x16                      ... bench:       9,035 ns/iter (+/- 1,306) = 7253 MB/s
test gen_jsf64_x8                       ... bench:       8,990 ns/iter (+/- 3,464) = 7289 MB/s
test gen_jsf64_x4                       ... bench:       4,487 ns/iter (+/- 1,424) = 7302 MB/s
test gen_jsf32_x8                       ... bench:       4,482 ns/iter (+/- 959) = 7311 MB/s
test gen_sfc16_x16                      ... bench:       4,272 ns/iter (+/- 1,205) = 7670 MB/s
test gen_sfc32_x8                       ... bench:       4,272 ns/iter (+/- 3,584) = 7670 MB/s
test gen_xorshift32_x4                  ... bench:       2,124 ns/iter (+/- 15) = 7713 MB/s
test gen_sfc32_x16                      ... bench:       8,342 ns/iter (+/- 4,882) = 7856 MB/s
test gen_sfc16_x32                      ... bench:       8,264 ns/iter (+/- 318) = 7930 MB/s
test gen_sfc16_x8                       ... bench:       2,065 ns/iter (+/- 1,155) = 7934 MB/s
test gen_sfc32_x4                       ... bench:       2,064 ns/iter (+/- 1,696) = 7937 MB/s
test gen_xorshift128_x8                 ... bench:       4,078 ns/iter (+/- 120) = 8035 MB/s
test gen_xorshift128_x16                ... bench:       7,974 ns/iter (+/- 5,880) = 8218 MB/s
test gen_xorshift32_x8                  ... bench:       3,912 ns/iter (+/- 233) = 8376 MB/s
test gen_jsf32_x4                       ... bench:       1,829 ns/iter (+/- 988) = 8957 MB/s
test gen_jsf64_x2                       ... bench:       1,823 ns/iter (+/- 136) = 8987 MB/s
test gen_xorshift128plus_x4             ... bench:       3,635 ns/iter (+/- 1,573) = 9014 MB/s
test gen_xorshift128plus_x8             ... bench:       7,000 ns/iter (+/- 242) = 9362 MB/s
test gen_xorshift128_x4                 ... bench:       1,520 ns/iter (+/- 510) = 10778 MB/s
test gen_xorshift128plus_x2             ... bench:       1,451 ns/iter (+/- 690) = 11291 MB/s
test gen_xorshift32_x16                 ... bench:       5,585 ns/iter (+/- 2,138) = 11734 MB/s
```
