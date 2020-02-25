use crate::pdre_api::utils::{str, Decoder, ParsedInput, Runtime};
use parity_scale_codec::Encode;
use sp_core::crypto::key_types::DUMMY;

pub fn ext_crypto_ed25519_public_keys_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new_keystore();

    let seed1 = input.get(0);
    let seed2 = input.get(1);

    // Generate first key
    let pubkey1 = rtm
        .call(
            "rtm_ext_crypto_ed25519_generate_version_1",
            &(DUMMY.0, Some(seed1)).encode(),
        )
        .decode_val();

    // Generate second key
    let pubkey2 = rtm
        .call(
            "rtm_ext_crypto_ed25519_generate_version_1",
            &(DUMMY.0, Some(seed2)).encode(),
        )
        .decode_val();

    let mut res = rtm
        .call(
            "rtm_ext_crypto_ed25519_public_keys_version_1",
            &DUMMY.0.encode(),
        )
        .decode_val();

    // TODO: decode this properly
    res.remove(0);
    assert_eq!(res.len(), 64);
    let res1 = &res[..32]; // first pubkey
    let res2 = &res[32..]; // second pubkey

    if pubkey1 != res1 && pubkey1 != res2 {
        panic!("Return value does not include pubkey")
    }

    if pubkey2 != res1 && pubkey2 != res2 {
        panic!("Return value does not include pubkey")
    }

    println!("1. Public key: {}", hex::encode(res1));
    println!("2. Public key: {}", hex::encode(res2));
}

pub fn ext_crypto_ed25519_generate_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new_keystore();

    let seed = input.get(0);

    let res = rtm
        .call(
            "rtm_ext_crypto_ed25519_generate_version_1",
            &(DUMMY.0, Some(seed)).encode(),
        )
        .decode_val();

    println!("{}", hex::encode(res));
}

pub fn ext_crypto_ed25519_sign_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new_keystore();

    let seed = input.get(0);
    let msg = input.get(1);

    // Generate a key
    let pubkey = rtm
        .call(
            "rtm_ext_crypto_ed25519_generate_version_1",
            &(DUMMY.0, Some(seed)).encode(),
        )
        .decode_val();

    // Sign message
    let res = rtm
        .call(
            "rtm_ext_crypto_ed25519_sign_version_1",
            &(DUMMY.0, &pubkey, msg).encode(),
        )
        .decode_option()
        .unwrap();

    println!("Message: {}", str(&msg));
    println!("Public key: {}", hex::encode(pubkey));
    println!("Signature: {}", hex::encode(res));
}

pub fn ext_crypto_ed25519_verify_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new_keystore();

    let seed = input.get(0);
    let msg = input.get(1);

    // Generate a key
    let pubkey = rtm
        .call(
            "rtm_ext_crypto_ed25519_generate_version_1",
            &(DUMMY.0, Some(seed)).encode(),
        )
        .decode_val();

    // Sign message
    let sig = rtm
        .call(
            "rtm_ext_crypto_ed25519_sign_version_1",
            &(DUMMY.0, &pubkey, &msg).encode(),
        )
        .decode_option()
        .unwrap();

    // Verify signature
    let verified = rtm
        .call(
            "rtm_ext_crypto_ed25519_verify_version_1",
            &(&sig, &msg, &pubkey).encode(),
        )
        .decode_bool();
    assert_eq!(verified, true);

    println!("Message: {}", str(&msg));
    println!("Public key: {}", hex::encode(&pubkey));
    println!("Signature: {}", hex::encode(&sig));
    if verified {
        println!("GOOD SIGNATURE");
    } else {
        println!("BAD SIGNATURE");
    }
}

pub fn ext_crypto_sr25519_public_keys_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new_keystore();

    let seed1 = input.get(0);
    let seed2 = input.get(1);

    // Generate first key
    let pubkey1 = rtm
        .call(
            "rtm_ext_crypto_sr25519_generate_version_1",
            &(DUMMY.0, Some(seed1)).encode(),
        )
        .decode_val();

    // Generate second key
    let pubkey2 = rtm
        .call(
            "rtm_ext_crypto_sr25519_generate_version_1",
            &(DUMMY.0, Some(seed2)).encode(),
        )
        .decode_val();

    let mut res = rtm
        .call(
            "rtm_ext_crypto_sr25519_public_keys_version_1",
            &DUMMY.0.encode(),
        )
        .decode_val();

    // TODO: decode this properly
    res.remove(0);
    assert_eq!(res.len(), 64);
    let res1 = &res[..32]; // first pubkey
    let res2 = &res[32..]; // second pubkey

    if pubkey1 != res1 && pubkey1 != res2 {
        panic!("Return value does not include pubkey")
    }

    if pubkey2 != res1 && pubkey2 != res2 {
        panic!("Return value does not include pubkey")
    }

    println!("1. Public key: {}", hex::encode(res1));
    println!("2. Public key: {}", hex::encode(res2));
}

pub fn ext_crypto_sr25519_generate_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new_keystore();

    let seed = input.get(0);
    let seed_opt = if seed.is_empty() { None } else { Some(seed) };

    // Generate a key
    let res = rtm
        .call(
            "rtm_ext_crypto_sr25519_generate_version_1",
            &(DUMMY.0, seed_opt).encode(),
        )
        .decode_val();

    println!("{}", hex::encode(res));
}

pub fn ext_crypto_sr25519_sign_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new_keystore();

    let seed = input.get(0);
    let msg = input.get(1);

    // Generate a key
    let pubkey = rtm
        .call(
            "rtm_ext_crypto_sr25519_generate_version_1",
            &(DUMMY.0, Some(seed)).encode(),
        )
        .decode_val();

    // Sign message
    let res = rtm
        .call(
            "rtm_ext_crypto_sr25519_sign_version_1",
            &(DUMMY.0, &pubkey, msg).encode(),
        )
        .decode_option()
        .unwrap();

    println!("Message: {}", str(&msg));
    println!("Public key: {}", hex::encode(pubkey));
    println!("Signature: {}", hex::encode(res));
}

pub fn ext_crypto_sr25519_verify_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new_keystore();

    let seed = input.get(0);
    let msg = input.get(1);

    // Generate a key
    let pubkey = rtm
        .call(
            "rtm_ext_crypto_sr25519_generate_version_1",
            &(DUMMY.0, Some(seed)).encode(),
        )
        .decode_val();

    // Sign message
    let sig = rtm
        .call(
            "rtm_ext_crypto_sr25519_sign_version_1",
            &(DUMMY.0, &pubkey, &msg).encode(),
        )
        .decode_option()
        .unwrap();

    let verified = rtm
        .call(
            "rtm_ext_crypto_sr25519_verify_version_1",
            &(&sig, &msg, &pubkey).encode(),
        )
        .decode_bool();

    assert_eq!(verified, true);

    println!("Message: {}", str(&msg));
    println!("Public key: {}", hex::encode(&pubkey));
    println!("Signature: {}", hex::encode(&sig));
    if verified {
        println!("GOOD SIGNATURE");
    } else {
        println!("BAD SIGNATURE");
    }
}

pub fn ext_crypto_secp256k1_ecdsa_recover_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new_keystore();

    let sig = input.get(0);
    let msg = input.get(1);

    let _ = rtm
        .call(
            "rtm_ext_crypto_secp256k1_ecdsa_recover_version_1",
            &(&sig, &msg).encode(),
        )
        .decode_val();

    // TODO...
}

// TODO
#[allow(dead_code)]
pub fn ext_crypto_secp256k1_ecdsa_recover_compressed_version_1(_input: ParsedInput) {}

pub fn ext_hashing_keccak_256_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new();

    let data = input.get(0);

    let res = rtm
        .call("rtm_ext_hashing_keccak_256_version_1", &(data).encode())
        .decode_val();

    println!("{}", hex::encode(res));
}

pub fn ext_hashing_sha2_256_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new();

    let data = input.get(0);

    let res = rtm
        .call("rtm_ext_hashing_sha2_256_version_1", &(data).encode())
        .decode_val();

    println!("{}", hex::encode(res));
}

pub fn ext_hashing_blake2_128_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new();

    let data = input.get(0);

    let res = rtm
        .call("rtm_ext_hashing_blake2_128_version_1", &(data).encode())
        .decode_val();

    println!("{}", hex::encode(res));
}

pub fn ext_hashing_blake2_256_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new();

    let data = input.get(0);

    let res = rtm
        .call("rtm_ext_hashing_blake2_256_version_1", &(data).encode())
        .decode_val();

    println!("{}", hex::encode(res));
}

pub fn ext_hashing_twox_256_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new();

    let data = input.get(0);

    let res = rtm
        .call("rtm_ext_hashing_twox_256_version_1", &(data).encode())
        .decode_val();

    println!("{}", hex::encode(res));
}

pub fn ext_hashing_twox_128_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new();

    let data = input.get(0);

    let res = rtm
        .call("rtm_ext_hashing_twox_128_version_1", &(data).encode())
        .decode_val();

    println!("{}", hex::encode(res));
}

pub fn ext_hashing_twox_64_version_1(input: ParsedInput) {
    let mut rtm = Runtime::new();

    let data = input.get(0);

    let res = rtm
        .call("rtm_ext_hashing_twox_64_version_1", &(data).encode())
        .decode_val();

    println!("{}", hex::encode(res));
}
