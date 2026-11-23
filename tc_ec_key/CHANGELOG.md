# Changelog

All notable changes to `tc_ec_key` are documented in this file.

## 0.1.0 - 2026-11-23

### Added

- `EcPrivateKey` and `EcPublicKey`, after Bouncy Castle's
  `ECPrivateKeyParameters` and `ECPublicKeyParameters`, with the generator G
  standing for `ECDomainParameters`: a point carries its curve, and the curve
  its order and cofactor. Both validate as Bouncy Castle does.
- `EcKeyGenerationParams` and `EcKeyPairGenerator`, after
  `ECKeyGenerationParameters` and `ECKeyPairGenerator`. `init` builds a
  `tc_ec_core` base multiplier for G, `PerCall` by default, so every key pair
  reuses its table.
- `EcError`.

### Compatibility

- The private scalar is drawn as Bouncy Castle draws it, rejecting a sparse
  NAF, with the weight counted in constant time.
- `EcKeyGenerationParams` holds no random source; the generator takes one when
  it is made.
- Always uses `alloc`, through `tc_bigint`.
