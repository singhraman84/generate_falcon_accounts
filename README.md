# Falcon Accounts (Substrate Experimental Integration)
This project explores integrating Falcon post-quantum keypairs into a Substrate-compatible account system. It provides deterministic account generation from a fixed seed and maps Falcon public keys into standard Substrate AccountId32 identifiers using Blake2 hashing and SS58 encoding.

The goal is to experiment with how post-quantum cryptography can coexist with existing Substrate account infrastructure.

# Overview
Each Falcon account is derived through the following pipeline:

- A 32-byte deterministic seed is used as input, for experimentation determinism is helpful
- A Falcon keypair is generated using falcon-rs
- The public key is extracted
- The public key is hashed using blake2_256
- The resulting 32-byte hash is used as a AccountId32
- The account is displayed in SS58 format for Substrate compatibility

# Example Output
=== FALCON 512 ===
AccountId32 (hex): e5d4361c0b402841991bfef371a734b4489d732e2ae9c6107b8f3df5ff0ffe46
SS58 Address:      5HG3rzNxMmxMmomx3bNZ4hq3vA7A8DhGmE11QHnfuboGjYrY

=== FALCON 1024 ===
AccountId32 (hex): c1926f85424f2df0fce97c172cfd4792af89d9e5b70326a62bc6358625766072
SS58 Address:      5GSWbiZ64aoCB8FYsbRUgUHJwBSTXhgsezwnogeVK8VEemeA

# Notes
- This project is experimental and not intended for production security systems.
- Falcon keys are used here only for account derivation, not for transaction signing inside the runtime. The intention of this project is not provide a way to generate valid account IDs which can be used in Substrate - Blockchain for experimenting Falcon based DSA algorithm integration. 
- SS58 encoding is used purely for compatibility with Substrate tooling.
