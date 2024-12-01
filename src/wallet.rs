use bitcoin::bip32::{ExtendedPubKey, ExtendedPrivKey};
use bitcoin::secp256k1::Secp256k1;
use bip39::Mnemonic;
use rand::rngs::OsRng;
use rand::RngCore;
use std::collections::HashSet;
use std::fs::{OpenOptions};
use std::io::{BufReader, Write};
use serde_json;

pub struct XpubManager {
    xpubs: HashSet<String>,
    file_path: String,
}

impl XpubManager {
    /// Create a new XpubManager instance with a given file path
    pub fn new(file_path: &str) -> Self {
        let mut manager = Self {
            xpubs: HashSet::new(),
            file_path: file_path.to_string(),
        };
        manager.load_from_file();
        manager
    }

    /// Check if there are any stored xpubs
    pub fn is_empty(&self) -> bool {
        self.xpubs.is_empty()
    }

    /// Get an iterator over the stored xpubs
    pub fn iter_xpubs(&self) -> impl Iterator<Item = &String> {
        self.xpubs.iter()
    }

    /// Get an xpub by index
    pub fn get_xpub_by_index(&self, index: usize) -> Option<&String> {
        self.xpubs.iter().nth(index)
    }

    /// Load xpubs from the file into memory
    fn load_from_file(&mut self) {
        let file = OpenOptions::new().read(true).open(&self.file_path);
        if let Ok(file) = file {
            let reader = BufReader::new(file);
            if let Ok(stored_xpubs) = serde_json::from_reader::<_, HashSet<String>>(reader) {
                self.xpubs = stored_xpubs;
            } else {
                println!("DEBUG: Failed to parse xpubs from file. Starting fresh.");
            }
        } else {
            println!("DEBUG: No existing xpub storage file found. Starting fresh.");
        }
    }

    /// Save xpubs from memory to the file
    pub fn save_to_file(&self) {
        println!("DEBUG: Saving xpubs to file...");
        let file_result = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.file_path);

        match file_result {
            Ok(mut file) => {
                if let Ok(json) = serde_json::to_string(&self.xpubs) {
                    if let Err(err) = file.write_all(json.as_bytes()) {
                        eprintln!("DEBUG: Error writing to file: {}", err);
                    } else {
                        println!("DEBUG: Successfully wrote xpubs to file.");
                    }
                } else {
                    println!("DEBUG: Failed to serialize xpubs to JSON.");
                }
            }
            Err(err) => eprintln!("DEBUG: Error opening file for writing: {}", err),
        }
    }

    /// Add an xpub to the manager
    pub fn add_xpub(&mut self, xpub: &str) -> bool {
        if !Self::validate_xpub(xpub) {
            println!("Invalid xpub format. Aborting addition.");
            return false;
        }

        if self.xpubs.insert(xpub.to_string()) {
            self.save_to_file();
            println!("xpub added successfully!");
            true
        } else {
            println!("xpub already exists!");
            false
        }
    }

    /// Validate the format of an xpub
    pub fn validate_xpub(xpub: &str) -> bool {
        xpub.parse::<ExtendedPubKey>().is_ok()
    }

    /// List all stored xpubs
    pub fn list_xpubs(&self) {
        if self.xpubs.is_empty() {
            println!("No xpubs stored.");
        } else {
            println!("Stored xpubs:");
            for xpub in &self.xpubs {
                println!("- {}", xpub);
            }
        }
    }

    /// Generate an address from an xpub using a specific index
    pub fn generate_address(&self, xpub: &str, index: u32) -> Option<String> {
        use bitcoin::bip32::{ChildNumber, DerivationPath, ExtendedPubKey};
        use bitcoin::Address;

        let extended_pubkey = xpub.parse::<ExtendedPubKey>().ok()?;
        let child_number = ChildNumber::from_normal_idx(index).ok()?;
        let derivation_path = DerivationPath::from(vec![child_number]);
        let child_key = extended_pubkey.derive_pub(&Secp256k1::new(), &derivation_path).ok()?;

        let bitcoin_public_key = bitcoin::PublicKey {
            inner: child_key.public_key,
            compressed: true,
        };

        let address = Address::p2wpkh(&bitcoin_public_key, bitcoin::Network::Bitcoin)
            .expect("Failed to generate address");

        Some(address.to_string())
    }
} // Closing the `impl XpubManager` block

/// Create a new wallet
pub fn create_wallet(manager: &mut XpubManager) {
    let mut entropy = [0u8; 16];
    OsRng.fill_bytes(&mut entropy);

    let mnemonic = Mnemonic::from_entropy(&entropy).expect("Failed to generate mnemonic");
    let seed = mnemonic.to_seed("");
    let xpriv = ExtendedPrivKey::new_master(bitcoin::Network::Bitcoin, &seed)
        .expect("Failed to generate extended private key");

    let secp = Secp256k1::new();
    let xpub = ExtendedPubKey::from_priv(&secp, &xpriv);

    println!("Mnemonic: {}", mnemonic.to_string());
    println!("xpub: {}", xpub);

    log::info!("Generated wallet with xpub: {}", xpub);

    if manager.add_xpub(&xpub.to_string()) {
        println!("xpub added to storage and saved to file!");
    }
}

/// Add an external xpub interactively
pub fn add_xpub_interactive(manager: &mut XpubManager) {
    println!("Enter xpub (or type 'exit' to quit):");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let xpub = input.trim();

        if xpub.eq_ignore_ascii_case("exit") {
            println!("Exiting add-xpub.");
            break;
        }

        if !XpubManager::validate_xpub(xpub) {
            println!("Invalid xpub format! Please try again.");
        } else {
            manager.add_xpub(xpub);
        }
    }
}
