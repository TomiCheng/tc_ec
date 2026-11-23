# tc_ec_key

[![crates.io](https://img.shields.io/crates/v/tc_ec_key.svg)](https://crates.io/crates/tc_ec_key)
[![docs.rs](https://docs.rs/tc_ec_key/badge.svg)](https://docs.rs/tc_ec_key)
[![CI](https://github.com/TomiCheng/tc_ec/actions/workflows/ci.yml/badge.svg)](https://github.com/TomiCheng/tc_ec/actions/workflows/ci.yml)
[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](#license)
![rustc](https://img.shields.io/badge/rustc-1.85+-blue.svg)

Elliptic-curve keys after Bouncy Castle's `ECPrivateKeyParameters`,
`ECPublicKeyParameters` and `ECKeyPairGenerator`, over any `tc_ec_core`
curve, shared by ECDH, ECDSA and the rest. The generator G stands for the
domain. `no_std` with `alloc`, no `unsafe`, Rust 1.85 or later.

## Types

- `EcPrivateKey` — the scalar d in `[1, n - 1]`, wiped on drop.
- `EcPublicKey` — the point Q, checked to be a valid point of the subgroup.
- `EcKeyGenerationParams` — the generator G a key-pair generator draws keys for.
- `EcKeyPairGenerator` — draws d and computes Q = d·G, by the curve's own
  multiplication or a base multiplier with a table for G.
- `EcError` — why a key or a domain was refused.

## Traits

- `tc_asym_cipher::KeyPairGenerator`, `KeyPairGeneratorInit` — implemented by
  `EcKeyPairGenerator`.
- `tc_asym_cipher::AsymmetricKeyParams` — implemented by both keys.

## Features

- None; `alloc` is always used, for the integers of `tc_bigint`.

## Usage

```toml
[dependencies]
tc_ec_key = "0.1.0"
tc_asym_cipher = "0.2.0"
tc_fp_curve = "0.1.0"
rand = "0.10"
```

```rust
use tc_asym_cipher::{KeyPairGenerator, KeyPairGeneratorInit};
use tc_ec_key::{EcKeyGenerationParams, EcKeyPairGenerator};
use tc_fp_curve::{FpCtCurve, named_curves::SECP256R1};

let field = SECP256R1.field();
let curve = SECP256R1.ct_curve(&field);
let mut generator = EcKeyPairGenerator::<&FpCtCurve<'_>, _>::new(rand::rng());
generator.init(&EcKeyGenerationParams::new(SECP256R1.ct_generator(&curve))).expect("a valid generator");
let (public_key, private_key) = generator.generate_key_pair().expect("initialized");
```

## Security

The curve type decides the timing. Over a constant-time curve, d is drawn and
Q computed in constant time; keys of a variable-time curve are for public
values only. Checking a public point is variable time, as it is public. The
documentation states the timing of every item.

## License

Licensed under either the [MIT license](LICENSE-MIT) or the
[Apache License, Version 2.0](LICENSE-APACHE), at your option.
