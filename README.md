## About

This repo is an attempt to optimize alt_bn128 calls in the solana (agave) validator. The current alt_bn128 primitives work with BE inputs, and the arkwork library works with LE. So in the code the input is converted from BE to LE and then the output is converted back.

## Benchmarks

add_bench/alt_bn128_addition time: [207.47 µs 209.40 µs 211.35 µs]
Found 4 outliers among 1000 measurements (0.40%)
2 (0.20%) high mild
2 (0.20%) high severe

add_bench/alt_bn128_addition_le time: [206.39 µs 208.34 µs 210.35 µs]
Found 14 outliers among 1000 measurements (1.40%)
13 (1.30%) high mild
1 (0.10%) high severe

mul_bench/alt_bn128_multiplication time: [271.75 µs 274.32 µs 276.94 µs]
Found 6 outliers among 1000 measurements (0.60%)
6 (0.60%) high mild

mul_bench/alt_bn128_multiplication_le time: [268.32 µs 271.02 µs 273.77 µs]
Found 17 outliers among 1000 measurements (1.70%)
17 (1.70%) high mild

paring_bench/alt_bn128_pairing time: [2.4531 ms 2.4804 ms 2.5089 ms]
Found 20 outliers among 1000 measurements (2.00%)
16 (1.60%) high mild
4 (0.40%) high severe

paring_bench/alt_bn128_pairing_le time: [2.4437 ms 2.4708 ms 2.4987 ms]
Found 18 outliers among 1000 measurements (1.80%)
16 (1.60%) high mild
2 (0.20%) high severe
