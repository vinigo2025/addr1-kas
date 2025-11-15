//! Пример генерации новой seed-фразы, приватного/публичного ключа и адреса Kaspa для mainnet

use kaspa_bip32::{Mnemonic, Language, WordCount, ExtendedPrivateKey, ExtendedPublicKey};
use kaspa_addresses::{Address, Prefix};

fn main() {
    // Генерация новой мнемонической фразы (24 слова)
    let mnemonic = Mnemonic::random(WordCount::Words24, Language::English).expect("mnemonic");
    println!("Mnemonic: {}", mnemonic.phrase_string());

    // Получение seed из мнемоники
    let seed = mnemonic.to_seed(""); // без BIP39 passphrase
    println!("Seed: {}", hex::encode(&seed));

    // Генерация приватного ключа из seed
    let xprv = ExtendedPrivateKey::new_master(&seed).expect("xprv");
    println!("Private key (xprv): {}", xprv.to_string());

    // Получение публичного ключа
    let xpub = ExtendedPublicKey::from_private(&xprv);
    println!("Public key (xpub): {}", xpub.to_string());

    // Получение адреса Kaspa для mainnet (P2PK)
    let public_key_bytes = xpub.public_key().to_bytes();
    let address = Address::new(Prefix::Mainnet, &public_key_bytes);
    println!("Kaspa address (mainnet): {}", address.to_string());
}
