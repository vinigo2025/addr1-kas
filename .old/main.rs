use clap::Parser;
use kaspa_addresses::{Address, Prefix, Version};

///   Kaspa   
#[derive(Parser)]
#[command(name = "kaspa_address_generator")]
#[command(about = "  Kaspa   ", long_about = None)]
struct Cli {
    ///    hex- (32 )
    #[arg(short = 'p', long = "pk", value_name = "PUBKEY")]
    pubkey: String,
}

//        ( )
pub fn generate_kaspa_address_from_pubkey(pubkey_bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    // ,       (32 )
    if pubkey_bytes.len() != 32 {
        return Err("    32 ".into());
    }
    
    //      
    let pubkey_array: [u8; 32] = pubkey_bytes
        .try_into()
        .expect("Failed to convert to 32 byte array");
    
    //    mainnet
    let address = Address::new(
        Prefix::Mainnet,    // Mainnet
        Version::PubKey,    // P2PKH 
        &pubkey_array,      // 32-   
    );
    
    //    (Bech32 )
    Ok(address.to_string())
}

//    
fn is_valid_kaspa_address(address_str: &str) -> bool {
    let result = std::panic::catch_unwind(|| {
        Address::constructor(address_str)
    });
    result.is_ok()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    
    //     hex
    let pubkey_bytes = hex::decode(args.pubkey.trim())?;
    
    //    
    if pubkey_bytes.len() != 32 {
        eprintln!(" :     32  (64 hex-)");
        eprintln!("   : {}  ({} hex-)", pubkey_bytes.len(), args.pubkey.trim().len());
        std::process::exit(1);
    }
    
    println!(" : {}", args.pubkey.trim());
    println!(" : {} ", pubkey_bytes.len());
    
    //   ( )
    let address = generate_kaspa_address_from_pubkey(&pubkey_bytes)?;
    println!("\n : {}", address);
    println!(" : {} ", address.len());
    
    //     
    let parsed_address = Address::constructor(&address);
    println!("\n---   ---");
    println!(": {:?}", parsed_address.prefix);
    println!(": {:?}", parsed_address.version);
    println!(" : {}", hex::encode(&parsed_address.payload));
    println!("  : {} ", parsed_address.payload.len());
    
    // ,        
    //  as_slice()   SmallVec  &[u8]
    if parsed_address.payload.as_slice() == pubkey_bytes.as_slice() {
        println!("       ");
    } else {
        println!("        ");
        println!(": {}", hex::encode(&pubkey_bytes));
        println!(":  {}", hex::encode(&parsed_address.payload));
    }
    
    //   
    if is_valid_kaspa_address(&address) {
        println!("\n  !");
    } else {
        println!("\n  !");
        std::process::exit(1);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_address_generation() -> Result<(), Box<dyn std::error::Error>> {
        let pubkey = [0x42u8; 32]; //   
        let address = generate_kaspa_address_from_pubkey(&pubkey)?;
        
        // ,     "kaspa:"
        assert!(address.starts_with("kaspa:"));
        
        // ,    
        let parsed = Address::constructor(&address);
        assert_eq!(parsed.prefix, Prefix::Mainnet);
        assert_eq!(parsed.payload.len(), 32);
        
        // ,       
        assert_eq!(parsed.payload.as_slice(), pubkey.as_slice());
        
        Ok(())
    }
    
    #[test]
    fn test_invalid_pubkey_length() {
        let short_pubkey = [0x42u8; 31]; //   
        let result = generate_kaspa_address_from_pubkey(&short_pubkey);
        assert!(result.is_err());
        
        let long_pubkey = [0x42u8; 33]; //   
        let result = generate_kaspa_address_from_pubkey(&long_pubkey);
        assert!(result.is_err());
    }
}
