use criterion::{criterion_group, criterion_main, Criterion};

// use crate::prelude::target_arch::{alt_bn128_addition, a};
use solana_alt_bn128_le::prelude::{
    alt_bn128_addition, alt_bn128_addition_le, alt_bn128_multiplication,
    alt_bn128_multiplication_le, alt_bn128_pairing, alt_bn128_pairing_le, convert_endianness_128,
    convert_endianness_64, ALT_BN128_G1_POINT_SIZE, ALT_BN128_PAIRING_ELEMENT_LEN,
};

fn add_bench(c: &mut Criterion) {
    let test = "18b18acfb4c2c30276db5411368e7185b311dd124691610c5d3b74034e093dc9063c909c4720840cb5134cb9f59fa749755796819658d32efc0d288198f3726607c2b7f58a84bd6145f00c9c2bc0bb1a187f20ff2c92963a88019e7c6a014eed06614e20c147e940f2d70da3f74c9a17df361706a4485c742bd6788478fa17d7";
    let input = array_bytes::hex2bytes_unchecked(test);
    let input_le = convert_endianness_64(&input);

    let mut group = c.benchmark_group("add_bench");

    group.sample_size(1000);

    group.bench_function("alt_bn128_addition", |b| {
        b.iter(|| alt_bn128_addition(&input))
    });

    group.bench_function("alt_bn128_addition_le", |b| {
        b.iter(|| alt_bn128_addition_le(&input_le))
    });

    group.finish();
}

fn mul_bench(c: &mut Criterion) {
    let test = "1a87b0584ce92f4593d161480614f2989035225609f08058ccfa3d0f940febe31a2f3c951f6dadcc7ee9007dff81504b0fcd6d7cf59996efdc33d92bf7f9f8f6ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    let input = array_bytes::hex2bytes_unchecked(test);
    let input_le = convert_endianness_64(&input);

    let mut group = c.benchmark_group("mul_bench");

    group.sample_size(1000);

    group.bench_function("alt_bn128_multiplication", |b| {
        b.iter(|| alt_bn128_multiplication(&input))
    });

    group.bench_function("alt_bn128_multiplication_le", |b| {
        b.iter(|| alt_bn128_multiplication_le(&input_le))
    });

    group.finish();
}

fn paring_bench(c: &mut Criterion) {
    let test = "1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f593034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf704bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a416782bb8324af6cfc93537a2ad1a445cfd0ca2a71acd7ac41fadbf933c2a51be344d120a2a4cf30c1bf9845f20c6fe39e07ea2cce61f0c9bb048165fe5e4de877550111e129f1cf1097710d41c4ac70fcdfa5ba2023c6ff1cbeac322de49d1b6df7c2032c61a830e3c17286de9462bf242fca2883585b93870a73853face6a6bf411198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c21800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa";
    let input = array_bytes::hex2bytes_unchecked(test);

    let mut input_new = Vec::new();
    let ele_len = input.len().saturating_div(ALT_BN128_PAIRING_ELEMENT_LEN);
    for i in 0..ele_len {
        input_new.push(convert_endianness_64(
            &input[i * ALT_BN128_PAIRING_ELEMENT_LEN
                ..i * ALT_BN128_PAIRING_ELEMENT_LEN + ALT_BN128_G1_POINT_SIZE],
        ));
        input_new.push(convert_endianness_128(
            &input[i * ALT_BN128_PAIRING_ELEMENT_LEN + ALT_BN128_G1_POINT_SIZE
                ..i * ALT_BN128_PAIRING_ELEMENT_LEN + ALT_BN128_PAIRING_ELEMENT_LEN],
        ));
    }
    let input_le: Vec<u8> = input_new.into_iter().flatten().collect();

    let mut group = c.benchmark_group("paring_bench");

    group.sample_size(1000);

    group.bench_function("alt_bn128_pairing", |b| {
        b.iter(|| alt_bn128_pairing(&input))
    });

    group.bench_function("alt_bn128_pairing_le", |b| {
        b.iter(|| alt_bn128_pairing_le(&input_le))
    });

    group.finish();
}

criterion_group!(alt_bn128, add_bench, mul_bench, paring_bench);
criterion_main!(alt_bn128);
