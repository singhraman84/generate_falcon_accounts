use falcon::prelude::*;
use sp_core::crypto::Ss58Codec;
use sp_core::blake2_256;
use sp_core::crypto::AccountId32;

// Fixed seed to generate deterministic keypair. I am keeping this as 32 bytes to retain consistency wit other DSA algorithms
const SEED: [u8; 32] = *b"12345678901234567890123456789012";

// The function to convert Falcon public key to AccountId32 (32 bytes)
fn pubkey_to_account(pubkey: &[u8]) -> AccountId32 {
    let hash = blake2_256(pubkey);
    AccountId32::from(hash)
}

fn main() {
  
    // Falcon 512 deterministic keypair generation and accountID32 conversion
    let kp512 = FnDsaKeyPair::generate_deterministic(&SEED, 9)
        .expect("Falcon512 keygen failed");

    let pk512 = kp512.public_key();
    let acc512 = pubkey_to_account(pk512);

    // Print the AccountID32 and its SS58 address for falcon 512. SS58 address is used because this is recognised as an account in Substrate Blockahin node
    println!("=== FALCON 512 ===");
    println!("AccountId32 (hex): {:02x?}", acc512);
    println!("SS58 Address:      {}", acc512.to_ss58check());


    // Falcon 1024 deterministic keypair generation and accountID32 conversion
    let kp1024 = FnDsaKeyPair::generate_deterministic(&SEED, 10)
        .expect("Falcon1024 keygen failed");

    let pk1024 = kp1024.public_key();
    let acc1024 = pubkey_to_account(pk1024);

    // Print the AccountID32 and its SS58 address for falcon 1024. SS58 address is used because this is recognised as an account in Substrate Blockahin node
    println!("\n=== FALCON 1024 ===");
    println!("AccountId32 (hex): {:02x?}", acc1024);
    println!("SS58 Address:      {}", acc1024.to_ss58check());


    // Determinism check
    let kp512_b = FnDsaKeyPair::generate_deterministic(&SEED, 9).unwrap();
    assert_eq!(kp512.public_key(), kp512_b.public_key());

    println!("\n Deterministic: same seed -> same Falcon keypair -> same account");
}
