use bip32::{XPrv, ChildNumber};
use secp256k1::{Secp256k1, SecretKey};
use hex;
use rand::RngCore;
use kaspa_addresses::{Address, Prefix, Version};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_suffix = "yes";
    let mut attempt_count = 0;
    
    loop {
        attempt_count += 1;
        if attempt_count % 100 == 0 {
            println!("Working... Attempt #{}", attempt_count);
        }
        
        //     
        let mut entropy = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut entropy);
        let mnemonic = bip39::Mnemonic::from_entropy(&entropy)?;
        
        let seed = mnemonic.to_seed("");

        // BIP32 -
        let master_key = XPrv::new(&seed)?;
        
        //    m/44'/111111'/0'/0/0
        let child = master_key
            .derive_child(ChildNumber::new(44, true)?)?
            .derive_child(ChildNumber::new(111111, true)?)?
            .derive_child(ChildNumber::new(0, true)?)?
            .derive_child(ChildNumber::new(0, false)?)?
            .derive_child(ChildNumber::new(0, false)?)?;
        
        let private_key_bytes = child.private_key().to_bytes();

        // x-only pubkey
        let x_only = x_only_pub(&private_key_bytes)?;

        //  Kaspa   x-only  
        let address = generate_kaspa_address_from_pubkey(&x_only)?;
        
        // ,     "gg7"
        if address.ends_with(target_suffix) {
            println!("\n=== SUCCESS! Found address with suffix '{}' ===", target_suffix);
            println!("MNEMONIC: {}", mnemonic);
            println!("Seed (hex): {}", hex::encode(&seed));
            println!("Private key (hex): {}", hex::encode(private_key_bytes));
            println!("X-only pubkey (hex): {}", hex::encode(x_only));
            println!("Kaspa address: {}", address);
            println!("Attempts: {}", attempt_count);
            
            //  
            if is_valid_kaspa_address(&address) {
                println!("Address validation: VALID");
            } else {
                println!("Address validation: INVALID");
            }
            
            break;
        }
    }

    Ok(())
}

fn x_only_pub(priv_key: &[u8]) -> Result<[u8; 32], Box<dyn std::error::Error>> {
    let secp = Secp256k1::new();
    
    //   
    let mut secret_key = SecretKey::from_slice(priv_key)?;
    
    //    
    let public_key = secret_key.public_key(&secp);
    
    //     (33 )
    let compressed_pubkey = public_key.serialize();
    
    // ,   y-  (    )
    if compressed_pubkey[0] == 0x03 {
        //  y ,   
        secret_key = secret_key.negate();
        let public_key = secret_key.public_key(&secp);
        let compressed_pubkey = public_key.serialize();
        let mut x_only = [0u8; 32];
        x_only.copy_from_slice(&compressed_pubkey[1..33]);
        Ok(x_only)
    } else {
        let mut x_only = [0u8; 32];
        x_only.copy_from_slice(&compressed_pubkey[1..33]);
        Ok(x_only)
    }
}

///  Kaspa     (32 )
pub fn generate_kaspa_address_from_pubkey(pubkey_bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    // ,     32 
    if pubkey_bytes.len() != 32 {
        return Err("    32 ".into());
    }
    
    //    32 
    let pubkey_array: [u8; 32] = pubkey_bytes
        .try_into()
        .expect("Failed to convert to 32 byte array");
    
    //    mainnet
    let address = Address::new(
        Prefix::Mainnet,    // Mainnet
        Version::PubKey,    // P2PKH 
        &pubkey_array,      // 32-  
    );
    
    //   (Bech32 )
    Ok(address.to_string())
}

///  Kaspa 
fn is_valid_kaspa_address(address_str: &str) -> bool {
    match Address::try_from(address_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}
